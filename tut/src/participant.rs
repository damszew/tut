#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, derive_more::Display,
)]
pub struct ParticipantId(uuid::Uuid);
impl ParticipantId {
    pub fn random() -> Self {
        // TODO: probably daily should create it

        Self(uuid::Uuid::new_v4())
    }
}
