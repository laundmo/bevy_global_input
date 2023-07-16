# bevy_global_input

Global Mouse and Keyboard events for bevy.

[![Crates.io](https://img.shields.io/crates/v/bevy_global_input.svg)](https://crates.io/crates/bevy_global_input)
[![Docs.rs](https://docs.rs/bevy_global_input/badge.svg)](https://docs.rs/bevy_global_input)

## Features

- Global keyboard events
- Global mouse position
- Global Hotkeys
- Moving the mouse

### Not Implemented / TODO

- Sending Keystrokes
- Converting to Bevy KeyCode (don't expose underlying library enums)

## Quickstart

log out global mouse position every half second.

```rust
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
        );
        // .run();
}

fn mouse_pos(pos: Res<GlobalMousePos>) {
    dbg!(pos);
}
```

Find more in [Examples](https://github.com/laundmo/bevy_global_input/tree/main/examples)

## compatible bevy versions

| bevy | bevy_global_input |
| ---- | ----------------- |
| 0.11 | 0.4.0             |
| 0.10 | 0.3.0             |
| 0.9  | 0.2.0             |
| 0.9  | 0.1.0             |
