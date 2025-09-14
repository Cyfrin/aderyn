macro_rules! make_route {
    ($tool:ty, $st:tt) => {{
        let t = <$tool>::new(std::sync::Arc::clone(&$st));
        rmcp::handler::server::tool::ToolRoute::new(
            rmcp::model::Tool::new(
                t.name().to_string(),
                t.description().to_string(),
                rmcp::handler::server::tool::cached_schema_for_type::<
                    <$tool as aderyn_core::context::mcp::ModelContextProtocolTool>::Input,
                >(),
            ),
            move |a| t.execute(a),
        )
    }};
}

pub(crate) use make_route;
