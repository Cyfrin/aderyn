use clap::{Parser, Subcommand};
use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::{
    fs::create_dir_all,
    io::{BufWriter, Cursor},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    #[clap(subcommand, name = "pilot")]
    pilot: PilotCommand,
}

#[derive(Debug, Subcommand)]
enum PilotCommand {
    /// Argument - Bot name (including path).
    /// For example if the name is `smart_bot`, you run
    /// "nyth new path/to/smart_bot"
    New {
        /// Separated by underscores, do not use spaces
        bot_name: String,
    },
    /// Argument - Detector name (including path).
    /// For example if the name is "unindexed_events", you run
    /// "nyth generate path/to/smart_bot/bot/src/unindexed_events".
    Generate {
        /// Do not append the word "detector" it will be done for you
        detector_name: String,
    },
}

// TODO:
// Add a "submit" type command that will "cargo run -- refresh-metadata" in the project
// directory and then do whatever with metadata/custom_bots.json

fn main() {
    let cmd_args = CommandLineArgs::parse();
    match cmd_args.pilot {
        PilotCommand::New { bot_name } => {
            create_dir_all(&bot_name).unwrap_or_else(|_| {
                eprintln!("Unable to create directory {} for bot!", bot_name);
                std::process::exit(1);
            });
            let archive: Vec<u8> = Vec::from(include_bytes!("../archive.zip"));
            let target_dir = PathBuf::from(bot_name);
            zip_extract::extract(Cursor::new(archive), &target_dir, false).unwrap();
            let _ = std::process::Command::new("git")
                .arg("init")
                .current_dir(&target_dir)
                .stdout(Stdio::inherit()) // This will stream the stdout
                .stderr(Stdio::inherit())
                .status();
            let foundry_workspace_dir = target_dir.join("bot/foundry_workspace");
            let _ = std::process::Command::new("forge")
                .arg("build")
                .current_dir(&foundry_workspace_dir)
                .stdout(Stdio::inherit()) // This will stream the stdout
                .stderr(Stdio::inherit())
                .status();
        }
        PilotCommand::Generate { detector_name } => {
            let filename = Path::new(&detector_name).file_name().to_owned().unwrap();
            let detector_name_camel_case = filename.to_string_lossy().to_ascii_lowercase();
            let detector_name_title_case = to_title_case(detector_name_camel_case.clone());

            // Step 1 : Create the detector module by following the template

            let template = include_str!("../templates/detector_rs.hbs");

            let reg = Handlebars::new();
            use std::fs::OpenOptions;

            create_dir_all(Path::new(&detector_name)).unwrap();

            let detector_path = std::fs::canonicalize(&detector_name).unwrap();

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(detector_path.join("detector.rs"))
                .unwrap();

            let mut bw = BufWriter::new(file);

            write!(
                bw,
                "{}",
                reg.render_template(
                    template,
                    &json!({
                        "detector_name_title_case": detector_name_title_case,
                        "detector_name_camel_case": detector_name_camel_case,
                    })
                )
                .unwrap()
            )
            .unwrap();

            // Step 2: Insert mod.rs

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(detector_path.join("mod.rs"))
                .unwrap();

            let mut bw = BufWriter::new(file);

            write!(bw, "pub(crate) mod detector;").unwrap();

            // Step 3: Register it with custom_detectors.rs

            let mut comps = detector_path.components().collect::<Vec<_>>();

            comps.pop();

            let mut custom_detector_rs_path: PathBuf = comps.iter().collect();
            custom_detector_rs_path.push("bot_brain");
            custom_detector_rs_path.push("custom_detectors.rs");

            let s = format!(
                "use crate::{}::detector::{};\n{}",
                detector_name_camel_case,
                detector_name_title_case,
                fs::read_to_string(&custom_detector_rs_path).unwrap()
            );

            let mut filelines = s.lines().collect::<Vec<_>>();

            let mut hook_line = -1;

            for (idx, line) in filelines.iter().enumerate() {
                if line.contains("// ADERYN-PILOT: 0x04 CUSTOM DETECTORS") {
                    hook_line = idx as isize;
                    break;
                }
            }

            let register = format!("\t\tBox::<{}>::default(),", &detector_name_title_case);

            filelines.insert(hook_line as usize, register.as_str());

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(custom_detector_rs_path)
                .unwrap();

            let mut bw = BufWriter::new(file);

            write!(bw, "{}", filelines.join("\n")).unwrap();

            // Step 4: Register with lib.rs

            let mut librs: PathBuf = comps.iter().collect();
            librs.push("lib.rs");

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&librs)
                .unwrap();

            let mut bw = BufWriter::new(file);

            write!(
                bw,
                "pub mod {};\n{}",
                detector_name_camel_case,
                fs::read_to_string(&librs).unwrap()
            )
            .unwrap();
        }
    }
}

fn to_title_case(camel_case: String) -> String {
    // Example
    // unindexed_events -> UnindexedEventsDetector
    // TODO: cleanup
    let words = camel_case.split('_');
    let mut changed_words = vec![];
    for word in words {
        let mut wc = word.chars();
        let first_letter = wc.next();
        if first_letter.is_none() {
            continue;
        }
        let capitalized = first_letter.unwrap().to_ascii_uppercase();
        let mut letters = vec![capitalized];
        for other_char in wc {
            letters.push(other_char)
        }
        let changed_word: String = letters.into_iter().collect();
        changed_words.push(changed_word);
    }
    changed_words.push("Detector".to_string());
    changed_words.join("")
}
