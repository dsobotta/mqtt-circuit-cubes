use std::{error::Error, time::Duration};

mod app;
mod crossterm;
mod ui;

pub struct BatteryCube {
    id: String,
    powerA: i16,
    powerB: i16,
    powerC: i16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(33);

    crate::crossterm::run(tick_rate, true)?;
    Ok(())
}

pub struct BackupApp {
    cubes: Vec<BatteryCube>,
}

impl BackupApp {
    pub fn default() -> Self {
        Self {
            cubes: vec![
                BatteryCube {
                    id: "Tenka0ca1".to_string(),
                    powerA: -255,
                    powerB: 0,
                    powerC: 255,
                },
                BatteryCube {
                    id: "Tenka0ca2".to_string(),
                    powerA: -128,
                    powerB: 0,
                    powerC: 128,
                },
            ],
        }
    }
}
