use std::time::Duration;

use bevy::prelude::*;
use bevy_iced::{
    BevyIcedApplication, IcedCache, IcedPlugin, IcedUiMessages, Instance, WithApplicationTypeExt,
};
use futures_timer::Delay;
use iced_native::{widget::Container, Alignment, Command, Length};
use iced_wgpu::{Button, Column, Text};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(IcedPlugin.with_application_type::<DelayUi>())
        .add_startup_system(setup)
        .run();
}

#[derive(Debug)]
struct DelayUi {
    value: String,
    button: iced_native::widget::button::State,
}

impl Default for DelayUi {
    fn default() -> Self {
        DelayUi {
            value: "Click to start".to_string(),
            button: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
enum DelayMessage {
    Start,
    EndDelay,
}

impl BevyIcedApplication for DelayUi {
    type Message = DelayMessage;

    fn new() -> (Self, iced_native::Command<Self::Message>) {
        (DelayUi::default(), Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            DelayMessage::Start => {
                self.value = "Wait...".to_string();
                Command::perform(Delay::new(Duration::from_secs(1)), |_| {
                    DelayMessage::EndDelay
                })
            }
            DelayMessage::EndDelay => {
                self.value = "Done".to_string();
                Command::none()
            }
        }
    }

    fn view(&mut self) -> iced_native::Element<Self::Message, iced_wgpu::Renderer> {
        Container::new(
            Column::new()
                .padding(20)
                .align_items(Alignment::Center)
                .push(
                    Button::new(&mut self.button, Text::new("Start")).on_press(DelayMessage::Start),
                )
                .push(Text::new(self.value.to_string()).size(30)),
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
        .insert(Instance::new(DelayUi::default()))
        .insert(IcedUiMessages::<DelayMessage>::default())
        .insert(IcedCache::default());
}
