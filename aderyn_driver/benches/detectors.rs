use std::path::PathBuf;

use aderyn_driver::{
    detector::get_all_issue_detectors,
    driver::{kick_off_report_creation, Args},
    process::PreprocessedConfig,
};

use criterion::{criterion_group, criterion_main, Criterion};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn bench_individual_detectors_on_contract_playground(c: &mut Criterion) {
    let root_path = PathBuf::from("../tests/contract-playground");
    let preprocessed_config =
        PreprocessedConfig { root_path, src: None, include: None, exclude: None };
    let contexts = aderyn_driver::compile::project(preprocessed_config, false).unwrap();

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
            kick_off_report_creation(Args {
                root: root_path.to_string_lossy().to_string(),
                output: String::from("aderyn-report-for-bench.md"),
                no_snippets: false,
                skip_cloc: true,
                path_excludes: None,
                path_includes: None,
                src: None,
                stdout: false,
                auditor_mode: false,
                highs_only: false,
                lsp: false,
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
