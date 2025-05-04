use crate::ast::{ImportDirective, SourceUnit, SourceUnitNode};
use eyre::{eyre, Result};
use std::io;

impl SourceUnit {
    pub fn source_line(&self, src: &str) -> Result<usize> {
        let source = match self.source.as_ref() {
            Some(source) => source.as_str(),
            _ => return Err(eyre!("not found")),
        };

        let values: Vec<Option<usize>> = src
            .split(':')
            .map(|token| {
                if token.is_empty() {
                    None
                } else {
                    token
                        .parse()
                        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
                        .ok()
                }
            })
            .collect();

        let index = values.first().and_then(|&value| value).ok_or_else(|| eyre!("not found"))?;

        if index > source.len() {
            return Err(eyre!("index out of bounds"));
        }

        Ok(source[..index].chars().filter(|&c| c == '\n').count() + 1)
    }

    pub fn import_directives(&self) -> Vec<&ImportDirective> {
        self.nodes
            .iter()
            .filter_map(|n| {
                let SourceUnitNode::ImportDirective(node) = n else {
                    return None;
                };
                Some(node)
            })
            .collect()
    }
}
