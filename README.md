# Mini CAD Viewer - Project Outline

## 1. Project Overview
- **Purpose**: Create a cross-platform, high-performance CAD viewer using Rust.
- **Scope**: Focus on viewing Revit or IFC-based CAD models, with a responsive UI and GPU-accelerated rendering.

## 2. Requirements
- **Functional Requirements**:
  - Load and display IFC/Revit models.
  - Cross-platform compatibility (Windows, macOS, Linux).
  - Smooth 3D navigation (zoom, pan, rotate).
  - Basic measurements and metadata display.
- **Non-functional Requirements**:
  - High performance (GPU-accelerated rendering).
  - Lightweight and modular design.
  - Responsive UI with minimal latency.

## 3. Tech Stack
- **Language**: Rust
- **GUI Framework**: `egui` or `druid` (for cross-platform GUI)
- **3D Rendering**: `wgpu` (Rust GPU abstraction)
- **CAD File Loading**: Rust bindings for `Assimp` (3D model loader) or integration with C++ library (e.g., Open Design Alliance Teigha)
- **Build System**: Cargo (Rust’s package manager)
- **Cross-Platform Support**: `winit` (windowing abstraction)

## 4. Design & Architecture
- **GUI Layer**:  
  - Define UI components (file loader, 3D view pane, metadata panel).
  - Handle user interactions (clicks, drag, zoom).
- **Model Layer**:
  - CAD model structure (vertices, edges, metadata).
  - File parsing and conversion from IFC/Revit to a renderable format.
- **Rendering Layer**:  
  - Use `wgpu` to render 3D geometry.
  - Camera controls (zoom, rotate).
- **Performance**:
  - Optimize data transfer to GPU.
  - Asynchronous file loading (to avoid blocking UI).

## 5. Implementation Plan
- **Phase 1**: Set up project structure with Cargo.
- **Phase 2**: Build basic UI (file menu, 3D viewport).
- **Phase 3**: Integrate CAD file loader (test with simple 3D models).
- **Phase 4**: Implement 3D rendering using `wgpu`.
- **Phase 5**: Add navigation (zoom, pan, rotate) and metadata display.
- **Phase 6**: Cross-platform testing and optimization.

## 6. Milestones & Timeline
- **Week 1**: Project setup, basic UI layout.
- **Week 2**: Integrate simple 3D model loader.
- **Week 3**: Basic rendering pipeline with `wgpu`.
- **Week 4**: Navigation and UI refinement.
- **Week 5**: Performance tuning and cross-platform testing.

## 7. Risks & Mitigations
- **Risk**: Complexity of parsing large CAD files.
  - **Mitigation**: Use incremental loading or simpler model formats first.
- **Risk**: Cross-platform rendering differences.
  - **Mitigation**: Use a well-supported abstraction like `wgpu`.

## 8. Future Enhancements
- Support for additional file formats (e.g., OBJ, STEP).
- Interactive annotations on models.
- Cloud synchronization for CAD projects.

## 9. References
- `wgpu` Documentation: https://wgpu.rs
- `egui` Guide: https://docs.rs/egui
- Assimp 3D Model
