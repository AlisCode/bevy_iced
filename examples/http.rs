use bevy::prelude::*;
use bevy_iced::{
    BevyIcedApplication, IcedCache, IcedPlugin, IcedSize, IcedUiMessages, Instance,
    WithApplicationTypeExt,
};
use iced_native::{
    widget::{Container, TextInput},
    Alignment, Command, Length,
};
use iced_wgpu::{Button, Column, Text};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(IcedPlugin.with_application_type::<HttpExample>())
        .add_startup_system(setup)
        .run();
}

#[derive(Debug)]
struct HttpExample {
    value: String,
    url: String,
    button: iced_native::widget::button::State,
    url_input: iced_native::widget::text_input::State,
}

#[derive(Clone, Debug)]
enum HttpExampleMessage {
    Start,
    SetResult(String),
    SetUrl(String),
}

impl Default for HttpExample {
    fn default() -> Self {
        HttpExample {
            value: "Click to fetch".to_string(),
            url: "".to_string(),
            button: Default::default(),
            url_input: Default::default(),
        }
    }
}

impl BevyIcedApplication for HttpExample {
    type Message = HttpExampleMessage;

    fn new() -> (Self, iced_native::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<HttpExampleMessage> {
        match message {
            HttpExampleMessage::Start => {
                self.value = "Fetching...".to_string();
                Command::perform(surf::get(&self.url).recv_string(), |res| {
                    HttpExampleMessage::SetResult(res.unwrap())
                })
            }
            HttpExampleMessage::SetResult(new) => {
                self.value = new;
                Command::none()
            }
            HttpExampleMessage::SetUrl(new) => {
                self.url = new;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> iced_native::Element<Self::Message, iced_wgpu::Renderer> {
        Container::new(
            Column::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(
                    TextInput::new(
                        &mut self.url_input,
                        "URL...",
                        &self.url,
                        HttpExampleMessage::SetUrl,
                    )
                    .max_width(400),
                )
                .push(
                    Button::new(&mut self.button, Text::new("Increment"))
                        .on_press(HttpExampleMessage::Start),
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
        .insert(Instance::new(HttpExample::default()))
        .insert(IcedUiMessages::<HttpExampleMessage>::default())
        .insert(IcedSize::default())
        .insert(IcedCache::default());
}
