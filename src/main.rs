use vulkan::{ vulkan_init, game_loop };


extern crate vulkano_shaders;
extern crate winit;
extern crate vulkano_win;
extern crate vulkano;
pub mod vulkan;

#[tokio::main]
async fn main() {

    let (event_loop, device) = vulkan_init::initialize();
    game_loop::window_loop(event_loop, device);
    
}
