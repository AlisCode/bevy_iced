mod application;
mod convert;
mod render;
mod resources;
mod systems;
mod user_interface;

use std::sync::{Arc, Mutex};

use bevy::{
    app::Plugin,
    render::{
        render_graph::RenderGraph, renderer::RenderDevice, texture::BevyDefault, RenderApp,
        RenderStage,
    },
};
pub use iced_native::event::Event as IcedEvent;

pub use application::{BevyIcedApplication, Instance};
pub use iced_native::Command;
pub use resources::{IcedCursor, IcedPrimitives, IcedSize, IcedUiMessages};
pub use user_interface::IcedCache;

#[derive(Debug, Default)]
pub struct IcedPlugin;

impl Plugin for IcedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Shared resources
        let settings = iced_wgpu::Settings::default();
        let device = app
            .world
            .get_resource::<RenderDevice>()
            .expect("Failed to get RenderDevice");
        let backend = iced_wgpu::Backend::new(
            device.wgpu_device(),
            settings,
            wgpu::TextureFormat::bevy_default(),
        );
        let renderer = IcedRenderer::new(iced_wgpu::Renderer::new(backend));
        app.insert_resource(renderer.clone());
        app.insert_resource(IcedCursor::default());
        app.insert_resource(IcedPrimitives::default());
        app.add_event::<IcedEvent>();

        // Iced Rendering
        let render_app = app
            .get_sub_app_mut(RenderApp)
            .expect("Failed to get RenderApp");
        render_app.insert_resource(renderer);
        render_app.add_system_to_stage(RenderStage::Extract, systems::extract_iced_primitives);

        let mut render_graph = render_app
            .world
            .get_resource_mut::<RenderGraph>()
            .expect("Failed to get Render Graph");
        render::setup_iced_pipeline(&mut *render_graph);

        // Common systems
        app.add_system(systems::read_iced_event);
    }
}

pub struct WithApplicationType<A: BevyIcedApplication, P> {
    _app_type: std::marker::PhantomData<A>,
    inner: P,
}

impl<A: BevyIcedApplication + 'static, P: Plugin> Plugin for WithApplicationType<A, P> {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.inner.build(app);
        app.add_system(systems::update_iced_user_interface::<A>);
        app.add_event::<A::Message>();
    }
}

pub trait WithApplicationTypeExt: Sized {
    fn with_application_type<A: BevyIcedApplication>(self) -> WithApplicationType<A, Self> {
        WithApplicationType {
            _app_type: std::marker::PhantomData,
            inner: self,
        }
    }
}

impl<A: BevyIcedApplication, P: Plugin> WithApplicationTypeExt for WithApplicationType<A, P> {}
impl WithApplicationTypeExt for IcedPlugin {}

#[derive(Clone)]
pub struct IcedRenderer(Arc<Mutex<iced_wgpu::Renderer>>);

impl IcedRenderer {
    pub fn new(renderer: iced_wgpu::Renderer) -> Self {
        IcedRenderer(Arc::new(Mutex::new(renderer)))
    }
}
