use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    participant::{Participant, ParticipantId},
    waiting_room::WaitingRoom,
};

#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    derive_more::Display,
    derive_more::From,
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

impl Daily {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }

    pub async fn event(self, event: Event) -> Result<(), String> {
        tracing::info!("Dispatching event: {event:?}"); // TODO: Nicer log/display

        let mut state_lock = self.state.write().await;
        let new_state = state_lock.clone().event(event)?;

        *state_lock = new_state;

        Ok(())
    }

    pub async fn state(&self) -> DailyState {
        self.state.read().await.clone()
    }
}

#[derive(Debug, Clone)]

pub enum DailyState {
    WaitingRoom(WaitingRoom),
    IceBreakerQuestion(()),
    IceBreakerAnswer(()),
    AmIBlocked(()),
    Finished(()),
}

#[derive(Debug, Clone)]
pub enum Event {
    Join(Participant),
    Ready(ParticipantId),
}

impl DailyState {
    // TODO: Error type
    pub fn event(self, event: Event) -> Result<Self, String> {
        match (self, event) {
            (Self::WaitingRoom(mut waiting_room), Event::Join(new_guy)) => {
                waiting_room.join(new_guy);

                Ok(Self::WaitingRoom(waiting_room))
            }
            (Self::WaitingRoom(mut waiting_room), Event::Ready(ready_guy)) => {
                waiting_room.mark_as_ready(ready_guy);

                Ok(Self::WaitingRoom(waiting_room))
            }

            (_, _) => Err("Invalid state".into()),
        }
    }
}

impl Default for DailyState {
    fn default() -> Self {
        Self::WaitingRoom(WaitingRoom::default())
    }
}
