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
```

Find more in [Examples](https://github.com/laundmo/bevy_global_input/tree/main/examples)

## compatible bevy versions

| bevy | bevy_global_input |
| ---- | ----------------- |
| 0.10 | 0.3.0             |
| 0.9  | 0.2.0             |
| 0.9  | 0.1.0             |
