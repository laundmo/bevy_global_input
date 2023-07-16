#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// Module providing the keyboard events and hotkey handler.
pub mod provide_mki;
/// Module providing the mouse events, position and movements.
pub mod provide_mouce;

pub use provide_mki::{GlobalHotkeyEvents, GlobalHotkeys, GlobalKeyEvents, GlobalKeys};
pub use provide_mouce::{
    GlobalMouseButtonEvents, GlobalMouseEvents, GlobalMousePos, GlobalScrollEvents, MouseControl,
};

/// PluginGroup for the mouse and keyboard plugins
pub struct GlobalInputPlugins;

impl PluginGroup for GlobalInputPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(provide_mki::KeyboardProvider)
            .add(provide_mouce::MousePosProvider)
    }
}
