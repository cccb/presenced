use anyhow::Result;

mod mqtt;

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting MQTT presence service");
    mqtt::start(mqtt::config::from_env()?).await?;
    Ok(())
}
