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
        app.insert_resource(IcedRenderer::new(renderer));
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

pub struct IcedRenderer(Arc<Mutex<iced_wgpu::Renderer>>);

impl IcedRenderer {
    pub fn new(renderer: iced_wgpu::Renderer) -> Self {
        IcedRenderer(Arc::new(Mutex::new(renderer)))
    }
}
