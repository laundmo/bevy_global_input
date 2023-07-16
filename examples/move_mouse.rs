use bevy::prelude::*;
use bevy_global_input::{
    provide_mouce::MouseControl, GlobalHotkeyEvents, GlobalHotkeys, GlobalInputPlugins, GlobalKeys,
};
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(GlobalInputPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, system)
        .run();
}

fn setup(mut hotkeys: ResMut<GlobalHotkeys>) {
    // adds a CTRL+Shit+Space global hotkey
    hotkeys.add(
        "ToggleMovement".to_string(),
        &[
            GlobalKeys::LeftControl,
            GlobalKeys::LeftShift,
            GlobalKeys::Space,
        ],
    );
    println!("Setup done! \n\n\t Press CTRL+Shift+Space to toggle movement.\n\n");
}

fn system(
    mut hotkey: EventReader<GlobalHotkeyEvents>,
    mut moving: Local<bool>,
    mut moves: EventWriter<MouseControl>,
    time: Res<Time>,
) {
    for e in hotkey.iter() {
        if e.0 == "ToggleMovement" {
            *moving = !*moving;
        }
    }
    if !*moving && time.elapsed().as_millis() % 6 == 0 {
        moves.send(MouseControl::MoveRelative(-1, 0));
    }
}
