use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use anyhow::{Result, anyhow};
use log::*;
use vulkanalia::Version;
use vulkanalia::loader::{LIBRARY, LibloadingLoader};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;

use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;
use vulkanalia::vk::ExtDebugUtilsExtension;

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };

    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() => {
                    unsafe { app.render(&window) }.unwrap()
                }
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe {
                        app.destroy();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })?;

    Ok(())
}

/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        unsafe {
            let loader = LibloadingLoader::new(LIBRARY)?;
            let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
            let instance = App::create_instance(window, &entry)?;
            Ok(Self { entry, instance })
        }
    }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }

    unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
        unsafe {
            let application_info = vk::ApplicationInfo::builder()
                .application_name(b"Vulkan Tutorial\0")
                .application_version(vk::make_version(1, 0, 0))
                .engine_name(b"No Engine\0")
                .engine_version(vk::make_version(1, 0, 0))
                .api_version(vk::make_version(1, 0, 0));

            let available_layers = entry
                .enumerate_instance_layer_properties()?
                .iter()
                .map(|l| l.layer_name)
                .collect::<HashSet<_>>();

            if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
                return Err(anyhow!("Validation layer requested but not supported."));
            }

            let layers = if VALIDATION_ENABLED {
                vec![VALIDATION_LAYER.as_ptr()]
            } else {
                Vec::new()
            };

            let mut extensions = vk_window::get_required_instance_extensions(window)
                .iter()
                .map(|e| e.as_ptr())
                .collect::<Vec<_>>();

            // Required by Vulkan SDK on macOS since 1.3.216.
            let flags =
                if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
                    info!("Enabling extensions for macOS portability.");
                    extensions.push(
                        vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION
                            .name
                            .as_ptr(),
                    );
                    extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
                    vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
                } else {
                    vk::InstanceCreateFlags::empty()
                };

            let info = vk::InstanceCreateInfo::builder()
                .application_info(&application_info)
                .enabled_extension_names(&extensions)
                .enabled_layer_names(&layers)
                .flags(flags);
            Ok(entry.create_instance(&info, None)?)
        }
    }
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
