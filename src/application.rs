use iced_native::{Command, Element};

struct Instance<A: BevyIcedApplication>(A);

pub trait BevyIcedApplication: Sized {
    type Message: std::fmt::Debug;

    fn new() -> (Self, Command<Self::Message>);

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message, iced_wgpu::Renderer>;
}
