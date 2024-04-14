use crate::fscloc::cloc;
use ignore::{DirEntry, WalkBuilder, WalkState::Continue};
use rayon::prelude::*;
use std::{collections::HashMap, path::Path, sync::Mutex};

pub fn count_lines_of_code(src: &Path, src_filepaths: &[String]) -> Mutex<HashMap<String, usize>> {
    let walker = WalkBuilder::new(src);
    let (tx, rx) = crossbeam_channel::unbounded();
    walker.build_parallel().run(|| {
        let tx = tx.clone();
        Box::new(move |res| {
            if let Ok(target) = res {
                let target_path = target.path().to_str();
                if target.file_type().unwrap().is_file() {
                    // dbg!(target_path.unwrap());
                    if src_filepaths
                        .iter()
                        .any(|fp| target_path.map_or(false, |path| path.contains(fp)))
                    {
                        let send = target.to_owned();
                        tx.send(send).unwrap();
                    }
                }
            }
            Continue
        })
    });

    drop(tx); // without this, the program would not terminate .. becoz receiver would
              // think that the `tx` is still waiting to send something.. but we're done
              // the clones have been dropped but not the original
              // refer rust docs for more on automatic garbage collection :)

    let lines_of_code = Mutex::new(HashMap::new());

    let receive = |target_file: DirEntry| {
        // println!("Processing: {:?}", target_file.path());
        let r_content = std::fs::read_to_string(target_file.path()).unwrap();
        let stats = cloc::get_stats(&r_content);
        let key = String::from(target_file.path().to_string_lossy());
        let mut lock = lines_of_code.lock().unwrap();
        // println!("Inserting: {} - {}", key, stats.code);
        lock.insert(key, stats.code);
    };

    rx.into_iter().par_bridge().for_each(receive);
    lines_of_code
}
