#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Those imports are necessary for different targets
use btleplug::api::{Central, CentralEvent};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
use rand::Rng;

use std::convert::TryInto;
use tauri::Manager as tauriManager;
use tauri::Window;
// the payload type must implement `Serialize`.
#[derive(serde::Serialize)]
struct Payload {
  message: String,
}

// Also in main.rs
fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![init_process])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

// Introducing Btleplug into Tauri

fn get_central(manager: &Manager) -> Adapter {
  let adapters = manager.adapters().unwrap();
  adapters.into_iter().nth(0).unwrap()
}
// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
  // get the first bluetooth adapter
  let manager = Manager::new().unwrap();
  // connect to the adapter
  let central = get_central(&manager);

  let event_receiver = central.event_receiver().unwrap();
  // start scanning for devices
  central.start_scan().unwrap();

  // The Rust thread API expects a fully owned closure by API.
  // So the move forces the closure to take ownership rather than borrowing, to fulfill the API.
  // "Because thread::spawn runs this closure in a new thread,
  // ...we should be able to access our value inside that new thread"
  std::thread::spawn(move || {
    while let Ok(event) = event_receiver.recv() {
      match event {
        // This is the generic name of "advertisement" that beacons are sending.
        CentralEvent::ManufacturerDataAdvertisement {
          address: _,
          manufacturer_id: _,
          data,
        } => {
          let mut dist: u32 = 0;
          // Btleplug is receiving a vector, it has no idea about the size of the data we are receiving
          // We need to cast it into a [u8] array of size 4
          let data: Option<[u8; 4]> = data.try_into().ok();
          if let Some(d) = data {
            match dist {
              // If the distance is superior to 6 meters, let's say we are safe
              mut dist if u32::from_be_bytes(d) > 600 => dist = 600,
              _ => dist = u32::from_be_bytes(d),
            }
            //Ok() discards the error if any, since this is only a test app we don't need
            // to do proper error handling.
          } else {
            let mut rng = rand::thread_rng();
            dist = rng.gen_range(20..600);
          }
          window.emit("distance_emitter", dist).ok();
        }
        _ => {}
      }
    }
  });
}
