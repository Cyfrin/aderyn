use crate::fscloc::cloc;
use ignore::{DirEntry, WalkBuilder, WalkState::Continue};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Mutex,
};

use super::cloc::Stats;

pub fn count_lines_of_code_and_collect_line_numbers_to_ignore(
    src: &Path,
    src_filepaths: &[String],
    skip_cloc: bool,
    included: &HashSet<PathBuf>,
) -> Mutex<HashMap<String, Stats>> {
    let (tx, rx) = crossbeam_channel::unbounded();

    let form_path = |path: &String| {
        // Soldiity compiler shenanigans ??
        // In the line  `==== ??? ====` of the output, we're supposed to see the filename
        // But sometimes solc puts filenames with path containing two forward slashes
        // Example `contracts/templegold//AuctionBase.sol` in there
        // Although there is a separate entry for `contracts/templegold/AuctionBase.sol`.
        // We want to omit reading the former
        if path.contains("//") {
            // When using foundry-compilers-aletheia, there is no separate entry so we'll
            // adjust the same entry
            let adjusted_path = path.replace("//", "/");
            Path::new(&src).join(adjusted_path)
        } else {
            Path::new(&src).join(path)
        }
    };

    let src_filepaths_as_paths = src_filepaths
        .iter()
        .map(form_path)
        .filter(|s| included.contains(s.strip_prefix(src).unwrap()))
        .collect::<Vec<_>>();

    if !src_filepaths_as_paths.is_empty() {
        // Only add the paths to WalkBuilder that we want to do analysis on.
        let mut walker = WalkBuilder::new(src_filepaths_as_paths[0].clone());

        for item in src_filepaths_as_paths.iter().skip(1) {
            walker.add(item);
        }

        walker.build_parallel().run(|| {
            let tx = tx.clone();
            Box::new(move |res| {
                if let Ok(target) = res {
                    if target.file_type().unwrap().is_file() {
                        let send = target.to_owned();
                        tx.send(send).unwrap();
                    }
                }
                Continue
            })
        });
    }

    drop(tx); // without this, the program would not terminate .. because receiver would
              // think that the `tx` is still waiting to send something.. but we're done
              // the clones have been dropped but not the original
              // refer rust docs for more on automatic garbage collection :)

    let lines_of_code = Mutex::new(HashMap::new());

    let receive = |target_file: DirEntry| {
        // println!("Processing: {:?}", target_file.path());
        let r_content = std::fs::read_to_string(target_file.path()).unwrap();
        let stats = cloc::get_stats(&r_content, skip_cloc);
        let key = String::from(target_file.path().to_string_lossy());
        let mut lock = lines_of_code.lock().unwrap();
        // println!("Inserting: {} - {}", key, stats.code);
        lock.insert(key, stats);
    };

    rx.into_iter().par_bridge().for_each(receive);
    lines_of_code
}
