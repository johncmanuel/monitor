use rdev::{Button, Event, EventType};
use serde::Serialize;
use ts_rs::TS;
use std::sync::Mutex;

// in the future, i'll include more types of events
#[derive(Serialize, Clone, Debug, Default, TS)]
#[ts(export, export_to = "../../../api/types/data.d.ts")]
pub struct Data {
    pub timestamp: u64,
    pub keypresses: u64,
    pub left_clicks: u64,
    pub right_clicks: u64,
    pub middle_clicks: u64,
}

pub fn run_listener(event: Event, data_tmp: &Mutex<Data>) {
    let mut data = data_tmp.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(_) => data.keypresses += 1,
        EventType::ButtonPress(button) => match button {
            Button::Left => data.left_clicks += 1,
            Button::Right => data.right_clicks += 1,
            Button::Middle => data.middle_clicks += 1,
            _ => (),
        },
        _ => (),
    }
}