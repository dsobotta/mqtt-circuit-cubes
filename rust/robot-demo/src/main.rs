use std::error::Error;

mod app;
mod crossterm;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    crate::crossterm::run(true).await.expect("failed to run crossterm");
    Ok(())
}
