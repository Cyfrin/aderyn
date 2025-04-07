pub mod ast;
pub mod audit;
pub mod context;
pub mod detect;
pub mod fscloc;
pub mod visitor;

pub use detect::report;

use audit::auditor::{get_auditor_detectors, AuditorPrinter, BasicAuditorPrinter};
use eyre::Result;
use prettytable::Row;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::{
    collections::{btree_map::Entry, BTreeMap},
    error::Error,
};

use crate::context::workspace_context::WorkspaceContext;

// TODO: Move this function to aderyn_driver/report

pub fn run_auditor_mode(contexts: &[WorkspaceContext]) -> Result<(), Box<dyn Error>> {
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

    for (title, (table_titles, table_rows)) in audit_detectors_with_output {
        let num_instances = table_rows.len();
        BasicAuditorPrinter::print(&title, table_titles, table_rows);
        if num_instances > 0 {
            println!("Number of instances: {}", num_instances);
        }
    }

    Ok(())
}
