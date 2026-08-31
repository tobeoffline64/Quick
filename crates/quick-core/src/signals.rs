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
            let is_active = GRAPH.with(|g| g.borrow().effects.contains_key(&sub));
            if is_active {
                CURRENT_OBSERVER.with(|o| {
                    let prev = o.replace(Some(sub));
                    effect();
                    o.replace(prev);
                });
            }
        }
    }

    /// Update value with a closure and trigger effects.
    pub fn update<F: FnOnce(&mut T)>(&self, f: F) {
        f(&mut self.value.borrow_mut());
        let effects = GRAPH.with(|g| g.borrow_mut().notify(self.id));
        for (sub, effect) in effects {
            let is_active = GRAPH.with(|g| g.borrow().effects.contains_key(&sub));
            if is_active {
                CURRENT_OBSERVER.with(|o| {
                    let prev = o.replace(Some(sub));
                    effect();
                    o.replace(prev);
                });
            }
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

/// Disposes a reactive effect, preventing it from running on future signal updates.
pub fn dispose_effect(id: NodeId) {
    GRAPH.with(|g| {
        let mut graph = g.borrow_mut();
        graph.effects.remove(&id);
        for subs in graph.subscribers.values_mut() {
            subs.remove(&id);
        }
        graph.pending_effects.remove(&id);
    });
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
        let is_active = GRAPH.with(|g| g.borrow().effects.contains_key(&sub));
        if is_active {
            CURRENT_OBSERVER.with(|o| {
                let prev = o.replace(Some(sub));
                effect();
                o.replace(prev);
            });
        }
    }
    res
}

/// DataContext provides a dynamic mapping of reactive signals and UI event action handlers.
#[derive(Default, Clone)]
pub struct DataContext {
    pub string_signals: std::collections::HashMap<String, Signal<String>>,
    pub bool_signals: std::collections::HashMap<String, Signal<bool>>,
    pub f32_signals: std::collections::HashMap<String, Signal<f32>>,
    pub action_handlers: std::collections::HashMap<String, Rc<RefCell<dyn FnMut()>>>,
}

impl DataContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_signal(&mut self, name: impl Into<String>, signal: Signal<String>) {
        self.string_signals.insert(name.into(), signal);
    }

    pub fn bind_bool_signal(&mut self, name: impl Into<String>, signal: Signal<bool>) {
        self.bool_signals.insert(name.into(), signal);
    }

    pub fn bind_f32_signal(&mut self, name: impl Into<String>, signal: Signal<f32>) {
        self.f32_signals.insert(name.into(), signal);
    }

    pub fn bind_action<F: FnMut() + 'static>(&mut self, name: impl Into<String>, handler: F) {
        self.action_handlers.insert(name.into(), Rc::new(RefCell::new(handler)));
    }
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

    #[test]
    fn test_dispose_effect() {
        let count = Signal::new(1);
        let run_count = Rc::new(RefCell::new(0));

        let c_cl = count.clone();
        let rc_cl = run_count.clone();
        let effect_id = create_effect(move || {
            let _ = c_cl.get();
            *rc_cl.borrow_mut() += 1;
        });

        assert_eq!(*run_count.borrow(), 1);

        count.set(2);
        assert_eq!(*run_count.borrow(), 2);

        dispose_effect(effect_id);

        count.set(3);
        // Effect should not run after disposal
        assert_eq!(*run_count.borrow(), 2);
    }

    #[test]
    fn test_diamond_computed_signals() {
        // A -> B = A * 2, A -> C = A + 10, D = B + C
        let a = Signal::new(1);
        let a1 = a.clone();
        let a2 = a.clone();

        let b = create_computed(move || a1.get() * 2);
        let c = create_computed(move || a2.get() + 10);

        let b_cl = b.clone();
        let c_cl = c.clone();
        let d = create_computed(move || b_cl.get() + c_cl.get());

        // Initial: a=1 => b=2, c=11 => d=13
        assert_eq!(d.get(), 13);

        // Update a=5 => b=10, c=15 => d=25
        a.set(5);
        assert_eq!(d.get(), 25);
    }

    #[test]
    fn test_cascading_effect_disposal() {
        let trigger = Signal::new(0);
        let b_ran = Rc::new(RefCell::new(0));

        let b_id_holder: Rc<RefCell<Option<NodeId>>> = Rc::new(RefCell::new(None));
        let b_id_for_a = b_id_holder.clone();

        let trig_a = trigger.clone();
        create_effect(move || {
            let val = trig_a.get();
            if val > 0 {
                if let Some(b_id) = *b_id_for_a.borrow() {
                    dispose_effect(b_id);
                }
            }
        });

        let trig_b = trigger.clone();
        let b_ran_cl = b_ran.clone();
        let b_id = create_effect(move || {
            let _ = trig_b.get();
            *b_ran_cl.borrow_mut() += 1;
        });
        *b_id_holder.borrow_mut() = Some(b_id);

        let initial_runs = *b_ran.borrow();
        assert_eq!(initial_runs, 1);

        // Update trigger -> effect A disposes effect B
        trigger.set(1);
        let runs_after_set1 = *b_ran.borrow();

        // Subsequent update -> effect B is disposed and must not run
        trigger.set(2);
        assert_eq!(*b_ran.borrow(), runs_after_set1);
    }

    #[test]
    fn test_deep_chained_computed_stress() {
        let root = Signal::new(1);
        let mut prev = root.clone();

        for _ in 0..50 {
            let p_cl = prev.clone();
            prev = create_computed(move || p_cl.get() + 1);
        }

        assert_eq!(prev.get(), 51);

        root.set(10);
        assert_eq!(prev.get(), 60);
    }

    #[test]
    fn test_high_volume_signal_batching_stress() {
        let sig = Signal::new(0);
        let run_count = Rc::new(RefCell::new(0));

        let s_cl = sig.clone();
        let rc_cl = run_count.clone();
        create_effect(move || {
            let _ = s_cl.get();
            *rc_cl.borrow_mut() += 1;
        });

        assert_eq!(*run_count.borrow(), 1);

        batch(|| {
            for i in 1..=5000 {
                sig.set(i);
            }
        });

        assert_eq!(sig.get(), 5000);
        assert_eq!(*run_count.borrow(), 2);
    }
}
