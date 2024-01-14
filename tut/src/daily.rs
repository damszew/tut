use std::{collections::HashSet, sync::Arc};

use tokio::sync::RwLock;

use crate::participant::Participant;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Event {
    NewPersonJoined(Participant),
    RaisedHand,
    PersonLeft(Participant),
}

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, derive_more::Display,
)]
pub struct DailyId(uuid::Uuid);

impl DailyId {
    pub fn random() -> Self {
        // TODO: probably daily should create it

        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Default)]
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

    #[tracing::instrument(skip_all, fields(participant=participant.name()))]
    pub async fn join(&self, participant: Participant) {
        tracing::info!("Joining daily");

        self.state
            .write()
            .await
            .participants
            .insert(participant.clone());
    }

    #[tracing::instrument(skip_all, fields(participant=participant.name()))]
    pub async fn leave(&self, participant: Participant) {
        tracing::info!("Leaving daily");

        self.state.write().await.participants.remove(&participant);
    }

    pub async fn state(&self) -> DailyState {
        self.state.read().await.clone()
    }
}
