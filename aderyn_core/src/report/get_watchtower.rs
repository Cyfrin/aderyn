use crate::watchtower::{lightchaser::LightChaser, utils::MetricsDatabase, WatchTower};

fn from() -> Box<dyn WatchTower> {
    let metrics_db: MetricsDatabase =
        serde_json::from_str(include_str!("../../../watchtower.metrics_db.json")).unwrap();
    Box::new(LightChaser { metrics_db })
}
