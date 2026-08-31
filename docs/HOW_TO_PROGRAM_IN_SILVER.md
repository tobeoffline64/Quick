# How to Program in Silver (`.silver`)
*The Complete Guide to Building Reactive Applications with Silver and Quick*

---

## Welcome to Silver!

**Silver** is a modern, reactivity-first programming language designed specifically for building fast, beautiful graphical applications on the **Quick UI Framework**. 

Whether you are building simple tools, high-performance desktop apps, or embedded touchscreens, Silver gives you the expressiveness of modern languages (like Swift and TypeScript) with the instant execution speed of a lightweight virtual machine and the native rendering power of Rust and GPU-accelerated Vello.

---

## Quickstart: Running Your First Silver Program

### 1. Using the Silver CLI
Install or build the `silver` CLI runner:
```bash
cargo build --release -p silver-cli
```

### 2. Running an Interactive Expression (Eval)
```bash
silver eval "let a = 15; let b = 27; a + b"
# Output: 42
```

### 3. Running a `.silver` Script File
Create a file named `hello.silver`:
```silver
// hello.silver
let name = "Developer"
signal clicks = 0

computed greeting = "Hello, \(name)! You clicked \(clicks) times."

fn click() {
    clicks += 1
}

click()
click()
greeting
```

Run it with:
```bash
silver run hello.silver
# Output: => Hello, Developer! You clicked 2 times.
```

---

## Chapter 1: Variables & Constants

Silver distinguishes between immutable constants (`let`) and mutable variables (`var`):

```silver
// Immutable constant (cannot be reassigned)
let app_name = "Quick Studio"
let max_width: int = 1920

// Mutable local variable
var current_tab = 1
current_tab = 2 // OK!
```

### Type Annotations (Optional)
Silver automatically infers types, but you can provide explicit type annotations:
```silver
let version: string = "1.0.0"
let target_fps: int = 120
let scaling: float = 1.25
let dark_mode: bool = true
let accent: color = #6750A4
```

---

## Chapter 2: Reactive State (`signal`, `computed`, `effect`)

Reactivity is the heart of Silver. Any variable declared with `signal` automatically connects to the Quick UI reactivity graph.

### 2.1 Declaring Signals
A `signal` holds mutable reactive state:
```silver
signal count: int = 0
signal is_running: bool = false
signal volume: float = 80.0
```

Mutating a signal triggers immediate updates to all dependent UI widgets:
```silver
count += 1
is_running = true
volume = 95.5
```

### 2.2 Computed Signals
A `computed` signal derives its value from other signals. It automatically recalculates whenever its dependencies change:

```silver
signal count = 0
signal item_price = 25.0

// Automatically recalculates when `count` or `item_price` updates!
computed total_cost = count * item_price
computed status_text = if count == 0 {
    "Your cart is empty."
} else {
    "Cart: \(count) items — Total: $\(total_cost)"
}
```

### 2.3 Reactive Effects
An `effect` executes side effects whenever the signals it reads change:
```silver
effect {
    print("State updated! Current count:", count)
}
```

---

## Chapter 3: Data Types & Literals

### 3.1 Numbers & Math
Silver supports integers (`int`) and floating-point numbers (`float`):
```silver
let width = 1280
let aspect_ratio = 16.0 / 9.0
let padding = (width - 1000) / 2
```

### 3.2 Strings & String Interpolation `\(...)`
Embed any expression directly into a string using `\(...)`:
```silver
let user = "Alice"
signal score = 950

let summary = "Player: \(user) | High Score: \(score * 2) pts"
```

### 3.3 First-Class Hex Color Literals `#RRGGBB[AA]`
Colors are native literals:
```silver
let primary = #6750A4           // M3 Purple
let surface = #1C1B1F           // Dark Surface
let semi_transparent = #FFFFFF80 // 50% Opacity White
let red = #FF0000
```

### 3.4 Collections: Lists & Maps
```silver
// Lists (Dynamic Arrays)
let colors = [#FF0000, #00FF00, #0000FF]
let tags = ["Rust", "Silver", "Vello", "Quick"]

// Maps (Key-Value Dictionaries)
let settings = {
    "theme": "material-you",
    "vsync": true,
    "fps_cap": 120
}
```

---

## Chapter 4: Control Flow

### 4.1 `if` / `else` (Statements and Expressions)
In Silver, `if` / `else` can be used as both statements and inline expressions:

```silver
// Expression (returns a value)
let theme_color = if is_dark { #1C1B1F } else { #FEF7FF }

// Statement
if count > 100 {
    print("Milestone reached!")
} else if count > 50 {
    print("Halfway there!")
} else {
    print("Keep going!")
}
```

### 4.2 Loops (`while` and `for ... in`)
```silver
// While loop
var i = 0
while i < 5 {
    print("Step:", i)
    i += 1
}

// For-in loop
for item in [10, 20, 30, 40] {
    print("Item:", item)
}
```

---

## Chapter 5: Functions

Functions are defined with the `fn` keyword:

