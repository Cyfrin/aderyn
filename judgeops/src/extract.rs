// This module will be used to extract audit feedback from the tagged report.judge.md

use aderyn_core::detect::detector::get_all_detectors_names;

pub(crate) struct FeedbackExtractor {
    pub(crate) markdown_content: String,
}

impl FeedbackExtractor {
    pub fn used_detectors_names(&self) -> Option<Vec<String>> {
        // Get a list of all the detectors that have participated in the finding
        // regardless of whether or not they have a finding.
        // (This will aid in keeping track of "experience" which can later allow
        // us to find "trigger rate.")
        let look_for = "## Detectors Used";

        let pos = self
            .markdown_content
            .find(look_for)
            .expect("Corrupted report, unable to find detectors");

        let start_pos = pos + look_for.len();

        let interested_portion = &self.markdown_content[start_pos..];
        let mut detector_names = vec![];
        for line in interested_portion.lines() {
            let pure_content = line.trim_start();
            if pure_content.starts_with("#") {
                // When we reach a new heading we know that we are at a different section
                // so we stop collecting names.
                break;
            }
            if !pure_content.is_empty() {
                detector_names.push(pure_content.to_string());
            }
        }

        // Now verify that these detectors actually exist
        let all_detectors_names = get_all_detectors_names();
        let valid = detector_names
            .iter()
            .all(|d| all_detectors_names.contains(d));

        if !valid {
            return None;
        }

        Some(detector_names)
    }

    pub fn negative_feedbacks(&self) -> Option<Vec<String>> {
        // Look for "@audit:FP" in the markdown (which stands for False Positives)
        // When you come across one, go up to find out which detector was responsible
        // in creating this issue.

        let look_for = "@audit:FP";
        let fp_positions: Vec<_> = self.markdown_content.match_indices(look_for).collect();
        let all_available_detectors = get_all_detectors_names();

        let mut bad_detectors = vec![]; // Well, "bad" as in a detector that produced at least 1 FP

        for (position, _) in fp_positions {
            let interested_portion = &self.markdown_content[..position];
            let look_for = "### Responsible : ";
            let intermediate_position = interested_portion
                .rfind(look_for)
                .expect("Corrupted report");
            let start_pos = intermediate_position + look_for.len();
            let end_position = &self.markdown_content[intermediate_position..]
                .find("\n")
                .expect("Corrupted report!");
            let detector_name =
                &self.markdown_content[start_pos..intermediate_position + *end_position];
            if !all_available_detectors.contains(&detector_name.to_string()) {
                return None;
            }
            bad_detectors.push(detector_name.to_string());
        }

        // NOTE - we may mark @audit:FP for multiple instances of findings from the same detector
        // But we only count once. Hence the dedup()
        // Rationale - Let's say 3 instances were FP and 4 instances were TP.
        // If we -1 for every FP instance and +1 for every TP instance, we end up with +1 which makes us
        // think it's performing well. But that may not be desired outcome
        bad_detectors.dedup();

        Some(bad_detectors)
    }

    pub fn triggered_detectors_names(&self) -> Option<Vec<String>> {
        // First get a list of all the detectors that have been triggered
        let look_for = "### Responsible : ";

        let places = self
            .markdown_content
            .match_indices(look_for)
            .collect::<Vec<_>>();

        let mut all_triggered_detectors = vec![];
        let all_available_detectors = get_all_detectors_names();

        for (pos, _) in places {
            let interested_portion = &self.markdown_content[pos + look_for.len()..];
            let detector_name_line = interested_portion
                .lines()
                .next()
                .take()
                .expect("Corrupted report");
            let detector_name = detector_name_line.trim();
            if !all_available_detectors.contains(&detector_name.to_string()) {
                return None;
            }
            all_triggered_detectors.push(detector_name.to_string());
        }

        Some(all_triggered_detectors)
    }

    pub fn positive_feedbacks(&self) -> Option<Vec<String>> {
        let triggered_detectors = self.triggered_detectors_names()?;
        let negative_feedbacks = self.negative_feedbacks()?;
        let mut positive_feedbacks = vec![];

        for detector in triggered_detectors {
            if !negative_feedbacks.contains(&detector) {
                positive_feedbacks.push(detector);
            }
        }

        Some(positive_feedbacks)
    }
}

#[cfg(test)]
mod extract_feedback_tests {
    use super::FeedbackExtractor;

    #[test]
    fn judge_can_extract_used_detectors_names() {
        let judged_report = include_str!("../../tests/post_audit.report-config.judge.md");
        let extractor = FeedbackExtractor {
            markdown_content: judged_report.to_string(),
        };
        let extracted = extractor.used_detectors_names();
        println!("Extracted list of used detectors: {:?}", extracted);
        assert!(extracted.is_some());
    }

    #[test]
    fn judge_can_extract_negative_feedback_detectors_names() {
        let judged_report = include_str!("../../tests/post_audit.report-config.judge.md");
        let extractor = FeedbackExtractor {
            markdown_content: judged_report.to_string(),
        };
        let extracted = extractor.negative_feedbacks();
        println!(
            "Extracted list of detectors that produced FP: {:?}",
            extracted
        );
        assert!(extracted.is_some());
        assert!(!extracted.unwrap().is_empty());
    }

    #[test]
    fn can_extract_triggered_detectors_names() {
        let judged_report = include_str!("../../tests/post_audit.report-config.judge.md");
        let extractor = FeedbackExtractor {
            markdown_content: judged_report.to_string(),
        };
        let extracted = extractor.triggered_detectors_names();
        println!(
            "Extracted list of detectors that were triggered: {:?}",
            extracted
        );
        assert!(extracted.is_some());
        assert!(!extracted.unwrap().is_empty());
    }

    #[test]
    fn judge_can_extract_positive_feedback_detectors_names() {
        let judged_report = include_str!("../../tests/post_audit.report-config.judge.md");
        let extractor = FeedbackExtractor {
            markdown_content: judged_report.to_string(),
        };
        let extracted = extractor.positive_feedbacks();
        println!(
            "Extracted list of detectors that were positive: {:?}",
            extracted
        );
        assert!(extracted.is_some());
        assert!(!extracted.unwrap().is_empty());
    }
}
