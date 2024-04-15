use std::path::PathBuf;

use aderyn_driver::{
    detector::get_all_issue_detectors,
    driver::{drive, Args},
};

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_individual_detectors_on_contract_playground(c: &mut Criterion) {
    let root_path = PathBuf::from("../tests/contract-playground");
    let (_, context) = aderyn_driver::with_project_root_at(&root_path, &None, &None);

    for mut detector in get_all_issue_detectors() {
        c.bench_function(detector.name().as_str(), |b| {
            b.iter(|| {
                let _ = detector.detect(&context).unwrap();
            })
        });
    }
}

fn bench_aderyn_on_contract_playground(c: &mut Criterion) {
    let root_path = PathBuf::from("../tests/contract-playground");

    c.bench_function("aderyn", |b| {
        b.iter(|| {
            drive(Args {
                root: root_path.to_string_lossy().to_string(),
                output: String::from("aderyn-report-for-bench.md"),
                no_snippets: false,
                exclude: None,
                scope: None,
                stdout: false,
            });
        });
    });
}

criterion_group!(
    detectors,                                         // Group name is the first argument
    bench_aderyn_on_contract_playground,               // Group 1
    bench_individual_detectors_on_contract_playground, // Group 2
);
criterion_main!(detectors);
