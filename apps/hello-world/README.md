# ⚡ Quick Hello World Application Project

This is a standalone starter project built with the **Quick** native UI framework.

---

## 📁 Project Structure

```text
apps/hello-world/
├── Cargo.toml          # Rust package configuration
├── README.md           # Project guide
├── run.sh              # Quick launch script
├── app.quick           # Declarative UI markup with embedded CSS & reactive bindings
└── src/
    └── main.rs         # Application entrypoint & reactive state binding
```

---

## 🚀 How to Run

### Run with Cargo:
```bash
cargo run -p hello-world
```

### Run with script:
```bash
./run.sh
```

### Build for Release:
```bash
cargo build --release -p hello-world
```

---

## 🛠️ How to Customize

1. **Modify UI Markup & Styling**:
   Edit `app.quick` to change layout, add new widgets, or tweak the CSS styles inside `<Style>...</Style>`.
2. **Add Reactive Signals & Business Logic**:
   Edit `src/main.rs` to declare new `Signal<T>` reactive states and bind them to the UI via `data_ctx.bind_signal()` and `data_ctx.bind_action()`.
