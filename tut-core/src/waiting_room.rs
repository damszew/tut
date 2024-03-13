use crate::{daily::DailyId, participant::Participant};

pub struct WaitingRoom {
    pub url: String,
    pub daily_id: DailyId,
    pub me: Participant,
    pub participants: Vec<Participant>,
}
