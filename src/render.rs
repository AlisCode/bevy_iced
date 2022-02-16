use bevy::{
    prelude::{Entity, With},
    render::{
        render_graph::{Node, RenderGraph},
        view::ExtractedWindows,
    },
    utils::HashSet,
};
use iced_native::Size;
use iced_wgpu::Viewport;
use wgpu::util::StagingBelt;

use crate::{IcedRenderer, IcedSize};

const ICED_UI_PASS: &'static str = "iced_ui_pass";

pub(crate) fn setup_iced_pipeline(render_graph: &mut RenderGraph) {
    render_graph.add_node(ICED_UI_PASS, IcedNode::default());
    render_graph
        .add_node_edge(bevy::core_pipeline::node::MAIN_PASS_DRIVER, ICED_UI_PASS)
        .expect("Failed to add iced_ui_pass to the render graph");
}

#[derive(Default)]
pub struct IcedNode {
    entities: HashSet<Entity>,
}

impl Node for IcedNode {
    fn update(&mut self, world: &mut bevy::prelude::World) {
        let mut query = world.query::<(Entity, With<IcedSize>)>();
        let new_entities = query.iter(&world).map(|(entity, _)| entity).collect();
        self.entities = new_entities;
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
        let texture_view = window.swap_chain_texture.as_ref().unwrap();

        let mut staging_belt = StagingBelt::new(1024); // TODO: persist stagingbelt ?

        let size = Size::new(window.physical_width, window.physical_height);
        let viewport = Viewport::with_physical_size(size, 1.);
        // TODO: this should be called in a "render" system and store primitives in a dedicated
        // (hidden) component
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
