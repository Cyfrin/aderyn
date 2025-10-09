use askama::Template;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Template)]
#[template(path = "mcp-tool-response/project_overview.md")]
#[builder(pattern = "owned")]
pub struct ProjectOverview {
    pub root: String,
    pub source: String,
    pub remappings: Vec<String>,
    pub compilation_units: Vec<CompilationUnit>,
}

#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct CompilationUnit {
    pub files: Vec<FileEntry>,
    pub included_count: usize,
}

#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct FileEntry {
    pub path: String,
    pub included: bool,
}
