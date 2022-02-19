use bevy::render::{
    render_graph::{Node, RenderGraph},
    view::ExtractedWindows,
};
use iced_native::Size;
use iced_wgpu::Viewport;
use wgpu::util::StagingBelt;

use crate::{IcedPrimitives, IcedRenderer};

const ICED_UI_PASS: &'static str = "iced_ui_pass";

pub(crate) fn setup_iced_pipeline(render_graph: &mut RenderGraph) {
    render_graph.add_node(ICED_UI_PASS, IcedNode::default());
    render_graph
        .add_node_edge(bevy::core_pipeline::node::MAIN_PASS_DRIVER, ICED_UI_PASS)
        .expect("Failed to add iced_ui_pass to the render graph");
}

#[derive(Default)]
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

        let mut staging_belt = StagingBelt::new(1024); // TODO: persist stagingbelt ?

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

        // TODO: Recall staging belt?
        // (needs async runtime)

        Ok(())
    }
}
