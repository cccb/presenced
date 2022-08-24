use super::msg::Msg;
use anyhow::Result;
use bytes::Bytes;
use rumqttc::{AsyncClient, QoS};

use super::state::{Eta, Etd, State, Status};

pub const TOPIC_STATUS: &str = "/presence/status";
pub const TOPIC_STATE: &str = "/presence/state";
pub const TOPIC_ETA: &str = "/presence/eta";
pub const TOPIC_ETD: &str = "/presence/etd";

pub struct Service {
    state: State,
    client: AsyncClient,
}

impl Service {
    /// Create new presence service
    pub fn new(client: AsyncClient) -> Self {
        Self {
            state: State::new(),
            client: client,
        }
    }

    /// Publish the current state
    async fn publish_state(&mut self) -> Result<()> {
        let payload = self.state.to_json()?;
        self.client
            .publish(TOPIC_STATE, QoS::AtLeastOnce, true, payload)
            .await?;
        Ok(())
    }

    /// Handle arrival message: Add a person to the state,
    /// if not present, identified by name
    async fn handle_arrival(&mut self, eta: Eta) -> Result<()> {
        // Update and publish state
        if self.state.arrive(eta) {
            self.publish_state().await?;
        }
        Ok(())
    }

    /// Handle departure message: Remove a person from
    /// the state
    async fn handle_departure(&mut self, etd: Etd) -> Result<()> {
        // Update and publish state
        if self.state.depart(etd) {
            self.publish_state().await?;
        }
        Ok(())
    }

    /// Handle status update
    async fn handle_status(&mut self, status: Status) -> Result<()> {
        if self.state.set_status(status) {
            self.publish_state().await?;
        }
        Ok(())
    }

    /// Handle incoming MQTT messages
    pub async fn handle(&mut self, topic: String, payload: Bytes) -> Result<()> {
        match Msg::decode(topic, payload) {
            Some(Msg::State(state)) => self.state = state,
            Some(Msg::Eta(eta)) => self.handle_arrival(eta).await?,
            Some(Msg::Etd(etd)) => self.handle_departure(etd).await?,
            Some(Msg::Status(status)) => self.handle_status(status).await?,
            None => {}
        };

        Ok(())
    }

    /// Start the MQTT presence service
    pub async fn start(&self) -> Result<()> {
        println!("starting mqtt presence service");

        // Subscribe to all required topics
        self.client.subscribe(TOPIC_STATE, QoS::AtMostOnce).await?;
        self.client.subscribe(TOPIC_STATUS, QoS::AtMostOnce).await?;
        self.client.subscribe(TOPIC_ETA, QoS::AtMostOnce).await?;
        self.client.subscribe(TOPIC_ETD, QoS::AtMostOnce).await?;

        Ok(())
    }
}
