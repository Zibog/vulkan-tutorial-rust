#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use std::collections::HashSet;

use anyhow::{Error, Ok, Result, anyhow};
use log::info;
use vulkanalia::{Entry, Instance, Version, loader::{LIBRARY, LibloadingLoader}, vk::{self, EntryV1_0, HasBuilder, InstanceV1_0, layer}};
use vulkanalia::window as vk_window;
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::EventLoop, window::{Window, WindowAttributes}};

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

/// Validation is enabled if the app is being compiled in debug mode
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
/// Useful standard validations are bundled in VK_LAYER_KHRONOS_validation
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

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
struct App {
    entry: Entry,
    instance: Instance,
}

impl App {
    /// Creates our Vulkan app
    unsafe fn create(window: &Window) -> Result<Self, Error> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }

    /// Renders a frame for our Vulkan app
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app
    unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }
}

/// The Vulkan handles and associated properties used by our Vulkan app
#[derive(Clone, Debug, Default)]
struct AppData {}

/// Creates Vulkan instance for specified winit window and Vulkan entry point
unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance, Error> {
    // App info with names of the app and engine, app, engine and api versions (technically optional)
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial\0")
        .application_version(vk::make_version(0, 1, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(0, 1, 0))
        .api_version(vk::make_version(0, 1, 0));

    // Collect supported app layers into HashSet
    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    // Check if validation layer was loaded successfully
    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layer requested but not supported"));
    }

    // Convert collected layers to null-terminated C strings
    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

    // Collect instance extensions which we want to use in Vulkan and convert them to null-terminated C strings
    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216 as it's not fully support Vulkan specifications
    let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        info!("Enabling extensions for macOS portability");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    // Build our instance info for Vulkan
    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);

    Ok(entry.create_instance(&info, None)?)
}
