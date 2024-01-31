use std::collections::HashMap;

use aderyn_core::detect::detector::IssueSeverity;
use aderyn_core::watchtower::lightchaser::LightChaser;
use aderyn_core::watchtower::utils::MetricsDatabase;
use aderyn_core::watchtower::{Metrics, WatchTower};

fn main() {
    let mut metrics = HashMap::new();

    metrics.insert(
        "sample-name".to_string(),
        Metrics {
            detector_name: "sample-name".to_string(),
            true_positives: 16,
            false_positives: 2,
            trigger_count: 19,
            experience: 30,
            current_severity: IssueSeverity::High,
        },
    );

    let db = MetricsDatabase {
        metrics,
        db_path: "aderyn_core/lc-metrics.json".to_string(),
    };
    let s = serde_json::to_string(&db).unwrap();
    println!("{}", s);

    db.register_new_detector("sample-a".to_string(), IssueSeverity::Medium);
    db.register_new_detector("sample-b".to_string(), IssueSeverity::Medium);
    db.register_new_detector("sample-c".to_string(), IssueSeverity::High);
    db.register_new_detector("sample-d".to_string(), IssueSeverity::NC);

    let watch_tower: Box<dyn WatchTower> = Box::new(LightChaser { metrics_db: db });
    watch_tower.register("detector_name".to_string(), IssueSeverity::NC);
}
