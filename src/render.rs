use std::sync::Mutex;

use bevy::render::{
    render_graph::{Node, RenderGraph},
    view::ExtractedWindows,
};
use iced_native::{
    futures::{executor::LocalPool, task::SpawnExt},
    Size,
};
use iced_wgpu::Viewport;
use wgpu::util::StagingBelt;

use crate::{IcedPrimitives, IcedRenderer};

const ICED_UI_PASS: &'static str = "iced_ui_pass";

pub(crate) fn setup_iced_pipeline(render_graph: &mut RenderGraph) {
    render_graph.add_node(ICED_UI_PASS, IcedNode::new());
    render_graph
        .add_node_edge(bevy::core_pipeline::node::MAIN_PASS_DRIVER, ICED_UI_PASS)
        .expect("Failed to add iced_ui_pass to the render graph");
}

pub struct IcedNode {
    staging_belt: Mutex<StagingBelt>,
}

impl IcedNode {
    pub fn new() -> Self {
        let staging_belt = Mutex::new(StagingBelt::new(1024));
        IcedNode { staging_belt }
    }
}

impl Node for IcedNode {
    fn update(&mut self, _world: &mut bevy::prelude::World) {
        // Recall the staging belt in update to avoid blocking when rendering
        // Can't use bevy's task pool from the render world, so we're spawning a local pool like in
        // the examples
        let mut staging_belt = self.staging_belt.lock().expect("Failed to get StagingBelt");
        let mut pool = LocalPool::new();
        pool.spawner()
            .spawn(staging_belt.recall())
            .expect("Failed to recall staging belt");
        pool.run_until_stalled();
    }

    fn run(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let renderer = world.get_resource::<IcedRenderer>().unwrap();
        let mut renderer = renderer.0.lock().unwrap();
        let device = &render_context.render_device;

        let window = world
            .get_resource::<ExtractedWindows>()
            .unwrap()
            .windows
            .values()
            .last()
            .unwrap();
        let primitives_res = world.get_resource::<IcedPrimitives>().unwrap();
        let primitives = primitives_res.0.lock().unwrap();
        let texture_view = window.swap_chain_texture.as_ref().unwrap();

        let mut staging_belt = self.staging_belt.lock().expect("Failed to get StagingBelt");

        let size = Size::new(window.physical_width, window.physical_height);
        let viewport = Viewport::with_physical_size(size, 1.);
        // TODO: this should be called in a "render" system and store primitives in a dedicated
        // (hidden) component
        let backend = renderer.backend_mut();
        for primitives in &*primitives {
            backend.present::<&str>(
                device.wgpu_device(),
                &mut staging_belt,
                &mut render_context.command_encoder,
                &texture_view,
                primitives,
                &viewport,
                &[], // TODO: Support overlay ?
            );
        }
        staging_belt.finish();

        Ok(())
    }
}
