use crate::{
    interface::lsp::LspReport,
    process::make_context,
    runner::{run_auditor_mode, run_detector_mode, run_lsp_mode},
};
use aderyn_core::detect::detector::{get_all_issue_detectors, IssueDetector, IssueSeverity};
use field_access::FieldAccess;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, FieldAccess)]
pub struct Args {
    pub auditor_mode: bool,
    pub input_config: CliArgsInputConfig,
    pub output_config: CliArgsOutputConfig,
    pub common_config: CliArgsCommonConfig,
}

#[derive(Debug, Clone)]
pub struct CliArgsInputConfig {
    pub root: String,
    pub src: Option<String>,
    pub path_excludes: Option<Vec<String>>,
    pub path_includes: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct CliArgsOutputConfig {
    pub output: String,
    pub stdout: bool,
    pub no_snippets: bool,
}

#[derive(Debug, Clone)]
pub struct CliArgsCommonConfig {
    pub lsp: bool,
    pub skip_cloc: bool,
    pub highs_only: bool,
}

/// One way pipeline. Used by CLI
pub fn kick_off_report_creation(args: Args) {
    let detectors = detector_list(&args);

    let run_pipeline = || -> Result<(), Box<dyn std::error::Error>> {
        let cx_wrapper =
            make_context(&args.input_config, &args.common_config).unwrap_or_else(|e| {
                eprintln!("Error making context: {}", e);
                std::process::exit(1);
            });

        if args.auditor_mode {
            run_auditor_mode(&cx_wrapper.contexts)?;
        } else {
            let root_rel_path = cx_wrapper.root_path;

            // Load the workspace context into the run function, which runs the detectors
            run_detector_mode(&cx_wrapper.contexts, root_rel_path, detectors, &args.output_config)?;
        }
        Ok(())
    };

    // Kick-off
    run_pipeline().unwrap_or_else(|e| {
        eprintln!("Error driving aderyn: {}", e);
        std::process::exit(1);
    });
}

/// Drives and returns results. Used by LSP
pub fn fetch_report_for_lsp(args: Args) -> Arc<Mutex<Option<LspReport>>> {
    let detectors = detector_list(&args);

    let ctx_wrapper = match make_context(&args.input_config, &args.common_config) {
        Ok(ctx_wrapper) => ctx_wrapper,
        Err(_) => {
            return Arc::new(tokio::sync::Mutex::new(None));
        }
    };

    let (root_rel_path, contexts) = (ctx_wrapper.root_path, ctx_wrapper.contexts);
    let lsp_report = run_lsp_mode(&contexts, root_rel_path, detectors);

    Arc::new(tokio::sync::Mutex::new(lsp_report))
}

fn detector_list(args: &Args) -> Vec<Box<dyn IssueDetector>> {
    get_all_issue_detectors()
        .into_iter()
        .filter(|d| !args.common_config.highs_only || d.severity() == IssueSeverity::High)
        .collect()
}
