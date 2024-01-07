use std::{collections::HashMap, sync::Arc};

use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::RwLock;

use crate::daily::{Daily, Participant};

#[derive(Debug, Clone)]
pub struct DailyRouter {
    dailies: Arc<RwLock<HashMap<String, Daily>>>,
}

impl DailyRouter {
    pub fn new() -> Self {
        Self {
            dailies: Default::default(),
        }
    }

    pub async fn create_daily(&self) -> String {
        let daily = Daily::new();

        // TODO: probably daily should create it
        let daily_id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();

        self.dailies.write().await.insert(daily_id.clone(), daily);

        daily_id
    }

    pub async fn daily_exists(&self, daily_id: &str) -> bool {
        self.dailies.read().await.contains_key(daily_id)
    }

    pub async fn join(&self, daily_id: &str, participant: Participant) -> Daily {
        self.dailies
            .read()
            .await
            .get(daily_id)
            .expect("Handle missing daily")
            .clone()
    }
}
