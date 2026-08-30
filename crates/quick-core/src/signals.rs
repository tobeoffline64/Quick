use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_NODE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u64);

impl NodeId {
    pub fn next() -> Self {
        Self(NEXT_NODE_ID.fetch_add(1, Ordering::Relaxed))
    }
}

thread_local! {
    static CURRENT_OBSERVER: RefCell<Option<NodeId>> = const { RefCell::new(None) };
    static GRAPH: RefCell<ReactiveGraph> = RefCell::new(ReactiveGraph::new());
}

struct ReactiveGraph {
    subscribers: std::collections::HashMap<NodeId, HashSet<NodeId>>,
    effects: std::collections::HashMap<NodeId, Rc<dyn Fn()>>,
    batch_depth: usize,
    pending_effects: HashSet<NodeId>,
}

impl ReactiveGraph {
    fn new() -> Self {
        Self {
            subscribers: std::collections::HashMap::new(),
            effects: std::collections::HashMap::new(),
            batch_depth: 0,
            pending_effects: HashSet::new(),
        }
    }

    fn track(&mut self, signal_id: NodeId) {
        if let Some(observer) = CURRENT_OBSERVER.with(|o| *o.borrow()) {
            self.subscribers.entry(signal_id).or_default().insert(observer);
        }
    }

    fn notify(&mut self, signal_id: NodeId) -> Vec<(NodeId, Rc<dyn Fn()>)> {
        let mut to_run = Vec::new();
        if let Some(subs) = self.subscribers.get(&signal_id) {
            for &sub in subs {
                if self.batch_depth > 0 {
                    self.pending_effects.insert(sub);
                } else if let Some(effect) = self.effects.get(&sub) {
                    to_run.push((sub, effect.clone()));
                }
            }
        }
        to_run
    }
}

/// A fine-grained reactive state container.
#[derive(Clone)]
pub struct Signal<T: 'static> {
    id: NodeId,
    value: Rc<RefCell<T>>,
}

impl<T: 'static + Clone> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            id: NodeId::next(),
            value: Rc::new(RefCell::new(initial)),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Read the signal's value and track dependency in the current reactive context.
    pub fn get(&self) -> T {
        GRAPH.with(|g| g.borrow_mut().track(self.id));
        self.value.borrow().clone()
    }

    /// Read without cloning by borrowing.
    pub fn with<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        GRAPH.with(|g| g.borrow_mut().track(self.id));
        f(&self.value.borrow())
    }

    /// Read without registering a dependency.
    pub fn get_untracked(&self) -> T {
        self.value.borrow().clone()
    }

    /// Set a new value and trigger all dependent effects.
    pub fn set(&self, new_val: T) {
        *self.value.borrow_mut() = new_val;
        let effects = GRAPH.with(|g| g.borrow_mut().notify(self.id));
        for (sub, effect) in effects {
            CURRENT_OBSERVER.with(|o| {
                let prev = o.replace(Some(sub));
                effect();
                o.replace(prev);
            });
        }
    }

    /// Update value with a closure and trigger effects.
    pub fn update<F: FnOnce(&mut T)>(&self, f: F) {
        f(&mut self.value.borrow_mut());
        let effects = GRAPH.with(|g| g.borrow_mut().notify(self.id));
        for (sub, effect) in effects {
            CURRENT_OBSERVER.with(|o| {
                let prev = o.replace(Some(sub));
                effect();
                o.replace(prev);
            });
        }
    }
}

/// Creates a new reactive signal.
pub fn create_signal<T: 'static + Clone>(initial: T) -> (Signal<T>, Signal<T>) {
    let sig = Signal::new(initial);
    (sig.clone(), sig)
}

/// Creates a reactive effect that automatically re-runs when its tracked signals change.
pub fn create_effect<F: Fn() + 'static>(effect_fn: F) -> NodeId {
    let id = NodeId::next();
    let effect_rc: Rc<dyn Fn()> = Rc::new(effect_fn);

    GRAPH.with(|g| {
        g.borrow_mut().effects.insert(id, effect_rc.clone());
    });

    // Run once to register initial dependencies
    CURRENT_OBSERVER.with(|o| {
        let prev = o.replace(Some(id));
        effect_rc();
        o.replace(prev);
    });

    id
}

/// Creates a computed / derived value that automatically updates when its dependencies update.
pub fn create_computed<T: 'static + Clone, F: Fn() -> T + 'static>(calc_fn: F) -> Signal<T> {
    let calc_rc = Rc::new(calc_fn);
    let initial = calc_rc();
    let signal = Signal::new(initial);
    let sig_clone = signal.clone();

    create_effect(move || {
        let new_val = calc_rc();
        sig_clone.set(new_val);
    });

    signal
}

