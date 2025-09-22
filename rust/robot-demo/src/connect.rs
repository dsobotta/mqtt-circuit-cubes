// Now connected (true) to peripheral "Tenka0ca1".
// Discover peripheral "Tenka0ca1" services...
// Checking characteristic Characteristic { uuid: 00002a00-0000-1000-8000-00805f9b34fb, service_uuid: 00001800-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a01-0000-1000-8000-00805f9b34fb, service_uuid: 00001800-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a02-0000-1000-8000-00805f9b34fb, service_uuid: 00001800-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a23-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a24-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a25-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a26-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a27-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a28-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a29-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a2a-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
// Checking characteristic Characteristic { uuid: 00002a50-0000-1000-8000-00805f9b34fb, service_uuid: 0000180a-0000-1000-8000-00805f9b34fb, properties: CharPropFlags(READ), descriptors: {} }
//
//
// Checking characteristic Characteristic {
//uuid: 00002a05-0000-1000-8000-00805f9b34fb,
//service_uuid: 00001801-0000-1000-8000-00805f9b34fb,
//properties: CharPropFlags(INDICATE),
//descriptors: {
//Descriptor {
//uuid: 00002902-0000-1000-8000-00805f9b34fb,
//service_uuid: 00001801-0000-1000-8000-00805f9b34fb,
//characteristic_uuid: 00002a05-0000-1000-8000-00805f9b34fb
//}
//}
//}
//
//
// Checking characteristic Characteristic {
// uuid: 6e400002-b5a3-f393-e0a9-e50e24dcca9e,
// service_uuid: 6e400001-b5a3-f393-e0a9-e50e24dcca9e,
// properties: CharPropFlags(WRITE_WITHOUT_RESPONSE),
// descriptors: {}
// }
//
//
// Checking characteristic Characteristic {
// uuid: 6e400003-b5a3-f393-e0a9-e50e24dcca9e,
// service_uuid: 6e400001-b5a3-f393-e0a9-e50e24dcca9e,
// properties: CharPropFlags(NOTIFY),
// descriptors: {
// Descriptor {
// uuid: 00002902-0000-1000-8000-00805f9b34fb,
// service_uuid: 6e400001-b5a3-f393-e0a9-e50e24dcca9e,
// characteristic_uuid: 6e400003-b5a3-f393-e0a9-e50e24dcca9e
// }
// }
// }
//
//
// Checking characteristic Characteristic {
// uuid: f000ffc1-0451-4000-b000-000000000000,
// service_uuid: f000ffc0-0451-4000-b000-000000000000,
// properties: CharPropFlags(WRITE_WITHOUT_RESPONSE | WRITE | NOTIFY),
// descriptors: {
// Descriptor {
// uuid: 00002901-0000-1000-8000-00805f9b34fb,
// service_uuid: f000ffc0-0451-4000-b000-000000000000,
// characteristic_uuid: f000ffc1-0451-4000-b000-000000000000
// },
// Descriptor {
// uuid: 00002902-0000-1000-8000-00805f9b34fb,
// service_uuid: f000ffc0-0451-4000-b000-000000000000,
// characteristic_uuid: f000ffc1-0451-4000-b000-000000000000
// }
// }
// }
//
//
// Checking characteristic Characteristic {
// uuid: f000ffc2-0451-4000-b000-000000000000,
// service_uuid: f000ffc0-0451-4000-b000-000000000000,
// properties: CharPropFlags(WRITE_WITHOUT_RESPONSE | WRITE | NOTIFY),
// descriptors: {
// Descriptor {
// uuid: 00002901-0000-1000-8000-00805f9b34fb,
// service_uuid: f000ffc0-0451-4000-b000-000000000000,
// characteristic_uuid: f000ffc2-0451-4000-b000-000000000000
// },
// Descriptor {
// uuid: 00002902-0000-1000-8000-00805f9b34fb,
// service_uuid: f000ffc0-0451-4000-b000-000000000000,
// characteristic_uuid: f000ffc2-0451-4000-b000-000000000000
// }
// }
// }
//
//
// Disconnecting from peripheral "Tenka0ca1"...

