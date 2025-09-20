// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const DEVICE_FILTER: &str = "Tenka";
const SERVICE_UUID: Uuid = Uuid::from_u128(0x6e400001_b5a3_f393_e0a9_e50e24dcca9e);
const UART_TX_UUID: Uuid = Uuid::from_u128(0x6e400002_b5a3_f393_e0a9_e50e24dcca9e);
const UART_RX_UUID: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);

pub enum CircuitCubeTerminal {
    A,
    B,
    C,
}

pub struct CircuitCubeBTLE {
    name: String,
    peripheral: Peripheral,
    service: Characteristic,
    tx: Characteristic,
    rx: Characteristic,
}
pub struct CircuitCube {
    btle: CircuitCubeBTLE,
    cmd_buf: String,
    // pow_a: i16,
    // pow_b: i16,
    // pow_c: i16,
    // batt: i16,
}

impl CircuitCube {
    pub fn new(btle_config: CircuitCubeBTLE) -> CircuitCube {
        CircuitCube {
            btle: btle_config,
            cmd_buf: String::with_capacity(5),
            // pow_a: 0,
            // pow_b: 0,
            // pow_c: 0,
            // batt: 0,
        }
    }

    pub async fn connect(&self) -> bool {
        let is_connected = self.btle.peripheral.is_connected().await.expect("failed to get connectd state");
        if is_connected {
            true
        } else if let Err(err) = self.btle.peripheral.connect().await {
            println!("Error connecting to peripheral, skipping: {}", err);
            false
        } else {
            self.btle.peripheral.is_connected().await.expect("failed to get connectd state")
        }
    }

    pub async fn disconnect(&self) {
        println!("Disconnecting from peripheral {:?}...", self.btle.name);
        self.btle.peripheral.disconnect().await.expect("failed to disconnect");
    }

    pub async fn set_power(&mut self, terminal: CircuitCubeTerminal, power: i16) {
        let c: char = match terminal {
            CircuitCubeTerminal::A => 'a',
            CircuitCubeTerminal::B => 'b',
            CircuitCubeTerminal::C => 'c',
        };
        //CircuitCube::gen_power_cmd(power, c, &mut self.cmd_buf);

        let pow = power.clamp(-255, 255);
        self.cmd_buf = format!("{pow:+03}{c}");

        let cmd_u8 = self.cmd_buf.as_bytes();
        self.btle
            .peripheral
            .write(&self.btle.tx, cmd_u8, btleplug::api::WriteType::WithoutResponse)
            .await
            .expect("failed to tx");
    }
}

fn gen_power_cmd(power: i16, output: char, out_str: &mut String) {
    let pow = power.clamp(-255, 255);
    let data = format!("{pow:+03}{output}");
    out_str.clear();
    out_str.push_str(data.as_str());
}

pub async fn get_all_cubes() -> Vec<CircuitCube> {
    let mut cubes: Vec<CircuitCube> = Vec::new();

    pretty_env_logger::init();

    let manager = Manager::new().await.expect("failed to initiate manager");
    let adapter_list = manager.adapters().await.expect("failed to get adapters");

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter.start_scan(ScanFilter::default()).await.expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(3)).await;

        for p in adapter.peripherals().await.unwrap() {
            if p.properties().await.unwrap().unwrap().local_name.iter().any(|name| name.contains(DEVICE_FILTER)) {
                let properties = p.properties().await.expect("failed to get peripheral proeprties");
                let local_name = properties.unwrap().local_name.unwrap_or(String::from("(peripheral name unknown)"));

                let mut service: Option<Characteristic> = None;
                let mut tx: Option<Characteristic> = None;
                let mut rx: Option<Characteristic> = None;

                for c in p.characteristics() {
                    match c.uuid {
                        SERVICE_UUID => service = Some(c),
                        UART_TX_UUID => tx = Some(c),
                        UART_RX_UUID => rx = Some(c),
                        _ => (),
                    }
                }

                if let (Some(s), Some(t), Some(r)) = (service, tx, rx) {
                    let btle_config = CircuitCubeBTLE {
                        name: local_name,
                        peripheral: p,
                        service: s,
                        tx: t,
                        rx: r,
                    };
                    cubes.push(CircuitCube::new(btle_config));
                }
            }
        }
    }
    cubes
}

pub async fn connect_to_cubes() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter.start_scan(ScanFilter::default()).await.expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(3)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties.unwrap().local_name.unwrap_or(String::from("(peripheral name unknown)"));
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
                                    peripheral.write(&characteristic, cmd_u8, btleplug::api::WriteType::WithoutResponse).await?;
                                }
                                UART_RX_UUID => {
                                    println!("FOUND UART_RX Characteristic {:?}", characteristic.uuid);
                                }
                                _ => {
                                    println!("Ignored Characteristic {:?}", characteristic.uuid);
                                }
                            }

                            // Subscribe to notifications from the characteristic with the selected UUID
                            if characteristic.uuid == UART_RX_UUID && characteristic.properties.contains(CharPropFlags::NOTIFY) {
                                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await?;
                                // Print the first 4 notifications received.
                                let mut notification_stream = peripheral.notifications().await?.take(4);
                                // Process while the BLE connection is not broken or stopped.
                                while let Some(data) = notification_stream.next().await {
                                    println!("Received data from {:?} [{:?}]: {:?}", local_name, data.uuid, data.value);
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
