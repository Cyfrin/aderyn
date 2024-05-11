use std::error::Error;

use prettytable::{format, row, Table};

use crate::{
    audit::attack_surface::AttackSurfaceDetector, context::workspace_context::WorkspaceContext,
};

pub fn get_auditor_detectors() -> Vec<Box<dyn AuditorDetector>> {
    vec![Box::<AttackSurfaceDetector>::default()]
}

#[derive(Clone)]
pub struct AuditorInstance {
    pub contract_name: String,
    pub function_name: String,
    pub source_code: String,
    pub address_source: String,
}

pub trait AuditorDetector: Send + Sync + 'static {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn instances(&self) -> Vec<AuditorInstance>;
}

pub trait AuditorPrinter {
    fn print(instances: &[AuditorInstance], title: &str) {
        let mut table = Table::new();

        println!();
        println!("{}:", title);
        table.set_titles(row!["Contract", "Function", "Code", "Address Source"]);

        for instance in instances {
            table.add_row(row![
                instance.contract_name,
                instance.function_name,
                instance.source_code,
                instance.address_source
            ]);
        }

        // Set the format of the table
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    }
}

pub struct BasicAuditorPrinter;
impl AuditorPrinter for BasicAuditorPrinter {}
