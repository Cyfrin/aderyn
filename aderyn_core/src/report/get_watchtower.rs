use crate::watchtower::{lightchaser::LightChaser, utils::MetricsDatabase, WatchTower};

pub fn from_metrics_db() -> Box<dyn WatchTower> {
    let metrics_db: MetricsDatabase =
        serde_json::from_str(include_str!("../../../watchtower.metrics_db.json"))
            .expect("database corrupted !");

    Box::new(LightChaser { metrics_db })
}
