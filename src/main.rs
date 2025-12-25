#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::{Error, Ok, Result};
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::EventLoop, window::{Window, WindowAttributes}};

fn main() -> Result<()> {
    pretty_env_logger::init();
    
    // Window

    let event_loop = EventLoop::builder().build().unwrap();
    let window_attributes = WindowAttributes::default()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768));
    let window = event_loop.create_window(window_attributes).unwrap();

    // App

    let mut app = unsafe { App::create(&window)? };
    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() => unsafe { app.render(&window) }.unwrap(),
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe { app.destroy(); }
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}

/// Our Vulkan app
#[derive(Clone, Debug)]
struct App {}

impl App {
    /// Creates our Vulkan app
    unsafe fn create(window: &Window) -> Result<Self, Error> {
        Ok(Self {})
    }

    /// Renders a frame for our Vulkan app
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app
    unsafe fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app
#[derive(Clone, Debug, Default)]
struct AppData {}
