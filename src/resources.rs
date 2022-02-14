use crossbeam_channel::{Receiver, Sender};

use bevy::prelude::Component;
use iced_native::Point;

#[derive(Default)]
pub struct IcedCursor(pub Point);

#[derive(Component)]
pub struct IcedUiMessages<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

impl<T> Default for IcedUiMessages<T> {
    fn default() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        IcedUiMessages { tx, rx }
    }
}
