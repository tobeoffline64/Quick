# 🎨 Material You (M3) Complete Design System & Engine Specification

**Target Workspace**: `quick-silver` UI Framework  
**Author**: Spec Miner (`spec_miner_1`)  
**Date**: 2026-08-30  
**Status**: Specification Complete / Implementation Ready  

---

## 📑 Table of Contents
1. [Executive Summary](#1-executive-summary)
2. [Pure Rust HCT Color Model & Colorimetry Algorithms](#2-pure-rust-hct-color-model--colorimetry-algorithms)
3. [Scheme Variants & Tonal Palette Generation](#3-scheme-variants--tonal-palette-generation)
4. [Complete Catalog of 32+ M3 Color Roles](#4-complete-catalog-of-32-m3-color-roles)
5. [Design Tokens Specification (Shapes, Elevations, State Layers)](#5-design-tokens-specification)
6. [M3 Base Component Suite Specifications](#6-m3-base-component-suite-specifications)
7. [Declarative `.quick` Markup Syntax & Theme Integration](#7-declarative-quick-markup-syntax--theme-integration)
8. [Rust Dynamic Theming API Specification](#8-rust-dynamic-theming-api-specification)
9. [Features Discovered Table](#9-features-discovered-table)
10. [Edge Cases Table](#10-edge-cases-table)
11. [Comprehensive Verification Plan & Test Matrix](#11-comprehensive-verification-plan--test-matrix)

---

## 1. Executive Summary

This document specifies the complete, authoritative technical requirements, mathematical algorithms, colorimetry pipelines, token registries, component behaviors, declarative syntax, and verification matrices for integrating Google Material You (Material Design 3 / M3) into the **Quick UI Framework**.

Quick delivers dynamic theming in **100% Pure Rust** with zero JavaScript/DOM overhead. The system comprises:
- **`quick-style`**: Pure Rust HCT / CAM16 color space, viewing conditions, gamut solver, tonal palette generator, 7 scheme variants, 47 M3 color roles (light/dark), shape/elevation/state tokens, and dynamic CSS generator.
- **`quick-widgets`**: Full base widget suite adhering to M3 specs: Button (5 variants), Card (3 variants), Switch, Checkbox, Slider, Chip, ProgressBar (determinate/indeterminate), and TextInput (Filled/Outlined).
- **`quick-markup`**: Declarative XML & TOML `.quick` parser and data binding runtime supporting `theme="material-you"`, reactive signal binds (`$signal`), and event handlers.
- **`apps/hello-world`**: Live desktop showcase application rendering with Skia 2D on Wayland/X11.

---

## 2. Pure Rust HCT Color Model & Colorimetry Algorithms

Google's HCT (Hue, Chroma, Tone) color space unifies **CAM16** (Color Appearance Model 2016) with **CIE $L^*$** (Perceptual Lightness from CIELAB). 
- **Hue ($H \in [0, 360)$)**: CAM16 hue angle representing perceptual color sensation.
- **Chroma ($C \ge 0$)**: CAM16 colorfulness relative to brightness of white.
- **Tone ($T \in [0, 100]$)**: CIELAB $L^*$ lightness, linearly proportional to human perception of luminance and WCAG contrast.

### 2.1 Standard Viewing Conditions (sRGB / D65)
The CAM16 transformation relies on standard sRGB viewing conditions:
- **White Point D65**: $X_w = 95.047$, $Y_w = 100.0$, $Z_w = 108.883$
- **Adapting Luminance**: $L_A = 11.72567653768094 \text{ cd/m}^2$ (standard ambient illumination: $(64 / \pi) \times 0.2$ or $11.7257$)
- **Background Luminance**: $Y_b = 18.418651851244416$ ($18.42\%$ gray background)
- **Surround Type**: Average surround, $F = 0.8$, $c = 0.69$, $N_c = 1.0$

#### Derived Constants:
1. $k = \frac{1}{5 L_A + 1}$
2. $F_L = 0.2 k^4 (5 L_A) + 0.1 (1 - k^4)^2 (5 L_A)^{1/3}$
3. $n = \frac{Y_b}{Y_w} = 0.1841865$
4. $D = \text{clamp}\left(F \left[1 - \frac{1}{3.6} e^{\frac{-L_A - 42}{92}}\right], 0.0, 1.0\right)$
5. $N_{bb} = N_{cb} = 0.725 \left(\frac{1}{n}\right)^{0.2} \approx 1.000$
6. $z = 1.48 + \sqrt{n} \approx 1.909$

---

### 2.2 Forward Pipeline: sRGB $\to$ Linear sRGB $\to$ CIE XYZ $\to$ CAM16 / HCT

#### Step 1: sRGB to Linear sRGB ($[0, 255] \to [0.0, 1.0]$)
For each channel $c \in \{r, g, b\}$ with $c_{\text{norm}} = \frac{c}{255.0}$:
$$f(c_{\text{norm}}) = \begin{cases} \frac{c_{\text{norm}}}{12.92} & \text{if } c_{\text{norm}} \le 0.04045 \\ \left(\frac{c_{\text{norm}} + 0.055}{1.055}\right)^{2.4} & \text{otherwise} \end{cases}$$

#### Step 2: Linear sRGB to CIE XYZ (D65)
$$\begin{bmatrix} X \\ Y \\ Z \end{bmatrix} = \begin{bmatrix} 0.41233895 & 0.35762064 & 0.18051042 \\ 0.21260000 & 0.71520000 & 0.07220000 \\ 0.01932141 & 0.11916382 & 0.95034478 \end{bmatrix} \begin{bmatrix} R_{\text{lin}} \times 100 \\ G_{\text{lin}} \times 100 \\ B_{\text{lin}} \times 100 \end{bmatrix}$$

#### Step 3: CIELAB Tone ($T = L^*$) Computation from $Y$
$$y_{\text{norm}} = \frac{Y}{100.0}$$
$$f(y_{\text{norm}}) = \begin{cases} y_{\text{norm}}^{1/3} & \text{if } y_{\text{norm}} > \frac{216}{24389} \approx 0.00885645 \\ \frac{841}{108} y_{\text{norm}} + \frac{4}{29} & \text{otherwise} \end{cases}$$
$$\text{Tone } T = L^* = 116 \cdot f(y_{\text{norm}}) - 16$$

#### Step 4: XYZ to CAM16 Cone Response ($M_{16}$ Matrix)
$$\begin{bmatrix} R_c \\ G_c \\ B_c \end{bmatrix} = \begin{bmatrix} 0.401288 & 0.650173 & -0.051461 \\ -0.250268 & 1.204414 & 0.045854 \\ -0.002079 & 0.048952 & 0.953127 \end{bmatrix} \begin{bmatrix} X \\ Y \\ Z \end{bmatrix}$$

#### Step 5: Chromatic Adaptation & Non-linear Compression
$$\begin{aligned}
R_c' &= \left[ \left(\frac{Y_w \cdot D}{R_w}\right) + 1 - D \right] R_c \\
G_c' &= \left[ \left(\frac{Y_w \cdot D}{G_w}\right) + 1 - D \right] G_c \\
B_c' &= \left[ \left(\frac{Y_w \cdot D}{B_w}\right) + 1 - D \right] B_c
\end{aligned}$$
Where $R_w, G_w, B_w$ are the cone responses of the white point.

Non-linear compression:
$$R_a' = \frac{400 \operatorname{sign}(R_c') (|F_L R_c' / 100|)^{0.42}}{27.13 + (|F_L R_c' / 100|)^{0.42}}$$
(and identically for $G_a'$ and $B_a'$).

#### Step 6: CAM16 Hue $h$ and Chroma $C$
$$\begin{aligned}
a &= R_a' - \frac{12}{11} G_a' + \frac{1}{11} B_a' \\
b &= \frac{1}{9} (R_a' + G_a' - 2 B_a') \\
h &= \left(\operatorname{atan2}(b, a) \times \frac{180}{\pi}\right) \pmod{360.0} \\
e_t &= \frac{1}{4} \left(\cos\left(h \frac{\pi}{180} + 2\right) + 3.8\right) \\
A &= \left[ 2 R_a' + G_a' + \frac{1}{20} B_a' - 0.305 \right] N_{bb} \\
J &= 100 \left( \frac{A}{A_w} \right)^{c \cdot z} \\
C &= t^{0.9} \sqrt{\frac{J}{100}} (1.64 - 0.29^n)^{0.73} \quad \text{where } t = \frac{\frac{50000}{13} N_c N_{cb} e_t \sqrt{a^2 + b^2}}{R_a' + G_a' + \frac{21}{20} B_a' + 0.305}
\end{aligned}$$

---

### 2.3 Inverse Pipeline & Tone-Preserving Gamut Mapping (HCT $\to$ sRGB)

Given a target Hue $h \in [0, 360)$, Chroma $C \ge 0$, and Tone $T \in [0, 100]$:

1. **Calculate Target $Y$ from Tone $T$**:
   $$Y = \begin{cases} 100 \times \left(\frac{T + 16}{116}\right)^3 & \text{if } T > 8.0 \\ 100 \times \frac{T \times 27}{24389} & \text{otherwise} \end{cases}$$

2. **Gamut Mapping via Binary Search**:
   Because not all theoretical $(h, C, T)$ tuples exist in the standard sRGB color gamut $[0, 255]^3$, the solver searches for the maximum realizable Chroma $c^* \le C$ that yields valid sRGB coordinates while **strictly preserving Hue $h$ and Tone $T$**:
   - Set search bounds: $c_{\text{low}} = 0.0$, $c_{\text{high}} = C$.
   - Iteratively compute CAM16 inverse for $(h, c_{\text{mid}}, Y)$.
   - Convert CAM16 XYZ $\to$ Linear sRGB $\to$ 8-bit sRGB.
   - If $R, G, B \in [0, 255]$, then $c_{\text{low}} = c_{\text{mid}}$; else $c_{\text{high}} = c_{\text{mid}}$.
   - Converge within $0.01$ chroma tolerance (max 16 binary search iterations).

3. **Tone Inversion & Dynamic Contrast Formulas**:
   - Tone inversion for Dark Mode: $T_{\text{dark}} = 100 - T_{\text{light}}$ (adjusted by role-specific tonal anchors).
   - Relative Luminance $Y_{\text{rel}} = \frac{Y}{100.0}$.
   - Contrast Ratio:
     $$CR = \frac{Y_{\text{lighter}} + 0.05}{Y_{\text{darker}} + 0.05}$$
   - Contrast Thresholds:
     - **WCAG AA Body Text**: $CR \ge 4.5:1$ ($\Delta T \ge 40$)
     - **WCAG AA Large Text / UI Components**: $CR \ge 3.0:1$ ($\Delta T \ge 25$)
     - **WCAG AAA Enhanced Text**: $CR \ge 7.0:1$ ($\Delta T \ge 60$)

---

## 3. Scheme Variants & Tonal Palette Generation

M3 Dynamic Schemes define **6 Tonal Palettes** derived from the seed color:
1. `primary`: Main brand accent palette.
2. `secondary`: Supporting, lower-chroma accent palette.
3. `tertiary`: Harmonic or contrasting accent palette.
4. `neutral`: Surface and background palette.
5. `neutral_variant`: Divider, outline, and container variant palette.
6. `error`: Semantic error palette.

Each Tonal Palette provides tones:
$$T \in [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 98, 99, 100]$$

### 3.1 The 7 Scheme Variant Specifications

| Scheme Variant | Primary Palette $(H, C)$ | Secondary Palette $(H, C)$ | Tertiary Palette $(H, C)$ | Neutral Palette $(H, C)$ | Neutral Variant $(H, C)$ | Error Palette $(H, C)$ |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **`TonalSpot`** *(Default)* | $H = h_{\text{seed}}$<br>$C = \max(48.0, c_{\text{seed}})$ | $H = h_{\text{seed}}$<br>$C = 16.0$ | $H = (h_{\text{seed}} + 60) \bmod 360$<br>$C = 24.0$ | $H = h_{\text{seed}}$<br>$C = 6.0$ | $H = h_{\text{seed}}$<br>$C = 8.0$ | $H = 25.0$<br>$C = 84.0$ |
| **`Vibrant`** | $H = h_{\text{seed}}$<br>$C = \max(74.0, c_{\text{seed}})$ | $H = (h_{\text{seed}} + 24) \bmod 360$<br>$C = 24.0$ | $H = (h_{\text{seed}} + 48) \bmod 360$<br>$C = 32.0$ | $H = h_{\text{seed}}$<br>$C = 10.0$ | $H = h_{\text{seed}}$<br>$C = 12.0$ | $H = 25.0$<br>$C = 84.0$ |
| **`Expressive`** | $H = (h_{\text{seed}} + 240) \bmod 360$<br>$C = 40.0$ | $H = (h_{\text{seed}} + 15) \bmod 360$<br>$C = 24.0$ | $H = (h_{\text{seed}} + 120) \bmod 360$<br>$C = 32.0$ | $H = (h_{\text{seed}} + 15) \bmod 360$<br>$C = 8.0$ | $H = (h_{\text{seed}} + 15) \bmod 360$<br>$C = 12.0$ | $H = 25.0$<br>$C = 84.0$ |
| **`Fidelity`** | $H = h_{\text{seed}}$<br>$C = c_{\text{seed}}$ | $H = h_{\text{seed}}$<br>$C = \max(c_{\text{seed}} - 32.0, c_{\text{seed}} \times 0.5)$ | $H = (h_{\text{seed}} + 60) \bmod 360$<br>$C = \max(c_{\text{seed}} - 16.0, 24.0)$ | $H = h_{\text{seed}}$<br>$C = \min(\frac{c_{\text{seed}}}{8}, 4.0)$ | $H = h_{\text{seed}}$<br>$C = \frac{c_{\text{seed}}}{8} + 4.0$ | $H = 25.0$<br>$C = 84.0$ |
| **`Content`** | $H = h_{\text{seed}}$<br>$C = c_{\text{seed}}$ | $H = h_{\text{seed}}$<br>$C = \max(c_{\text{seed}} - 32.0, c_{\text{seed}} \times 0.4)$ | $H = (h_{\text{seed}} + 60) \bmod 360$<br>$C = \max(c_{\text{seed}} - 16.0, 24.0)$ | $H = h_{\text{seed}}$<br>$C = \min(\frac{c_{\text{seed}}}{8}, 4.0)$ | $H = h_{\text{seed}}$<br>$C = \frac{c_{\text{seed}}}{8} + 4.0$ | $H = 25.0$<br>$C = 84.0$ |
| **`Monochrome`** | $H = h_{\text{seed}}$<br>$C = 0.0$ | $H = h_{\text{seed}}$<br>$C = 0.0$ | $H = h_{\text{seed}}$<br>$C = 0.0$ | $H = h_{\text{seed}}$<br>$C = 0.0$ | $H = h_{\text{seed}}$<br>$C = 0.0$ | $H = 25.0$<br>$C = 84.0$ |
| **`Neutral`** | $H = h_{\text{seed}}$<br>$C = 12.0$ | $H = h_{\text{seed}}$<br>$C = 8.0$ | $H = h_{\text{seed}}$<br>$C = 16.0$ | $H = h_{\text{seed}}$<br>$C = 2.0$ | $H = h_{\text{seed}}$<br>$C = 2.0$ | $H = 25.0$<br>$C = 84.0$ |

---

## 4. Complete Catalog of 32+ M3 Color Roles

The engine maps tonal palettes into **47 individual M3 Color Roles** for both Light Mode and Dark Mode:

| # | Role Token Name | Palette Source | Light Mode Tone | Dark Mode Tone | Description / Semantic Purpose |
|---|----------------|----------------|-----------------|----------------|--------------------------------|
| 1 | `primary` | Primary | Tone 40 | Tone 80 | High-emphasis fills, primary buttons, active toggles |
| 2 | `on_primary` | Primary | Tone 100 | Tone 20 | Text and icons displayed on top of `primary` |
| 3 | `primary_container` | Primary | Tone 90 | Tone 30 | Medium-emphasis fills, prominent cards, tonal buttons |
| 4 | `on_primary_container` | Primary | Tone 10 | Tone 90 | Text and icons on top of `primary_container` |
| 5 | `inverse_primary` | Primary | Tone 80 | Tone 40 | Accent color used on `inverse_surface` (e.g. snackbars) |
| 6 | `primary_fixed` | Primary | Tone 90 | Tone 90 | Static high-emphasis container fill across all themes |
| 7 | `primary_fixed_dim` | Primary | Tone 80 | Tone 80 | Dimmed static primary container fill |
| 8 | `on_primary_fixed` | Primary | Tone 10 | Tone 10 | Content on top of `primary_fixed` |
| 9 | `on_primary_fixed_variant`| Primary | Tone 30 | Tone 30 | Lower-emphasis content on `primary_fixed` |
| 10 | `secondary` | Secondary | Tone 40 | Tone 80 | Less prominent components (filter chips, secondary actions) |
| 11 | `on_secondary` | Secondary | Tone 100 | Tone 20 | Text and icons on top of `secondary` |
| 12 | `secondary_container` | Secondary | Tone 90 | Tone 30 | Selected chips, switch tracks, badge backgrounds |
| 13 | `on_secondary_container` | Secondary | Tone 10 | Tone 90 | Text and icons on top of `secondary_container` |
| 14 | `secondary_fixed` | Secondary | Tone 90 | Tone 90 | Static secondary container fill |
| 15 | `secondary_fixed_dim` | Secondary | Tone 80 | Tone 80 | Dimmed static secondary container fill |
| 16 | `on_secondary_fixed` | Secondary | Tone 10 | Tone 10 | Content on top of `secondary_fixed` |
| 17 | `on_secondary_fixed_variant`| Secondary | Tone 30 | Tone 30 | Lower-emphasis content on `secondary_fixed` |
| 18 | `tertiary` | Tertiary | Tone 40 | Tone 80 | Contrasting accent for badges, input highlights, balances |
| 19 | `on_tertiary` | Tertiary | Tone 100 | Tone 20 | Text and icons on top of `tertiary` |
| 20 | `tertiary_container` | Tertiary | Tone 90 | Tone 30 | Soft tertiary container fill |
| 21 | `on_tertiary_container` | Tertiary | Tone 10 | Tone 90 | Content on top of `tertiary_container` |
| 22 | `tertiary_fixed` | Tertiary | Tone 90 | Tone 90 | Static tertiary container fill |
| 23 | `tertiary_fixed_dim` | Tertiary | Tone 80 | Tone 80 | Dimmed static tertiary container fill |
| 24 | `on_tertiary_fixed` | Tertiary | Tone 10 | Tone 10 | Content on top of `tertiary_fixed` |
| 25 | `on_tertiary_fixed_variant`| Tertiary | Tone 30 | Tone 30 | Lower-emphasis content on `tertiary_fixed` |
| 26 | `surface` | Neutral | Tone 98 | Tone 6 | Base window background and page surface |
| 27 | `surface_dim` | Neutral | Tone 87 | Tone 6 | Dimmed base surface (recessed background areas) |
| 28 | `surface_bright` | Neutral | Tone 98 | Tone 24 | Brightened base surface (elevated light areas) |
| 29 | `surface_container_lowest`| Neutral | Tone 100 | Tone 4 | Lowest elevation card / sub-panel background |
| 30 | `surface_container_low` | Neutral | Tone 96 | Tone 10 | Low elevation card and resting container background |
| 31 | `surface_container` | Neutral | Tone 94 | Tone 12 | Standard card, sheet, and dialog background |
| 32 | `surface_container_high` | Neutral | Tone 92 | Tone 17 | Elevated navigation bars, active sheets, search bars |
| 33 | `surface_container_highest`| Neutral | Tone 90 | Tone 22 | Highest elevation modal headers, slider inactive tracks |
| 34 | `on_surface` | Neutral | Tone 10 | Tone 90 | Primary text, titles, body copy, and icons |
| 35 | `surface_variant` | Neutral Variant| Tone 90 | Tone 30 | Variant container backgrounds, text input fill |
| 36 | `on_surface_variant` | Neutral Variant| Tone 30 | Tone 80 | Secondary text, captions, placeholder labels, icons |
| 37 | `outline` | Neutral Variant| Tone 50 | Tone 60 | Component borders, outlined buttons, card strokes |
| 38 | `outline_variant` | Neutral Variant| Tone 80 | Tone 30 | Subtle dividers, inactive borders, card outlines |
| 39 | `surface_tint` | Primary | Tone 40 | Tone 80 | Color used for elevation tinting overlay |
| 40 | `inverse_surface` | Neutral | Tone 20 | Tone 90 | Inverted snackbar and tooltip backgrounds |
| 41 | `inverse_on_surface` | Neutral | Tone 95 | Tone 20 | Text and icons on `inverse_surface` |
| 42 | `shadow` | Neutral | Tone 0 (#000) | Tone 0 (#000) | Elevation drop shadow color |
| 43 | `scrim` | Neutral | Tone 0 (#000) | Tone 0 (#000) | Full-screen modal overlay backdrop scrim |
| 44 | `error` | Error | Tone 40 | Tone 80 | High-emphasis error state fill and invalid borders |
| 45 | `on_error` | Error | Tone 100 | Tone 20 | Text and icons on top of `error` |
| 46 | `error_container` | Error | Tone 90 | Tone 30 | Soft error banner and warning chip backgrounds |
| 47 | `on_error_container` | Error | Tone 10 | Tone 90 | Text and icons on top of `error_container` |

---

## 5. Design Tokens Specification

### 5.1 Shape Scale System (`md-sys-shape`)
Directly maps standard corner radius tokens:

| Token Name | Key / CSS Variable | Radius ($px$) | Applicable Widget Types |
| :--- | :--- | :--- | :--- |
| `corner_none` | `--md-sys-shape-corner-none` | $0.0\text{ px}$ | Fullscreen canvas, square media, edge-to-edge containers |
| `corner_extra_small` | `--md-sys-shape-corner-extra-small` | $4.0\text{ px}$ | Filled text field top corners, snackbars, checkboxes |
| `corner_small` | `--md-sys-shape-corner-small` | $8.0\text{ px}$ | Small chips, tooltip overlays, progress bars |
| `corner_medium` | `--md-sys-shape-corner-medium` | $12.0\text{ px}$ | Small dialogs, sub-cards, nested container boxes |
| `corner_large` | `--md-sys-shape-corner-large` | $16.0\text{ px}$ | Standard cards, alert dialogs, modal sheets |
| `corner_extra_large`| `--md-sys-shape-corner-extra-large`| $28.0\text{ px}$ | Large FABs, search bars, navigation drawers |
| `corner_full` | `--md-sys-shape-corner-full` | $9999.0\text{ px}$ | Buttons, filter chips, pill badges, switch tracks |

---

### 5.2 Elevation & Dual-Pass Shadow System (`md-sys-elevation`)
Elevation is rendered using two shadow layers (Key Shadow + Ambient Shadow) combined with surface tinting:

| Level | Elevation ($dp$) | Key Shadow ($x, y, \text{blur}, \text{spread}, \text{color}$) | Ambient Shadow ($x, y, \text{blur}, \text{spread}, \text{color}$) | Surface Tint Overlay ($\%$) |
| :--- | :--- | :--- | :--- | :--- |
| **Level 0** | $0\text{ dp}$ | `none` | `none` | $0\%$ |
| **Level 1** | $1\text{ dp}$ | `0px 1px 2px 0px rgba(0,0,0,0.30)` | `0px 1px 3px 1px rgba(0,0,0,0.15)` | $5\%$ of `surface_tint` |
| **Level 2** | $3\text{ dp}$ | `0px 1px 2px 0px rgba(0,0,0,0.30)` | `0px 2px 6px 2px rgba(0,0,0,0.15)` | $8\%$ of `surface_tint` |
| **Level 3** | $6\text{ dp}$ | `0px 1px 3px 0px rgba(0,0,0,0.30)` | `0px 4px 8px 3px rgba(0,0,0,0.15)` | $11\%$ of `surface_tint` |
| **Level 4** | $8\text{ dp}$ | `0px 2px 3px 0px rgba(0,0,0,0.30)` | `0px 6px 10px 4px rgba(0,0,0,0.15)` | $12\%$ of `surface_tint` |
| **Level 5** | $12\text{ dp}$ | `0px 4px 4px 0px rgba(0,0,0,0.30)` | `0px 8px 12px 6px rgba(0,0,0,0.15)` | $14\%$ of `surface_tint` |

---

### 5.3 State Layer Opacities (`md-sys-state`)
State layers are alpha overlays blended on component container surfaces using the respective `on_<surface/container>` color:

| Interaction State | State Layer Opacity | Applied On-Color Target |
| :--- | :--- | :--- |
| **Hover** | $8\%$ ($0.08$) | `on_primary`, `on_secondary_container`, `on_surface`, etc. |
| **Focus** | $12\%$ ($0.12$) | `on_primary`, `on_surface`, etc. (plus $2\text{px}$ focus ring) |
| **Pressed** | $12\%$ ($0.12$) | `on_primary`, `on_surface`, etc. (with radial ripple/tint) |
| **Dragged** | $16\%$ ($0.16$) | Elevation elevates to Level 4 during dragging |
| **Disabled Content** | $38\%$ ($0.38$) | Content opacity for inactive labels and icons |
| **Disabled Container**| $12\%$ ($0.12$) | Container background opacity for disabled components |

---

## 6. M3 Base Component Suite Specifications

### 6.1 Button (`quick_widgets::button::Button`)
- **Supported Variants**: `Filled` (default), `Tonal`, `Elevated`, `Outlined`, `Text`
- **Geometry**: Pill radius `corner_full` ($9999\text{ px}$), standard height $40\text{ px}$, insets: symmetric $10\text{ px}$ vertical, $24\text{ px}$ horizontal.
- **Variant Color Mappings**:
  - **`Filled`**: Container = `primary`, Text/Icon = `on_primary`. Hover: $+8\%$ `on_primary`.
  - **`Tonal`**: Container = `secondary_container`, Text/Icon = `on_secondary_container`. Hover: $+8\%$ `on_secondary_container`.
  - **`Elevated`**: Container = `surface_container_low`, Text/Icon = `primary`, Shadow = Level 1 (Resting), Level 2 (Hover).
  - **`Outlined`**: Container = `transparent`, Text/Icon = `primary`, Border = `outline` ($1\text{ px}$). Hover: $+8\%$ `primary` container fill.
  - **`Text`**: Container = `transparent`, Text/Icon = `primary`, No border. Hover: $+8\%$ `primary` container fill.
- **Interaction**: Pointer down -> pressed state; pointer up -> on_click callback execution.

---

### 6.2 Card (`quick_widgets::card::Card`)
- **Supported Variants**: `Elevated` (default), `Filled`, `Outlined`
- **Geometry**: Radius `corner_large` ($16.0\text{ px}$ or $20.0\text{ px}$), padding $16.0\text{ px} - 24.0\text{ px}$, gap $16.0\text{ px}$.
- **Variant Color Mappings**:
  - **`Elevated`**: Container = `surface_container_low`, Shadow = Level 1 (dual-pass drop shadow).
  - **`Filled`**: Container = `surface_container_highest`, Border = none, Shadow = Level 0.
  - **`Outlined`**: Container = `surface`, Border = `outline_variant` ($1\text{ px}$), Shadow = Level 0.

---

### 6.3 Selection Controls

#### A. Switch (`quick_widgets::switch::Switch`)
- **Geometry**: Track $52\text{ px} \times 32\text{ px}$, radius $16\text{ px}$ (`corner_full`).
- **Thumb**:
  - Selected / Checked: Diameter $24\text{ px}$, offset $4\text{ px}$ from right edge.
  - Unselected / Unchecked: Diameter $16\text{ px}$, offset $7\text{ px}$ from left edge.
- **Colors**:
  - Checked: Track = `primary`, Thumb = `on_primary`.
  - Unchecked: Track = `surface_container_highest`, Border = `outline` ($2\text{ px}$), Thumb = `outline`.
- **Event Dispatch**: Click/drag toggles `checked` Signal and fires `on_change(bool)`.

#### B. Checkbox (`quick_widgets::checkbox::Checkbox`)
- **Geometry**: Touch target $24\text{ px} \times 24\text{ px}$, visual box $18\text{ px} \times 18\text{ px}$ (or $20\text{ px} \times 20\text{ px}$), corner radius $4\text{ px}$ (`corner_extra_small`).
- **States & Rendering**:
  - Checked: Container = `primary`, Stroke = `on_primary` checkmark vector ($2\text{ px}$ line width).
  - Indeterminate: Container = `primary`, Stroke = `on_primary` horizontal bar ($2\text{ px}$ line width).
  - Unchecked: Container = `transparent`, Border = `outline` / `on_surface_variant` ($2\text{ px}$).
- **Event Dispatch**: Click toggles `checked` Signal and fires `on_change(bool)`.

#### C. Slider (`quick_widgets::slider::Slider`)
- **Geometry**: Touch target height $36\text{ px}$, Track height $8\text{ px}$, track radius $4\text{ px}$ (`corner_extra_small`).
- **Thumb**: Diameter $20\text{ px}$ (`corner_full`), color `primary` / `on_primary`.
- **Track Colors**:
  - Active (left of thumb): `primary`.
  - Inactive (right of thumb): `surface_container_highest` or `secondary_container`.
- **Event Dispatch**: Dragging updates `value` Signal in real-time clamped to $[min, max]$ and fires `on_change(f32)`.

#### D. Chip (`quick_widgets::chip::Chip`)
- **Variants**: Assist, Filter, Input, Suggestion.
- **Geometry**: Height $32\text{ px}$, corner radius $8\text{ px}$ (`corner_small`) or $9999\text{ px}$ (`corner_full`), padding symmetric($6\text{ px}, 14\text{ px}$).
- **States & Colors**:
  - Selected: Container = `secondary_container`, Border = none or `secondary_container`, Text = `on_secondary_container`.
  - Unselected: Container = `surface`, Border = `outline_variant` ($1\text{ px}$), Text = `on_surface_variant`.
- **Event Dispatch**: Click toggles `selected` Signal (if bound) and executes `on_click`.

---

### 6.4 Progress Bar (`quick_widgets::progress::ProgressBar`)
- **Geometry**: Height $8\text{ px}$ (or $4\text{ px}$), track radius $4\text{ px}$ (`corner_extra_small`).
- **Modes**:
  - **Determinate**: Active indicator width = $\text{width} \times \operatorname{clamp}\left(\frac{\text{progress} - \min}{\max - \min}, 0.0, 1.0\right)$. Active color = `primary`, Inactive track = `surface_container_highest`.
  - **Indeterminate**: Animated sliding pill traversing the track smoothly.

---

### 6.5 Text Input (`quick_widgets::text_input::TextInput`)
- **Variants**: `Filled` (top radius $4\text{ px}$, bottom active underline indicator) and `Outlined` (all radius $4\text{ px} - 8\text{ px}$, full perimeter outline).
- **Colors & States**:
  - Resting: Background = `surface_container_highest` (Filled) or `transparent` (Outlined), Border = `outline` ($1\text{ px}$), Text = `on_surface`, Placeholder = `on_surface_variant`.
  - Focused: Border = `primary` ($2\text{ px}$ active stroke).
  - Error: Border = `error` ($2\text{ px}$ error stroke), Helper/Error Text = `error`.
- **Interactivity**: Mouse click focuses/unfocuses; Keyboard inputs mutate string buffer, handling text, spaces, Backspace, and Delete with real-time `on_change` callback.

---

## 7. Declarative `.quick` Markup Syntax & Theme Integration

Developers can declare complete Material You UIs using either XML or TOML formats.

### 7.1 XML Syntax Example (`app.quick`)
```xml
<VStack id="app-root" theme="material-you" style="width: 100%; height: 100%; padding: 32px; background: surface; align-items: center; justify-content: center;">
    
    <Style>
        Card.main-card {
            background: surface_container;
            border-radius: corner_large;
            border-color: outline_variant;
            border-width: 1px;
            padding: 32px;
            gap: 16px;
            width: 90%;
            max-width: 580px;
        }
        Text.pill-badge {
            font-size: 11px;
            font-weight: bold;
            color: on_primary_container;
            background: primary_container;
            padding: 6px 14px;
            border-radius: corner_full;
        }
        Button.btn-primary {
            background: primary;
            color: on_primary;
            border-radius: corner_full;
            padding: 10px 24px;
        }
    </Style>

    <Card class="main-card" variant="elevated">
        <Text class="pill-badge" text="MATERIAL YOU THEME PACKAGE ACTIVE" />
        <Text text="$greeting" style="font-size: 24px; color: on_surface; font-weight: bold;" />
        
        <HStack style="justify-content: space-between; align-items: center; width: 100%;">
            <Text text="Dynamic GPU Acceleration" style="color: on_surface;" />
            <Switch checked="$gpu_enabled" onchange="toggle_gpu" />
        </HStack>

        <VStack style="width: 100%; gap: 6px;">
            <Text text="Brightness Scale" style="color: on_surface_variant;" />
            <Slider min="0" max="100" value="$brightness" onchange="on_slider" />
        </VStack>

        <ProgressBar progress="$brightness" min="0" max="100" />

        <HStack style="gap: 8px;">
            <Chip text="Wayland EGL" selected="$chip_wayland" onclick="toggle_wayland" />
            <Chip text="Pure Rust" selected="$chip_rust" onclick="toggle_rust" />
        </HStack>

        <TextInput placeholder="Enter text..." text="$user_name" onchange="on_text" />

        <HStack style="gap: 16px;">
            <Button id="btn-submit" variant="filled" text="Submit" onclick="on_click" />
            <Button id="btn-reset" variant="outlined" text="Reset" onclick="on_reset" />
        </HStack>
    </Card>
</VStack>
```

### 7.2 Parsed Attribute Binding Matrix

| Component Tag | Supported Attributes | Target Widget Field / Property |
| :--- | :--- | :--- |
| `<VStack>`, `<HStack>` | `theme="material-you"`, `style="..."`, `class="..."`, `id="..."` | Applies theme package rules, layout direction, insets, gap |
| `<Button>` | `variant="filled|tonal|elevated|outlined|text"`, `text="..."`, `onclick="..."` | Button variant styling, label text, click action |
| `<Card>` | `variant="elevated|filled|outlined"`, `style="..."`, `class="..."` | Card variant styling, shadow rendering, borders |
| `<Switch>` | `checked="$bool_signal"`, `onchange="..."` | Two-way reactive boolean signal binding, state change handler |
| `<Checkbox>` | `checked="$bool_signal"`, `onchange="..."` | Two-way reactive boolean signal binding, check toggle handler |
| `<Slider>` | `value="$f32_signal"`, `min="0"`, `max="100"`, `onchange="..."` | Reactive float signal binding, range parameters, drag handler |
| `<Chip>` | `text="..."`, `selected="$bool_signal"`, `onclick="..."` | Chip text label, selection toggle signal, click action |
| `<ProgressBar>` | `progress="$f32_signal"`, `min="0"`, `max="100"` | Reactive float progress binding, min/max scale |
| `<TextInput>` | `placeholder="..."`, `text="$str_signal"`, `onchange="..."` | Placeholder string, text signal two-way binding, change handler |

---

## 8. Rust Dynamic Theming API Specification

### 8.1 Public Interface: `quick_style::theme`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemeVariant {
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Monochrome,
    Neutral,
}

#[derive(Debug, Clone)]
pub struct ThemePackage {
    pub name: String,
    pub colors: HashMap<String, Color>,
    pub shapes: HashMap<String, f32>,
    pub is_dark: bool,
    pub contrast: f64,
}

impl ThemePackage {
    /// Create an empty theme package with given name.
    pub fn new(name: impl Into<String>) -> Self;

    /// Generate dynamic M3 theme from seed color with default contrast (0.0).
    pub fn from_seed_color(
        seed_hex: &str,
        variant: SchemeVariant,
        is_dark: bool,
    ) -> Result<Self, String>;

    /// Generate dynamic M3 theme from seed color with explicit contrast level (-1.0 to 1.0).
    pub fn from_seed_color_with_contrast(
        seed_hex: &str,
        variant: SchemeVariant,
        is_dark: bool,
        contrast: f64,
    ) -> Result<Self, String>;

    /// Built-in baseline Material You Dark Theme.
    pub fn material_you() -> Self;

    /// Built-in baseline Material You Light Theme.
    pub fn material_you_light() -> Self;

    /// Generate standard CSS stylesheet matching theme color roles and component selectors.
    pub fn generate_css(&self) -> String;
}
```

---

## 9. Features Discovered Table

| # | Category | Feature | Description | Inputs | Outputs | Error Behavior | Discovered Via |
|---|----------|---------|-------------|--------|---------|----------------|----------------|
| 1 | Colorimetry | `CAM16` Color Model | Full Color Appearance Model 2016 forward/inverse transformation with standard sRGB viewing conditions | CIE XYZ coordinates, D65 viewing conditions | CAM16 Hue $h$, Chroma $C$, Lightness $J$ | Clamps negative or non-finite inputs gracefully | Spec Doc §2, `material-colors` |
| 2 | Colorimetry | `HCT` Color Space | Google HCT color space combining CAM16 $h, C$ with CIELAB Tone $T = L^*$ | Hue ($0..360$), Chroma ($\ge 0$), Tone ($0..100$) | 8-bit sRGB `Color` struct | Out-of-gamut chroma binary searched to gamut boundary | Spec Doc §2.A, `quick-style` |
| 3 | Theming | 7 Scheme Variants | Palettes for `TonalSpot`, `Vibrant`, `Expressive`, `Fidelity`, `Content`, `Monochrome`, `Neutral` | Seed Hex Color string, `SchemeVariant` enum | 6 Tonal Palettes (Primary, Secondary, Tertiary, Neutral, NeutralVariant, Error) | Returns Err on invalid hex string | Spec Doc §2.A, `quick-style` |
| 4 | Theming | 47 M3 Color Roles | Complete Light and Dark mode role derivation (`primary`, `surface_container_*`, `outline`, etc.) | Tonal Palettes, `is_dark` flag | Map of 47 Color roles | Fallbacks to default palette if seed extraction fails | Spec Doc §2.B, `quick-style` |
| 5 | Tokens | `md-sys-shape` System | Shape tokens `corner-none` ($0\text{px}$) to `corner-full` ($9999\text{px}$) | Shape token identifier | `f32` corner radius in pixels | Defaults to $0.0\text{px}$ if token not found | Spec Doc §2.C, `quick-style` |
| 6 | Tokens | `md-sys-elevation` System | Dual-pass drop shadow (Key + Ambient) + dynamic surface tint percentage (Levels 0..5) | Elevation Level (0..5) | Shadow blur, offsets, alphas, tint % | Levels $> 5$ clamped to Level 5 | Spec Doc §2.D, `quick-render` |
| 7 | Tokens | `md-sys-state` System | State layer alpha overlays (Hover 8%, Focus 12%, Pressed 12%, Dragged 16%, Disabled 38%) | Interaction state enum | Alpha overlay float ($0.0..1.0$) | Invalid states return $0.0$ | Spec Doc §2.E, `quick-widgets` |
| 8 | Widgets | M3 Button Variants | 5 Button variants: `Filled`, `Tonal`, `Elevated`, `Outlined`, `Text` with pill geometry | Variant enum, label text, click callback | Rendered Button with state feedback | Missing theme falls back to default accent | Spec Doc §3, `quick-widgets` |
| 9 | Widgets | M3 Card Variants | 3 Card variants: `Elevated` (drop shadow), `Filled`, `Outlined` (outline border) | Variant enum, children widgets | Rendered container box | Unknown variant defaults to Elevated | Spec Doc §3, `quick-widgets` |
| 10 | Widgets | M3 Selection Controls | `Switch` (pill track + thumb), `Checkbox` (stroke check), `Slider` (track + thumb), `Chip` | Signals (`bool`/`f32`), callbacks | Interactive controls with full event handling | Clamps values to valid bounds | Spec Doc §3, `quick-widgets` |
| 11 | Widgets | M3 Progress Bars | `ProgressBar` determinate (ratio fill) & indeterminate modes | Progress float signal ($0.0..1.0$ or $0..100$) | Rendered progress indicator | Non-finite values clamp to min | Spec Doc §3, `quick-widgets` |
| 12 | Widgets | M3 TextInput | `TextInput` Filled & Outlined variants with focus state and keyboard dispatch | Placeholder string, text signal, change callback | Interactive text entry box | Ignores non-printable control chars | Spec Doc §3, `quick-widgets` |
| 13 | Markup | Declarative `.quick` Theme | `<VStack theme="material-you">` loads dynamic theme rules into stylesheet cascade | `.quick` markup string/file, `DataContext` | Widget hierarchy & combined `StyleSheet` | Malformed XML/TOML returns Err without panic | Spec Doc §4, `quick-markup` |
| 14 | Dynamic API | `ThemePackage::from_seed_color` | Programmatic Rust dynamic theming API generating CSS & role maps from Hex seeds | Seed Hex, `SchemeVariant`, `is_dark`, `contrast` | Fully resolved `ThemePackage` instance | Returns Err for invalid hex syntax | Spec Doc §5, `quick-style` |

---

## 10. Edge Cases Table

| # | Feature | Input | Observed / Specified Behavior |
|---|---------|-------|-------------------------------|
| 1 | Color Hex Parsing | `"#12"`, `"#gggggg"`, `""` | Returns `Err("Invalid hex color")` without panic; falls back to default. |
| 2 | Color Gamut Clipping | Impossible $(h=120, C=120, T=90)$ (super-saturated light green) | Binary search clips chroma to maximum in-gamut chroma ($\approx 42.5$) while strictly preserving Hue ($120^\circ$) and Tone ($90$). |
| 3 | Monochrome Scheme | Seed color `"#FF0000"` (pure red) with `SchemeVariant::Monochrome` | Sets Chroma $= 0.0$ across all primary, secondary, tertiary, and neutral palettes; generates pure grayscale tonal palette. |
| 4 | Slider Value Bounds | `value = NaN`, `value = 150.0` with `min = 0.0, max = 100.0` | `NaN` resolves to `min` ($0.0$); out-of-range values clamped strictly to $[0.0, 100.0]$. |
| 5 | Progress Bar Zero Range | `min = 50.0, max = 50.0` | Division by zero avoided; fill percentage evaluates safely to $0.0\%$. |
| 6 | Checkbox Click Bounds | Pointer down inside $24\times 24\text{px}$ bounds, pointer up released outside | Click cancelled; `checked` signal remains unchanged; `is_pressed` resets to `false`. |
| 7 | TextInput Key Backspace | `Backspace` or `Delete` on empty input (`value = ""`) | No-op; avoids out-of-bounds slice or underflow panic. |
| 8 | Deeply Nested Markup | 30+ levels of nested `<Card>` and `<VStack>` elements | Layout engine (`Taffy`) resolves leaf nodes recursively without stack overflow. |
| 9 | High-DPI / Window Resize | Window resized to $(0, 0)$ or $(3840, 2160)$ | Softbuffer & layout engines handle empty bounds gracefully without divide-by-zero or crashes. |

---

## 11. Comprehensive Verification Plan & Test Matrix

### 11.1 Unit Test Suites Matrix
1. **`quick-style` Tests**:
   - `test_cam16_forward_and_inverse_roundtrip`: Verify sRGB <-> CAM16 <-> sRGB round-trip within $\Delta E < 1.0$.
   - `test_hct_tone_luminance_linearity`: Verify CIELAB Tone $T=50$ corresponds to $Y \approx 18.42$.
   - `test_scheme_variants_generation`: Verify 7 scheme variants generate proper chroma offsets.
   - `test_all_47_color_roles`: Verify light & dark mode role tone assignments.
   - `test_theme_package_from_seed_color`: Verify `ThemePackage::from_seed_color("#6750A4", SchemeVariant::Vibrant, true)`.

2. **`quick-widgets` Tests**:
   - `test_button_variants_and_states`: Test Filled, Tonal, Elevated, Outlined, Text buttons and hover/pressed states.
   - `test_card_elevations_and_shadows`: Verify drop shadow painting for Elevated cards.
   - `test_selection_controls_signals`: Test reactive toggles for Switch, Checkbox, Slider, and Chip.
   - `test_progress_bar_ranges`: Test determinate and indeterminate progress bars.
   - `test_text_input_editing`: Test typing, focus, and backspace handling.

3. **`quick-markup` Tests**:
   - `test_quick_theme_attribute_injection`: Test `<VStack theme="material-you">` generates and injects M3 rules.
   - `test_quick_variant_attribute_binding`: Test `variant="outlined"` binds correctly to widgets.
   - `test_two_way_signal_bindings`: Test `$sig` bindings for `checked`, `value`, `progress`, `text`.

4. **Acceptance Criteria Validation**:
   - `cargo check --workspace --all-targets` passes with 0 errors and 0 warnings.
   - `cargo test --workspace` passes with 100% success rate across all crates.
   - `cargo run -p hello-world` launches the desktop application without panics.
