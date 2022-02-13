use bevy::prelude::{Query, ResMut};
use iced_native::{Point, Size};

use crate::application::{BevyIcedApplication, Instance};
use crate::user_interface::IcedCache;
use crate::IcedRenderer;

pub fn update_iced_user_interface<A: BevyIcedApplication + 'static>(
    renderer: ResMut<IcedRenderer>,
    mut query: Query<(&mut Instance<A>, &mut IcedCache)>,
) {
    let mut renderer = renderer.0.lock().unwrap();
    for (mut instance, mut cache) in query.iter_mut() {
        let mut ui =
            cache.build_user_interface(&mut instance.0, &mut renderer, Size::new(1280., 720.));

        let events = []; // TODO: Fetch events
        let cursor_position = Point::new(0., 0.); // TODO: Store cursor position
        let mut clipboard = iced_native::clipboard::Null; // TODO: Handle clipboard
        let mut messages = vec![]; // TODO: Store messages
        ui.update(
            &events,
            cursor_position,
            &mut renderer,
            &mut clipboard,
            &mut messages,
        );

        let _mouse_interaction = ui.draw(&mut renderer, cursor_position);

        cache.destroy_user_interface(ui);
    }
}
