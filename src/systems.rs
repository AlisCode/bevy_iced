use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{EventReader, EventWriter, MouseButton, Query, Res, ResMut};
use bevy::window::{CursorEntered, CursorLeft, CursorMoved};
use iced_native::{Command, Event as IcedEvent, Point, Size};

use crate::application::{BevyIcedApplication, Instance};
use crate::user_interface::IcedCache;
use crate::{IcedCursor, IcedRenderer};

pub fn update_iced_user_interface<A: BevyIcedApplication + 'static>(
    renderer: ResMut<IcedRenderer>,
    cursor: Res<IcedCursor>,
    mut iced_events: EventReader<IcedEvent>,
    mut query: Query<(&mut Instance<A>, &mut IcedCache)>,
) {
    let events: Vec<IcedEvent> = iced_events.iter().cloned().collect();
    let cursor_position = cursor.0;
    let mut renderer = renderer.0.lock().unwrap();
    for (mut instance, mut cache) in query.iter_mut() {
        let mut ui =
            cache.build_user_interface(&mut instance.0, &mut renderer, Size::new(1280., 720.));

        let mut clipboard = iced_native::clipboard::Null; // TODO: Handle clipboard
        let mut messages = Vec::new();
        ui.update(
            &events,
            cursor_position,
            &mut renderer,
            &mut clipboard,
            &mut messages,
        );

        let _mouse_interaction = ui.draw(&mut renderer, cursor_position);
        cache.destroy_user_interface(ui);

        let _commands: Vec<Command<A::Message>> = messages
            .into_iter()
            .map(|msg| instance.0.update(msg))
            .collect();
        // TODO: execute commands
    }
}

pub fn read_iced_event(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_entered_events: EventReader<CursorEntered>,
    mut cursor_left_events: EventReader<CursorLeft>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut cursor_position: ResMut<IcedCursor>,
    mut iced_events: EventWriter<IcedEvent>,
) {
    let mut events = Vec::new();

    for ev in cursor_moved_events.iter() {
        events.push(IcedEvent::Mouse(iced_native::mouse::Event::CursorMoved {
            position: Point::new(ev.position.x, 720. - ev.position.y), // TODO: Fix computation
        }));
        cursor_position.0.x = ev.position.x;
        cursor_position.0.y = 720. - ev.position.y; // TODO: Fix computation
    }

    for _ev in cursor_entered_events.iter() {
        events.push(IcedEvent::Mouse(
            iced_native::mouse::Event::CursorEntered {},
        ));
    }

    for _ev in cursor_left_events.iter() {
        events.push(IcedEvent::Mouse(iced_native::mouse::Event::CursorLeft {}));
    }

    for ev in mouse_button_events.iter() {
        let button = match ev.button {
            MouseButton::Left => Some(iced_native::mouse::Button::Left),
            MouseButton::Right => Some(iced_native::mouse::Button::Right),
            MouseButton::Middle => Some(iced_native::mouse::Button::Middle),
            _ => None,
        };
        let pressed = ev.state.is_pressed();
        if let Some(button) = button {
            if pressed {
                events.push(IcedEvent::Mouse(iced_native::mouse::Event::ButtonPressed(
                    button,
                )));
            } else {
                events.push(IcedEvent::Mouse(iced_native::mouse::Event::ButtonReleased(
                    button,
                )));
            }
        }
    }

    iced_events.send_batch(events.into_iter());
}
