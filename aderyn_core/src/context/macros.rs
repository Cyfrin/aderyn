macro_rules! make_route {
    ($tool:ty, $st:tt) => {{
        let t = <$tool>::new(std::sync::Arc::clone(&$st));
        rmcp::handler::server::tool::ToolRoute::new(
            rmcp::model::Tool::new(
                t.name().to_string(),
                t.description().to_string(),
                rmcp::handler::server::tool::cached_schema_for_type::<
                    <$tool as crate::context::mcp::ModelContextProtocolTool>::Input,
                >(),
            ),
            move |a| t.execute(a),
        )
    }};
}

macro_rules! mcp_success {
    ($resp:expr_2021) => {{
        use askama::Template;
        use serde::Serialize;

        let t = $resp;

        fn assert_traits<T: Serialize + Template + ?Sized>(_t: &T) {}

        // Enforce that $resp implements Serialize
        assert_traits(&t);

        // Serialize and render
        let json_value = serde_json::to_value(&t).expect("failed to serialize structured content");
        let text = t.render().expect("failed to render response");

        let call_tool_response = rmcp::model::CallToolResult {
            content: vec![rmcp::model::Content::text(&text)],
            structured_content: Some(json_value),
            is_error: Some(false),
            meta: None,
        };

        Ok(call_tool_response)
    }};
}

macro_rules! mcp_error {
    ($msg:expr_2021) => {
        Ok(rmcp::model::CallToolResult::error(vec![rmcp::model::Content::text($msg)]))
    };
    ($fmt:expr_2021, $($arg:tt)*) => {
        Ok(rmcp::model::CallToolResult::error(vec![rmcp::model::Content::text(format!($fmt, $($arg)*))]))
    };
}

pub(crate) use make_route;
pub(crate) use mcp_error;
pub(crate) use mcp_success;
