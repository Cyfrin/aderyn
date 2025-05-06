use std::error::Error;

use prettytable::Row;

use crate::{
    audit::{
        attack_surface::AttackSurfaceDetector,
        public_functions_no_sender::PublicFunctionsNoSenderChecksDetector,
    },
    context::workspace::WorkspaceContext,
};

pub fn get_auditor_detectors() -> Vec<Box<dyn AuditorDetector>> {
    vec![
        Box::<AttackSurfaceDetector>::default(),
        Box::<PublicFunctionsNoSenderChecksDetector>::default(),
    ]
}

pub trait AuditorDetector: Send + Sync + 'static {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>>;

    fn title(&self) -> String;

    fn skeletal_clone(&self) -> Box<dyn AuditorDetector>;

    fn table_titles(&self) -> Row;

    fn table_rows(&self) -> Vec<Row>;
}
