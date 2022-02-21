use bevy::prelude::Bundle;

use crate::{
    resources::IcedFlags, user_interface::IcedCache, BevyIcedApplication, IcedInstance, IcedSize,
    IcedUiMessages,
};

#[derive(Bundle)]
/// Private bundle to spawn all bevy_iced internals that we want to hide
/// to the user for simplicity
pub(crate) struct PrivateIcedBundle<A: BevyIcedApplication + 'static> {
    pub(crate) instance: IcedInstance<A>,
    pub(crate) cache: IcedCache,
    pub(crate) messages: IcedUiMessages<A::Message>,
}

#[derive(Bundle)]
pub struct IcedBundle<A: BevyIcedApplication + 'static> {
    flags: IcedFlags<A>,
    size: IcedSize,
}

impl<A: BevyIcedApplication> Default for IcedBundle<A>
where
    A::Flags: Default,
{
    fn default() -> Self {
        IcedBundle {
            flags: IcedFlags::default(),
            size: IcedSize::default(),
        }
    }
}
