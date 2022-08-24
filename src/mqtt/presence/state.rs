use anyhow::Result;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Eta {
    pub name: String,
    pub time: Option<DateTime<Utc>>,
    pub note: Option<String>,
}

impl Eta {
    pub fn from_bytes(payload: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(payload.as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Etd {
    pub name: String,
    pub time: Option<DateTime<Utc>>,
}

impl Etd {
    pub fn from_bytes(payload: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(payload.as_ref())
    }
}

/// Person with name and optional message?
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Person(pub String, pub Option<String>);

/// Club status
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "thursday")]
    Thursday,
}

impl Status {
    pub fn from_bytes(payload: Bytes) -> Option<Self> {
        let status: Result<Self, serde_json::Error> = serde_json::from_slice(payload.as_ref());
        match status {
            Ok(status) => return Some(status),
            Err(_) => {} // don't care.
        }
        // Fallback to manual decoding
        if payload == Bytes::from("open") {
            return Some(Status::Open);
        }
        if payload == Bytes::from("closed") {
            return Some(Status::Closed);
        }
        if payload == Bytes::from("thursday") {
            return Some(Status::Thursday);
        }
        None
    }
}

/// The state holds the current status
/// of the club and the presence of members
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    status: Status,
    people: Vec<Person>,
}

impl State {
    /// New state
    pub fn new() -> Self {
        Self {
            status: Status::Closed,
            people: vec![],
        }
    }

    /// Add a person to the state, identified by name
    pub fn arrive(&mut self, eta: Eta) -> bool {
        let mut change = false;

        let person = Person(eta.name.clone(), eta.note);

        // Try to update, if not found - append
        let mut found = false;
        let mut people: Vec<Person> = vec![];
        for p in &self.people {
            let Person(name, _) = p;
            if name.to_lowercase() == eta.name.to_lowercase() {
                people.push(person.clone());
                found = true;
                if p != &person {
                    change = true;
                }
            } else {
                people.push(p.clone());
            }
        }
        if !found {
            people.push(person);
            change = true;
        }
        self.people = people;
        change
    }

    /// Remove a person from the state, identified by name
    pub fn depart(&mut self, etd: Etd) -> bool {
        let mut change = false;
        let mut people: Vec<Person> = vec![];
        for p in &self.people {
            let Person(name, _) = p;
            if name.to_lowercase() != etd.name.to_lowercase() {
                people.push(p.clone());
            } else {
                change = true;
            }
        }
        self.people = people;

        // Set status to closed when everyone left
        if change && self.people.len() == 0usize {
            self.status = Status::Closed;
        }

        change
    }

    /// Set the club status
    pub fn set_status(&mut self, status: Status) -> bool {
        if self.status != status {
            self.status = status;
            true
        } else {
            false
        }
    }

    /// Decode from MQTT message
    pub fn from_bytes(payload: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(payload.as_ref())
    }

    /// Serialize state to json
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
}
