#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use btleplug::api::{Central, CentralEvent};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};

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

// adapter retrieval works differently depending on your platform right now.
// API needs to be aligned.

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

  std::thread::spawn(move || {
    while let Ok(event) = event_receiver.recv() {
      match event {
        CentralEvent::ManufacturerDataAdvertisement {
          address: _,
          manufacturer_id: _,
          data,
        } => {
          let data: Option<[u8; 4]> = data.try_into().ok();
          if let Some(d) = data {
            window
              .emit("distance_emitter", u32::from_be_bytes(d).to_string())
              .unwrap()
            //.ok();
          }
        }
        _ => {}
      }
    }
  });
}
