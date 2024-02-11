use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::daily::{Daily, DailyId};

#[derive(Debug, Clone, Default)]
pub struct DailyRouter {
    dailies: Arc<RwLock<HashMap<DailyId, Daily>>>,
}

impl DailyRouter {
    pub fn new() -> Self {
        Self {
            // TODO: Fake daily
            dailies: Arc::new(RwLock::new(
                [(
                    uuid::uuid!("00000000-0000-0000-0000-000000000000").into(),
                    Daily::new(),
                )]
                .into(),
            )),
        }
    }

    pub async fn create_daily(&self) -> DailyId {
        let daily = Daily::new();

        let daily_id = DailyId::random();
        self.dailies.write().await.insert(daily_id.clone(), daily);

        daily_id
    }

    pub async fn get(&self, daily_id: &DailyId) -> Option<Daily> {
        self.dailies.read().await.get(daily_id).cloned()
    }
}
