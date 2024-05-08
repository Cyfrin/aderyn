use std::error::Error;

use crate::{
    audit::attack_surface::AttackSurfaceDetector, context::workspace_context::WorkspaceContext,
};

pub fn get_auditor_detectors() -> Vec<Box<dyn AuditorDetector>> {
    vec![Box::<AttackSurfaceDetector>::default()]
}

pub trait AuditorDetector: Send + Sync + 'static {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn print(&self, _context: &WorkspaceContext) {}
}
