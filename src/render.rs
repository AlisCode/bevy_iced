use bevy::render::{
    render_graph::{Node, RenderGraph},
    view::ExtractedWindows,
};
use iced_native::Size;
use iced_wgpu::Viewport;
use wgpu::util::StagingBelt;

use crate::IcedRenderer;

const ICED_UI_PASS: &'static str = "iced_ui_pass";

pub(crate) fn setup_iced_pipeline(render_graph: &mut RenderGraph) {
    render_graph.add_node(ICED_UI_PASS, IcedNode);
    render_graph
        .add_node_edge(bevy::core_pipeline::node::MAIN_PASS_DRIVER, ICED_UI_PASS)
        .expect("Failed to add iced_ui_pass to the render graph");
}

pub struct IcedNode;

impl Node for IcedNode {
    fn run(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let renderer = world.get_resource::<IcedRenderer>().unwrap();
        let mut renderer = renderer.0.lock().unwrap();
        let device = &render_context.render_device;

        let texture_view = world
            .get_resource::<ExtractedWindows>()
            .unwrap()
            .windows
            .values()
            .last()
            .unwrap()
            .swap_chain_texture
            .as_ref()
            .unwrap();

        let mut staging_belt = StagingBelt::new(1024); // TODO: persist stagingbelt
        let viewport = Viewport::with_physical_size(Size::new(1280, 720), 1.); // TODO: persist Viewport / handle resize

        renderer.with_primitives(|backend, primitives| {
            backend.present::<&str>(
                device.wgpu_device(),
                &mut staging_belt,
                &mut render_context.command_encoder,
                &texture_view,
                primitives,
                &viewport,
                &[], // TODO: Support overlay ?
            );
        });

        // TODO: Recall staging belt?
        // (needs async runtime)

        Ok(())
    }
}
