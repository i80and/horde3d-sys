# Low-level [Horde3D](http://www.horde3d.org/) bindings for [Rust](https://www.rust-lang.org/)

See [crates.io](https://crates.io/crates/horde3d-sys).

This crate exposes the raw Horde3D API, so the [Horde3D manual](http://www.horde3d.org/docs/manual.html) applies.

## Requirements

* build-essentials (gcc, etc.)

## Example

```rust
extern crate glutin;
extern crate horde3d_sys;

use glutin::GlContext;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Dungeon")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_gl(glutin::GlRequest::Latest)
        .with_gl_profile(glutin::GlProfile::Compatibility);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.context().make_current().unwrap();
        eprintln!("{:?}", horde3d_sys::h3dInit(horde3d_sys::H3DRenderDevice::OpenGL4));
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

        gl_window.swap_buffers().unwrap();
    }

    unsafe {
        horde3d_sys::h3dRelease();
    }
}
```
