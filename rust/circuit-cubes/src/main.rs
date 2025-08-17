// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const DEVICE_FILTER: &str = "Tenka";
const SERVICE_UUID: Uuid = Uuid::from_u128(0x6e400001_b5a3_f393_e0a9_e50e24dcca9e);
const UART_TX_UUID: Uuid = Uuid::from_u128(0x6e400002_b5a3_f393_e0a9_e50e24dcca9e);
const UART_RX_UUID: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);

//     def __gen_power_cmd(self, power: int, output: str) -> str:
// clamped_power: int = max(-255, min(power, 255))
// return "{:+04d}{}".format(clamped_power, output)
fn gen_power_cmd(power: i32, output: char, out_str: &mut String) {
    let pow = power.clamp(-255, 255);
    let data = format!("{pow:+03}{output}");
    out_str.clear();
    out_str.push_str(data.as_str());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(3)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                // println!("Peripheral {:?} is connected: {:?}", &local_name, is_connected);
                // Check if it's the peripheral we want.
                if local_name.contains(DEVICE_FILTER) {
                    println!("Found matching peripheral {:?}...", &local_name);
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            eprintln!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    let is_connected = peripheral.is_connected().await?;
                    println!("Now connected ({:?}) to peripheral {:?}.", is_connected, &local_name);
                    if is_connected {
                        println!("Discover peripheral {:?} services...", local_name);
                        peripheral.discover_services().await?;
                        for characteristic in peripheral.characteristics() {
                            // println!("Checking characteristic {:?}", characteristic);

                            match characteristic.uuid {
                                SERVICE_UUID => {
                                    println!("FOUND SERVICE  Characteristic {:?}", characteristic.uuid);
                                }
                                UART_TX_UUID => {
                                    println!("FOUND UART_TX  Characteristic {:?}", characteristic.uuid);
                                    println!("Setting Power...");
                                    let mut cmd = String::new();
                                    gen_power_cmd(100, 'a', &mut cmd);
                                    let cmd_u8 = cmd.as_bytes();
                                    peripheral
                                        .write(&characteristic, cmd_u8, btleplug::api::WriteType::WithoutResponse)
                                        .await?;
                                }
                                UART_RX_UUID => {
                                    println!("FOUND UART_RX Characteristic {:?}", characteristic.uuid);
                                }
                                _ => {
                                    println!("Ignored Characteristic {:?}", characteristic.uuid);
                                }
                            }

                            // Subscribe to notifications from the characteristic with the selected UUID
                            if characteristic.uuid == UART_RX_UUID
                                && characteristic.properties.contains(CharPropFlags::NOTIFY)
                            {
                                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await?;
                                // Print the first 4 notifications received.
                                let mut notification_stream = peripheral.notifications().await?.take(4);
                                // Process while the BLE connection is not broken or stopped.
                                while let Some(data) = notification_stream.next().await {
                                    println!(
                                        "Received data from {:?} [{:?}]: {:?}",
                                        local_name, data.uuid, data.value
                                    );
                                }
                            }
                        }
                        println!("Disconnecting from peripheral {:?}...", local_name);
                        peripheral.disconnect().await?;
                    }
                } else {
                    // println!("Skipping unknown peripheral {:?}", peripheral);
                }
            }
        }
    }
    Ok(())
}
