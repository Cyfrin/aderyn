use std::{error::Error, str::FromStr};

use prettytable::{format, Row, Table};
use strum::{Display, EnumString};

use crate::{
    audit::{
        attack_surface::AttackSurfaceDetector, entry_points::EntryPointsDetector,
        public_functions_no_sender::PublicFunctionsNoSenderChecksDetector,
    },
    context::workspace_context::WorkspaceContext,
};

pub fn get_all_auditor_detectors() -> Vec<Box<dyn AuditorDetector>> {
    vec![
        Box::<AttackSurfaceDetector>::default(),
        Box::<PublicFunctionsNoSenderChecksDetector>::default(),
        Box::<EntryPointsDetector>::default(),
    ]
}

pub fn get_all_auditor_detectors_names() -> Vec<String> {
    get_all_auditor_detectors().iter().map(|d| d.name()).collect()
}

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum AuditorDetectorNamePool {
    AttackSurface,
    NoSenderChecks,
    EntryPoints,
    // NOTE: `Undecided` will be the default name (for new bots).
    // If it's accepted, a new variant will be added to this enum before normalizing it in aderyn
    Undecided,
}

pub fn get_auditor_detector_by_name(name: &str) -> Box<dyn AuditorDetector> {
    // Expects a valid detector_name
    let detector_name = AuditorDetectorNamePool::from_str(name).unwrap();
    match detector_name {
        AuditorDetectorNamePool::AttackSurface => Box::<AttackSurfaceDetector>::default(),
        AuditorDetectorNamePool::NoSenderChecks => {
            Box::<PublicFunctionsNoSenderChecksDetector>::default()
        }
        AuditorDetectorNamePool::EntryPoints => Box::<EntryPointsDetector>::default(),
        AuditorDetectorNamePool::Undecided => Box::<AttackSurfaceDetector>::default(),
    }
}

impl dyn AuditorDetector {
    pub fn skeletal_clone(&self) -> Box<dyn AuditorDetector> {
        get_auditor_detector_by_name(self.name().as_str())
    }
}

pub trait AuditorDetector: Send + Sync + 'static {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn description(&self) -> String {
        String::from("Description")
    }

    fn table_titles(&self) -> Row {
        Row::new(vec![])
    }

    fn table_rows(&self) -> Vec<Row> {
        vec![]
    }

    fn name(&self) -> String {
        format!("{}", AuditorDetectorNamePool::Undecided)
    }
}

pub trait AuditorPrinter {
    fn print(title: &str, table_titles: Row, instances: Vec<Row>) {
        let mut table = Table::new();

        println!();
        println!("{}:", title);
        table.set_titles(table_titles);

        for instance in instances {
            table.add_row(instance);
        }

        // Set the format of the table
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    }
}

pub struct BasicAuditorPrinter;
impl AuditorPrinter for BasicAuditorPrinter {}
