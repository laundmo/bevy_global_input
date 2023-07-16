use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use mouce::{common::MouseEvent, Mouse};
use std::thread;

pub(crate) struct MousePosProvider;

/// Enum of mouse buttons
pub type GlobalMouseButton = mouce::common::MouseButton;
/// Enum of Up/Down scroll direction
pub type GlobalScrollDirection = mouce::common::ScrollDirection;

/// Contains all mouse events. Using more concrete events like [`GlobalMouseButtonEvents`] and [`GlobalScrollEvents`] is recommended.
#[derive(Debug, Deref, Event)]
pub struct GlobalMouseEvents(mouce::common::MouseEvent);

/// Scroll wheel events. One event per scroll step, only contains the up/down information.
#[derive(Debug, Deref, Event)]
pub struct GlobalScrollEvents(GlobalScrollDirection);

/// Mouse button press events.
#[derive(Debug, Deref, Event)]
pub struct GlobalMouseButtonEvents(GlobalMouseButton);

impl Plugin for MousePosProvider {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalMousePos>()
            .add_event::<MouseControl>()
            .add_event::<GlobalMouseEvents>()
            .add_event::<GlobalScrollEvents>()
            .add_event::<GlobalMouseButtonEvents>()
            .add_systems(Startup, setup_hook)
            .add_systems(Startup, setup_mover)
            .add_systems(Update, store_last_pos.after(read_stream))
            .add_systems(Update, split_events.after(read_stream))
            .add_systems(Update, mover_events)
            .add_systems(Update, read_stream);
    }
}

/// Resource which stores the global mouse position.
#[derive(Resource, Default, Debug)]
pub struct GlobalMousePos {
    /// absolute x coordinate of the mouse. Coordinates span across all monitors.
    pub x: i32,
    /// absolute y coordinate of the mouse. Coordinates span across all monitors.
    pub y: i32,
}

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<MouseEvent>);

fn setup_hook(mut commands: Commands) {
    let (tx, rx) = unbounded::<MouseEvent>();
    thread::spawn(move || {
        let mut manager = Mouse::new();
        let hook_result = manager.hook(Box::new(move |e| tx.send(*e).unwrap_or(())));
        match hook_result {
            Ok(_) => {}
            // Hooking may require user privileges on some systems
            // e.g. requires super user for Linux
            Err(err) => if mouce::error::Error::PermissionDenied == err {},
        }
        loop {
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    commands.insert_resource(StreamReceiver(rx));
}

fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<GlobalMouseEvents>) {
    for from_stream in receiver.try_iter() {
        events.send(GlobalMouseEvents(from_stream));
    }
}

fn split_events(
    mut events: EventReader<GlobalMouseEvents>,
    mut scrolls: EventWriter<GlobalScrollEvents>,
    mut clicks: EventWriter<GlobalMouseButtonEvents>,
) {
    for event in events.iter() {
        match event {
            GlobalMouseEvents(e) => match e {
                MouseEvent::Scroll(scroll) => scrolls.send(GlobalScrollEvents(*scroll)),
                MouseEvent::Press(click) => clicks.send(GlobalMouseButtonEvents(*click)),
                _ => {}
            },
        }
    }
}

fn store_last_pos(mut events: EventReader<GlobalMouseEvents>, mut mouse: ResMut<GlobalMousePos>) {
    // Try to use AbsoluteMove events
    let abs_move = events
        .iter()
        .filter_map(|e| match e.0 {
            MouseEvent::AbsoluteMove(x, y) => Some((x, y)),
            _ => None,
        })
        .last();

    match abs_move {
        Some(coords) => {
            mouse.x = coords.0;
            mouse.y = coords.1;
        }
        // No events, fall back to manual get_position call
        None => {
            let manager = Mouse::new();
            match manager.get_position() {
                Ok(pos) => {
                    mouse.x = pos.0;
                    mouse.y = pos.1;
                }
                Err(_) => {
                    mouse.x = 0;
                    mouse.y = 0;
                }
            }
        }
    }
}

/// Event which allows control over the mouse.
///
/// ```ignore
/// fn system(
///     mut moves: EventWriter<MouseControl>,
/// ) {
///     /// Move mouse to the left
///     moves.send(MouseControl::MoveRelative(-1, 0));
/// }
/// ```

#[derive(Copy, Clone, Debug, Event)]
pub enum MouseControl {
    /// Moves the cursor to a absolute position (x, y), in the same coordinate system as [`GlobalMousePos`]
    MoveTo(usize, usize),
    /// Moves the cursor relative to it's current position (x_offset, y_offset).
    MoveRelative(i32, i32),
    /// Presses down a mouse buttons.
    PressButton(GlobalMouseButton),
    /// Releases a mouse button.
    ReleaseButton(GlobalMouseButton),
    /// Clicks a mouse button. Equivalent to sending a [`MouseControl::PressButton`] and [`MouseControl::ReleaseButton`] during the same frame.
    ClickButton(GlobalMouseButton),
    /// Moves the scroll wheel up or down one step.
    ScrollWheel(GlobalScrollDirection),
}

fn setup_mover(mut commands: Commands) {
    let (tx, rx) = unbounded::<MouseControl>();
    thread::spawn(move || {
        let manager = Mouse::new();
        for event in rx.iter() {
            match event {
                MouseControl::MoveTo(x, y) => {
                    manager.move_to(x, y).ok();
                }
                MouseControl::MoveRelative(x_offset, y_offset) => {
                    manager.move_relative(x_offset, y_offset).ok();
                }
                MouseControl::PressButton(button) => {
                    manager.press_button(&button).ok();
                }
                MouseControl::ReleaseButton(button) => {
                    manager.release_button(&button).ok();
                }
                MouseControl::ClickButton(button) => {
                    manager.click_button(&button).ok();
                }
                MouseControl::ScrollWheel(direction) => {
                    manager.scroll_wheel(&direction).ok();
                }
            };
        }
    });
    commands.insert_resource(ControlEventSender(tx));
}

#[derive(Resource, Deref)]
struct ControlEventSender(Sender<MouseControl>);

fn mover_events(mut ev: EventReader<MouseControl>, tx: ResMut<ControlEventSender>) {
    for event in ev.iter() {
        tx.send(*event).ok();
    }
}
