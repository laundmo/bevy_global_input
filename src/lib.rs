#![doc = include_str!("../README.md")]

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod provide_mki;
pub mod provide_mouce;

pub use provide_mki::{GlobalHotkeyEvents, GlobalHotkeys, GlobalKeyEvents, GlobalKeys};
pub use provide_mouce::{
    GlobalMouseButtonEvents, GlobalMouseEvents, GlobalMousePos, GlobalScrollEvents,
};

pub struct GlobalInputPlugins;

impl PluginGroup for GlobalInputPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(provide_mki::KeyboardProvider)
            .add(provide_mouce::MousePosProvider)
    }
}
