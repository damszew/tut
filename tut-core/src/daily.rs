use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::participant::ParticipantId;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Event {
    NewPersonJoined(ParticipantId),
    RaisedHand,
    PersonLeft(ParticipantId),
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

pub enum DailyStep {
    #[default]
    Waiting,
    Started,
    Finished,
}

#[derive(Debug, Clone, Default)]
pub struct DailyState {
    pub participants: HashMap<ParticipantId, bool>,
    pub step: DailyStep,
}

impl Daily {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }

    #[tracing::instrument(skip_all, fields(participant_id))]
    pub async fn join(&self, participant_id: ParticipantId) {
        tracing::info!("Joining daily");

        self.state
            .write()
            .await
            .participants
            .insert(participant_id, false);
    }

    #[tracing::instrument(skip_all, fields(participant_id))]
    pub async fn ready_for_next_step(&self, participant_id: ParticipantId) {
        tracing::info!("Participant is ready for next step");

        let mut state_lock = self.state.write().await;
        state_lock.participants.insert(participant_id, true);

        let are_everyone_ready = state_lock.participants.values().all(|is_ready| *is_ready);

        if are_everyone_ready {
            tracing::info!(current = ?state_lock.step, "Going to next step");
            state_lock.step = match state_lock.step {
                DailyStep::Waiting => DailyStep::Started,
                DailyStep::Started => DailyStep::Finished,
                DailyStep::Finished => DailyStep::Finished,
            };
        }
    }

    #[tracing::instrument(skip_all, fields(participant_id))]
    pub async fn leave(&self, participant_id: ParticipantId) {
        tracing::info!("Leaving daily");

        self.state
            .write()
            .await
            .participants
            .remove(&participant_id);
    }

    pub async fn state(&self) -> DailyState {
        self.state.read().await.clone()
    }
}
