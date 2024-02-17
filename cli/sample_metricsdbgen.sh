rm "$@"
cargo run --bin judgeops -- -m "$@" apply-judgement judgeops/samples/feedbacks/mix.json
cargo run --bin judgeops -- -m "$@" apply-judgement judgeops/samples/feedbacks/neg_high.json
cargo run --bin judgeops -- -m "$@" apply-judgement judgeops/samples/feedbacks/neg_med.json
cargo run --bin judgeops -- -m "$@" apply-judgement judgeops/samples/feedbacks/pos_med.json