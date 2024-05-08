use std::error::Error;

use crate::context::workspace_context::WorkspaceContext;

pub trait AuditorDetector {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn print(&self, _context: &WorkspaceContext) {}
}
