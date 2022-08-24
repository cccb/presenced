use anyhow::Result;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions};

use crate::mqtt::presence;

/// Connction main loop
pub async fn start(options: MqttOptions) -> Result<()> {
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    let mut presence = presence::Service::new(client.clone());

    // Start presence service
    presence.start().await?;

    // Handle events
    loop {
        match eventloop.poll().await {
            Ok(event) => {
                match event {
                    Event::Incoming(Incoming::Publish(msg)) => {
                        // Invoke handlers
                        presence
                            .handle(msg.topic.clone(), msg.payload.clone())
                            .await?;
                    }
                    _ => {}
                };
            }
            Err(err) => return Err(err.into()),
        }
    }
}
