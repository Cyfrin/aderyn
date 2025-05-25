use crate::{ast::*, context::workspace::WorkspaceContext};

impl ContractDefinition {
    /// Returns sequence of all inherited contracts including itself in C3 linearized hierarchy
    #[inline]
    pub fn c3<'a>(
        &'a self,
        context: &'a WorkspaceContext,
    ) -> impl Iterator<Item = &'a ContractDefinition> {
        self.linearized_base_contracts.iter().flat_map(|c_id| context.nodes.get(c_id)).flat_map(
            |n| {
                if let ASTNode::ContractDefinition(c) = n {
                    return Some(c);
                }
                None
            },
        )
    }

    #[inline]
    pub fn next_in<'a>(
        &'a self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
    ) -> Option<&'a ContractDefinition> {
        let mut base_c3 = base_contract.c3(context);
        while let Some(c) = base_c3.next() {
            if c.id == self.id {
                return base_c3.next();
            }
        }
        None
    }

    #[inline]
    pub fn is_in<'a>(
        &'a self,
        context: &'a WorkspaceContext,
        base_contract: &'a ContractDefinition,
    ) -> bool {
        let base_c3 = base_contract.c3(context);
        for c in base_c3 {
            if c.id == self.id {
                return true;
            }
        }
        false
    }

    pub fn entrypoint_functions<'a>(
        &'a self,
        context: &'a WorkspaceContext,
    ) -> Option<Vec<&'a FunctionDefinition>> {
        context.entrypoint_functions(self)
    }
}

impl FunctionCall {
    /// Returns the function definition referenced by the function call. In practice, it's not
    /// always the case that the function call will resolve to the referenced declaration. However,
    /// the the type identifier of the real function (possibly overriding function) would be
    /// conserved i.e the same as the suspected target function
    ///
    /// Also see [`FunctionCall::is_internal_call`]
    #[inline]
    pub fn suspected_target_function<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a FunctionDefinition> {
        // The most common forms of expressions when making a function call is
        // 1) xyz()
        // 2) A.xyz() where A is super or any parent class or library name or a something on which
        //    library is being used for. (using lib for uint8) .... 6.xyz()
        match self.expression.as_ref() {
            Expression::Identifier(Identifier { referenced_declaration: Some(id), .. })
            | Expression::MemberAccess(MemberAccess { referenced_declaration: Some(id), .. }) => {
                if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(id) {
                    Some(func)
                } else {
                    None
                }
            }
            // TODO: Improve this function heuristics by exhausting enum possibilities for
            // expression
            _ => None,
        }
    }
    /// Returns the function definition or variable declarartion referenced by the function call.
    /// Also see [`FunctionCall::suspected_target_function`]
    #[inline]
    pub fn suspected_function_selector(&self, context: &WorkspaceContext) -> Option<String> {
        match self.expression.as_ref() {
            Expression::Identifier(Identifier { referenced_declaration: Some(id), .. }) => {
                if let Some(ASTNode::FunctionDefinition(func)) = context.nodes.get(id) {
                    func.function_selector.clone()
                } else {
                    None
                }
            }
            Expression::MemberAccess(MemberAccess { referenced_declaration: Some(id), .. }) => {
                let suspect = context.nodes.get(id)?;
                match suspect {
                    ASTNode::FunctionDefinition(func) => func.function_selector.clone(),
                    // could be referencing a public state variable (psuedo getter method)
                    ASTNode::VariableDeclaration(var) => var.function_selector.clone(),
                    _ => None,
                }
            }
            // TODO: Improve this function heuristics by exhausting enum possibilities for
            // expression.
            _ => None,
        }
    }
}

impl ModifierInvocation {
    /// Returns the modifier definition referenced by the modifier invocation. In practice, it's not
    /// always the case that the function call will resolve to the referenced declaration. However,
    /// the the type identifier of the real modifier (possibly overriding modifier) would be
    /// conserved i.e the same as the suspected target modifier
    #[inline]
    pub fn suspected_target_modifier<'a>(
        &self,
        context: &'a WorkspaceContext,
    ) -> Option<&'a ModifierDefinition> {
        let target_id = self.modifier_name.referenced_declaration()?;
        if let Some(ASTNode::ModifierDefinition(modifier)) = context.nodes.get(&target_id) {
            return Some(modifier);
        }
        None
    }
}
