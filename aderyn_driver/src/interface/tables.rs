use aderyn_core::{audit_tools, context::workspace::WorkspaceContext};
use prettytable::*;

pub fn print_audit_info_tables(
    contexts: &[WorkspaceContext],
) -> Result<(), Box<dyn std::error::Error>> {
    let tables = audit_tools::prepare_audit_tables(contexts)?;
    for (title, (table_titles, table_rows)) in tables {
        let num_instances = table_rows.len();
        let mut table = Table::new();

        println!();
        println!("{}:", title);
        table.set_titles(table_titles);

        for instance in table_rows {
            table.add_row(instance);
        }

        // Set the format of the table
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        if num_instances > 0 {
            println!("Number of instances: {}", num_instances);
        }
    }
    Ok(())
}
