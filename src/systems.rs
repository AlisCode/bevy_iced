use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, MouseButton, Query, Res, ResMut, With, World,
};
use bevy::tasks::IoTaskPool;
use bevy::window::{CursorEntered, CursorLeft, CursorMoved, ReceivedCharacter, Windows};
use iced_native::command::Action;
use iced_native::keyboard::Modifiers;
use iced_native::{Command, Event as IcedEvent, Point};

use crate::application::{BevyIcedApplication, IcedInstance};
use crate::bundles::PrivateIcedBundle;
use crate::resources::{IcedFlags, IcedPrimitives, IcedSize, IcedUiMessages};
use crate::user_interface::IcedCache;
use crate::{IcedCursor, IcedRenderer};
use iced_native::futures::FutureExt;

pub fn spawn_iced_user_interface<A: BevyIcedApplication + 'static>(world: &mut World) {
    let mut query = world.query_filtered::<Entity, With<IcedFlags<A>>>();
    let entities: Vec<Entity> = query.iter(world).collect();
    for entity in entities {
        let mut entity = world.entity_mut(entity);
        let flags: IcedFlags<A> = entity.remove().expect("Must have IcedFlags<A> here");

        let (app, _cmd) = A::new(flags.flags); // TODO: use cmd
        let instance = IcedInstance(app);
        let messages = IcedUiMessages::<A::Message>::default();

        entity.insert_bundle(PrivateIcedBundle {
            instance,
            cache: IcedCache::default(),
            messages,
        });
    }
}

pub fn update_iced_user_interface<A: BevyIcedApplication + 'static>(
    renderer: ResMut<IcedRenderer>,
    cursor: Res<IcedCursor>,
    io_task_pool: Res<IoTaskPool>,
    windows: Res<Windows>,
    mut iced_events: EventReader<IcedEvent>,
    iced_primitives: ResMut<IcedPrimitives>,
    mut query: Query<(
        &mut IcedInstance<A>,
        &mut IcedCache,
        &IcedUiMessages<A::Message>,
        &IcedSize,
    )>,
) {
    let mut primitives = iced_primitives.0.lock().unwrap();
    primitives.clear();
    let window = windows
        .get_primary()
        .expect("Failed to find primary window"); // TODO: support multiple windows
    let events: Vec<IcedEvent> = iced_events.iter().cloned().collect();
    let cursor_position = cursor.0;
    let mut renderer = renderer.0.lock().unwrap();
    for (mut instance, mut cache, ui_messages, size) in query.iter_mut() {
        let mut ui =
            cache.build_user_interface(&mut instance.0, &mut renderer, size.resolve(window));

        let mut clipboard = iced_native::clipboard::Null; // TODO: Handle clipboard
        let mut messages = ui_messages.pending_messages();
        ui.update(
            &events,
            cursor_position,
            &mut renderer,
            &mut clipboard,
            &mut messages,
        );

        let _mouse_interaction = ui.draw(&mut renderer, cursor_position);
        cache.destroy_user_interface(ui);

        let commands: Vec<Command<A::Message>> = messages
            .into_iter()
            .map(|msg| instance.0.update(msg))
            .collect();

        let actions: Vec<_> = commands
            .into_iter()
            .flat_map(|c| c.actions().into_iter())
            .collect();

        for action in actions {
            match action {
                Action::Future(fut) => {
                    let tx = ui_messages.clone_tx();
                    let future = fut.then(|msg| async move {
                        tx.send(msg).unwrap();
                    });
                    io_task_pool.spawn(future).detach(); // TODO maybe store the tasks somewhere ?
                }
                Action::Window(_) => (),
                Action::Clipboard(_) => (),
            }
        }

        primitives.push(renderer.take_primitives());
    }
}

/// Extracts all iced primitives to the rendering subapp
pub fn extract_iced_primitives(mut commands: Commands, primitives: Res<IcedPrimitives>) {
    commands.insert_resource(primitives.clone())
}

pub fn read_iced_event(
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_entered_events: EventReader<CursorEntered>,
    mut cursor_left_events: EventReader<CursorLeft>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut received_char_events: EventReader<ReceivedCharacter>,
    mut cursor_position: ResMut<IcedCursor>,
    mut iced_events: EventWriter<IcedEvent>,
) {
    let window = windows
        .get_primary()
        .expect("Failed to find primary window");
    let mut events = Vec::new();

    for ev in cursor_moved_events.iter() {
        cursor_position.0.y = window.height() - ev.position.y;
        events.push(IcedEvent::Mouse(iced_native::mouse::Event::CursorMoved {
            position: Point::new(ev.position.x, cursor_position.0.y),
        }));
        cursor_position.0.x = ev.position.x;
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

    for ev in keyboard_events.iter() {
        let pressed = ev.state.is_pressed();
        let modifiers = Modifiers::default();
        if let Some(key_code) = ev.key_code {
            // convert into iced key_code
            let key_code = crate::convert::convert_keycode(key_code);
            if pressed {
                events.push(IcedEvent::Keyboard(
                    iced_native::keyboard::Event::KeyPressed {
                        key_code,
                        modifiers, // TODO: Handle modifiers
                    },
                ));
            } else {
                events.push(IcedEvent::Keyboard(
                    iced_native::keyboard::Event::KeyReleased {
                        key_code,
                        modifiers, // TODO: Handle modifiers
                    },
                ));
            }
        }
    }

    for ev in received_char_events.iter() {
        events.push(IcedEvent::Keyboard(
            iced_native::keyboard::Event::CharacterReceived(ev.char),
        ));
    }

    iced_events.send_batch(events.into_iter());
}
