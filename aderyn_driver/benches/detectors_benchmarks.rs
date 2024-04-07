use std::path::PathBuf;

use aderyn_driver::{
    detection_modules::nc::UselessPublicFunctionDetector, detector::IssueDetector,
};

use criterion::{criterion_group, criterion_main, Criterion};

/*  Simple TEMPLATE to do benchmark */

// fn hello_world_bench(c: &mut Criterion) {
//     c.bench_function("hello_world", |b| {
//         b.iter(|| {
//             println!("Hello World!");
//         })
//     });
// }

// criterion_group!(hello_world, hello_world_bench);
// criterion_main!(hello_world);

fn bench_useless_public_function_detector(c: &mut Criterion) {
    let root_path = PathBuf::from("../tests/contract-playground");
    let (_, context) = aderyn_driver::with_project_root_at(&root_path, &None, &None);
    let mut detector = UselessPublicFunctionDetector::default();

    c.bench_function(detector.name().as_str(), |b| {
        b.iter(|| {
            let _ = detector.detect(&context).unwrap();
        })
    });
}

criterion_group!(nc, bench_useless_public_function_detector);
criterion_main!(nc);
