use std::collections::HashMap;

use super::Battle;

const K: f64 = 32.0;
const SCALE: f64 = 400.0;
const BASE: f64 = 10.0;
const INIT_RATING: f64 = 1000.0;

pub fn score(battles: &[Battle]) -> HashMap<String, f64> {
    let mut rating: HashMap<String, f64> = HashMap::new();

    for battle in battles {
        let rating_a = *rating
            .get(&battle.first_contestant_id)
            .unwrap_or(&INIT_RATING);

        let rating_b = *rating
            .get(&battle.second_contestant_id)
            .unwrap_or(&INIT_RATING);

        let expected_score_a = 1.0 / (1.0 + BASE.powf((rating_b - rating_a) / SCALE));
        let expected_score_b = 1.0 / (1.0 + BASE.powf((rating_a - rating_b) / SCALE));

        let actual_score_a = battle.first_contestant_score;
        let actual_score_b = battle.second_contestant_score;

        rating.insert(
            battle.first_contestant_id.clone(),
            rating_a + (K * (actual_score_a - expected_score_a)),
        );

        rating.insert(
            battle.second_contestant_id.clone(),
            rating_b + (K * (actual_score_b - expected_score_b)),
        );
    }
    rating
}
