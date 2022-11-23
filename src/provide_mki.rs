use std::collections::BTreeMap;

use crossbeam_channel::{unbounded, Receiver, Sender};

use bevy::prelude::*;
use mki::{Action, Keyboard};

pub type GlobalKeys = mki::Keyboard;

#[derive(Debug, Deref)]
pub struct GlobalHotkeyEvents(pub String);

#[derive(Debug, Deref)]
pub struct GlobalKeyEvents(pub Keyboard);

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<Keyboard>);

pub(crate) struct KeyboardProvider;

impl Plugin for KeyboardProvider {
    fn build(&self, app: &mut App) {
        app.add_event::<GlobalKeyEvents>()
            .init_resource::<GlobalHotkeys>()
            .add_event::<GlobalHotkeyEvents>()
            .add_startup_system(send_events)
            .add_system(read_stream_events)
            .add_system(read_stream_hotkeys);
    }
}

#[derive(Resource)]
pub struct GlobalHotkeys {
    rx: Receiver<String>,
    tx: Sender<String>,
    map: BTreeMap<String, Vec<Keyboard>>,
}

impl FromWorld for GlobalHotkeys {
    fn from_world(_: &mut World) -> Self {
        let (tx, rx) = unbounded::<String>();
        GlobalHotkeys {
            rx,
            tx,
            map: BTreeMap::new(),
        }
    }
}

impl GlobalHotkeys {
    pub fn add(&mut self, key: String, sequence: &[Keyboard]) {
        let tx = self.tx.clone();

        if self.map.contains_key(&key) {
            self.remove(&key);
        }

        self.map
            .entry(key.clone())
            .or_insert_with(|| sequence.to_vec());
        mki::register_hotkey(sequence, move || tx.send(key.clone()).unwrap_or(()));
    }

    pub fn remove(&mut self, key: &str) {
        match self.map.remove(key) {
            Some(sequence) => mki::unregister_hotkey(&sequence),
            None => warn!(
                "Tried to remove global hotkey \"{}\" which was not registered.",
                key
            ),
        }
    }
}

fn send_events(mut commands: Commands) {
    let (tx, rx) = unbounded::<Keyboard>();

    mki::bind_any_key(Action::sequencing_kb(move |ev| tx.send(ev).unwrap_or(())));

    commands.insert_resource(StreamReceiver(rx));
}

fn read_stream_events(receiver: Res<StreamReceiver>, mut events: EventWriter<GlobalKeyEvents>) {
    for from_stream in receiver.try_iter() {
        events.send(GlobalKeyEvents(from_stream));
    }
}
fn read_stream_hotkeys(
    hotkeys: Res<GlobalHotkeys>,
    mut hotkey_events: EventWriter<GlobalHotkeyEvents>,
) {
    for event in hotkeys.rx.try_iter() {
        hotkey_events.send(GlobalHotkeyEvents(event));
    }
}
