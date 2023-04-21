mod app;
mod assets;
mod command_buffer;
mod depth_object;
mod descriptor_layout;
mod descriptor_pool;
mod framebuffer;
mod generate_mipmaps;
mod image_view;
mod instance;
mod logical_device;
mod metrics;
mod msaa;
mod physical_device;
mod pipeline;
mod render_pass;
mod single_time_cmd;
mod swapchain;
mod sync_object;
mod texture_image;
mod texture_sampler;
mod uniform_buffer;
mod vertex;
mod vertex_buffer;

use app::VulkanApp;

use anyhow::Result;
use vulkanalia::vk::DeviceV1_0;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run(window_title: &str) -> Result<()> {
    // Window

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(window_title)
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = VulkanApp::new(&window)?;
    // TEMP
    app.load_model_obj("viking_room")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !app.destroying && !app.minimized => {
                unsafe { app.render(&window) }.expect("Failed to render");
            }
            // Destroy our Vulkan app.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                app.destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe {
                    app.device
                        .device_wait_idle()
                        .expect("Failed to wait for device to idle");
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if size.width == 0 || size.height == 0 {
                    app.minimized = true;
                } else {
                    app.minimized = false;
                    app.resized = true;
                }
            }
            _ => {}
        }
    });
}
