mod application;

use bevy::{app::Plugin, render::renderer::RenderDevice};

pub use application::BevyIcedApplication;

#[derive(Debug, Default)]
pub struct IcedPlugin;

impl Plugin for IcedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let settings = iced_wgpu::Settings::default();
        let device = app
            .world
            .get_resource::<RenderDevice>()
            .expect("Failed to get RenderDevice");
        let backend = iced_wgpu::Backend::new(
            device.wgpu_device(),
            settings,
            wgpu::TextureFormat::R8Unorm, // TODO: don't pick at random, needs the wgpu Intance
        );
        let renderer = iced_wgpu::Renderer::new(backend);
        app.insert_non_send_resource(renderer);
    }
}
