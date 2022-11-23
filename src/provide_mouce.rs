use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver};
use mouce::{common::MouseEvent, Mouse};
use std::thread;

pub(crate) struct MousePosProvider;

#[derive(Debug, Deref)]
pub struct GlobalMouseEvents(mouce::common::MouseEvent);
#[derive(Debug, Deref)]
pub struct GlobalScrollEvents(mouce::common::ScrollDirection);
#[derive(Debug, Deref)]
pub struct GlobalMouseButtonEvents(mouce::common::MouseButton);

impl Plugin for MousePosProvider {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalMousePos>()
            .add_event::<GlobalMouseEvents>()
            .add_event::<GlobalScrollEvents>()
            .add_event::<GlobalMouseButtonEvents>()
            .add_startup_system(setup_hook)
            .add_system(store_last_pos.after(read_stream))
            .add_system(split_events.after(read_stream))
            .add_system(read_stream);
    }
}

#[derive(Resource, Default, Debug)]
pub struct GlobalMousePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<MouseEvent>);

fn setup_hook(mut commands: Commands) {
    let (tx, rx) = bounded::<MouseEvent>(10);
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
