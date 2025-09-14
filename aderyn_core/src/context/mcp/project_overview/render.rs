use askama::Template;
use derive_builder::Builder;

#[derive(Builder, Template)]
#[template(path = "mcp/project_overview.txt")]
#[builder(pattern = "owned")]
pub struct ProjectOverview {
    pub root: String,
    pub source: String,
    pub remappings: Vec<String>,
    pub compilation_units: Vec<CompilationUnit>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CompilationUnit {
    pub files: Vec<FileEntry>,
    pub included_count: usize,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct FileEntry {
    pub path: String,
    pub included: bool,
}
