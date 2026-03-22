# CAD Viewer — Project 1

Part of the [AEC Software Development Roadmap](../aec-software-roadmap/ROADMAP.md).

## Purpose

Learn the full rendering pipeline: from loading a 3D file to pixels on screen.
No black boxes. This is a focused learning project, not a production tool.

## Scope

- Load 3D models in OBJ or glTF format
- Render geometry using `wgpu` (vertex buffers, index buffers, basic shader)
- Camera controls: arcball rotation, zoom, pan
- Minimal `egui` UI: file open dialog + 3D viewport

**Out of scope for this project**: IFC/Revit file loading, metadata display,
measurements. Those belong to Project 2 (IFC Parser / Query Engine).

## Tech Stack

- **Language**: Rust
- **Rendering**: `wgpu`
- **Windowing**: `winit`
- **UI**: `egui`
- **Build**: Cargo

## Success Metric

Understand exactly what happens between "load file" and "pixel on screen."

## Architecture

```
src/
  main.rs         -- entry point, app loop
  app.rs          -- top-level application state
  renderer/       -- wgpu setup, render pipeline, shaders
  camera.rs       -- arcball camera, input handling
  model.rs        -- mesh data structures, OBJ/glTF loading
  ui.rs           -- egui panels and file dialog
```
