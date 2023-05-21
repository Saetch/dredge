use std::sync::Arc;

use vulkano::{sync::{self, GpuFuture, event}, device::Device, swapchain::{SwapchainCreateInfo, SwapchainCreationError, Swapchain}};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::Window};

use super::vulkan_init::window_size_dependent_setup;

pub fn window_loop(event_loop: EventLoop<()>, device: Arc<Device>){
    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                recreate_swapchain = true;
            }
            Event::RedrawEventsCleared => {
                previous_frame_end
                .as_mut()
                .take()
                .unwrap()
                .cleanup_finished();
            },
            _ => {}
        }
      });
}



#[inline(always)]
fn check_swapchain(recreate_swapchain: &mut bool){
    if *recreate_swapchain {
        let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
        let image_extent: [u32; 2] = window.inner_size().into();
    
        let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
            image_extent,
            ..swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };
    
        swapchain = new_swapchain;
        framebuffers =
            window_size_dependent_setup(&new_images, render_pass.clone(), &mut viewport);
        *recreate_swapchain = false;
    }
}