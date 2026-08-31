# Quick UI Framework Development Guidelines

## 1. High-DPI & Coordinate Systems
- Always convert window events from physical pixels to logical points using `scale_factor` before hit-testing widget bounds:
  ```rust
  Point::new(position.x as f32 / scale_factor, position.y as f32 / scale_factor)
  ```
- All layout calculations, styles, fonts, and widget bounding boxes operate strictly in **logical points**.

## 2. Layout Engine & Taffy Tree Connectivity
- Container widgets (`TabControl`, `ScrollViewer`, `Card`, `VStack`, `HStack`) must connect all child nodes to their Taffy node using `engine.new_with_children(&style, &child_nodes)`.
- Never create container nodes with `new_leaf()` if they have children; unattached children will evaluate to `(0, 0)` size and collapse.

## 3. Layout Caching & Event Routing
- Layout computation is heavy: only recompute Taffy layout when the window size changes or when layout is explicitly marked dirty (`ensure_layout`).
- Pointer move events (`PointerPhase::Moved`) must perform fast O(1) hit-testing against pre-computed `child_bounds`. Never call `layout_engine.reset()` inside `handle_event`.

## 4. Widget Event Handling & Hover Invalidation
- When a widget's hover state changes (`prev_hover != self.is_hovered`), `handle_event` must return `true` so the window event loop triggers an immediate redraw (`window.request_redraw()`).
- On `PointerPhase::Up` or `PointerPhase::Cancel` outside widget bounds, clear pressed state and return `false` without firing action callbacks.

## 5. Software Rasterizer Performance
- Always use the thread-safe `GlyphCache` for TrueType text rendering rather than re-rasterizing Bezier curves per frame.
- In `fill_rounded_rect` and `fill_rect`, fast-fill solid rectangular rows using memory span fills (`buffer[start..end].fill(pixel)`), restricting distance calculations only to corner squares (`r × r`).

## 6. GNOME HIG Adaptive Standards
- Center view switchers (`TabControl`) with 160–260px proportional pills in the header bar.
- Structure multi-component layouts in boxed preference groups with 12px border radius, 1px border strokes, generous padding, and uppercase category headers.

## 7. Graphics Pipeline: Vello GPU Compute & Pure-Rust Architecture
- **Zero C++ Dependencies**: Never reintroduce C++ rendering engines (such as Skia or Qt). All graphics pipelines must remain 100% pure Rust and memory-safe.
- **Dual-Backend Support**:
  - **Primary**: Vello GPU compute pipeline (`quick-render::vello_scene` + `quick-window::vello_surface`) for 120+ FPS hardware acceleration.
  - **Fallback**: CPU `SoftwareRasterizer + softbuffer` when GPU compute is unavailable or when running in headless CI mode (`QUICK_HEADLESS=1`).
- **Vello Type Consistency**: Always import `kurbo` and `peniko` via `vello::kurbo` and `vello::peniko` to prevent dependency version conflicts.
- **Vello Clipping**: Always use `scene.push_layer(BlendMode::default(), 1.0, transform, &clip_shape)` paired with `scene.pop_layer()`.
- **Swapchain Presentation**: Pass `&SurfaceTexture` directly to `renderer.render_to_surface(&device, &queue, &scene, &surface_texture, &params)`.
