use std::path::PathBuf;

use aderyn_driver::detector::get_all_issue_detectors;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_on_contract_playground(c: &mut Criterion) {
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

criterion_group!(detectors, bench_on_contract_playground);
criterion_main!(detectors);
