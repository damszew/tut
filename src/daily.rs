#[derive(Debug, Clone, serde::Deserialize)]
pub enum Event {
    NewPersonJoined,
    RaisedHand,
    PersonLeft,
}

pub type Participant = ();

#[derive(Debug, Clone)]
pub struct Daily {}

impl Daily {
    pub fn new() -> Self {
        todo!()
    }

    // TODO: Maybe split like websocket?
    pub async fn send(&self, event: Event) {
        todo!()
    }

    pub async fn recv(&self) -> Result<Event, anyhow::Error> {
        todo!()
    }
}
