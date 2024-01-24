use std::{fs::OpenOptions, io::BufWriter, path::PathBuf, process::Stdio};

use clap::{Parser, Subcommand};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    #[clap(subcommand, name = "assembler_env")]
    assembler_env: BotFrameworkEnvironment,
}

#[derive(Debug, Subcommand)]
enum BotFrameworkEnvironment {
    Dev {
        relative_path_to_aderyn_driver: String,
    },
    Prod {
        aderyn_driver_version: Option<String>,
    },
}

fn main() {
    let cmd_args = CommandLineArgs::parse();

    println!("[*] Assembling bot creation framework");

    /*
     * Assembler Plan
     * --------------
     *
     * Goal: Assemble `bot_detectors` so that `nyth` can use the locally modified latest
     *       version for the development phase and latest available online crate version for production.
     *
     * `bot_detectors`
     *  - Cargo.toml depends on `aderyn_driver` through local file system routing by default (in the codebase)
     *
     *
     * Desired behaviour
     * -----------------
     *
     * Dev setup:
     *
     * When you run `cargo run --bin nyth -- new path/to/project`, the dev project created
     * by `nyth` which is a replica of `bot_detectors` should be able to access `aderyn_driver`
     * in its `Cargo.toml` locally.
     *
     * Prod setup:
     *
     * When you run `cargo run --bin nyth -- new path/to/project`, the prod project created
     * by `nyth` which is a replica of `bot_detectors` should be able to access `aderyn_driver`
     * in its `Cargo.toml` either fom crates.io or if that's not specified, figure it by looking at the
     * version field in `aderyn_driver/Cargo.toml`
     *
     *
     * Way to achieve this
     * -------------------
     *
     * The above modification in `Cargo.toml` must be done in a separate space from this codebase
     * and then Zipped into an archive finally place it inside `nyth` so that those bytes can be
     * picked up at compile time from `nyth/src/main.rs`
     *
     */

    match cmd_args.assembler_env {
        BotFrameworkEnvironment::Prod {
            aderyn_driver_version,
        } => {
            // Manipulate Cargo.toml
            let old_content = std::fs::read_to_string("bot_detectors/Cargo.toml").unwrap();

            let mut hook: isize = -1;
            for (idx, line) in old_content.lines().enumerate() {
                if line.starts_with("aderyn_driver") {
                    hook = idx as isize;
                    break;
                }
            }

            let mut to_insert_content_lines = old_content.lines().collect::<Vec<_>>();

            let new_aderyn_driver_line = format!(
                "aderyn_driver = {{ version = \"{}\" }}",
                aderyn_driver_version.unwrap_or_else(get_currently_coded_version),
            );

            to_insert_content_lines[hook as usize] = &new_aderyn_driver_line;

            // Replace with temporary content
            println!("[*] Writing to bot_detectors/Cargo.toml");
            write_to_cargo_toml(to_insert_content_lines.join("\n"));

            // Make archive from new changes for nyth's new command to use
            pack_bytes_and_create_archive();

            // Restore the old content in Cargo.toml
            println!("[*] Restoring bot_detectors/Cargo.toml");
            write_to_cargo_toml(old_content);
        }
        BotFrameworkEnvironment::Dev {
            relative_path_to_aderyn_driver,
        } => {
            // Manipulate Cargo.toml
            let old_content = std::fs::read_to_string("bot_detectors/Cargo.toml").unwrap();

            let mut hook: isize = -1;
            for (idx, line) in old_content.lines().enumerate() {
                if line.starts_with("aderyn_driver") {
                    hook = idx as isize;
                    break;
                }
            }

            let mut to_insert_content_lines = old_content.lines().collect::<Vec<_>>();

            let new_aderyn_driver_line = format!(
                "aderyn_driver = {{ path = \"{}\" }}",
                relative_path_to_aderyn_driver,
            );

            to_insert_content_lines[hook as usize] = &new_aderyn_driver_line;

            // Replace with temporary content
            println!("[*] Writing to bot_detectors/Cargo.toml");
            write_to_cargo_toml(to_insert_content_lines.join("\n"));

            // Make archive from new changes for nyth's new command to use
            pack_bytes_and_create_archive();

            // Restore the old content in Cargo.toml
            println!("[*] Restoring bot_detectors/Cargo.toml");
            write_to_cargo_toml(old_content);
        }
    }
}

fn get_currently_coded_version() -> String {
    let content = std::fs::read_to_string("aderyn_driver/Cargo.toml").unwrap();
    for line in content.lines() {
        if let Some(version_str) = line.strip_prefix("version = \"") {
            return version_str[..version_str.len() - 1].to_string();
        }
    }
    panic!("This shouldn't happen");
}

fn write_to_cargo_toml(content: String) {
    std::fs::remove_file(PathBuf::from("bot_detectors/Cargo.toml")).unwrap();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(PathBuf::from("bot_detectors/Cargo.toml"))
        .unwrap();

    let mut bw = BufWriter::new(file);

    write!(bw, "{}", content).unwrap();
}

fn pack_bytes_and_create_archive() {
    println!("[*] Packing bytes");
    // Remove the existing `archive.zip`
    let _ = std::process::Command::new("touch")
        .arg("nyth/archive.zip")
        .stdout(Stdio::inherit()) // This will stream the stdout
        .stderr(Stdio::inherit())
        .status();

    let _ = std::process::Command::new("rm")
        .arg("nyth/archive.zip")
        .stdout(Stdio::inherit()) // This will stream the stdout
        .stderr(Stdio::inherit())
        .status();

    // Put the new archive which reflects changes made in `bot_detectors`
    let _output = std::process::Command::new("zip")
        .args([
            "-r9",
            "nyth/archive.zip",
            "bot_detectors",
            "-x",
            "bot_detectors/target/*",
        ])
        .stdout(Stdio::inherit()) // This will stream the stdout
        .stderr(Stdio::inherit())
        .status();

    println!("[*] Created nyth/archive.zip");
}
