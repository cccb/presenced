use bytes::Bytes;

use super::{
    service::{TOPIC_ETA, TOPIC_ETD, TOPIC_STATE, TOPIC_STATUS},
    state::{Eta, Etd, State, Status},
};

/// Msg is an incoming message
#[derive(Debug)]
pub enum Msg {
    State(State),
    Status(Status),
    Eta(Eta),
    Etd(Etd),
}

/// Helper for logging a decode error
fn log_message_decode_error(topic: &str, err: serde_json::Error) -> Option<Msg> {
    println!(
        "WARNING: received invalid payload for topic {}: {}",
        topic, err,
    );
    None
}

fn log_status_decode_error() -> Option<Msg> {
    println!("WARNING: could not decode status, invalid payload");
    None
}

impl Msg {
    /// Decode payload into message
    pub fn decode(topic: String, payload: Bytes) -> Option<Self> {
        match topic.as_str() {
            TOPIC_ETA => match Eta::from_bytes(payload) {
                Ok(eta) => Some(Msg::Eta(eta)),
                Err(e) => log_message_decode_error(&topic, e),
            },
            TOPIC_ETD => match Etd::from_bytes(payload) {
                Ok(etd) => Some(Msg::Etd(etd)),
                Err(e) => log_message_decode_error(&topic, e),
            },
            TOPIC_STATE => match State::from_bytes(payload) {
                Ok(state) => Some(Msg::State(state)),
                Err(e) => log_message_decode_error(&topic, e),
            },
            TOPIC_STATUS => match Status::from_bytes(payload) {
                Some(status) => Some(Msg::Status(status)),
                None => log_status_decode_error(),
            },
            _ => None,
        }
    }
}
