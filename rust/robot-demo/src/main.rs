use std::error::Error;

mod app;
mod crossterm;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    crate::crossterm::run(true)?;
    Ok(())
}

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
