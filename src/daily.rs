use tokio::sync::broadcast;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Event {
    NewPersonJoined,
    RaisedHand,
    PersonLeft,
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
    pub async fn send(&self, event: Event) {
        self.sender.send(event).expect("Handle channel send error");
    }

    pub async fn subscribe(&self) -> broadcast::Receiver<Event> {
        // Send before joining, so I don't receive event about myself
        // Also ignore Err when there are no other receivers
        let _ = self.sender.send(Event::NewPersonJoined);

        self.sender.subscribe()
    }
}
