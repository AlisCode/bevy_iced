use bevy::prelude::Component;
use iced_native::{Command, Element};

#[derive(Debug, Component)]
pub struct IcedInstance<A: BevyIcedApplication>(pub A); // TODO: Probably need to erase that

impl<A: BevyIcedApplication> IcedInstance<A> {
    pub fn new(application: A) -> Self {
        IcedInstance(application)
    }
}

pub trait BevyIcedApplication: Sized + Send + Sync {
    /// Flags used to set the default state of the application
    type Flags: Send + Sync;
    /// Units of changes that are produced by the view or by external systems
    /// that will be used to update the state of this UI application
    type Message: std::fmt::Debug + Send + Sync;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>);

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message, iced_wgpu::Renderer>;
}
