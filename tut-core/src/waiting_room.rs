use crate::participant::{Participant, ParticipantId};

#[derive(Debug, Clone, Default)]
pub struct WaitingRoom {
    pub participants: Vec<Participant>,
    pub ready_participants: Vec<ParticipantId>,
}

impl WaitingRoom {
    pub fn join(&mut self, new_guy: Participant) {
        self.participants.push(new_guy);
    }

    pub fn mark_as_ready(&mut self, guy: ParticipantId) {
        self.ready_participants.push(guy);
    }
}