/// Batch multiple signal writes to notify subscribers only once at the end.
pub fn batch<R, F: FnOnce() -> R>(f: F) -> R {
    GRAPH.with(|g| g.borrow_mut().batch_depth += 1);
    let res = f();
    let to_run: Vec<(NodeId, Rc<dyn Fn()>)> = GRAPH.with(|g| {
        let mut graph = g.borrow_mut();
        if graph.batch_depth > 0 {
            graph.batch_depth -= 1;
        }
        if graph.batch_depth == 0 {
            let pending = std::mem::take(&mut graph.pending_effects);
            pending
                .into_iter()
                .filter_map(|sub| graph.effects.get(&sub).cloned().map(|e| (sub, e)))
                .collect()
        } else {
            Vec::new()
        }
    });
    for (sub, effect) in to_run {
        CURRENT_OBSERVER.with(|o| {
            let prev = o.replace(Some(sub));
            effect();
            o.replace(prev);
        });
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_basic() {
        let count = Signal::new(0);
        assert_eq!(count.get(), 0);
        count.set(5);
        assert_eq!(count.get(), 5);
        count.update(|v| *v += 10);
        assert_eq!(count.get(), 15);
    }

    #[test]
    fn test_signal_with_borrow() {
        let text = Signal::new("Hello Quick".to_string());
        let len = text.with(|s| s.len());
        assert_eq!(len, 11);
    }

    #[test]
    fn test_create_signal_helper() {
        let (getter, setter) = create_signal(42);
        assert_eq!(getter.get(), 42);
        setter.set(100);
        assert_eq!(getter.get(), 100);
    }

    #[test]
    fn test_computed_signal() {
        let a = Signal::new(2);
        let b = Signal::new(3);
        let a_cl = a.clone();
        let b_cl = b.clone();
        let sum = create_computed(move || a_cl.get() + b_cl.get());

        assert_eq!(sum.get(), 5);
        a.set(10);
        assert_eq!(sum.get(), 13);
        b.set(20);
        assert_eq!(sum.get(), 30);
    }

    #[test]
    fn test_signal_batching() {
        let a = Signal::new(1);
        let b = Signal::new(10);
        let run_count = Rc::new(RefCell::new(0));

        let a_cl = a.clone();
        let b_cl = b.clone();
        let rc_cl = run_count.clone();

        create_effect(move || {
            let _ = a_cl.get() + b_cl.get();
            *rc_cl.borrow_mut() += 1;
        });

        // initial run
        assert_eq!(*run_count.borrow(), 1);

        batch(|| {
            a.set(2);
            b.set(20);
        });

        // effect should run only once for the batch
        assert_eq!(*run_count.borrow(), 2);
    }

    #[test]
    fn test_nested_signal_batching() {
        let a = Signal::new(1);
        let b = Signal::new(10);
        let c = Signal::new(100);
        let run_count = Rc::new(RefCell::new(0));

        let a_cl = a.clone();
        let b_cl = b.clone();
        let c_cl = c.clone();
        let rc_cl = run_count.clone();

        create_effect(move || {
            let _ = a_cl.get() + b_cl.get() + c_cl.get();
            *rc_cl.borrow_mut() += 1;
        });

        assert_eq!(*run_count.borrow(), 1);

        batch(|| {
            a.set(2);
            batch(|| {
                b.set(20);
                c.set(200);
            });
            a.set(3);
        });

        // Nested batch should NOT flush early; effect should run only ONCE for the entire outer batch
        assert_eq!(*run_count.borrow(), 2);
    }

    #[test]
    fn test_signal_untracked() {
        let a = Signal::new(5);
        assert_eq!(a.get_untracked(), 5);
        a.set(15);
        assert_eq!(a.get_untracked(), 15);
    }

    #[test]
    fn test_chained_computed_signals() {
        let root = Signal::new(1);
        let r1 = root.clone();
        let s2 = create_computed(move || r1.get() * 2);
        let s2_cl = s2.clone();
        let s3 = create_computed(move || s2_cl.get() + 10);
        let s3_cl = s3.clone();
        let s4 = create_computed(move || format!("Value: {}", s3_cl.get()));

        assert_eq!(s4.get(), "Value: 12");
        root.set(5);
        assert_eq!(s4.get(), "Value: 20");
    }
}
