#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    derive_more::Display,
)]
pub struct ParticipantId(uuid::Uuid);
impl ParticipantId {
    pub fn random() -> Self {
        // TODO: probably daily should create it

        Self(uuid::Uuid::new_v4())
    }
}

impl From<&str> for ParticipantId {
    fn from(value: &str) -> Self {
        Self(value.parse().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Participant {
    pub id: ParticipantId,
    pub name: String,
    pub is_ready: bool, // TODO: enum?
}
