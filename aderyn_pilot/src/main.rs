use clap::{Parser, Subcommand};
use handlebars::Handlebars;
use serde_json::json;
use std::io::Write;
use std::path::Path;
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
    New { bot_name: String },
    Generate { detector_name: String },
}

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
        }
        PilotCommand::Generate { detector_name } => {
            let filename = Path::new(&detector_name).file_name().to_owned().unwrap();
            let detector_name_camel_case = filename.to_string_lossy().to_ascii_lowercase();
            let detector_name_title_case = to_title_case(detector_name_camel_case.clone());

            //// create the module //////

            let template = include_str!("../templates/detector_rs.hbs");

            let reg = Handlebars::new();
            use std::fs::OpenOptions;

            let base_path = PathBuf::from(detector_name);

            create_dir_all(&base_path).unwrap();

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(base_path.join("detector.rs"))
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

            //////////////////////////////

            // mod.rs

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(base_path.join("mod.rs"))
                .unwrap();

            let mut bw = BufWriter::new(file);

            write!(bw, "{}", "pub(crate) mod detector;").unwrap();

            ///////////////////////////////

            //TODO: custom_detector.rs - add imports at the top, and insert it before
            //ADERYN-PILOT: 0x04
        }
    }
}

fn to_title_case(camel_case: String) -> String {
    // Example
    // unindexed_events -> UnindexedEventsDetector
    // TODO: cleanup
    let words = camel_case.split("_");
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
