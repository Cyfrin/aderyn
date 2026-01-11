macro_rules! generate_capturable_methods {
    ($( $name:ident ),* $(,)*) => {

        #[derive(Clone)]
        pub enum Capturable {
            $($name($name),)*
            YulFunctionCall(YulFunctionCall),
            YulIdentifier(YulIdentifier),
            YulLiteral(YulLiteral),
            ASTNode(ASTNode),
        }

        $(
            impl From<$name> for Capturable {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }

            impl From<&$name> for Capturable {
                fn from(value: &$name) -> Self {
                    Self::$name(value.clone())
                }
            }
        )*

        impl From<ASTNode> for Capturable {
            fn from(value: ASTNode) -> Self {
                Self::ASTNode(value)
            }
        }

        impl From<&ASTNode> for Capturable {
            fn from(value: &ASTNode) -> Self {
                Self::ASTNode(value.clone())
            }
        }


        impl Capturable {
            pub fn make_key(&self, context: &WorkspaceContext) -> (String, usize, String) {
                match self {
                    Self::ASTNode(node) => context.get_node_sort_key(node),
                    Self::YulFunctionCall(n) => context.get_node_sort_key(&n.into()),
                    Self::YulIdentifier(n) => context.get_node_sort_key(&n.into()),
                    Self::YulLiteral(n) => context.get_node_sort_key(&n.into()),
                    $(Self::$name(n) => context.get_node_sort_key(&n.into()),)*
                }
            }
            pub fn id(&self) -> Option<NodeID> {
                match self {
                    Self::ASTNode(ast_node) => ast_node.id(),
                    Self::YulFunctionCall(_) => None,
                    Self::YulIdentifier(_) => None,
                    Self::YulLiteral(_) => None,
                    $(Self::$name(n) => Some(n.id),)*
                }
            }
        }
    };
}

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

pub(crate) use generate_capturable_methods;
pub(crate) use make_route;
pub(crate) use mcp_error;
pub(crate) use mcp_success;
