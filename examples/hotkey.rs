use bevy::prelude::*;
use bevy_global_input::{GlobalHotkeyEvents, GlobalHotkeys, GlobalInputPlugins, GlobalKeys};
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(GlobalInputPlugins)
        .add_startup_system(setup)
        .add_system(system)
        .run();
}

fn setup(mut hotkeys: ResMut<GlobalHotkeys>) {
    // adds a CTRL+Shit+Space global hotkey
    hotkeys.add(
        "PrintSomething",
        &[
            GlobalKeys::LeftControl,
            GlobalKeys::LeftShift,
            GlobalKeys::Space,
        ],
    );
}

fn system(mut ev: EventReader<GlobalHotkeyEvents>) {
    for e in ev.iter() {
        if e.0 == "PrintSomething" {
            println!("PrintSomething hotkey event received");
        }
    }
}
