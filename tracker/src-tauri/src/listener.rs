use rdev::{listen, Button, Event, EventType};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::thread;
use ts_rs::TS;

// in the future, i'll include more types of events
#[derive(Serialize, Clone, Debug, Default, TS)]
#[ts(export, export_to = "../../../api/types/data.d.ts")]
pub struct Data {
    // timestamp
    pub ts: u64,
    // key presses
    pub kp: u64,
    // left clicks
    pub lc: u64,
    // right clicks
    pub rc: u64,
    // middle clicks
    pub mc: u64,
}

pub fn start_listener(data_clone: Arc<Mutex<Data>>) {
    thread::spawn(move || {
        if let Err(error) = listen(move |event| run_listener(event, &data_clone)) {
            eprintln!("Error while listening to events: {:?}", error);
        }
    });
}

fn run_listener(event: Event, data_tmp: &Mutex<Data>) {
    let mut data = data_tmp.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(_) => data.kp += 1,
        EventType::ButtonPress(button) => match button {
            Button::Left => data.lc += 1,
            Button::Right => data.rc += 1,
            Button::Middle => data.mc += 1,
            _ => (),
        },
        _ => (),
    }
}
