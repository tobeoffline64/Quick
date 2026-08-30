# E2E Test Infra: Quick UI Framework — Material You (M3)

## Test Philosophy
- **Opaque-Box & Requirement-Driven**: Tests derive directly from `ORIGINAL_REQUEST.md` and `material_you_full_theme_and_component_integration.md` specifications, exercising the public APIs, declarative markup, and widget contracts without coupling to private internal structures.
- **Methodology**: Category-Partition + Boundary Value Analysis (BVA) + Pairwise Combinatorial Testing + Real-World Workload Testing.
- **Headless Execution**: Uses in-memory rendering canvas and mock event dispatches, running 100% reliably in CI and headless environments.

---

## Feature Inventory & Test Matrix

| # | Feature | Requirement Source | Tier 1 (Feature) | Tier 2 (Boundary) | Tier 3 (Cross-Feature) | Tier 4 (Scenario) |
|---|---------|-------------------|:----------------:|:-----------------:|:----------------------:|:-----------------:|
| 1 | Pure Rust CAM16 & HCT Color Space | Spec §2 | 5 tests | 5 tests | ✓ | ✓ |
| 2 | Tone-Preserving Gamut Solver | Spec §2.3 | 5 tests | 5 tests | ✓ | ✓ |
| 3 | Dynamic Contrast & Tone Inversion | Spec §2.3 | 5 tests | 5 tests | ✓ | ✓ |
| 4 | 6 Tonal Palettes Generation | Spec §3 | 5 tests | 5 tests | ✓ | ✓ |
| 5 | 7 Dynamic Scheme Variants | Spec §3.1 | 7 tests | 5 tests | ✓ | ✓ |
| 6 | 32+ M3 Color Roles (Light & Dark) | Spec §4 | 6 tests | 5 tests | ✓ | ✓ |
| 7 | Design Tokens (Shapes, Elevation, State) | Spec §5 | 5 tests | 5 tests | ✓ | ✓ |
| 8 | Dynamic `ThemePackage` API | Spec §8 | 5 tests | 5 tests | ✓ | ✓ |
| 9 | M3 Button Component (5 variants) | Spec §6.1 | 5 tests | 5 tests | ✓ | ✓ |
| 10 | M3 Card Component (3 variants + dual shadows) | Spec §6.2 | 5 tests | 5 tests | ✓ | ✓ |
| 11 | M3 Switch Selection Control | Spec §6.3 | 5 tests | 5 tests | ✓ | ✓ |
| 12 | M3 Checkbox Selection Control | Spec §6.4 | 5 tests | 5 tests | ✓ | ✓ |
| 13 | M3 Slider Selection Control | Spec §6.5 | 5 tests | 5 tests | ✓ | ✓ |
| 14 | M3 Chip Selection Control (4 variants) | Spec §6.6 | 5 tests | 5 tests | ✓ | ✓ |
| 15 | M3 ProgressBar Component (determinate & indeterminate) | Spec §6.7 | 5 tests | 5 tests | ✓ | ✓ |
| 16 | M3 TextInput Component (Filled & Outlined) | Spec §6.8 | 5 tests | 5 tests | ✓ | ✓ |
| 17 | Declarative `.quick` Markup Integration | Spec §7 | 5 tests | 5 tests | ✓ | ✓ |
| 18 | Live Showcase & Full Integration | Spec §11 | 5 tests | 5 tests | ✓ | ✓ |

---

## Test Architecture
- **Location**: `tests/` directory (e.g., `tests/e2e_m3_theme.rs`, `tests/e2e_m3_widgets.rs`, `tests/e2e_m3_markup.rs`, `tests/e2e_m3_scenarios.rs`)
- **Invocation Command**: `cargo test --test e2e_m3_theme --test e2e_m3_widgets --test e2e_m3_markup --test e2e_m3_scenarios` (and included in `cargo test --workspace`)
- **Pass/Fail Semantics**: Standard Rust `#[test]` assertions with detailed error messages on failure, zero panics, exit code 0.

---

## Real-World Application Scenarios (Tier 4)
| # | Scenario | Features Exercised | Complexity |
|---|----------|--------------------|------------|
| 1 | Dynamic Wallpaper Theme Switching (Light/Dark mode + 7 Scheme Variants) | F1, F2, F4, F5, F6, F8, F17 | High |
| 2 | Material 3 Settings Form (Switches, Checkboxes, Sliders, TextInputs, Filled Buttons) | F9, F11, F12, F13, F16, F17 | High |
| 3 | Filterable Card Dashboard with Chips and Elevated Cards | F10, F14, F7, F9, F17 | High |
| 4 | Asynchronous Task Manager with Determinate & Indeterminate ProgressBars | F15, F9, F10, F14, F17 | Medium |
| 5 | End-to-End Declarative `.quick` Markup UI Compilation and Event Dispatch | F9, F10, F11, F12, F13, F14, F15, F16, F17, F18 | High |

---

## Coverage Thresholds
- **Tier 1 (Feature Coverage)**: $\ge 90$ test cases across all 18 features (happy-path isolation).
- **Tier 2 (Boundary & Corner Cases)**: $\ge 90$ test cases (extremes: tone 0/100, chroma 0/max, empty text, negative slider values, unknown scheme variants, invalid hex strings).
- **Tier 3 (Cross-Feature Combinations)**: $\ge 20$ pairwise interaction test cases.
- **Tier 4 (Real-World Scenarios)**: $\ge 5$ end-to-end composite workload tests.
- **Total Minimum Target**: $\ge 205$ comprehensive assertions/tests across the test suite.
