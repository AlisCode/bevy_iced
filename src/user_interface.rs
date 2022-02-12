use bevy::prelude::Component;
use iced_native::{user_interface::Cache, Size, UserInterface};

use crate::BevyIcedApplication;

#[derive(Component)]
pub struct IcedCache(Option<Cache>);

impl Default for IcedCache {
    fn default() -> Self {
        IcedCache(Some(Cache::new()))
    }
}

impl IcedCache {
    pub fn build_user_interface<'a, A: BevyIcedApplication>(
        &mut self,
        application: &'a mut A,
        renderer: &mut iced_wgpu::Renderer,
        size: Size,
    ) -> UserInterface<'a, A::Message, iced_wgpu::Renderer> {
        let cache = self.0.take().unwrap();
        let view = application.view();
        UserInterface::build(view, size, cache, renderer)
    }

    pub fn destroy_user_interface<Msg>(&mut self, ui: UserInterface<'_, Msg, iced_wgpu::Renderer>) {
        let cache = ui.into_cache();
        self.0 = Some(cache);
    }
}

// Persist (store as resource)
// * Renderer (backend)
// * Future executor
//
// Persist (store in world) per-UI:
// * Application (BevyIcedApplication implementor)
// * Cache
// * Size (bevy size)
// * WGPU Texture ?
//
// When we update :
// * Messages
// * UI events (click, keypress, drag, etc)
// * Clipboard
//
//
