use crate::{ast::*, context::workspace::WorkspaceContext};
use eyre::Result;

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

            $(
                #[derive(Default)]
                pub struct [<Extract $node s>] {
                    pub extracted: Vec<$node>,
                }
                impl [<Extract $node s>] {
                    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
                        let mut extractor = Self::default();
                        node.accept(&mut extractor).unwrap_or_default();
                        extractor
                    }
                }
                impl ASTConstVisitor for [<Extract $node s>] {
                    fn [<visit_ $node:snake>](&mut self, node: &$node) -> Result<bool> {
                        self.extracted.push(node.clone());
                        Ok(true)
                    }
                }
            )*
        }
    };
}

define_ast_const_visitor! {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Break,
    Conditional,
    Continue,
    ContractDefinition,
    DoWhileStatement,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    EmitStatement,
    EnumDefinition,
    EnumValue,
    ErrorDefinition,
    EventDefinition,
    ExpressionStatement,
    ForStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    Identifier,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    Mapping,
    MemberAccess,
    ModifierDefinition,
    ModifierInvocation,
    NewExpression,
    OverrideSpecifier,
    ParameterList,
    PlaceholderStatement,
    PragmaDirective,
    Return,
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    TryCatchClause,
    TryStatement,
    TupleExpression,
    UnaryOperation,
    UncheckedBlock,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    YulAssignment,
    YulBlock,
    YulCase,
    YulExpression,
    YulExpressionStatement,
    YulForLoop,
    YulFunctionCall,
    YulFunctionDefinition,
    YulIdentifier,
    YulIf,
    YulLiteral,
    YulStatement,
    YulSwitch,
    YulTypedName,
    YulVariableDeclaration,
}

// ExtractImmediateChildren is an extractor that extracts immediate children from a node
#[derive(Default)]
pub(crate) struct ExtractImmediateChildrenIDs {
    pub extracted: Vec<NodeID>,
}

impl ExtractImmediateChildrenIDs {
    pub(crate) fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractImmediateChildrenIDs = Self::default();
        node.accept_metadata(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractImmediateChildrenIDs {
    fn visit_immediate_children(
        &mut self,
        _node_id: NodeID,
        node_children_ids: Vec<NodeID>,
    ) -> Result<()> {
        self.extracted.extend(node_children_ids);
        Ok(())
    }
}

// Extract Reference Declaration IDs
#[derive(Default)]
pub struct ExtractReferencedDeclarations {
    pub extracted: Vec<NodeID>,
}

impl ExtractReferencedDeclarations {
    pub fn from<T: Node + ?Sized>(node: &T) -> Self {
        let mut extractor: ExtractReferencedDeclarations = Self::default();
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractReferencedDeclarations {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
}

// Extract Reference Declaration IDs
pub struct ExtractReferencedDeclarationsConditionally<'a> {
    pub extracted: Vec<NodeID>,
    pub condition: Box<dyn Fn(NodeID, &'a WorkspaceContext) -> bool>,
    pub context: &'a WorkspaceContext,
}

impl<'a> ExtractReferencedDeclarationsConditionally<'a> {
    pub fn from<T: Node + ?Sized>(
        node: &T,
        context: &'a WorkspaceContext,
        condition: Box<dyn Fn(NodeID, &'a WorkspaceContext) -> bool>,
    ) -> Self {
        let mut extractor: ExtractReferencedDeclarationsConditionally =
            ExtractReferencedDeclarationsConditionally { extracted: vec![], condition, context };
        node.accept(&mut extractor).unwrap_or_default();
        extractor
    }
}

impl ASTConstVisitor for ExtractReferencedDeclarationsConditionally<'_> {
    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        if let Some(referenced_id) = node.referenced_declaration {
            self.extracted.push(referenced_id);
        }
        Ok(true)
    }
    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        if !self.condition.as_ref()(node.id, self.context) {
            return Ok(true);
        }
        self.extracted.push(node.referenced_declaration);
        Ok(true)
    }
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
