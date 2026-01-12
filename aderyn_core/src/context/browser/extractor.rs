use crate::{ast::*, context::workspace::WorkspaceContext};
use eyre::Result;

macro_rules! generate_extraction_library {
    ($($node:ident),* $(,)?) => {
        paste::paste! {
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

generate_extraction_library! {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Conditional,
    ContractDefinition,
    ElementaryTypeName,
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
    StructDefinition,
    StructuredDocumentation,
    TryStatement,
    TryCatchClause,
    TupleExpression,
    UnaryOperation,
    UserDefinedTypeName,
    UsingForDirective,
    VariableDeclaration,
    WhileStatement,
    DoWhileStatement,
    Break,
    Continue,
    PlaceholderStatement,
}

/////////// EXTRACTION UTILS FOR CRATE - LEVEL ACCESS //////////////////

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
