use anyhow::Result;

mod mqtt;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting MQTT presence service");

    // Start MQTT service
    mqtt::start(mqtt::config::from_env()?).await?;

    Ok(())
}
