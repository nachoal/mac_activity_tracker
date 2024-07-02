use rdev::{listen, EventType, Button};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct ActivityData {
    left_clicks: u32,
    right_clicks: u32,
    middle_clicks: u32,
    keypresses: u32,
    mouse_movements: u32,
}

impl ActivityData {
    fn new() -> Self {
        ActivityData {
            left_clicks: 0,
            right_clicks: 0,
            middle_clicks: 0,
            keypresses: 0,
            mouse_movements: 0,
        }
    }
}

fn main() {
    let data = Arc::new(Mutex::new(ActivityData::new()));
    let data_clone = data.clone();
    let mut event_count = 0;

    if let Err(error) = listen(move |event| {
        let mut data = data_clone.lock().unwrap();
        match event.event_type {
            EventType::ButtonPress(button) => {
                match button {
                    Button::Left => data.left_clicks += 1,
                    Button::Right => data.right_clicks += 1,
                    Button::Middle => data.middle_clicks += 1,
                    _ => (),
                }
            },
            EventType::KeyPress(_) => {
                data.keypresses += 1;
            },
            EventType::MouseMove { .. } => {
                data.mouse_movements += 1;
            },
            _ => (),
        }

        event_count += 1;
        if event_count >= 100 { // Save every 100 events
            if let Err(e) = save_data(&data) {
                println!("Failed to save data: {:?}", e);
            }
            event_count = 0; // Reset event count after saving
        }
    }) {
        println!("Error listening: {:?}", error);
    }

    // Ensure data is saved when the program exits normally or after the last batch of events
    {
        let data_guard = data.lock();
        if let Ok(data) = data_guard {
            let _ = save_data(&data);
        }
    }
}

fn save_data(data: &ActivityData) -> std::io::Result<()> {
    println!("Saving data to file...");
    let mut file = File::create("activity_data.json")?;
    let json = serde_json::to_string(data)?;
    file.write_all(json.as_bytes())?;
    println!("Data saved successfully.");
    Ok(())
}