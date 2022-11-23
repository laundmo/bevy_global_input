use bevy::{prelude::*, time::FixedTimestep};
use bevy_global_input::{GlobalInputPlugins, GlobalMousePos};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(GlobalInputPlugins)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 2.0))
                .with_system(mouse_pos),
        )
        .run();
}

fn mouse_pos(pos: Res<GlobalMousePos>) {
    dbg!(pos);
}
