use std::env;

use anyhow::Result;
use rumqttc::MqttOptions;

const ENV_MQTT_URL: &str = "PRESENCED_MQTT_URL";

/// Load MQTT config from environment
pub fn from_env() -> Result<MqttOptions> {
    let mut url = env::var(ENV_MQTT_URL)?;
    url += "?client_id=presenced_mqtt_service";
    let options = MqttOptions::parse_url(url)?;
    Ok(options)
}
