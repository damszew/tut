use std::{collections::HashSet, fmt::Display, sync::Arc};

use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::RwLock;

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

#[derive(Debug, Clone)]
pub struct Daily {
    state: Arc<RwLock<DailyState>>,
}
#[derive(Debug, Clone, Default)]
pub struct DailyState {
    pub driver: String,
    pub participants: HashSet<Participant>,
}

impl Daily {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }

    #[tracing::instrument(skip_all, fields(participant=participant.name))]
    pub async fn join(&self, participant: Participant) {
        tracing::info!("Joining daily");

        self.state
            .write()
            .await
            .participants
            .insert(participant.clone());
    }

    #[tracing::instrument(skip_all, fields(participant=participant.name))]
    pub async fn leave(&self, participant: Participant) {
        tracing::info!("Leaving daily");

        self.state.write().await.participants.remove(&participant);
    }

    pub async fn state(&self) -> DailyState {
        self.state.read().await.clone()
    }
}
