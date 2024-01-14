#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Participant {
    name: uuid::Uuid,
}

impl Participant {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

impl Default for Participant {
    // TODO: Just for now
    fn default() -> Self {
        Self {
            name: uuid::Uuid::new_v4(),
        }
    }
}
