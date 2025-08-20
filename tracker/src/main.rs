use rdev::{listen, Event, EventType, Button};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::{mem, thread};
use std::time::Duration;
use reqwest;

#[derive(Serialize, Clone, Debug, Default)]
struct Data {
    keypresses: u64,
    left_clicks: u64,
    right_clicks: u64,
    middle_clicks: u64,
    scrolls: u64,
}

fn main() {
    let data = Arc::new(Mutex::new(Data::default()));
    let data_clone = Arc::clone(&data);

    let client = reqwest::Client::new();
    let api_url = "http://localhost:8000"; 

    // create new thread for event listening
    thread::spawn(move || {
        listen(move |event| callback(event, &data_clone)).expect("Could not listen for events");
    });

    println!("Listening for events...");

    // gonna set this to 10 mins? for capturing, then send it to the API
    // let interval_length_secs = 60 * 10;
    let interval_length_secs = 10;

    loop {
        thread::sleep(Duration::from_secs(interval_length_secs));
        let data_snapshot = {
            let mut data_guard = data.lock().unwrap();
            mem::replace(&mut *data_guard, Data::default())
        };
        let total_events = data_snapshot.keypresses
            + data_snapshot.left_clicks
            + data_snapshot.right_clicks
            + data_snapshot.middle_clicks
            + data_snapshot.scrolls;
        if total_events > 0 {
            // wonder if there's a logger based on environment (dev, prod, etc.)
            println!("Events in the last {} seconds:", interval_length_secs);
            println!("Key presses: {}", data_snapshot.keypresses);
            println!("Left clicks: {}", data_snapshot.left_clicks);
            println!("Right clicks: {}", data_snapshot.right_clicks);
            println!("Middle clicks: {}", data_snapshot.middle_clicks);
            println!("Scrolls: {}", data_snapshot.scrolls);
        } else {
            println!("No events in the last {} seconds.", interval_length_secs);
        }
    }
}

fn callback(event: Event, data_tmp: &Arc<Mutex<Data>>) {
    let mut data = data_tmp.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(_) => {
            data.keypresses += 1;
        }
        EventType::ButtonPress(button) => match button {
            Button::Left => {
                data.left_clicks += 1;
            }
            Button::Right => {
                data.right_clicks += 1;
            }
            Button::Middle => {
                data.middle_clicks += 1;
            }
            // skip other events
            _ => ()
        }
        EventType::Wheel { delta_x: _, delta_y: _ } => {
            data.scrolls += 1;
        }
        // skip other events
        _ => ()
    }
}

async fn send_to_api(client: &reqwest::Client, data: Data, api_url: &str) {
    let response = client.post(api_url)
        .json(&data)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Data sent successfully.");
            } else {
                println!("Failed to send data: {}", resp.status());
            }
        }
        Err(e) => {
            println!("Error sending data: {}", e);
        }
    }
}