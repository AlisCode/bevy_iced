use bevy::prelude::*;
use bevy_iced::{BevyIcedApplication, IcedPlugin};
use iced_native::{Alignment, Command};
use iced_wgpu::{Button, Column, Text};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(IcedPlugin)
        .run();
}

#[derive(Debug, Default)]
struct Counter {
    value: i32,
    increment_button: iced_native::widget::button::State,
    decrement_button: iced_native::widget::button::State,
}

#[derive(Clone, Debug)]
enum CounterMessage {
    Increment,
    Decrement,
}

impl BevyIcedApplication for Counter {
    type Message = CounterMessage;

    fn new() -> (Self, iced_native::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn view(&mut self) -> iced_native::Element<Self::Message, iced_wgpu::Renderer> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(CounterMessage::Increment),
            )
            .push(Text::new(self.value.to_string()).size(30))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(CounterMessage::Decrement),
            )
            .into()
    }
}
