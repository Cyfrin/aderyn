use std::{path::PathBuf, thread};
use xshell::{Shell, cmd};

use crate::{
    flags::Reportgen,
    report_config::{ReportConfig, ReportgenConfig},
};

fn get_project_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().parent().unwrap().to_path_buf()
}

fn run_report(report: &crate::report_config::ReportConfig, release: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let project_root = get_project_root();
    sh.change_dir(&project_root);

    // Run pre_command if present
    if let Some(pre_cmd) = &report.pre_command {
        println!("Running pre-command for {}: {}", report.name, pre_cmd);
        cmd!(sh, "bash -c {pre_cmd}").run()?;
    }

    // Build the command
    let mut cmd = cmd!(sh, "cargo run");

    // Set env vars
    for (key, val) in &report.env {
        cmd = cmd.env(key, val);
    }

    if release {
        cmd = cmd.arg("--release");
    }

    cmd = cmd.arg("--").arg("--skip-update-check");
    cmd = cmd.arg(&report.root);

    for arg in &report.args {
        cmd = cmd.arg(arg);
    }

    cmd = cmd.arg("-o").arg(&report.output);

    println!("Running: {} -> {}", report.name, report.output);
    cmd.run()?;
    Ok(())
}

fn run_all_parallel(config: &ReportgenConfig, release: bool) -> anyhow::Result<()> {
    let reports: Vec<ReportConfig> = config.reports.clone();
    let mut handles = vec![];

    for report in reports {
        let name = report.name.clone();
        let handle = thread::spawn(move || run_report(&report, release));
        handles.push((name, handle));
    }

    let mut errors = vec![];
    for (name, handle) in handles {
        match handle.join() {
            Ok(Ok(())) => println!("Completed: {}", name),
            Ok(Err(e)) => errors.push(format!("{}: {}", name, e)),
            Err(_) => errors.push(format!("{}: thread panicked", name)),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        anyhow::bail!("Some reports failed:\n{}", errors.join("\n"))
    }
}

fn run_all_sequential(config: &ReportgenConfig, release: bool) -> anyhow::Result<()> {
    for report in &config.reports {
        run_report(report, release)?;
    }
    Ok(())
}

fn ci_verify_report(report: &crate::report_config::ReportConfig) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let project_root = get_project_root();
    sh.change_dir(&project_root);

    // Run CI setup if present
    if let Some(setup_cmd) = &report.ci_setup {
        println!("Running CI setup: {}", setup_cmd);
        cmd!(sh, "bash -c {setup_cmd}").run()?;
    }

    // Generate temp output path
    let baseline = &report.output;
    let workflow_output = baseline
        .replace(".md", "-workflow.md")
        .replace(".json", "-workflow.json")
        .replace(".sarif", "-workflow.sarif");

    // Build the command
    let mut cmd = cmd!(sh, "cargo run");

    // Use ci_env if present, otherwise use env
    let env_vars = if report.ci_env.is_empty() { &report.env } else { &report.ci_env };

    for (key, val) in env_vars {
        cmd = cmd.env(key, val);
    }

    cmd = cmd.arg("--").arg("--skip-update-check");
    cmd = cmd.arg(&report.root);

    for arg in &report.args {
        cmd = cmd.arg(arg);
    }

    cmd = cmd.arg("-o").arg(&workflow_output);

    println!("Generating: {} -> {}", report.name, workflow_output);
    cmd.run()?;

    // Diff against baseline
    println!("Comparing {} with {}", baseline, workflow_output);
    let diff_result = cmd!(sh, "diff {baseline} {workflow_output}").run();

    match diff_result {
        Ok(_) => {
            // Clean up workflow file on success
            let _ = std::fs::remove_file(&workflow_output);
            println!("OK: {} matches baseline", report.name);
            Ok(())
        }
        Err(_) => {
            anyhow::bail!("FAIL: {} differs from baseline {}", workflow_output, baseline);
        }
    }
}

pub fn reportgen(choice: Reportgen) -> anyhow::Result<()> {
    let project_root = get_project_root();
    let config = ReportgenConfig::load(&project_root)?;

    // Handle --list-json
    if choice.list_json {
        println!("{}", config.to_json());
        return Ok(());
    }

    // Handle --ci-verify
    if let Some(name) = &choice.ci_verify {
        let report = config.find_by_name(name).ok_or_else(|| {
            anyhow::anyhow!("Unknown report: {}. Use --list-json to see available reports.", name)
        })?;
        return ci_verify_report(report);
    }

    // Handle --all
    if choice.all {
        if choice.parallel {
            return run_all_parallel(&config, choice.release);
        } else {
            return run_all_sequential(&config, choice.release);
        }
    }

    // Handle --name
    if let Some(name) = &choice.name {
        let report = config.find_by_name(name).ok_or_else(|| {
            anyhow::anyhow!("Unknown report: {}. Use --list-json to see available reports.", name)
        })?;
        return run_report(report, choice.release);
    }

    // No arguments - show help
    println!("Usage: cargo xtask reportgen [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -n, --name <NAME>     Run a specific report by name");
    println!("  -a, --all             Run all reports");
    println!("      --parallel        Run all reports in parallel (requires --all)");
    println!("      --list-json       Output report names as JSON (for CI matrix)");
    println!("      --ci-verify <NAME> CI mode: run setup, generate, diff against baseline");
    println!("      --release         Run in release mode");
    println!();
    println!("Available reports:");
    for report in &config.reports {
        println!("  {:20} - {}", report.name, report.description);
    }

    Ok(())
}
