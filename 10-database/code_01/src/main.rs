use database::item::get;
use env_logger::{Builder, Target};
use log::info;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();

    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=1234 dbname=session10",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let item = get(&client, "0000000020").await?;
    info!("Retrieved item: {:?}", item);
    info!("Finished!");

    Ok(())
}
