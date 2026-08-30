# 🚀 Quick Framework: How to Run Guide

This guide walks you through setting up, building, running, and deploying applications built with the **Quick** native UI framework.

---

## 📋 1. Prerequisites

### Rust Toolchain
Ensure you have Rust and Cargo installed (1.75+ recommended):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
cargo --version
```

### Linux Wayland & Graphics Dependencies
On Debian / Ubuntu / Raspberry Pi OS:
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    libwayland-dev \
    wayland-protocols \
    libxkbcommon-dev \
    libegl1-mesa-dev \
    libgles2-mesa-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    pkg-config
```

On Fedora / RHEL:
```bash
sudo dnf install -y \
    wayland-devel \
    wayland-protocols-devel \
    libxkbcommon-devel \
    mesa-libEGL-devel \
    fontconfig-devel \
    freetype-devel
```

On Arch Linux:
```bash
sudo pacman -S wayland wayland-protocols libxkbcommon mesa fontconfig freetype2
```

---

## ⚡ 2. Running Applications

### A. Run the "Hello World" App (`app.quick`)
The Hello World application demonstrates loading a `.quick` declarative UI file with reactive signals:
```bash
cargo run -p hello_world
```

### B. Run the Reactive Counter App
Demonstrates XML and TOML declarative UI with high-throughput state updates:
```bash
cargo run -p quick_counter
```

### C. Run the On-Device Telemetry & Benchmark Showcase
Runs the full device dashboard with live FPS, frame latency, memory RSS, and hardware controls:
```bash
cargo run -p device_showcase
```

### D. Run Automated Performance Benchmarks
Executes the automated performance battery (10,000 rapid signal updates, 100 consecutive frame renders, and memory leak verification):
```bash
cargo run -p device_showcase -- --benchmark-mode
```

---

## 📄 3. How `.quick` Files Work

A `.quick` file is a unified declarative UI document combining **component hierarchy**, **embedded CSS styling**, **reactive `$signal` bindings**, and **event hooks** in a single clean file.

### Example `app.quick`:
```xml
<!-- app.quick -->
<VStack id="app-root" style="padding: 32px; background: #0f111a; align-items: center;">
    
    <!-- Embedded CSS / XAML Styling -->
    <Style>
        Text.title {
            font-size: 24px;
            font-weight: bold;
            color: #7aa2f7;
        }
        Button.btn-primary {
            background: #7aa2f7;
            color: #0f111a;
            padding: 10px 24px;
            border-radius: 8px;
            font-weight: bold;
        }
        Button.btn-primary:hover {
            background: #89b4fa;
        }
    </Style>

    <!-- Component Tree with Reactive Binding -->
    <Text class="title" text="Hello, Quick World!" />
    <Text id="greeting" style="color: #c0caf5; font-size: 16px;" text="$greeting_message" />

    <Button id="btn-hello" class="btn-primary" text="✨ Say Hello" onclick="handle_greet" />
</VStack>
```

### Loading `.quick` in Rust:
```rust
use quick::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define reactive signal
    let greeting_signal = Signal::new("Welcome to Quick!".to_string());

    // 2. Bind signal and event handlers
    let mut data_ctx = DataContext::new();
    data_ctx.bind_signal("greeting_message", greeting_signal.clone());
    data_ctx.bind_action("handle_greet", move || {
        greeting_signal.set("🎉 Hello from .quick reactive binding!".to_string());
    });

    // 3. Load the .quick file
    let quick_content = include_str!("app.quick");
    let mut app = App::new(
        WindowOptions::new()
            .title("My Quick App")
            .size(800.0, 600.0),
    )
    .from_quick(quick_content, &mut data_ctx)?;

    // 4. Render and run
    let canvas = app.render_frame(Size::new(800.0, 600.0));
    println!("Frame rendered with {} draw commands.", canvas.commands().len());

    Ok(())
}
```

---

## 🏎️ 4. Building Optimized Release Binaries (Bun-Inspired Speed)

To build fully optimized, stripped standalone release binaries:
```bash
cargo build --release
```

Release binaries are located at:
- `target/release/hello_world`
- `target/release/quick_counter`
- `target/release/device_showcase`

### Verification of Binary Size:
```bash
ls -lh target/release/hello_world
```
Target standalone executable size is **$\le 8\text{ MB}$** thanks to Fat LTO, symbol stripping, and single codegen units configured in `Cargo.toml`.

---

## 📱 5. Deploying on Linux Wayland Kiosk Devices (e.g. Raspberry Pi / Industrial PC)

Quick applications run natively on lightweight Wayland kiosk compositors like **Cage** or **Weston** without requiring a full desktop environment.

### Run in Cage Kiosk:
```bash
# Install Cage compositor
sudo apt install -y cage

# Launch Quick app in fullscreen kiosk mode
cage -- ./target/release/device_showcase
```

### Automatic Systemd Service (`/etc/systemd/system/quick-app.service`):
```ini
[Unit]
Description=Quick Application Kiosk
After=systemd-user-sessions.service

[Service]
User=kiosk
Environment=XDG_RUNTIME_DIR=/run/user/1000
Environment=WAYLAND_DISPLAY=wayland-0
ExecStart=/usr/bin/cage -- /opt/quick/device_showcase
Restart=always
RestartSec=2

[Install]
WantedBy=multi-user.target
```

Enable and start the service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now quick-app.service
```