```silver
// Simple function
fn greet(name: string) -> string {
    return "Welcome to Silver, \(name)!"
}

// Action handler for UI buttons
fn increment_counter() {
    count += 1
}

// Function with parameters and calculations
fn calculate_discount(price: float, discount_percent: float) -> float {
    let factor = 1.0 - (discount_percent / 100.0)
    return price * factor
}
```

---

## Chapter 6: Connecting Silver to Quick (`.quick` + `.silver`)

The true superpower of Silver is how seamlessly it powers **Quick declarative UI markup** (`.quick`).

### Application Architecture
Every Quick application is composed of two companion files:
1. `app.silver`: Contains all reactive state (`signal`, `computed`) and event handling logic (`fn`).
2. `app.quick`: Contains the declarative Material You XML layout that displays state (`$signal`) and connects user gestures (`on_click="func"`).

```
  app.silver (Logic & Signals)        app.quick (Layout & Styling)
 ┌────────────────────────────┐      ┌─────────────────────────────┐
 │ signal count = 0           │ <──> │ <Text content="$count" />   │
 │ fn increment() { count+=1 }│ <──> │ <Button on_click="increment"│
 └────────────────────────────┘      └─────────────────────────────┘
```

---

## Chapter 7: Complete Real-World Tutorials

### Tutorial 1: Material You Reactive Counter

#### `app.silver`
```silver
// Reactive State
signal count: int = 0
signal is_active: bool = true

// Derived State
computed count_text = "Current Tally: \(count)"
computed banner = if count == 0 {
    "Tap Increment to begin counting."
} else {
    "You have pressed the button \(count) times!"
}

// Actions
fn increment() {
    count += 1
}

fn reset() {
    count = 0
}

fn toggle_active() {
    is_active = !is_active
}
```

#### `app.quick`
```xml
<Window title="Silver Counter" width="600" height="450" theme="material-you">
    <Stack direction="vertical" spacing="20" padding="24">
        <Card variant="elevated" padding="24">
            <Stack direction="vertical" spacing="12">
                <Text content="$count_text" font_size="28" />
                <Text content="$banner" font_size="16" />
                
                <Stack direction="horizontal" spacing="12">
                    <Button variant="filled" label="Increment (+1)" on_click="increment" />
                    <Button variant="tonal" label="Reset (0)" on_click="reset" />
                </Stack>
            </Stack>
        </Card>

        <Card variant="outlined" padding="16">
            <Stack direction="horizontal" spacing="16">
                <Text content="Enable Tracking" font_size="16" />
                <Switch checked="$is_active" on_toggle="toggle_active" />
            </Stack>
        </Card>
    </Stack>
</Window>
```

---

### Tutorial 2: Dynamic System Monitor & Theme Controller

#### `dashboard.silver`
```silver
signal cpu_usage: float = 34.5
signal gpu_enabled: bool = true
signal brightness: float = 80.0
signal theme_seed: color = #6750A4

computed status_summary = "System Load: \(cpu_usage)% | Brightness: \(brightness)%"
computed performance_tier = if cpu_usage > 80.0 {
    "Warning: High Load"
} else {
    "Optimal Performance"
}

fn boost_performance() {
    gpu_enabled = true
    brightness = 100.0
}

fn energy_saver() {
    gpu_enabled = false
    brightness = 40.0
}
```

#### `dashboard.quick`
```xml
<Window title="System Dashboard" width="700" height="500" theme="material-you">
    <Stack direction="vertical" spacing="16" padding="24">
        <Card variant="elevated" padding="20">
            <Stack direction="vertical" spacing="12">
                <Text content="Hardware Control Center" font_size="22" />
                <Text content="$status_summary" font_size="15" />
                <Text content="$performance_tier" font_size="14" />
                
                <Slider value="$brightness" min="10" max="100" />
            </Stack>
        </Card>

        <Stack direction="horizontal" spacing="12">
            <Button variant="filled" label="Max Performance" on_click="boost_performance" />
            <Button variant="outlined" label="Energy Saver" on_click="energy_saver" />
        </Stack>
    </Stack>
</Window>
```

---

## Chapter 8: Syntax Quick Reference & Cheat Sheet

| Feature | Silver Syntax Example |
| :--- | :--- |
| **Constant** | `let pi = 3.14159` |
| **Variable** | `var score = 100` |
| **Signal (State)** | `signal count: int = 0` |
| **Computed (Derived)** | `computed label = "Count: \(count)"` |
| **Effect** | `effect { print(count) }` |
| **Function** | `fn add(a: int, b: int) -> int { return a + b }` |
| **If-Expression** | `let val = if active { "On" } else { "Off" }` |
| **String Interpolation**| `"Value is: \(x + y)"` |
| **Hex Color** | `#6750A4`, `#1C1B1F`, `#FFFFFF80` |
| **List Literal** | `[1, 2, 3, 4, 5]` |
| **Map Literal** | `{"key": "value", "id": 42}` |
| **Markup Signal Bind** | `<Text content="$label" />` |
| **Markup Action Bind** | `<Button on_click="increment" />` |
