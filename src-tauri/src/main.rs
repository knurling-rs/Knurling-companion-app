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
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time::Duration};

use tauri::Manager as tauriManager;
use tauri::Window;

static DEMO: AtomicBool = AtomicBool::new(false);

// the payload type must implement `Serialize`.
#[derive(serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
fn toggle_demo() -> bool {
  !DEMO
    // Ordering::SeqCst is some low level stuff to make sure it is written consequently in memory
    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(!x))
    .unwrap()
}
// Also in main.rs
fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![init_process, toggle_demo])
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
    let mut rng = rand::thread_rng();

    loop {
      if DEMO.load(Ordering::SeqCst) {
        window.emit("distance_emitter", rng.gen_range(20..500)).ok();
        thread::sleep(Duration::from_millis(100));
      } else if let Ok(event) = event_receiver.recv() {
        match event {
          // This is the generic name of "advertisement" that beacons are sending.
          CentralEvent::ManufacturerDataAdvertisement {
            address: _,
            manufacturer_id: manufacturer_id,
            data,
          } => {
            // 0xFFFF	has special meaning
            // it is reserved as default vendor ID when no Device ID service record is present
            // in a remote device.
            //https://www.bluetooth.com/specifications/assigned-numbers/company-identifiers/
            if manufacturer_id == 0xffff {
              // Btleplug is receiving a vector, it has no idea about the size of the data we are receiving
              // We need to cast it into a [u8] array of size 4
              let data: Option<[u8; 4]> = data.try_into().ok();
              if let Some(d) = data {
                let mut dist: u32 = u32::from_be_bytes(d);
                if dist > 600 {
                  dist = 525;
                } else {
                  dist = u32::from_be_bytes(d);
                }
                //Ok() discards the error if any, since this is only a test app we don't need
                // to do proper error handling.
                window.emit("distance_emitter", dist).ok();
              }
            }
          }
          _ => {}
        }
      }
    }
  });
}
