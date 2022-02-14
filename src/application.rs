use bevy::prelude::Component;
use iced_native::{Command, Element};

#[derive(Debug, Component)]
pub struct Instance<A: BevyIcedApplication>(pub A); // TODO: Probably need to erase that

impl<A: BevyIcedApplication> Instance<A> {
    pub fn new(application: A) -> Self {
        Instance(application)
    }
}

pub trait BevyIcedApplication: Sized + Send + Sync {
    type Message: std::fmt::Debug + Send + Sync;

    fn new() -> (Self, Command<Self::Message>);

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message, iced_wgpu::Renderer>;
}