// use crate::io::circuitcube::{DEVICE_FILTER, SERVICE_UUID, UART_RX_UUID, UART_TX_UUID};
// use btleplug::api::{Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter};
// use btleplug::platform::{Adapter, Manager, Peripheral};
// use futures::stream::StreamExt;
// use std::time::Duration;
// use tokio::time;

// fn gen_power_cmd(power: i16, output: char, out_str: &mut String) {
//     let pow = power.clamp(-255, 255);
//     let data = format!("{pow:+03}{output}");
//     out_str.clear();
//     out_str.push_str(data.as_str());
// }

// pub async fn connect_to_cubes() -> anyhow::Result<()> {
//     pretty_env_logger::init();

//     let manager = Manager::new().await?;
//     let adapter_list = manager.adapters().await?;
//     if adapter_list.is_empty() {
//         eprintln!("No Bluetooth adapters found");
//     }

//     for adapter in adapter_list.iter() {
//         println!("Starting scan...");
//         adapter.start_scan(ScanFilter::default()).await.expect("Can't scan BLE adapter for connected devices...");
//         time::sleep(Duration::from_secs(3)).await;
//         let peripherals = adapter.peripherals().await?;

//         if peripherals.is_empty() {
//             eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
//         } else {
//             // All peripheral devices in range.
//             for peripheral in peripherals.iter() {
//                 let properties = peripheral.properties().await?;
//                 let is_connected = peripheral.is_connected().await?;
//                 let local_name = properties.unwrap().local_name.unwrap_or(String::from("(peripheral name unknown)"));
//                 // println!("Peripheral {:?} is connected: {:?}", &local_name, is_connected);
//                 // Check if it's the peripheral we want.
//                 if local_name.contains(DEVICE_FILTER) {
//                     println!("Found matching peripheral {:?}...", &local_name);
//                     if !is_connected {
//                         // Connect if we aren't already connected.
//                         if let Err(err) = peripheral.connect().await {
//                             eprintln!("Error connecting to peripheral, skipping: {err}");
//                             continue;
//                         }
//                     }
//                     let is_connected = peripheral.is_connected().await?;
//                     println!("Now connected ({:?}) to peripheral {:?}.", is_connected, &local_name);
//                     if is_connected {
//                         println!("Discover peripheral {local_name} services...");
//                         peripheral.discover_services().await?;
//                         for characteristic in peripheral.characteristics() {
//                             // println!("Checking characteristic {:?}", characteristic);

//                             match characteristic.uuid {
//                                 SERVICE_UUID => {
//                                     println!("FOUND SERVICE  Characteristic {:?}", characteristic.uuid);
//                                 }
//                                 UART_TX_UUID => {
//                                     println!("FOUND UART_TX  Characteristic {:?}", characteristic.uuid);
//                                     println!("Setting Power...");
//                                     let mut cmd = String::new();
//                                     gen_power_cmd(100, 'a', &mut cmd);
//                                     let cmd_u8 = cmd.as_bytes();
//                                     peripheral.write(&characteristic, cmd_u8, btleplug::api::WriteType::WithoutResponse).await?;
//                                 }
//                                 UART_RX_UUID => {
//                                     println!("FOUND UART_RX Characteristic {:?}", characteristic.uuid);
//                                 }
//                                 _ => {
//                                     println!("Ignored Characteristic {:?}", characteristic.uuid);
//                                 }
//                             }

//                             // Subscribe to notifications from the characteristic with the selected UUID
//                             if characteristic.uuid == UART_RX_UUID && characteristic.properties.contains(CharPropFlags::NOTIFY) {
//                                 println!("Subscribing to characteristic {:?}", characteristic.uuid);
//                                 peripheral.subscribe(&characteristic).await?;
//                                 // Print the first 4 notifications received.
//                                 let mut notification_stream = peripheral.notifications().await?.take(4);
//                                 // Process while the BLE connection is not broken or stopped.
//                                 while let Some(data) = notification_stream.next().await {
//                                     println!("Received data from {:?} [{:?}]: {:?}", local_name, data.uuid, data.value);
//                                 }
//                             }
//                         }
//                         println!("Disconnecting from peripheral {local_name}...");
//                         peripheral.disconnect().await?;
//                     }
//                 } else {
//                     // println!("Skipping unknown peripheral {:?}", peripheral);
//                 }
//             }
//         }
//     }
//     Ok(())
// }
