use std::path::PathBuf;

use aderyn_driver::{
    detector::get_all_issue_detectors,
    driver::{drive, Args},
};

use criterion::{criterion_group, criterion_main, Criterion};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn bench_individual_detectors_on_contract_playground(c: &mut Criterion) {
    let root_path = PathBuf::from("../tests/contract-playground");
    let contexts = aderyn_driver::with_project_root_at(&root_path, &None, &None, &None, &None);

    for detector in get_all_issue_detectors() {
        c.bench_function(detector.name().as_str(), |b| {
            b.iter(|| {
                contexts.par_iter().for_each(|context| {
                    let mut d = detector.skeletal_clone();
                    let _ = d.detect(context).unwrap();
                })
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
                skip_build: true,
                skip_cloc: true,
                skip_update_check: true,
                path_excludes: None,
                path_includes: None,
                src: None,
                stdout: false,
                auditor_mode: false,
                highs_only: false,
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
