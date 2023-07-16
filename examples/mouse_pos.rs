use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_global_input::{GlobalInputPlugins, GlobalMousePos};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(GlobalInputPlugins)
        .add_systems(
            Update,
            mouse_pos.run_if(on_timer(Duration::from_secs_f32(0.5))),
        )
        .run();
}

fn mouse_pos(pos: Res<GlobalMousePos>) {
    dbg!(pos);
}
