/*
 * Helps you carve a path from one file to another
 *
 * use case:
 *  Wherever you chose to keep report.md, you need a rel. pathway to link back
 *  to solidity file. This is important because absolute paths are out of scope
 *  if you want to embed them as links in markdown.
 */

use std::path::{Component, PathBuf};

pub fn carve_shortest_path(from_file: PathBuf, to_file: PathBuf) -> PathBuf {
    assert!(from_file.exists());
    assert!(to_file.exists());
    assert!(from_file.is_file());
    assert!(to_file.is_file());
    assert!(from_file.is_absolute());
    assert!(to_file.is_absolute());

    let mut to_file_comps = to_file.components();
    let mut from_file_comps = from_file.components();

    // curr_tfc - `current` `t`o_`f`ile `c`omponent
    let mut curr_tfc = to_file_comps.next();

    // curr_ffc - `current` `f`rom_`f`ile `c`omponent
    let mut curr_ffc = from_file_comps.next();

    let mut buffer = PathBuf::new();

    // Hold the max length common starting path in the buffer
    loop {
        match (curr_tfc, curr_ffc) {
            (Some(tfc), Some(ffc)) => {
                if tfc != ffc {
                    break;
                }
                buffer.push(ffc);
                curr_tfc = to_file_comps.next();
                curr_ffc = from_file_comps.next();
            }
            (_, _) => {
                break;
            }
        }
    }

    // Now, we are at the common place

    // High level 2 step plan to get to the `to_file`
    // 1. Do '../' until you reach a common place
    //     |==> you can reverse this problem (since we only care about no. of steps)
    //     |==> ask how many directories forward you should go to reach `from_file`
    //     |==> That's how many times you must come back!
    // 2. Now, go forward till you reach the `to_file`

    // STEP 1
    // Calculate '../' count
    let mut count_back = 0;

    // Keep looking foreward until you reach the to_file
    loop {
        match curr_ffc {
            Some(ffc) => {
                buffer.push(ffc);
                match ffc {
                    Component::Normal(_) => {
                        if buffer.is_file() {
                            break;
                        }
                    }
                    _ => (),
                }
                count_back += 1;
                curr_ffc = from_file_comps.next();
            }
            None => {
                break;
            }
        }
    }

    let mut backward_comps = (0..count_back)
        .map(|_| Component::ParentDir)
        .collect::<Vec<_>>();

    // STEP 2
    // Now, let's capture the forward path for  `to_file`
    let mut forward_comps = vec![];

    while let Some(comp) = curr_tfc {
        forward_comps.push(comp);
        curr_tfc = to_file_comps.next();
    }

    // Finally, concatenate both components
    backward_comps.extend(forward_comps.iter());
    let final_route = backward_comps
        .iter()
        .map(|c| c.as_os_str())
        .collect::<PathBuf>();

    final_route
}
