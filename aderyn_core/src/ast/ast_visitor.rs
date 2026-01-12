use eyre::Result;

use crate::ast::*;

macro_rules! define_ast_const_visitor {
    ($($node:ident),* $(,)?) => {
        paste::paste! {
            pub trait ASTConstVisitor {
                $(
                    fn [<visit_ $node:snake>](&mut self, node: &$node) -> Result<bool> {
                        self.visit_node(node)
                    }
                    fn [<end_visit_ $node:snake>](&mut self, node: &$node) -> Result<()> {
                        self.end_visit_node(node)
                    }
                )*

                fn visit_node(&mut self, _node: &impl Node) -> Result<bool> {
                    Ok(true)
                }
                fn end_visit_node(&mut self, _node: &impl Node) -> Result<()> {
                    Ok(())
                }

                fn visit_immediate_children(
                    &mut self,
                    _node_id: NodeID,
                    _node_children_ids: Vec<NodeID>,
                ) -> Result<()> {
                    Ok(())
                }

                fn visit_node_id(&mut self, _node_id: Option<NodeID>) -> Result<()> {
                    Ok(())
                }
            }
        }
    };
}

define_ast_const_visitor! {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    UncheckedBlock,
    Conditional,
    ContractDefinition,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    EmitStatement,
    EnumDefinition,
    EnumValue,
    EventDefinition,
    ErrorDefinition,
    ExpressionStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    ForStatement,
    Identifier,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    MemberAccess,
    NewExpression,
    Mapping,
    ModifierDefinition,
    ModifierInvocation,
    OverrideSpecifier,
    ParameterList,
    PragmaDirective,
    Return,
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    TryStatement,
    TryCatchClause,
    TupleExpression,
    UnaryOperation,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    DoWhileStatement,
    Continue,
    PlaceholderStatement,
    Break,
    // Yul nodes
    YulBlock,
    YulStatement,
    YulExpression,
    YulLiteral,
    YulIdentifier,
    YulFunctionCall,
    YulIf,
    YulSwitch,
    YulCase,
    YulForLoop,
    YulAssignment,
    YulVariableDeclaration,
    YulTypedName,
    YulExpressionStatement,
    YulFunctionDefinition,
}

pub trait Node {
    /// [`Node::accept`] is designed to propagate
    fn accept(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    /// [`Node::accept_metadata`] is designed to propagate into the AST subtree
    /// although it doesn't happen by itself. [`Node::accept`] triggers the propagation
    fn accept_metadata(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    /// [`Node::accept_id`] is not designed to propagate into the AST subtree
    fn accept_id(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
}

pub fn list_accept(list: &Vec<impl Node>, visitor: &mut impl ASTConstVisitor) -> Result<()> {
    for elem in list {
        elem.accept(visitor)?;
    }
    Ok(())
}
