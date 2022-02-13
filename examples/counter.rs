use bevy::prelude::*;
use bevy_iced::{BevyIcedApplication, IcedCache, IcedPlugin, Instance, WithApplicationTypeExt};
use iced_native::{widget::Container, Alignment, Command, Length};
use iced_wgpu::{Button, Column, Text};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(IcedPlugin.with_application_type::<Counter>())
        .add_startup_system(setup)
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
        Container::new(
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
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Instance::new(Counter::default()))
        .insert(IcedCache::default());
}
