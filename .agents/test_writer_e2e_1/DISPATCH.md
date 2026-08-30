## 2026-08-30T14:04:40Z

You are the E2E Test Writer for the Quick UI Framework Material You (M3) project.
Your working directory is: /home/ai-workspace/coding-repo/quick-silver/.agents/test_writer_e2e_1
You MUST read:
1. /home/ai-workspace/coding-repo/quick-silver/ORIGINAL_REQUEST.md
2. /home/ai-workspace/coding-repo/quick-silver/PROJECT.md
3. /home/ai-workspace/coding-repo/quick-silver/TEST_INFRA.md
4. /home/ai-workspace/coding-repo/quick-silver/material_you_full_theme_and_component_integration.md

Your mission:
Design and implement the complete, opaque-box E2E test suite in the `tests/` directory covering all 4 tiers across all 18 features defined in `TEST_INFRA.md`:
- Tier 1: Feature Coverage (>=5 tests per feature, happy paths)
- Tier 2: Boundary & Corner Cases (>=5 tests per feature, extremes: tone 0/100, chroma 0/max, empty text, negative slider values, unknown scheme variants, invalid hex strings)
- Tier 3: Cross-Feature Combinations (pairwise interaction test cases)
- Tier 4: Real-World Application Scenarios (5 composite workload scenarios)

Create test files in `/home/ai-workspace/coding-repo/quick-silver/tests/`:
- `tests/e2e_m3_theme.rs`: HCT, CAM16, gamut solver, contrast, tonal palettes, 7 schemes, 32+ color roles, tokens, ThemePackage APIs.
- `tests/e2e_m3_widgets.rs`: Button (5 variants), Card (3 variants + dual shadows), Switch, Checkbox (with indeterminate), Slider (with steps), Chip (4 variants), ProgressBar (determinate & indeterminate), TextInput (Filled & Outlined), and state layer opacities.
- `tests/e2e_m3_markup.rs`: Declarative markup parser, `theme="material-you"`, `variant`, `selected`, `checked`, `value`, `progress` signal attributes, and dynamic CSS resolution.
- `tests/e2e_m3_scenarios.rs`: Real-world application scenarios (wallpaper theme switching, settings form, dashboard, task manager, full declarative app).

Also, when the test suite is created and verified, publish `TEST_READY.md` at `/home/ai-workspace/coding-repo/quick-silver/TEST_READY.md` following the template in `TEST_INFRA.md`.
Ensure the test files compile cleanly against the public interfaces defined in `PROJECT.md § Interface Contracts`.

Write your report to:
`/home/ai-workspace/coding-repo/quick-silver/.agents/test_writer_e2e_1/report.md`
And write your `progress.md` and `handoff.md` in your working directory.
When done, message parent with a summary and the path to your report.
