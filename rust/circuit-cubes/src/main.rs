use libcircuitcubes::io::connect;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    connect::connect_to_cubes().await
}
