use std::error::Error;

mod app;
mod crossterm;
mod ui;

use std::time::Duration;

use stickup::backends::virtual_input::VirtualDevice;
use stickup::DeviceManager;
use stickup::{Device, InputKind};

fn main() {
    // Create a new manager and automatically detect devices
    let mut manager = DeviceManager::new();

    // Get a snapshot of all device states at this moment
    let snapshot = manager.snapshot();

    println!("--- Device Snapshot ---");

    // Iterate through each detected device
    for (device_id, state) in snapshot.iter() {
        println!("Device: {}", device_id);

        // Print all axis values
        for (axis, value) in &state.axes {
            println!("  Axis {} = {}", axis, value);
        }

        // Print all button states
        for (button, pressed) in &state.buttons {
            println!("  Button {} is {}", button, if *pressed { "pressed" } else { "released" });
        }
    }

    // Create a virtual device with a custom ID and name
    let mut device = VirtualDevice::new("virtual:demo", "Demo Virtual Device");

    // Inject some sample input
    device.set_axis(0, 0.75);
    device.press_button(1);

    // Poll the device and print the emitted input events
    for event in device.poll() {
        match event {
            InputKind::AxisMoved { axis, value } => {
                println!("(Virtual) Axis {} = {}", axis, value);
            }
            InputKind::ButtonPressed { button } => {
                println!("(Virtual) Button {} pressed", button);
            }
            InputKind::ButtonReleased { button } => {
                println!("(Virtual) Button {} released", button);
            }
        }
    }
}

// fn main() -> Result<(), Box<dyn Error>> {
//     crate::crossterm::run(true)?;
//     Ok(())
// }

// pub struct BatteryCube {
//     id: String,
//     powerA: i16,
//     powerB: i16,
//     powerC: i16,
// }

// pub struct BackupApp {
//     cubes: Vec<BatteryCube>,
// }

// impl BackupApp {
//     pub fn default() -> Self {
//         Self {
//             cubes: vec![
//                 BatteryCube {
//                     id: "Tenka0ca1".to_string(),
//                     powerA: -255,
//                     powerB: 0,
//                     powerC: 255,
//                 },
//                 BatteryCube {
//                     id: "Tenka0ca2".to_string(),
//                     powerA: -128,
//                     powerB: 0,
//                     powerC: 128,
//                 },
//             ],
//         }
//     }
// }
