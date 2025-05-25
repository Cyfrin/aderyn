use crate::{audit::auditor::get_auditor_detectors, context::workspace::WorkspaceContext};
use eyre::Result;
use prettytable::Row;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::{
    collections::{btree_map::Entry, BTreeMap},
    error::Error,
};

pub type AT = Vec<(String, (Row, Vec<Row>))>;

pub fn prepare_audit_tables(contexts: &[WorkspaceContext]) -> Result<AT, Box<dyn Error>> {
    let audit_detectors_with_output = get_auditor_detectors()
        .par_iter_mut()
        .flat_map(|detector| {
            // Keys -> detector's title
            // Value -> (table titles, table rows)
            let mut grouped_instances: BTreeMap<String, (Row, Vec<Row>)> = BTreeMap::new();

            for context in contexts {
                let mut d = detector.skeletal_clone();
                if let Ok(found) = d.detect(context) {
                    if found {
                        match grouped_instances.entry(d.title()) {
                            Entry::Occupied(o) => o.into_mut().1.extend(d.table_rows()),
                            Entry::Vacant(v) => {
                                v.insert((d.table_titles(), d.table_rows()));
                            }
                        };
                    }
                }
            }

            grouped_instances
        })
        .collect::<Vec<_>>();

    Ok(audit_detectors_with_output)
}
