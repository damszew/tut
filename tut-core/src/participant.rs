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
    fn random() -> Self {
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
    pub id: ParticipantId, // TODO: getters?
    pub name: String,
}

impl Participant {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ParticipantId::random(),
            name: name.into(),
        }
    }
}
