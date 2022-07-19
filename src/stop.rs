use vulkano_win::VkSurfaceBuild;

use vulkano::instance::{Instance, InstanceCreateInfo};

use winit::event_loop::EventLoop;

fn main() {
    let required_extexnsions = vulkano_win::required_extensions();

    let instance = Instance::new(InstanceCreateInfo {
        enabled_extensions: required_extexnsions,
        ..Default::default()
    })
    .unwrap();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();
}
