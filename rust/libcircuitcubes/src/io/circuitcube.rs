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

use btleplug::api::{BDAddr, Central, Characteristic, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

pub const DEVICE_FILTER: &str = "Tenka";
pub const SERVICE_UUID: Uuid = Uuid::from_u128(0x6e400001_b5a3_f393_e0a9_e50e24dcca9e);
pub const UART_TX_UUID: Uuid = Uuid::from_u128(0x6e400002_b5a3_f393_e0a9_e50e24dcca9e);
pub const UART_RX_UUID: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);

pub enum CircuitCubeTerminal {
    A,
    B,
    C,
}

pub struct CircuitCubeBTLE {
    // name: String,
    address: BDAddr,
    peripheral: Peripheral,
    // service: Characteristic,
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

    pub fn get_addr(&self) -> BDAddr {
        self.btle.address
    }

    pub async fn connect(&self) -> bool {
        let is_connected = self.btle.peripheral.is_connected().await.expect("failed to get connectd state");
        if is_connected {
            true
        } else if let Err(err) = self.btle.peripheral.connect().await {
            println!("Error connecting to peripheral, skipping: {err}");
            false
        } else {
            self.btle.peripheral.is_connected().await.expect("failed to get connectd state")
        }
    }

    pub async fn disconnect(&self) {
        println!("Disconnecting from peripheral {:?}...", self.btle.address);
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
        self.cmd_buf = format!("{pow:+04}{c}");

        let cmd_u8 = self.cmd_buf.as_bytes();
        self.btle
            .peripheral
            .write(&self.btle.tx, cmd_u8, btleplug::api::WriteType::WithoutResponse)
            .await
            .expect("failed to tx");
    }
}

pub async fn get_all_cubes() -> Vec<CircuitCube> {
    let mut cubes: Vec<CircuitCube> = Vec::new();

    // pretty_env_logger::init();

    let manager = Manager::new().await.expect("failed to initiate manager");
    let adapter_list = manager.adapters().await.expect("failed to get adapters");

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter.start_scan(ScanFilter::default()).await.expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(3)).await;

        for p in adapter.peripherals().await.unwrap() {
            if p.properties().await.unwrap().unwrap().local_name.iter().any(|name| name.contains(DEVICE_FILTER)) {
                let properties = p.properties().await.expect("failed to get peripheral proeprties");
                // let local_name = properties.unwrap().local_name.unwrap_or(String::from("(peripheral name unknown)"));
                let address = properties.unwrap().address;

                // let mut service: Option<Characteristic> = None;
                let mut tx: Option<Characteristic> = None;
                let mut rx: Option<Characteristic> = None;

                let is_connected = p.is_connected().await.expect("failed to get connectd state");
                if is_connected {
                    println!("connected")
                } else if let Err(err) = p.connect().await {
                    println!("Error connecting to peripheral, skipping: {err}");
                } else {
                    p.is_connected().await.expect("failed to get connectd state");
                }

                p.discover_services().await.expect("failed to discover services");
                for c in p.characteristics() {
                    match c.uuid {
                        // SERVICE_UUID => {
                        //     println!("found service characteristic");
                        //     service = Some(c);
                        // }
                        UART_TX_UUID => {
                            println!("found tx characteristic");
                            tx = Some(c);
                        }
                        UART_RX_UUID => {
                            println!("found rx characteristic");
                            rx = Some(c);
                        }
                        _ => (),
                    }
                }

                // if let (Some(s), Some(t), Some(r)) = (service, tx, rx) {
                if let (Some(t), Some(r)) = (tx, rx) {
                    let btle_config = CircuitCubeBTLE {
                        // name: local_name,
                        address,
                        peripheral: p,
                        // service: s,
                        tx: t,
                        rx: r,
                    };
                    cubes.push(CircuitCube::new(btle_config));
                }
            }
        }
    }
    cubes.sort_by_key(|c| c.btle.address);
    cubes
}
