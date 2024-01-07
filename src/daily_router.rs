use std::collections::HashMap;

use crate::daily::{Daily, Participant};

#[derive(Debug, Clone)]
pub struct DailyRouter {
    dailies: HashMap<String, Daily>,
}

impl DailyRouter {
    pub fn new() -> Self {
        todo!()
    }

    pub fn create_daily(&self) -> String {
        todo!()
    }

    pub fn daily_exists(&self, daily_id: &str) -> bool {
        todo!()
    }

    pub fn join(&self, daily_id: String, participant: Participant) -> Daily {
        todo!()
    }
}
