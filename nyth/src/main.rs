use clap::{Parser, Subcommand, ValueEnum};
use handlebars::Handlebars;
use serde_json::json;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::{env, fs};
use std::{
    fs::create_dir_all,
    io::{BufWriter, Cursor},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    #[clap(subcommand)]
    pilot: PilotCommand,
}

#[derive(Debug, Subcommand)]
enum PilotCommand {
    /// Initializes a new bot with the given name.
    /// For example, to create a bot named "smart_bot", you run
    /// "nyth init path/to/smart_bot"
    Init {
        /// Bot name (including path), separated by underscores, do not use spaces.
        bot_name: String,
    },
    /// Creates a new detector with the given name.
    /// For example, to create a detector named "unindexed_events", you run
    /// "nyth new unindexed_events".
    New {
        /// Choose the type of detector to create: issue or detector.
        #[clap(value_enum)]
        detector_type: DetectorType,
        /// Name of the detector, without appending the word "detector".
        detector_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
enum DetectorType {
    Issue,
    Reusable,
}

// TODO:
// Add a "submit" type command that will "cargo run -- refresh-metadata" in the project
// directory and then do whatever with metadata/custom_bots.json

fn main() {
    let cmd_args = CommandLineArgs::parse();
    match cmd_args.pilot {
        PilotCommand::Init { bot_name } => {
            let target_dir = PathBuf::from(&bot_name);
            if target_dir.as_path().exists() {
                let panic_message = format!(
                    "Nyth cannot initialize on \"{}\" as it already exists on disk!",
                    target_dir.to_string_lossy()
                );
                eprintln!("{}", panic_message);
                std::process::exit(1);
            }

            create_dir_all(&bot_name).unwrap_or_else(|_| {
                eprintln!("Unable to create directory {} for bot!", bot_name);
                std::process::exit(1);
            });
            let archive: Vec<u8> = Vec::from(include_bytes!("../archive.zip"));

            zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();
            let _ = std::process::Command::new("git")
                .arg("init")
                .current_dir(&target_dir)
                .stdout(Stdio::inherit()) // This will stream the stdout
                .stderr(Stdio::inherit())
                .status();
            let foundry_workspace_dir = target_dir.join("foundry_workspace");

            let _ = fs::remove_dir_all(foundry_workspace_dir.join("lib/forge-std"));

            let _ = std::process::Command::new("forge")
                .args(["install", "foundry-rs/forge-std", "--no-commit"])
                .current_dir(&foundry_workspace_dir)
                .stdout(Stdio::inherit()) // This will stream the stdout
                .stderr(Stdio::inherit())
                .status();

            let _ = std::process::Command::new("forge")
                .arg("build")
                .current_dir(&foundry_workspace_dir)
                .stdout(Stdio::inherit()) // This will stream the stdout
                .stderr(Stdio::inherit())
                .status();
            println!("Bot initialized successfully at {}", bot_name);
        }
        PilotCommand::New {
            detector_type,
            detector_name,
        } => {
            let mut nyth_dir: PathBuf = env::current_dir().unwrap();
            nyth_dir.push("nyth.toml");
            if !nyth_dir.exists() {
                eprintln!("You are not in a nyth project directory!");
                std::process::exit(1);
            }
            if detector_type == DetectorType::Issue {
                create_issue_detector(&detector_name);
            } else {
                create_reusable_detector(&detector_name);
            }
        }
    }

    fn create_reusable_detector(detector_name: &str) {
        let mut detector_path = PathBuf::from("src");
        detector_path.push(detector_name);
        let filename = Path::new(&detector_path).file_name().to_owned().unwrap();
        let detector_name_snake_case = filename.to_string_lossy().to_ascii_lowercase();
        let detector_name_title_case = to_title_case(detector_name_snake_case.clone());
        let detector_name_kebab_case = to_kebab_case(detector_name_snake_case.clone());

        // Step 1 : Create the detector module by following the template

        let template = include_str!("../templates/reusable_detector_rs.hbs");

        let reg = Handlebars::new();
        use std::fs::OpenOptions;

        create_dir_all(Path::new(&detector_path)).unwrap();

        let detector_path = std::fs::canonicalize(detector_path).unwrap();

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
                    "detector_name_snake_case": detector_name_snake_case,
                    "detector_name_kebab_case": detector_name_kebab_case,
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

        // Step 3: Register with lib.rs

        let mut comps = detector_path.components().collect::<Vec<_>>();

        comps.pop();

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
            detector_name_snake_case,
            fs::read_to_string(&librs).unwrap()
        )
        .unwrap();

        println!(
            "Reusable detector created successfully at {}",
            detector_path.to_str().unwrap()
        );
    }

    fn create_issue_detector(detector_name: &str) {
        let mut detector_path = PathBuf::from("src");
        detector_path.push(detector_name);
        let filename = Path::new(&detector_path).file_name().to_owned().unwrap();
        let detector_name_snake_case = filename.to_string_lossy().to_ascii_lowercase();
        let detector_name_title_case = to_title_case(detector_name_snake_case.clone());
        let detector_name_kebab_case = to_kebab_case(detector_name_snake_case.clone());
        // Step 1 : Create the detector module by following the template

        let template = include_str!("../templates/issue_detector_rs.hbs");

        let reg = Handlebars::new();
        use std::fs::OpenOptions;

        create_dir_all(Path::new(&detector_path)).unwrap();

        let detector_path = std::fs::canonicalize(detector_path).unwrap();

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
                    "detector_name_snake_case": detector_name_snake_case,
                    "detector_name_kebab_case": detector_name_kebab_case,
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
            detector_name_snake_case,
            detector_name_title_case,
            fs::read_to_string(&custom_detector_rs_path).unwrap()
        );

        let mut filelines = s.lines().collect::<Vec<_>>();

        let mut hook_line = -1;

        for (idx, line) in filelines.iter().enumerate() {
            if line.contains("// ADERYN-PILOT: 0x02 CUSTOM DETECTORS") {
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
            detector_name_snake_case,
            fs::read_to_string(&librs).unwrap()
        )
        .unwrap();

        println!(
            "Issue detector created successfully at {}",
            detector_path.to_str().unwrap()
        );
    }
}

fn to_title_case(snake_case: String) -> String {
    // Example
    // unindexed_events -> UnindexedEventsDetector
    // TODO: cleanup
    let words = snake_case.split('_');
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

fn to_kebab_case(snake_case: String) -> String {
    // Example
    // unindexed_events -> unindexed-events
    snake_case.replace('_', "-")
}
