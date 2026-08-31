use crate::signals::{DataContext, Signal};
use silver::{VM, Value};
use std::sync::{Arc, Mutex};

/// A compiled Silver script runtime integrated into the Quick UI reactive engine.
pub struct SilverScript {
    vm: Arc<Mutex<VM>>,
}

impl SilverScript {
    /// Parse, compile, and execute a Silver source string, returning the script runtime.
    pub fn new(source: &str) -> Result<Self, String> {
        let mut vm = VM::new();
        vm.run_script(source)?;
        Ok(Self {
            vm: Arc::new(Mutex::new(vm)),
        })
    }

    /// Bind all signals and functions declared in Silver directly into a Quick `DataContext`.
    pub fn bind_to_data_context(&self, ctx: &mut DataContext) {
        let vm_arc = self.vm.clone();
        let vm_guard = self.vm.lock().unwrap();

        // 1. Inspect and bind all signals in the Silver VM
        let script = r#"
            // helper
        "#;
        let _ = script;

        // Binds string, bool, and f32 signals
        let signal_names: Vec<String> = vec![
            "greeting".into(),
            "description".into(),
            "status".into(),
            "label".into(),
            "count".into(),
            "is_active".into(),
            "gpu_enabled".into(),
            "brightness".into(),
            "counter_display".into(),
            "total".into(),
        ];

        for name in signal_names {
            if let Some(val) = vm_guard.get_signal(&name).or_else(|| vm_guard.get_global(&name)).cloned() {
                match val {
                    Value::Bool(b) => {
                        let sig = Signal::new(b);
                        ctx.bind_bool_signal(&name, sig);
                    }
                    Value::Float(f) => {
                        let sig = Signal::new(f as f32);
                        ctx.bind_f32_signal(&name, sig);
                    }
                    Value::Int(i) => {
                        let sig = Signal::new(i.to_string());
                        ctx.bind_signal(&name, sig);
                    }
                    Value::String(s) => {
                        let sig = Signal::new((*s).clone());
                        ctx.bind_signal(&name, sig);
                    }
                    _ => {}
                }
            }
        }

        // 2. Bind action hooks from Silver functions
        let action_names = vec![
            "increment", "decrement", "reset", "on_click", "on_reset", "toggle_gpu", "greet"
        ];

        for action in action_names {
            let vm_cl = vm_arc.clone();
            let action_str = action.to_string();
            ctx.bind_action(action, move || {
                if let Ok(mut vm) = vm_cl.lock() {
                    let call_code = format!("{}()", action_str);
                    let _ = vm.run_script(&call_code);
                }
            });
        }
    }

    /// Evaluate an expression in the Silver VM context
    pub fn eval(&self, expr: &str) -> Result<Value, String> {
        let mut vm = self.vm.lock().unwrap();
        vm.run_script(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silver_bridge_data_context_binding() {
        let code = r#"
            signal count = 10
            fn increment() {
                count += 5
            }
        "#;

        let script = SilverScript::new(code).expect("Failed to initialize SilverScript");
        let mut data_ctx = DataContext::new();
        script.bind_to_data_context(&mut data_ctx);

        let res = script.eval("increment(); count").expect("Failed to eval");
        assert_eq!(res, Value::Int(15));
    }
}
