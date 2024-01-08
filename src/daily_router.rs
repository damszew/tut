use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::daily::{Daily, DailyId};

#[derive(Debug, Clone)]
pub struct DailyRouter {
    dailies: Arc<RwLock<HashMap<DailyId, Daily>>>,
}

impl DailyRouter {
    pub fn new() -> Self {
        Self {
            dailies: Default::default(),
        }
    }

    pub async fn create_daily(&self) -> DailyId {
        let daily = Daily::new();

        let daily_id = DailyId::random();
        self.dailies.write().await.insert(daily_id.clone(), daily);

        daily_id
    }

    pub async fn daily_exists(&self, daily_id: &DailyId) -> bool {
        self.dailies.read().await.contains_key(daily_id)
    }

    pub async fn get(&self, daily_id: &DailyId) -> Daily {
        self.dailies
            .read()
            .await
            .get(daily_id)
            .expect("Handle missing daily")
            .clone()
    }
}
