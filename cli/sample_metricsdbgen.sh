cargo run --bin judgeops -- -m "$@" --reset 
cargo run --bin judgeops -- -m "$@" -a
cargo run --bin judgeops -- -m "$@" give-feedback tests/sample_feedbacks/mix.json
cargo run --bin judgeops -- -m "$@" give-feedback tests/sample_feedbacks/neg_high.json
cargo run --bin judgeops -- -m "$@" give-feedback tests/sample_feedbacks/neg_high.json
cargo run --bin judgeops -- -m "$@" give-feedback tests/sample_feedbacks/neg_med.json
cargo run --bin judgeops -- -m "$@" give-feedback tests/sample_feedbacks/pos_med.json