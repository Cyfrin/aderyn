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

pub(crate) use mcp_error;
pub(crate) use mcp_success;
