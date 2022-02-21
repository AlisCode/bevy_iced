use std::sync::{Arc, Mutex};

use crossbeam_channel::{Receiver, Sender};

use bevy::{
    prelude::{Component, Size},
    window::Window,
};
use iced_native::{Point, Size as IcedNativeSize};

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

#[derive(Clone, Default, Component)]
/// TODO: Should probably also store Overlay ?
pub struct IcedPrimitives(pub(crate) Arc<Mutex<Vec<Vec<iced_wgpu::Primitive>>>>);

#[derive(Component)]
pub enum IcedSize {
    Fullscreen,
    Fixed(Size<f32>),
}

impl Default for IcedSize {
    fn default() -> IcedSize {
        IcedSize::Fullscreen
    }
}

impl IcedSize {
    pub fn resolve(&self, window: &Window) -> IcedNativeSize<f32> {
        match self {
            IcedSize::Fullscreen => IcedNativeSize::new(window.width(), window.height()),
            IcedSize::Fixed(size) => crate::convert::size(size.clone()),
        }
    }

    /// Returns (size, scale_factor)
    pub fn resolve_physical(&self, window: &Window) -> (IcedNativeSize<u32>, f64) {
        match self {
            IcedSize::Fullscreen => (
                IcedNativeSize::new(window.physical_width(), window.physical_height()),
                window.scale_factor(),
            ),
            IcedSize::Fixed(size) => (crate::convert::size_u32(size.clone()), 1.),
        }
    }
}

#[derive(Debug, Component)]
pub struct IcedFlags<A: BevyIcedApplication> {
    pub(crate) flags: A::Flags,
}

impl<A: BevyIcedApplication> Default for IcedFlags<A>
where
    A::Flags: Default,
{
    fn default() -> Self {
        IcedFlags {
            flags: Default::default(),
        }
    }
}
