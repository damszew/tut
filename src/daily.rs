use std::fmt::Display;

use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::broadcast;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Event {
    NewPersonJoined,
    RaisedHand,
    PersonLeft,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub struct DailyId(String);

impl Display for DailyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl DailyId {
    pub fn random() -> Self {
        // TODO: probably daily should create it
        let daily_id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();

        Self(daily_id)
    }
}

pub type Participant = ();

const CAPACITY: usize = 200;

#[derive(Debug, Clone)]
pub struct Daily {
    sender: broadcast::Sender<Event>,
}

impl Daily {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(CAPACITY);

        Self { sender }
    }

    // TODO: Maybe split like websocket?
    // TODO: This should accept actions not events
    pub async fn send(&self, event: Event) {
        self.sender.send(event).expect("Handle channel send error");
    }

    pub async fn join(&self, _participant: Participant) -> broadcast::Receiver<Event> {
        // Send before joining, so I don't receive event about myself
        // Also ignore Err when there are no other receivers
        let _ = self.sender.send(Event::NewPersonJoined);

        self.sender.subscribe()
    }
}
