use crate::{daily::DailyId, participant::Participant};

pub struct WaitingRoom {
    pub url: String,
    pub daily_id: DailyId,
    pub am_i_ready: bool,
    pub participants: Vec<Participant>,
}
