#![allow(clippy::borrowed_box)]
use serde::{Deserialize, Serialize, Serializer};
use std::{
    collections::{hash_map, BTreeMap, HashMap},
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    str::FromStr,
};
use strum::EnumCount;

use crate::detect::detector::IssueSeverity;

use super::{Metrics, Tag};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MetricsDatabase {
    #[serde(serialize_with = "ordered_map")]
    pub metrics: HashMap<String, Metrics>,

    #[serde(serialize_with = "ordered_map")]
    pub tags: HashMap<String, Tag>,

    pub db_path: String,
}

// https://stackoverflow.com/questions/42723065/how-to-sort-hashmap-keys-when-serializing-with-serde
/// For use with serde's [serialize_with] attribute
fn ordered_map<S, K: Ord + Serialize, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl FromStr for MetricsDatabase {
    type Err = serde_json::Error;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(content)
    }
}

impl MetricsDatabase {
    pub fn from_path(path: String) -> MetricsDatabase {
        let fpath = PathBuf::from(&path);
        if fpath.exists() && std::fs::metadata(&path).unwrap().len() > 0 {
            std::fs::read_to_string(&path).unwrap().parse().unwrap()
        } else {
            let db = MetricsDatabase {
                db_path: path.clone(),
                ..Default::default()
            };
            if fpath.exists() {
                let db_file = PathBuf::from(db.db_path.clone());
                if let Ok(mut file) = OpenOptions::new().write(true).open(db_file) {
                    file.write_all(serde_json::to_string_pretty(&db).unwrap().as_bytes())
                        .unwrap();
                }
            } else {
                db.create_if_not_exists();
            }
            db
        }
    }

    pub fn self_delete(&self) {
        let db_file = PathBuf::from(self.db_path.clone());
        if db_file.exists() {
            std::fs::remove_file(db_file).unwrap();
        }
    }

    pub fn create_if_not_exists(&self) {
        let db_file = PathBuf::from(self.db_path.clone());
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(db_file)
        {
            let db = MetricsDatabase {
                db_path: self.db_path.clone(),
                ..Default::default()
            };
            file.write_all(serde_json::to_string_pretty(&db).unwrap().as_bytes())
                .unwrap();
        }
    }

    pub fn get_current_db(&self) -> MetricsDatabase {
        let db_content = std::fs::read_to_string(&self.db_path).unwrap();
        serde_json::from_str(&db_content).unwrap()
    }

    pub fn save_db(&self, metrics_db: MetricsDatabase) {
        let db_content = serde_json::to_string_pretty(&metrics_db).unwrap();
        std::fs::write(&self.db_path, db_content).unwrap();
    }

    pub fn register_new_detector(&self, name: String, current_severity: IssueSeverity) {
        let mut state = self.get_current_db();

        if state.metrics.contains_key(&name) {
            panic!("Database already contains that key !")
        }

        let payload = Metrics {
            detector_name: name.clone(),
            true_positives: IssueSeverity::COUNT as u64,
            false_positives: 0,
            trigger_count: IssueSeverity::COUNT as u64,
            experience: IssueSeverity::COUNT as u64,
            current_severity,
        };

        state.metrics.insert(name, payload);
        self.save_db(state);
    }

    pub fn increase_true_positive_with_trigger_count(&self, name: String) {
        let mut state = self.get_current_db();
        let current_tp = state.metrics.get(&name).unwrap().true_positives;
        let current_trigger_count = state.metrics.get(&name).unwrap().trigger_count;
        state.metrics.get_mut(&name).unwrap().true_positives = current_tp + 1;
        state.metrics.get_mut(&name).unwrap().trigger_count = current_trigger_count + 1;
        self.save_db(state);
    }

    pub fn increase_false_positive_with_trigger_count(&self, name: String) {
        let mut state = self.get_current_db();
        let current_tp = state.metrics.get(&name).unwrap().false_positives;
        let current_trigger_count = state.metrics.get(&name).unwrap().trigger_count;
        state.metrics.get_mut(&name).unwrap().false_positives = current_tp + 1;
        state.metrics.get_mut(&name).unwrap().trigger_count = current_trigger_count + 1;
        self.save_db(state);
    }

    pub fn increase_trigger_count(&self, name: String) {
        let mut state = self.get_current_db();
        let current_trigger_count = state.metrics.get(&name).unwrap().trigger_count;
        state.metrics.get_mut(&name).unwrap().trigger_count = current_trigger_count + 1;
        self.save_db(state);
    }

    pub fn get_metrics(&self, detector_name: String) -> Metrics {
        let state = self.get_current_db();
        state.metrics.get(&detector_name).unwrap().clone()
    }

    pub fn get_all_metrics(&self) -> HashMap<String, Metrics> {
        let state = self.get_current_db();
        state.metrics.clone()
    }

    pub fn increase_experience(&self, name: String) {
        let mut state = self.get_current_db();
        let current_exp = state.metrics.get(&name).unwrap().experience;
        state.metrics.get_mut(&name).unwrap().experience = current_exp + 1;
        self.save_db(state);
    }

    pub fn get_all_detectors_names(&self) -> Vec<String> {
        let state = self.get_current_db();
        state.metrics.into_keys().collect()
    }

    pub fn tag_detector_with_message(&self, detector_name: String, message: String) {
        let mut state = self.get_current_db();
        if let hash_map::Entry::Vacant(e) = state.tags.entry(detector_name.clone()) {
            e.insert(Tag {
                messages: vec![message],
            });
        } else {
            let tag = state.tags.get_mut(&detector_name).unwrap();
            tag.messages.push(message)
        }
        self.save_db(state)
    }

    pub fn remove_tag(&self, detector_name: String) {
        let mut state = self.get_current_db();
        state
            .tags
            .remove(&detector_name)
            .unwrap_or_else(|| panic!("{} is invalid", &detector_name));
        self.save_db(state)
    }
}
