use bevy::prelude::*;
use bevy_global_input::{
    GlobalInputPlugins, GlobalKeyEvents, GlobalMouseButtonEvents, GlobalScrollEvents,
};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(GlobalInputPlugins)
        .add_system(key_events)
        .add_system(mouse_button_events)
        .add_system(scroll_events)
        .run();
}

fn key_events(mut ev: EventReader<GlobalKeyEvents>) {
    for e in ev.iter() {
        dbg!(e);
    }
}

fn mouse_button_events(mut ev: EventReader<GlobalMouseButtonEvents>) {
    for e in ev.iter() {
        dbg!(e);
    }
}

fn scroll_events(mut ev: EventReader<GlobalScrollEvents>) {
    for e in ev.iter() {
        dbg!(e);
    }
}
