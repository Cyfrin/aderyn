use std::{
    io::{self, Result, Write},
    path::PathBuf,
};

use crate::context::workspace_context::WorkspaceContext;
use serde::Serialize;
use serde_json::Value;
use serde_sarif::sarif::{
    ArtifactLocation, Location, Message, PhysicalLocation, Region, Result as SarifResult, Run,
    Tool, ToolComponent,
};

use super::{printer::ReportPrinter, reporter::Report, Issue};

#[derive(Serialize)]
pub struct SarifContent {
    #[serde(rename = "$schema")]
    schema: String,
    version: String,
    runs: Vec<Run>,
}

pub struct SarifPrinter;

impl ReportPrinter<()> for SarifPrinter {
    fn print_report<W: Write>(
        &self,
        writer: W,
        report: &Report,
        _: &[WorkspaceContext],
        _: PathBuf,
        _: Option<String>,
        _: bool,
        stdout: bool,
        _detectors_used: &[(String, String)],
    ) -> Result<()> {
        let runs = vec![Run {
            tool: Tool {
                driver: ToolComponent {
                    name: "Aderyn".to_string(),
                    organization: Some("Cyfrin".to_string()),
                    associated_component: None,
                    contents: None,
                    dotted_quad_file_version: None,
                    download_uri: None,
                    full_description: None,
                    full_name: Some("Cyfrin - Aderyn".to_string()),
                    global_message_strings: None,
                    guid: None,
                    information_uri: Some("https://github.com/Cyfrin/aderyn".to_string()),
                    is_comprehensive: None,
                    language: None,
                    localized_data_semantic_version: None,
                    locations: None,
                    minimum_required_localized_data_semantic_version: None,
                    notifications: None,
                    product: None,
                    product_suite: None,
                    properties: None,
                    release_date_utc: None,
                    rules: None,
                    semantic_version: Some(env!("CARGO_PKG_VERSION").to_string()),
                    short_description: None,
                    supported_taxonomies: None,
                    taxa: None,
                    translation_metadata: None,
                    version: Some(env!("CARGO_PKG_VERSION").to_string()),
                },
                extensions: None,
                properties: None,
            },
            results: Some(create_sarif_results(report)),
            column_kind: None,
            addresses: None,
            artifacts: None,
            automation_details: None,
            baseline_guid: None,
            conversion: None,
            default_encoding: None,
            default_source_language: None,
            external_property_file_references: None,
            graphs: None,
            invocations: None,
            language: None,
            logical_locations: None,
            newline_sequences: None,
            original_uri_base_ids: None,
            policies: None,
            properties: None,
            redaction_tokens: None,
            run_aggregates: None,
            special_locations: None,
            taxonomies: None,
            thread_flow_locations: None,
            translations: None,
            version_control_provenance: None,
            web_requests: None,
            web_responses: None,
        }];

        let sarif_report = SarifContent {
            schema: "http://json.schemastore.org/sarif-2.1.0-rtm.6".to_string(),
            version: "2.1.0".to_string(),
            runs,
        };

        let value = serde_json::to_value(sarif_report).unwrap();
        if stdout {
            println!("STDOUT START");
            let _ = serde_json::to_writer_pretty(io::stdout(), &value);
            println!("STDOUT END");
            return Ok(());
        }
        _ = serde_json::to_writer_pretty(writer, &value);
        Ok(())
    }
}

fn create_sarif_results(report: &Report) -> Vec<SarifResult> {
    let mut sarif_results: Vec<SarifResult> = Vec::new();
    for high in report.highs.iter() {
        let sarif_result = SarifResult {
            rule_id: Some(high.detector_name.clone()),
            message: Message {
                text: Some(high.description.clone()),
                arguments: None,
                id: None,
                markdown: None,
                properties: None,
            },
            level: Some(Value::String("warning".to_string())),
            locations: Some(create_sarif_locations(high)),
            rule_index: None,
            analysis_target: None,
            code_flows: None,
            correlation_guid: None,
            fixes: None,
            graph_traversals: None,
            hosted_viewer_uri: None,
            kind: None,
            partial_fingerprints: None,
            properties: None,
            rank: None,
            related_locations: None,
            web_request: None,
            web_response: None,
            attachments: None,
            baseline_state: None,
            fingerprints: None,
            graphs: None,
            guid: None,
            occurrence_count: None,
            provenance: None,
            rule: None,
            stacks: None,
            suppressions: None,
            taxa: None,
            work_item_uris: None,
        };
        sarif_results.push(sarif_result);
    }
    for low in report.lows.iter() {
        let sarif_result = SarifResult {
            rule_id: Some(low.detector_name.clone()),
            message: Message {
                text: Some(low.description.clone()),
                arguments: None,
                id: None,
                markdown: None,
                properties: None,
            },
            level: Some(Value::String("note".to_string())),
            locations: Some(create_sarif_locations(low)),
            rule_index: None,
            analysis_target: None,
            code_flows: None,
            correlation_guid: None,
            fixes: None,
            graph_traversals: None,
            hosted_viewer_uri: None,
            kind: None,
            partial_fingerprints: None,
            properties: None,
            rank: None,
            related_locations: None,
            web_request: None,
            web_response: None,
            attachments: None,
            baseline_state: None,
            fingerprints: None,
            graphs: None,
            guid: None,
            occurrence_count: None,
            provenance: None,
            rule: None,
            stacks: None,
            suppressions: None,
            taxa: None,
            work_item_uris: None,
        };
        sarif_results.push(sarif_result);
    }
    sarif_results
}

fn create_sarif_locations(issue: &Issue) -> Vec<Location> {
    let mut locations: Vec<Location> = Vec::new();
    for ((filename, _line_number, source_location), _value) in issue.instances.iter() {
        if let Some((offset, len)) = source_location.split_once(':') {
            let location = Location {
                physical_location: Some(PhysicalLocation {
                    address: None,
                    artifact_location: Some(ArtifactLocation {
                        uri: Some(filename.clone()),
                        uri_base_id: None,
                        description: None,
                        index: None,
                        properties: None,
                    }),
                    context_region: None,
                    properties: None,
                    region: Some(Region {
                        char_offset: Some(offset.parse().unwrap()),
                        char_length: Some(len.parse().unwrap()),

                        byte_length: None,
                        byte_offset: None,
                        end_column: None,
                        end_line: None,
                        message: None,
                        properties: None,
                        snippet: None,
                        source_language: None,
                        start_column: None,
                        start_line: None,
                    }),
                }),
                properties: None,
                annotations: None,
                id: None,
                logical_locations: None,
                relationships: None,
                message: None,
            };
            locations.push(location);
        }
    }
    locations
}
