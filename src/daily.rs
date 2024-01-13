use std::{collections::HashSet, fmt::Display, sync::Arc};

use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::{broadcast, RwLock};

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Event {
    NewPersonJoined(Participant),
    RaisedHand,
    PersonLeft(Participant),
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

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Participant {
    name: String,
}

impl Participant {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl Default for Participant {
    // TODO: Just for now
    fn default() -> Self {
        Self {
            name: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
        }
    }
}

const CAPACITY: usize = 200;

#[derive(Debug, Clone)]
pub struct Daily {
    sender: broadcast::Sender<Event>,
    state: Arc<RwLock<DailyState>>,
}
#[derive(Debug, Clone, Default)]
pub struct DailyState {
    pub driver: String,
    pub participants: HashSet<Participant>,
}

impl Daily {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(CAPACITY);

        Self {
            sender,
            state: Default::default(),
        }
    }

    // TODO: Maybe split like websocket?
    // TODO: This should accept actions not events
    pub async fn send(&self, event: Event) {
        self.sender.send(event).expect("Handle channel send error");
    }

    pub async fn join(&self, participant: Participant) -> broadcast::Receiver<Event> {
        // Send before joining, so I don't receive event about myself
        // Also ignore Err when there are no other receivers
        let _ = self
            .sender
            .send(Event::NewPersonJoined(participant.clone()));

        self.state.write().await.participants.insert(participant);

        self.sender.subscribe()
    }

    pub async fn leave(&self, participant: Participant) {
        // Also ignore Err when there are no other receivers
        self.state.write().await.participants.remove(&participant);

        let _ = self.sender.send(Event::PersonLeft(participant));
    }

    // TODO: Consider splitting into more focused methods
    pub async fn state(&self) -> DailyState {
        self.state.read().await.clone()
    }
}
