use super::{yul::*, *};

#[derive(Default)]
pub struct AstBuilder {
    id: i64,
    scope: i64,
}

impl AstBuilder {
    pub fn next_node_id(&mut self) -> i64 {
        let result = self.id;
        self.id += 1;
        result
    }

    pub fn next_scope(&mut self) -> i64 {
        let result = self.scope;
        self.scope += 1;
        result
    }

    pub fn loc_to_src(&self, loc: &solang_parser::pt::Loc) -> String {
        let solang_parser::pt::Loc::File(_, start, end) = loc else {
            return "0:0:0".to_string();
        };
        format!("{}:{}:0", *start, *end - *start)
    }

    pub fn build_source_unit(&mut self, input: &solang_parser::pt::SourceUnit) -> SourceUnit {
        let source_unit_id = self.next_node_id();
        let source_unit_scope = self.next_scope();

        let mut result = SourceUnit {
            license: None,
            nodes: vec![],
            exported_symbols: None,
            experimental_solidity: None,
            absolute_path: None,
            id: source_unit_id,
            source: None,
        };

        for part in input.0.iter() {
            match part {
                solang_parser::pt::SourceUnitPart::PragmaDirective(loc, identifier, literal) => {
                    let Some(identifier) = identifier else {
                        todo!()
                    };
                    let Some(literal) = literal else { todo!() };
                    result.nodes.push(SourceUnitNode::PragmaDirective(
                        self.build_pragma_directive(loc, identifier, literal),
                    ));
                }

                solang_parser::pt::SourceUnitPart::ImportDirective(input) => {
                    result.nodes.push(SourceUnitNode::ImportDirective(
                        self.build_import_directive(source_unit_scope, input),
                    ));
                }

                solang_parser::pt::SourceUnitPart::ContractDefinition(input) => {
                    result.nodes.push(SourceUnitNode::ContractDefinition(
                        self.build_contract_definition(source_unit_scope, input),
                    ));
                }

                solang_parser::pt::SourceUnitPart::EnumDefinition(input) => {
                    result.nodes.push(SourceUnitNode::EnumDefinition(
                        self.build_enum_definition(input),
                    ));
                }

                solang_parser::pt::SourceUnitPart::StructDefinition(input) => {
                    result.nodes.push(SourceUnitNode::StructDefinition(
                        self.build_struct_definition(source_unit_scope, input),
                    ));
                }

                solang_parser::pt::SourceUnitPart::EventDefinition(input) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::ErrorDefinition(input) => {
                    result.nodes.push(SourceUnitNode::ErrorDefinition(
                        self.build_error_definition(input),
                    ));
                }

                solang_parser::pt::SourceUnitPart::FunctionDefinition(input) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::VariableDefinition(input) => {
                    result.nodes.push(SourceUnitNode::VariableDeclaration(
                        self.build_variable_declaration(source_unit_scope, input),
                    ));
                }

                solang_parser::pt::SourceUnitPart::TypeDefinition(input) => {
                    result
                        .nodes
                        .push(SourceUnitNode::UserDefinedValueTypeDefinition(
                            self.build_user_defined_value_type_definition(input),
                        ));
                }

                solang_parser::pt::SourceUnitPart::Annotation(annotation) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::Using(using) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::StraySemicolon(_) => {}
            }
        }

        result
    }

    pub fn build_pragma_directive(
        &mut self,
        loc: &solang_parser::pt::Loc,
        identifier: &solang_parser::pt::Identifier,
        literal: &solang_parser::pt::StringLiteral,
    ) -> PragmaDirective {
        //
        // TODO: check identifier and split literal into expected parts
        //
        PragmaDirective {
            literals: vec![identifier.name.clone(), literal.string.clone()],
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_import_directive(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::Import,
    ) -> ImportDirective {
        match input {
            solang_parser::pt::Import::Plain(file, loc) => {
                ImportDirective {
                    file: file.string.clone(),
                    source_unit: -1, // TODO: use imported `source_unit.id`
                    scope,
                    absolute_path: Some(file.string.clone()),
                    unit_alias: String::new(),
                    name_location: Some("-1:-1:-1".to_string()), // TODO
                    symbol_aliases: vec![],
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                }
            }

            solang_parser::pt::Import::GlobalSymbol(_, _, _) => todo!(),
            solang_parser::pt::Import::Rename(_, _, _) => todo!(),
        }
    }

    pub fn build_contract_definition(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::ContractDefinition,
    ) -> ContractDefinition {
        let contract_scope = self.next_scope();

        ContractDefinition {
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: Some(self.loc_to_src(&input.name.as_ref().map(|x| x.loc).unwrap())),
            documentation: None,
            kind: match input.ty {
                solang_parser::pt::ContractTy::Abstract(_) => ContractKind::Contract,
                solang_parser::pt::ContractTy::Contract(_) => ContractKind::Contract,
                solang_parser::pt::ContractTy::Interface(_) => ContractKind::Interface,
                solang_parser::pt::ContractTy::Library(_) => ContractKind::Library,
            },
            is_abstract: Some(matches!(
                input.ty,
                solang_parser::pt::ContractTy::Abstract(_)
            )),
            base_contracts: input
                .base
                .iter()
                .map(|base| InheritanceSpecifier {
                    base_name: self.build_identifier_path(&base.name),
                    arguments: base
                        .args
                        .as_ref()
                        .map(|args| args.iter().map(|x| self.build_expression(x)).collect()),
                    src: self.loc_to_src(&base.loc),
                    id: self.next_node_id(),
                })
                .collect(),
            contract_dependencies: vec![], // TODO
            used_events: None,             // TODO
            used_errors: None,             // TODO
            nodes: input
                .parts
                .iter()
                .map(|part| match part {
                    solang_parser::pt::ContractPart::StructDefinition(x) => {
                        Some(ContractDefinitionNode::StructDefinition(
                            self.build_struct_definition(contract_scope, x),
                        ))
                    }

                    solang_parser::pt::ContractPart::EventDefinition(x) => Some(
                        ContractDefinitionNode::EventDefinition(self.build_event_definition(x)),
                    ),

                    solang_parser::pt::ContractPart::EnumDefinition(x) => Some(
                        ContractDefinitionNode::EnumDefinition(self.build_enum_definition(x)),
                    ),

                    solang_parser::pt::ContractPart::ErrorDefinition(x) => Some(
                        ContractDefinitionNode::ErrorDefinition(self.build_error_definition(x)),
                    ),

                    solang_parser::pt::ContractPart::VariableDefinition(x) => {
                        Some(ContractDefinitionNode::VariableDeclaration(
                            self.build_variable_declaration(contract_scope, x),
                        ))
                    }

                    solang_parser::pt::ContractPart::FunctionDefinition(x) => match x.ty {
                        solang_parser::pt::FunctionTy::Modifier => {
                            Some(ContractDefinitionNode::ModifierDefinition(
                                self.build_modifier_definition(contract_scope, x),
                            ))
                        }
                        _ => Some(ContractDefinitionNode::FunctionDefinition(
                            self.build_function_definition(contract_scope, x),
                        )),
                    },

                    solang_parser::pt::ContractPart::TypeDefinition(x) => {
                        Some(ContractDefinitionNode::UserDefinedValueTypeDefinition(
                            self.build_user_defined_value_type_definition(x),
                        ))
                    }

                    solang_parser::pt::ContractPart::Annotation(_) => None,

                    solang_parser::pt::ContractPart::Using(x) => {
                        Some(ContractDefinitionNode::UsingForDirective(
                            self.build_using_for_directive(x),
                        ))
                    }

                    solang_parser::pt::ContractPart::StraySemicolon(_) => None,
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect(),
            scope,
            fully_implemented: None,         // TODO
            linearized_base_contracts: None, // TODO
            internal_function_ids: None,     // TODO
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_enum_definition(
        &mut self,
        input: &solang_parser::pt::EnumDefinition,
    ) -> EnumDefinition {
        EnumDefinition {
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            members: input
                .values
                .iter()
                .map(|value| {
                    EnumValue {
                        name: value
                            .as_ref()
                            .map(|x| x.name.clone())
                            .unwrap_or_else(String::new),
                        name_location: None, // TODO
                        src: value.as_ref().map(|x| self.loc_to_src(&x.loc)).unwrap(),
                        id: self.next_node_id(),
                    }
                })
                .collect(),
            canonical_name: None, // TODO
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_struct_definition(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::StructDefinition,
    ) -> StructDefinition {
        StructDefinition {
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            visibility: Visibility::Public,
            members: input
                .fields
                .iter()
                .map(|field| {
                    VariableDeclaration {
                        base_functions: None,
                        constant: false,
                        documentation: None,
                        function_selector: None,
                        indexed: None,
                        mutability: None, // TODO
                        name: field
                            .name
                            .as_ref()
                            .map(|x| x.name.clone())
                            .unwrap_or_else(String::new),
                        name_location: field.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
                        overrides: None,
                        scope,
                        state_variable: false,
                        storage_location: StorageLocation::Default,
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                        type_name: Some(self.build_type_name(&field.ty)),
                        value: None,
                        visibility: Visibility::Public,
                        src: self.loc_to_src(&field.loc),
                        id: self.next_node_id(),
                    }
                })
                .collect(),
            scope,
            canonical_name: None, // TODO
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_event_definition(
        &mut self,
        input: &solang_parser::pt::EventDefinition,
    ) -> EventDefinition {
        let event_scope = self.next_scope();

        EventDefinition {
            anonymous: input.anonymous,
            documentation: None,
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            parameters: ParameterList {
                parameters: input
                    .fields
                    .iter()
                    .map(|field| VariableDeclaration {
                        base_functions: None,
                        constant: false,
                        documentation: None,
                        function_selector: None,
                        indexed: Some(field.indexed),
                        mutability: None,
                        name: field
                            .name
                            .as_ref()
                            .map(|x| x.name.clone())
                            .unwrap_or_else(String::new),
                        name_location: field.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
                        overrides: None,
                        scope: event_scope,
                        state_variable: false,
                        storage_location: StorageLocation::Default,
                        type_descriptions: TypeDescriptions {
                            type_identifier: None,
                            type_string: None,
                        },
                        type_name: Some(self.build_type_name(&field.ty)),
                        value: None,
                        visibility: Visibility::Public,
                        src: self.loc_to_src(&field.loc),
                        id: self.next_node_id(),
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_error_definition(
        &mut self,
        input: &solang_parser::pt::ErrorDefinition,
    ) -> ErrorDefinition {
        let error_scope = self.next_scope();

        ErrorDefinition {
            documentation: None,
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            parameters: ParameterList {
                parameters: input
                    .fields
                    .iter()
                    .map(|field| VariableDeclaration {
                        base_functions: None,
                        constant: false,
                        documentation: None,
                        function_selector: None,
                        indexed: None,
                        mutability: None,
                        name: field
                            .name
                            .as_ref()
                            .map(|x| x.name.clone())
                            .unwrap_or_else(String::new),
                        name_location: field.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
                        overrides: None,
                        scope: error_scope,
                        state_variable: false,
                        storage_location: StorageLocation::Default,
                        type_descriptions: TypeDescriptions {
                            type_identifier: None,
                            type_string: None,
                        },
                        type_name: Some(self.build_type_name(&field.ty)),
                        value: None,
                        visibility: Visibility::Public,
                        src: self.loc_to_src(&field.loc),
                        id: self.next_node_id(),
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_function_definition(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::FunctionDefinition,
    ) -> FunctionDefinition {
        let mut visibility = Visibility::Internal;
        let mut state_mutability = StateMutability::NonPayable;
        let mut is_virtual = None;
        let mut overrides = None;
        let mut modifiers = vec![];

        for attr in input.attributes.iter() {
            match attr {
                solang_parser::pt::FunctionAttribute::Visibility(x) => match x {
                    solang_parser::pt::Visibility::External(_) => visibility = Visibility::External,
                    solang_parser::pt::Visibility::Public(_) => visibility = Visibility::Public,
                    solang_parser::pt::Visibility::Internal(_) => visibility = Visibility::Internal,
                    solang_parser::pt::Visibility::Private(_) => visibility = Visibility::Private,
                },

                solang_parser::pt::FunctionAttribute::Mutability(x) => match x {
                    solang_parser::pt::Mutability::Pure(_) => {
                        state_mutability = StateMutability::Pure
                    }
                    solang_parser::pt::Mutability::View(_) => {
                        state_mutability = StateMutability::View
                    }
                    solang_parser::pt::Mutability::Constant(_) => {
                        panic!("Invalid function state mutability: Constant")
                    }
                    solang_parser::pt::Mutability::Payable(_) => {
                        state_mutability = StateMutability::Payable
                    }
                },

                solang_parser::pt::FunctionAttribute::Virtual(_) => is_virtual = Some(true),

                solang_parser::pt::FunctionAttribute::Immutable(_) => {
                    panic!("Invalid function attribute: Immutable")
                }

                solang_parser::pt::FunctionAttribute::Override(loc, x) => {
                    overrides = Some(OverrideSpecifier {
                        overrides: x.iter().map(|x| self.build_identifier_path(x)).collect(),
                        src: self.loc_to_src(loc),
                        id: self.next_node_id(),
                    })
                }

                solang_parser::pt::FunctionAttribute::BaseOrModifier(loc, x) => {
                    modifiers.push(ModifierInvocation {
                        arguments: x.args.as_ref().map(|args| {
                            args.iter().map(|arg| self.build_expression(arg)).collect()
                        }),
                        modifier_name: self.build_identifier_path(&x.name),
                        src: self.loc_to_src(loc),
                        id: self.next_node_id(),
                        kind: None, // TODO
                    })
                }

                solang_parser::pt::FunctionAttribute::Error(_) => {}
            }
        }

        let function_scope = self.next_scope();

        FunctionDefinition {
            base_functions: None, // TODO
            body: input
                .body
                .as_ref()
                .map(|body| self.build_block(function_scope, body)),
            documentation: None,               // TODO
            function_selector: None,           // TODO
            implemented: input.body.is_some(), // TODO: is this correct?
            kind: match input.ty {
                solang_parser::pt::FunctionTy::Constructor => FunctionKind::Constructor,
                solang_parser::pt::FunctionTy::Function => FunctionKind::Function,
                solang_parser::pt::FunctionTy::Fallback => FunctionKind::Fallback,
                solang_parser::pt::FunctionTy::Receive => FunctionKind::Receive,
                solang_parser::pt::FunctionTy::Modifier => {
                    panic!("Invalid function kind: Modifier")
                } // TODO: handle ahead of time?
            },
            modifiers,
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            overrides,
            parameters: ParameterList {
                parameters: input
                    .params
                    .iter()
                    .map(|(loc, parameter)| {
                        VariableDeclaration {
                            base_functions: None,
                            constant: false,
                            documentation: None,
                            function_selector: None,
                            indexed: None,
                            mutability: None,
                            name: parameter
                                .as_ref()
                                .map(|x| {
                                    x.name
                                        .as_ref()
                                        .map(|x| x.name.clone())
                                        .unwrap_or_else(String::new)
                                })
                                .unwrap(),
                            name_location: parameter
                                .as_ref()
                                .map(|x| x.name.as_ref().map(|x| self.loc_to_src(&x.loc)))
                                .unwrap(),
                            overrides: None,
                            scope: function_scope,
                            state_variable: false,
                            storage_location: parameter
                                .as_ref()
                                .map(|x| self.build_storage_location(&x.storage))
                                .unwrap(),
                            type_descriptions: TypeDescriptions {
                                type_identifier: None, // TODO
                                type_string: None,     // TODO
                            },
                            type_name: Some(
                                parameter
                                    .as_ref()
                                    .map(|x| self.build_type_name(&x.ty))
                                    .unwrap(),
                            ),
                            value: None,
                            visibility,
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        }
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            return_parameters: ParameterList {
                parameters: input
                    .returns
                    .iter()
                    .map(|(loc, parameter)| {
                        VariableDeclaration {
                            base_functions: None,
                            constant: false,
                            documentation: None,
                            function_selector: None,
                            indexed: None,
                            mutability: None,
                            name: parameter
                                .as_ref()
                                .map(|x| {
                                    x.name
                                        .as_ref()
                                        .map(|x| x.name.clone())
                                        .unwrap_or_else(String::new)
                                })
                                .unwrap(),
                            name_location: parameter
                                .as_ref()
                                .map(|x| x.name.as_ref().map(|x| self.loc_to_src(&x.loc)))
                                .unwrap(),
                            overrides: None,
                            scope: function_scope,
                            state_variable: false,
                            storage_location: parameter
                                .as_ref()
                                .map(|x| self.build_storage_location(&x.storage))
                                .unwrap(),
                            type_descriptions: TypeDescriptions {
                                type_identifier: None, // TODO
                                type_string: None,     // TODO
                            },
                            type_name: Some(
                                parameter
                                    .as_ref()
                                    .map(|x| self.build_type_name(&x.ty))
                                    .unwrap(),
                            ),
                            value: None,
                            visibility,
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        }
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            scope,
            state_mutability,
            super_function: None, // TODO
            is_virtual,
            visibility,
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_modifier_definition(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::FunctionDefinition,
    ) -> ModifierDefinition {
        let mut visibility = Visibility::Internal;
        let mut state_mutability = StateMutability::NonPayable;
        let mut is_virtual = None;
        let mut overrides = None;
        let mut modifiers = vec![];

        for attr in input.attributes.iter() {
            match attr {
                solang_parser::pt::FunctionAttribute::Visibility(x) => match x {
                    solang_parser::pt::Visibility::External(_) => visibility = Visibility::External,
                    solang_parser::pt::Visibility::Public(_) => visibility = Visibility::Public,
                    solang_parser::pt::Visibility::Internal(_) => visibility = Visibility::Internal,
                    solang_parser::pt::Visibility::Private(_) => visibility = Visibility::Private,
                },

                solang_parser::pt::FunctionAttribute::Mutability(x) => match x {
                    solang_parser::pt::Mutability::Pure(_) => {
                        state_mutability = StateMutability::Pure
                    }
                    solang_parser::pt::Mutability::View(_) => {
                        state_mutability = StateMutability::View
                    }
                    solang_parser::pt::Mutability::Constant(_) => {
                        panic!("Invalid function state mutability: Constant")
                    }
                    solang_parser::pt::Mutability::Payable(_) => {
                        state_mutability = StateMutability::Payable
                    }
                },

                solang_parser::pt::FunctionAttribute::Virtual(_) => is_virtual = Some(true),

                solang_parser::pt::FunctionAttribute::Immutable(_) => {
                    panic!("Invalid function attribute: Immutable")
                }

                solang_parser::pt::FunctionAttribute::Override(loc, x) => {
                    overrides = Some(OverrideSpecifier {
                        overrides: x.iter().map(|x| self.build_identifier_path(x)).collect(),
                        src: self.loc_to_src(loc),
                        id: self.next_node_id(),
                    })
                }

                solang_parser::pt::FunctionAttribute::BaseOrModifier(loc, x) => {
                    modifiers.push(ModifierInvocation {
                        arguments: x.args.as_ref().map(|args| {
                            args.iter().map(|arg| self.build_expression(arg)).collect()
                        }),
                        modifier_name: self.build_identifier_path(&x.name),
                        src: self.loc_to_src(loc),
                        id: self.next_node_id(),
                        kind: None, // TODO
                    })
                }

                solang_parser::pt::FunctionAttribute::Error(_) => {}
            }
        }

        let modifier_scope = self.next_scope();

        ModifierDefinition {
            body: input
                .body
                .as_ref()
                .map(|body| self.build_block(modifier_scope, body))
                .unwrap(),
            overrides,
            documentation: None,
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            parameters: ParameterList {
                parameters: input
                    .params
                    .iter()
                    .map(|(loc, parameter)| {
                        VariableDeclaration {
                            base_functions: None,
                            constant: false,
                            documentation: None,
                            function_selector: None,
                            indexed: None,
                            mutability: None,
                            name: parameter
                                .as_ref()
                                .map(|x| {
                                    x.name
                                        .as_ref()
                                        .map(|x| x.name.clone())
                                        .unwrap_or_else(String::new)
                                })
                                .unwrap(),
                            name_location: parameter
                                .as_ref()
                                .map(|x| x.name.as_ref().map(|x| self.loc_to_src(&x.loc)))
                                .unwrap(),
                            overrides: None,
                            scope: modifier_scope,
                            state_variable: false,
                            storage_location: parameter
                                .as_ref()
                                .map(|x| self.build_storage_location(&x.storage))
                                .unwrap(),
                            type_descriptions: TypeDescriptions {
                                type_identifier: None, // TODO
                                type_string: None,     // TODO
                            },
                            type_name: Some(
                                parameter
                                    .as_ref()
                                    .map(|x| self.build_type_name(&x.ty))
                                    .unwrap(),
                            ),
                            value: None,
                            visibility,
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        }
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            is_virtual,
            visibility,
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_variable_declaration(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::VariableDefinition,
    ) -> VariableDeclaration {
        let mut visibility = Visibility::Public;
        let mut mutability = None;
        let mut constant = false;
        let mut overrides = None;

        for attr in input.attrs.iter() {
            match attr {
                solang_parser::pt::VariableAttribute::Visibility(x) => match x {
                    solang_parser::pt::Visibility::External(_) => visibility = Visibility::External,
                    solang_parser::pt::Visibility::Public(_) => visibility = Visibility::Public,
                    solang_parser::pt::Visibility::Internal(_) => visibility = Visibility::Internal,
                    solang_parser::pt::Visibility::Private(_) => visibility = Visibility::Private,
                },

                solang_parser::pt::VariableAttribute::Constant(_) => constant = true,

                solang_parser::pt::VariableAttribute::Immutable(_) => {
                    mutability = Some(Mutability::Immutable)
                }

                solang_parser::pt::VariableAttribute::Override(loc, x) => {
                    overrides = Some(OverrideSpecifier {
                        overrides: x.iter().map(|x| self.build_identifier_path(x)).collect(),
                        src: self.loc_to_src(loc),
                        id: self.next_node_id(),
                    })
                }
            }
        }

        VariableDeclaration {
            base_functions: None,
            constant,
            documentation: None,
            function_selector: None,
            indexed: None,
            mutability,
            name: input
                .name
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            overrides,
            scope,
            state_variable: false, // TODO: is this in the type expression?
            storage_location: StorageLocation::Default, // TODO: is this in the type expression?
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            type_name: Some(self.build_type_name(&input.ty)),
            value: input.initializer.as_ref().map(|x| self.build_expression(x)),
            visibility,
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_user_defined_value_type_definition(
        &mut self,
        input: &solang_parser::pt::TypeDefinition,
    ) -> UserDefinedValueTypeDefinition {
        UserDefinedValueTypeDefinition {
            underlying_type: self.build_type_name(&input.ty),
            name: input.name.name.clone(),
            name_location: Some(self.loc_to_src(&input.name.loc)),
            canonical_name: None, // TODO
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_using_for_directive(
        &mut self,
        input: &solang_parser::pt::Using,
    ) -> UsingForDirective {
        UsingForDirective {
            library_name: match &input.list {
                solang_parser::pt::UsingList::Library(path) => self.build_identifier_path(path),
                solang_parser::pt::UsingList::Functions(_) => todo!(),
                solang_parser::pt::UsingList::Error => todo!(),
            },
            type_name: input.ty.as_ref().map(|x| self.build_type_name(x)),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_storage_location(
        &mut self,
        input: &Option<solang_parser::pt::StorageLocation>,
    ) -> StorageLocation {
        input
            .as_ref()
            .map(|x| match x {
                solang_parser::pt::StorageLocation::Memory(_) => StorageLocation::Memory,
                solang_parser::pt::StorageLocation::Storage(_) => StorageLocation::Storage,
                solang_parser::pt::StorageLocation::Calldata(_) => StorageLocation::Calldata,
            })
            .unwrap_or_else(|| StorageLocation::Default)
    }

    pub fn build_identifier_path(
        &mut self,
        path: &solang_parser::pt::IdentifierPath,
    ) -> IdentifierPath {
        IdentifierPath {
            name: path
                .identifiers
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<_>>()
                .join("."), // TODO
            referenced_declaration: None, // TODO
            src: self.loc_to_src(&path.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_type_name(&mut self, input: &solang_parser::pt::Expression) -> TypeName {
        match input {
            solang_parser::pt::Expression::Type(_loc, ty) => match ty {
                solang_parser::pt::Type::Address => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: "address".to_string(),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::AddressPayable => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: "address payable".to_string(),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::Payable => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: "payable".to_string(),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::Bool => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: "bool".to_string(),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None,     // TODO
                    },
                }),

                solang_parser::pt::Type::String => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: "string".to_string(),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::Int(bits) => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: format!("int{bits}"),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::Uint(bits) => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: format!("uint{bits}"),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::Bytes(bytes) => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: format!("bytes{bytes}"),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::Rational => todo!(),

                solang_parser::pt::Type::DynamicBytes => {
                    TypeName::ElementaryTypeName(ElementaryTypeName {
                        state_mutability: None, // TODO
                        name: "bytes".to_string(),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }

                solang_parser::pt::Type::Mapping {
                    loc,
                    key,
                    key_name,
                    value,
                    value_name,
                } => TypeName::Mapping(Mapping {
                    key_type: Box::new(self.build_type_name(key.as_ref())),
                    value_type: Box::new(self.build_type_name(value.as_ref())),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None,     // TODO
                    },
                }),

                solang_parser::pt::Type::Function {
                    params,
                    attributes,
                    returns,
                } => {
                    let mut visibility = Visibility::Internal;
                    let mut state_mutability = StateMutability::NonPayable;
                    let mut is_virtual = None;
                    let mut overrides = None;
                    let mut modifiers = vec![];

                    for attr in attributes.iter() {
                        match attr {
                            solang_parser::pt::FunctionAttribute::Visibility(x) => match x {
                                solang_parser::pt::Visibility::External(_) => {
                                    visibility = Visibility::External
                                }
                                solang_parser::pt::Visibility::Public(_) => {
                                    visibility = Visibility::Public
                                }
                                solang_parser::pt::Visibility::Internal(_) => {
                                    visibility = Visibility::Internal
                                }
                                solang_parser::pt::Visibility::Private(_) => {
                                    visibility = Visibility::Private
                                }
                            },

                            solang_parser::pt::FunctionAttribute::Mutability(x) => match x {
                                solang_parser::pt::Mutability::Pure(_) => {
                                    state_mutability = StateMutability::Pure
                                }
                                solang_parser::pt::Mutability::View(_) => {
                                    state_mutability = StateMutability::View
                                }
                                solang_parser::pt::Mutability::Constant(_) => {
                                    panic!("Invalid function state mutability: Constant")
                                }
                                solang_parser::pt::Mutability::Payable(_) => {
                                    state_mutability = StateMutability::Payable
                                }
                            },

                            solang_parser::pt::FunctionAttribute::Virtual(_) => {
                                is_virtual = Some(true)
                            }

                            solang_parser::pt::FunctionAttribute::Immutable(_) => {
                                panic!("Invalid function attribute: Immutable")
                            }

                            solang_parser::pt::FunctionAttribute::Override(loc, x) => {
                                overrides = Some(OverrideSpecifier {
                                    overrides: x
                                        .iter()
                                        .map(|x| self.build_identifier_path(x))
                                        .collect(),
                                    src: self.loc_to_src(loc),
                                    id: self.next_node_id(),
                                })
                            }

                            solang_parser::pt::FunctionAttribute::BaseOrModifier(loc, x) => {
                                modifiers.push(ModifierInvocation {
                                    arguments: x.args.as_ref().map(|args| {
                                        args.iter().map(|arg| self.build_expression(arg)).collect()
                                    }),
                                    modifier_name: self.build_identifier_path(&x.name),
                                    src: self.loc_to_src(loc),
                                    id: self.next_node_id(),
                                    kind: None, // TODO
                                })
                            }

                            solang_parser::pt::FunctionAttribute::Error(_) => {}
                        }
                    }

                    TypeName::FunctionTypeName(FunctionTypeName {
                        visibility,
                        state_mutability,
                        parameter_types: ParameterList {
                            parameters: params
                                .iter()
                                .map(|(loc, parameter)| {
                                    VariableDeclaration {
                                        base_functions: None,
                                        constant: false,
                                        documentation: None,
                                        function_selector: None,
                                        indexed: None,
                                        mutability: None,
                                        name: parameter
                                            .as_ref()
                                            .map(|x| {
                                                x.name
                                                    .as_ref()
                                                    .map(|x| x.name.clone())
                                                    .unwrap_or_else(String::new)
                                            })
                                            .unwrap(),
                                        name_location: parameter
                                            .as_ref()
                                            .map(|x| {
                                                x.name.as_ref().map(|x| self.loc_to_src(&x.loc))
                                            })
                                            .unwrap(),
                                        overrides: None,
                                        scope: -1, // TODO
                                        state_variable: false,
                                        storage_location: parameter
                                            .as_ref()
                                            .map(|x| self.build_storage_location(&x.storage))
                                            .unwrap(),
                                        type_descriptions: TypeDescriptions {
                                            type_identifier: None, // TODO
                                            type_string: None,     // TODO
                                        },
                                        type_name: Some(
                                            parameter
                                                .as_ref()
                                                .map(|x| self.build_type_name(&x.ty))
                                                .unwrap(),
                                        ),
                                        value: None,
                                        visibility,
                                        src: self.loc_to_src(loc),
                                        id: self.next_node_id(),
                                    }
                                })
                                .collect(),
                            src: "-1:-1:-1".to_string(), // TODO
                            id: self.next_node_id(),
                        },
                        return_parameter_types: ParameterList {
                            parameters: returns
                                .as_ref()
                                .map(|(returns, _attrs)| {
                                    returns
                                        .iter()
                                        .map(|(loc, parameter)| {
                                            VariableDeclaration {
                                                base_functions: None,
                                                constant: false,
                                                documentation: None,
                                                function_selector: None,
                                                indexed: None,
                                                mutability: None,
                                                name: parameter
                                                    .as_ref()
                                                    .map(|x| {
                                                        x.name
                                                            .as_ref()
                                                            .map(|x| x.name.clone())
                                                            .unwrap_or_else(String::new)
                                                    })
                                                    .unwrap(),
                                                name_location: parameter
                                                    .as_ref()
                                                    .map(|x| {
                                                        x.name
                                                            .as_ref()
                                                            .map(|x| self.loc_to_src(&x.loc))
                                                    })
                                                    .unwrap(),
                                                overrides: None,
                                                scope: -1, // TODO
                                                state_variable: false,
                                                storage_location: parameter
                                                    .as_ref()
                                                    .map(|x| {
                                                        self.build_storage_location(&x.storage)
                                                    })
                                                    .unwrap(),
                                                type_descriptions: TypeDescriptions {
                                                    type_identifier: None, // TODO
                                                    type_string: None,     // TODO
                                                },
                                                type_name: Some(
                                                    parameter
                                                        .as_ref()
                                                        .map(|x| self.build_type_name(&x.ty))
                                                        .unwrap(),
                                                ),
                                                value: None,
                                                visibility,
                                                src: self.loc_to_src(loc),
                                                id: self.next_node_id(),
                                            }
                                        })
                                        .collect()
                                })
                                .unwrap_or_else(|| vec![]),
                            src: "-1:-1:-1".to_string(), // TODO
                            id: self.next_node_id(),
                        },
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                    })
                }
            },

            solang_parser::pt::Expression::ArraySubscript(_loc, ty, len) => {
                TypeName::ArrayTypeName(ArrayTypeName {
                    base_type: Box::new(self.build_type_name(ty)),
                    length: len.as_ref().map(|x| self.build_literal(x)),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None,     // TODO
                    },
                })
            }

            solang_parser::pt::Expression::Variable(identifier) => {
                TypeName::UserDefinedTypeName(UserDefinedTypeName {
                    path_node: None,            // TODO
                    referenced_declaration: -1, // TODO
                    name: Some(identifier.name.clone()),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None,     // TODO
                    },
                })
            }

            _ => panic!("Unhandled type name expression: {input:#?}"),
        }
    }

    pub fn build_block(&mut self, scope: i64, input: &solang_parser::pt::Statement) -> Block {
        match input {
            solang_parser::pt::Statement::Block {
                loc, statements, ..
            } => Block {
                statements: statements
                    .iter()
                    .map(|stmt| self.build_statement(scope, stmt))
                    .collect(),
                src: self.loc_to_src(loc),
                id: self.next_node_id(),
            },
            stmt => panic!("Invalid block statement: {stmt:?}"),
        }
    }

    pub fn build_block_or_statement(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::Statement,
    ) -> BlockOrStatement {
        match input {
            solang_parser::pt::Statement::Block { .. } => {
                BlockOrStatement::Block(Box::new(self.build_block(scope, input)))
            }
            _ => BlockOrStatement::Statement(Box::new(self.build_statement(scope, input))),
        }
    }

    pub fn build_statement(
        &mut self,
        scope: i64,
        input: &solang_parser::pt::Statement,
    ) -> Statement {
        match input {
            solang_parser::pt::Statement::Block { unchecked, .. } => {
                if !*unchecked {
                    panic!("Generic block passed as statement: {input:#?}");
                }

                let unchecked_scope = self.next_scope();

                Statement::UncheckedBlock(self.build_block(unchecked_scope, input))
            }

            solang_parser::pt::Statement::Assembly {
                loc,
                dialect,
                flags,
                block,
            } => Statement::InlineAssembly(self.build_inline_assembly(
                loc,
                dialect.as_ref(),
                flags.as_ref().map(|x| x.as_slice()),
                block,
            )),

            solang_parser::pt::Statement::Args(_, _) => todo!(),

            solang_parser::pt::Statement::If(loc, condition, true_body, false_body) => {
                let if_true_scope = self.next_scope();
                let if_false_scope = self.next_scope();

                Statement::IfStatement(IfStatement {
                    condition: self.build_expression(condition),
                    true_body: self.build_block_or_statement(if_true_scope, true_body),
                    false_body: false_body
                        .as_ref()
                        .map(|x| self.build_block_or_statement(if_false_scope, x)),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Statement::While(loc, condition, body) => {
                let while_scope = self.next_scope();

                Statement::WhileStatement(WhileStatement {
                    condition: self.build_expression(condition),
                    body: self.build_block_or_statement(while_scope, body),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Statement::Expression(loc, x) => match x {
                solang_parser::pt::Expression::Variable(identifier) => {
                    match identifier.name.as_str() {
                        "_" => Statement::PlaceholderStatement {
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        },

                        _ => Statement::ExpressionStatement(ExpressionStatement {
                            expression: self.build_expression(x),
                        }),
                    }
                }

                _ => Statement::ExpressionStatement(ExpressionStatement {
                    expression: self.build_expression(x),
                }),
            },

            solang_parser::pt::Statement::VariableDefinition(loc, variable, value) => {
                Statement::VariableDeclarationStatement(VariableDeclarationStatement {
                    assignments: vec![], // TODO
                    declarations: vec![Some(VariableDeclaration {
                        base_functions: None, // TODO
                        constant: false,
                        documentation: None,
                        function_selector: None, // TODO
                        indexed: None,
                        mutability: None, // TODO
                        name: variable
                            .name
                            .as_ref()
                            .map(|x| x.name.clone())
                            .unwrap_or_else(String::new),
                        name_location: variable.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
                        overrides: None, // TODO
                        scope,
                        state_variable: false, // TODO
                        storage_location: self.build_storage_location(&variable.storage),
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None,     // TODO
                        },
                        type_name: Some(self.build_type_name(&variable.ty)),
                        value: None,
                        visibility: Visibility::Public, // TODO
                        src: self.loc_to_src(&variable.loc),
                        id: self.next_node_id(),
                    })],
                    initial_value: value.as_ref().map(|x| self.build_expression(x)),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Statement::For(loc, init, condition, update, body) => {
                let for_scope = self.next_scope();

                Statement::ForStatement(ForStatement {
                    initialization_expression: init
                        .as_ref()
                        .map(|x| Box::new(self.build_statement(for_scope, x))),
                    condition: condition.as_ref().map(|x| self.build_expression(x)),
                    loop_expression: update.as_ref().map(|x| {
                        Box::new(Statement::ExpressionStatement(ExpressionStatement {
                            expression: self.build_expression(x),
                        }))
                    }),
                    body: body
                        .as_ref()
                        .map(|x| self.build_block_or_statement(for_scope, x))
                        .unwrap(),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Statement::DoWhile(loc, body, condition) => {
                let do_while_scope = self.next_scope();

                Statement::DoWhileStatement(DoWhileStatement {
                    body: self.build_block_or_statement(do_while_scope, body),
                    condition: self.build_expression(condition),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Statement::Continue(loc) => Statement::Continue {
                src: self.loc_to_src(loc),
                id: self.next_node_id(),
            },

            solang_parser::pt::Statement::Break(loc) => Statement::Break {
                src: self.loc_to_src(loc),
                id: self.next_node_id(),
            },

            solang_parser::pt::Statement::Return(loc, value) => {
                Statement::Return(Return {
                    function_return_parameters: -1, // TODO
                    expression: value.as_ref().map(|x| self.build_expression(x)),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Statement::Revert(loc, function, arguments) => {
                match function {
                    Some(function) => Statement::RevertStatement(RevertStatement {
                        error_call: FunctionCall {
                            kind: FunctionCallKind::FunctionCall,
                            try_call: Some(false),
                            names: vec![],
                            arguments: arguments
                                .iter()
                                .map(|arg| self.build_expression(arg))
                                .collect(),
                            expression: Box::new(Expression::Identifier(Identifier {
                                argument_types: None, // TODO
                                name: function
                                    .identifiers
                                    .iter()
                                    .map(|x| x.name.clone())
                                    .collect::<Vec<_>>()
                                    .join("."), // TODO
                                overloaded_declarations: vec![], // TODO
                                referenced_declaration: -1, // TODO
                                type_descriptions: TypeDescriptions {
                                    type_identifier: None, // TODO
                                    type_string: None,     // TODO
                                },
                                src: self.loc_to_src(&function.loc),
                                id: self.next_node_id(),
                            })),
                            argument_types: None,     // TODO
                            is_constant: false,       // TODO
                            is_l_value: false,        // TODO
                            is_pure: false,           // TODO
                            l_value_requested: false, // TODO
                            type_descriptions: TypeDescriptions {
                                type_identifier: None, // TODO
                                type_string: None,     // TODO
                            },
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        },
                    }),

                    None => Statement::ExpressionStatement(ExpressionStatement {
                        expression: Expression::FunctionCall(FunctionCall {
                            kind: FunctionCallKind::FunctionCall,
                            try_call: Some(false),
                            names: vec![],
                            arguments: arguments
                                .iter()
                                .map(|arg| self.build_expression(arg))
                                .collect(),
                            expression: Box::new(Expression::Identifier(Identifier {
                                argument_types: None, // TODO
                                name: "revert".to_string(),
                                overloaded_declarations: vec![], // TODO
                                referenced_declaration: -1,      // TODO
                                type_descriptions: TypeDescriptions {
                                    type_identifier: None, // TODO
                                    type_string: None,     // TODO
                                },
                                src: self.loc_to_src(loc),
                                id: self.next_node_id(),
                            })),
                            argument_types: None,     // TODO
                            is_constant: false,       // TODO
                            is_l_value: false,        // TODO
                            is_pure: false,           // TODO
                            l_value_requested: false, // TODO
                            type_descriptions: TypeDescriptions {
                                type_identifier: None, // TODO
                                type_string: None,     // TODO
                            },
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        }),
                    }),
                }
            }

            solang_parser::pt::Statement::RevertNamedArgs(_, _, _) => todo!(),

            solang_parser::pt::Statement::Emit(_loc, x) => {
                Statement::EmitStatement(EmitStatement {
                    event_call: self.build_expression(x),
                })
            }

            solang_parser::pt::Statement::Try(_, _, _, _) => todo!(),
            solang_parser::pt::Statement::Error(_) => todo!(),
        }
    }

    pub fn build_literal(&mut self, input: &solang_parser::pt::Expression) -> Literal {
        match input {
            solang_parser::pt::Expression::BoolLiteral(loc, x) => Literal {
                hex_value: None, // TODO
                value: Some(format!("{x}")),
                subdenomination: None,
                kind: LiteralKind::Bool,
                argument_types: None,
                is_constant: false,       // TODO
                is_l_value: false,        // TODO
                is_pure: false,           // TODO
                l_value_requested: false, // TODO
                type_descriptions: TypeDescriptions {
                    type_identifier: None, // TODO
                    type_string: None,     // TODO
                },
                src: self.loc_to_src(loc),
                id: self.next_node_id(),
            },

            solang_parser::pt::Expression::NumberLiteral(loc, x, _, _) => Literal {
                hex_value: None, // TODO
                value: Some(format!("{x}")),
                subdenomination: None,
                kind: LiteralKind::Number,
                argument_types: None,
                is_constant: false,       // TODO
                is_l_value: false,        // TODO
                is_pure: false,           // TODO
                l_value_requested: false, // TODO
                type_descriptions: TypeDescriptions {
                    type_identifier: None, // TODO
                    type_string: None,     // TODO
                },
                src: self.loc_to_src(loc),
                id: self.next_node_id(),
            },

            solang_parser::pt::Expression::HexNumberLiteral(loc, value, _) => Literal {
                hex_value: Some(value.clone()), // TODO
                value: None,
                subdenomination: None,
                kind: LiteralKind::String,
                argument_types: None,
                is_constant: false,       // TODO
                is_l_value: false,        // TODO
                is_pure: false,           // TODO
                l_value_requested: false, // TODO
                type_descriptions: TypeDescriptions {
                    type_identifier: None, // TODO
                    type_string: None,     // TODO
                },
                src: self.loc_to_src(loc),
                id: self.next_node_id(),
            },

            solang_parser::pt::Expression::StringLiteral(x) => Literal {
                hex_value: None, // TODO
                value: Some(
                    x.iter()
                        .map(|x| x.string.clone())
                        .collect::<Vec<_>>()
                        .join(""),
                ), // TODO: why is it a vec?
                subdenomination: None,
                kind: LiteralKind::String,
                argument_types: None,
                is_constant: false,       // TODO
                is_l_value: false,        // TODO
                is_pure: false,           // TODO
                l_value_requested: false, // TODO
                type_descriptions: TypeDescriptions {
                    type_identifier: None, // TODO
                    type_string: None,     // TODO
                },
                src: "-1:-1:-1".to_string(), // TODO
                id: self.next_node_id(),
            },

            solang_parser::pt::Expression::AddressLiteral(loc, x) => Literal {
                hex_value: None, // TODO
                value: Some(x.clone()),
                subdenomination: None,
                kind: LiteralKind::Address,
                argument_types: None,
                is_constant: false,       // TODO
                is_l_value: false,        // TODO
                is_pure: false,           // TODO
                l_value_requested: false, // TODO
                type_descriptions: TypeDescriptions {
                    type_identifier: None, // TODO
                    type_string: None,     // TODO
                },
                src: self.loc_to_src(loc),
                id: self.next_node_id(),
            },

            _ => panic!("Invalid literal expression: {input:#?}"),
        }
    }

    pub fn build_unary_operation(
        &mut self,
        loc: &solang_parser::pt::Loc,
        value: &solang_parser::pt::Expression,
        operator: &str,
        prefix: bool,
    ) -> UnaryOperation {
        UnaryOperation {
            prefix,
            sub_expression: Box::new(self.build_expression(value)),
            operator: operator.to_string(),
            argument_types: None,     // TODO
            is_constant: false,       // TODO
            is_l_value: false,        // TODO
            is_pure: false,           // TODO
            l_value_requested: false, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_binary_operation(
        &mut self,
        loc: &solang_parser::pt::Loc,
        lhs: &solang_parser::pt::Expression,
        operator: &str,
        rhs: &solang_parser::pt::Expression,
    ) -> BinaryOperation {
        BinaryOperation {
            common_type: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            left_expression: Box::new(self.build_expression(lhs)),
            right_expression: Box::new(self.build_expression(rhs)),
            operator: operator.to_string(),
            argument_types: None, // TODO
            is_constant: false,
            is_l_value: false,
            is_pure: false,
            l_value_requested: false,
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_assignment(
        &mut self,
        loc: &solang_parser::pt::Loc,
        lhs: &solang_parser::pt::Expression,
        operator: &str,
        rhs: &solang_parser::pt::Expression,
    ) -> Assignment {
        Assignment {
            left_hand_side: Box::new(self.build_expression(lhs)),
            right_hand_side: Box::new(self.build_expression(rhs)),
            operator: operator.to_string(),
            argument_types: None,     // TODO
            is_constant: false,       // TODO
            is_l_value: false,        // TODO
            is_pure: false,           // TODO
            l_value_requested: false, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_conditional(
        &mut self,
        loc: &solang_parser::pt::Loc,
        condition: &solang_parser::pt::Expression,
        true_expression: &solang_parser::pt::Expression,
        false_expression: &solang_parser::pt::Expression,
    ) -> Conditional {
        Conditional {
            condition: Box::new(self.build_expression(condition)),
            true_expression: Box::new(self.build_expression(true_expression)),
            false_expression: Box::new(self.build_expression(false_expression)),
            argument_types: None,     // TODO
            is_constant: false,       // TODO
            is_l_value: false,        // TODO
            is_pure: false,           // TODO
            l_value_requested: false, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_new_expression(
        &mut self,
        loc: &solang_parser::pt::Loc,
        expression: &solang_parser::pt::Expression,
    ) -> NewExpression {
        NewExpression {
            argument_types: None, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            type_name: self.build_type_name(expression),
            is_constant: false,
            is_l_value: false,
            is_pure: false,
            l_value_requested: false,
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_member_access(
        &mut self,
        loc: &solang_parser::pt::Loc,
        expression: &solang_parser::pt::Expression,
        member: &solang_parser::pt::Identifier,
    ) -> MemberAccess {
        MemberAccess {
            member_name: member.name.clone(),
            expression: Box::new(self.build_expression(expression)),
            referenced_declaration: None, // TODO
            argument_types: None,         // TODO
            is_constant: false,
            is_l_value: false,
            is_pure: false,
            l_value_requested: false,
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_function_call(
        &mut self,
        loc: &solang_parser::pt::Loc,
        expression: &solang_parser::pt::Expression,
        arguments: &[solang_parser::pt::Expression],
    ) -> FunctionCall {
        FunctionCall {
            kind: FunctionCallKind::FunctionCall,
            try_call: None, // TODO
            names: vec![],  // TODO
            arguments: arguments.iter().map(|x| self.build_expression(x)).collect(),
            expression: Box::new(self.build_expression(expression)),
            argument_types: None,     // TODO
            is_constant: false,       // TODO
            is_l_value: false,        // TODO
            is_pure: false,           // TODO
            l_value_requested: false, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_function_call_options(
        &mut self,
        loc: &solang_parser::pt::Loc,
        call: &solang_parser::pt::Expression,
        options: &solang_parser::pt::Statement,
    ) -> FunctionCallOptions {
        let solang_parser::pt::Statement::Args(loc, args) = options else {
            panic!("Invalid function call options: {options:#?}");
        };

        FunctionCallOptions {
            names: args.iter().map(|arg| arg.name.name.clone()).collect(),
            options: args
                .iter()
                .map(|arg| self.build_expression(&arg.expr))
                .collect(),
            arguments: None,      // TODO
            argument_types: None, // TODO
            expression: Box::new(self.build_expression(call)),
            is_constant: false,       // TODO
            is_l_value: false,        // TODO
            is_pure: false,           // TODO
            l_value_requested: false, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_index_access(
        &mut self,
        loc: &solang_parser::pt::Loc,
        array: &solang_parser::pt::Expression,
        index: &solang_parser::pt::Expression,
    ) -> IndexAccess {
        IndexAccess {
            base_expression: Box::new(self.build_expression(array)),
            index_expression: Some(Box::new(self.build_expression(index))),
            argument_types: None,     // TODO
            is_constant: false,       // TODO
            is_l_value: false,        // TODO
            is_pure: false,           // TODO
            l_value_requested: false, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_index_range_access(
        &mut self,
        loc: &solang_parser::pt::Loc,
        array: &solang_parser::pt::Expression,
        start: Option<&solang_parser::pt::Expression>,
        end: Option<&solang_parser::pt::Expression>,
    ) -> IndexRangeAccess {
        IndexRangeAccess {
            base_expression: Box::new(self.build_expression(array)),
            start_expression: start.as_ref().map(|x| Box::new(self.build_expression(x))),
            end_expression: end.as_ref().map(|x| Box::new(self.build_expression(x))),
            is_constant: false,       // TODO
            is_l_value: false,        // TODO
            is_pure: false,           // TODO
            l_value_requested: false, // TODO
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None,     // TODO
            },
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_expression(&mut self, input: &solang_parser::pt::Expression) -> Expression {
        match input {
            solang_parser::pt::Expression::PostIncrement(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "++", false))
            }

            solang_parser::pt::Expression::PostDecrement(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "--", false))
            }

            solang_parser::pt::Expression::New(loc, x) => {
                Expression::NewExpression(self.build_new_expression(loc, x))
            }

            solang_parser::pt::Expression::ArraySubscript(loc, array, index) => {
                Expression::IndexAccess(self.build_index_access(
                    loc,
                    array,
                    index.as_ref().unwrap(),
                ))
            }

            solang_parser::pt::Expression::ArraySlice(loc, array, start, end) => {
                Expression::IndexRangeAccess(self.build_index_range_access(
                    loc,
                    array,
                    start.as_ref().map(|x| x.as_ref()),
                    end.as_ref().map(|x| x.as_ref()),
                ))
            }

            solang_parser::pt::Expression::Parenthesis(_, expression) => {
                self.build_expression(expression)
            }

            solang_parser::pt::Expression::MemberAccess(loc, expression, member) => {
                Expression::MemberAccess(self.build_member_access(loc, expression, member))
            }

            solang_parser::pt::Expression::FunctionCall(loc, expression, arguments) => {
                Expression::FunctionCall(self.build_function_call(loc, expression, arguments))
            }

            solang_parser::pt::Expression::FunctionCallBlock(loc, call, options) => {
                Expression::FunctionCallOptions(
                    self.build_function_call_options(loc, call, options),
                )
            }

            solang_parser::pt::Expression::NamedFunctionCall(_, _, _) => todo!(),

            solang_parser::pt::Expression::Not(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "!", true))
            }

            solang_parser::pt::Expression::BitwiseNot(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "~", true))
            }

            solang_parser::pt::Expression::Delete(_, _) => todo!(),

            solang_parser::pt::Expression::PreIncrement(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "++", true))
            }

            solang_parser::pt::Expression::PreDecrement(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "--", true))
            }

            solang_parser::pt::Expression::UnaryPlus(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "+", true))
            }

            solang_parser::pt::Expression::Negate(loc, x) => {
                Expression::UnaryOperation(self.build_unary_operation(loc, x, "-", true))
            }

            solang_parser::pt::Expression::Power(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "**", rhs))
            }

            solang_parser::pt::Expression::Multiply(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "*", rhs))
            }

            solang_parser::pt::Expression::Divide(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "/", rhs))
            }

            solang_parser::pt::Expression::Modulo(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "%", rhs))
            }

            solang_parser::pt::Expression::Add(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "+", rhs))
            }

            solang_parser::pt::Expression::Subtract(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "-", rhs))
            }

            solang_parser::pt::Expression::ShiftLeft(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "<<", rhs))
            }

            solang_parser::pt::Expression::ShiftRight(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, ">>", rhs))
            }

            solang_parser::pt::Expression::BitwiseAnd(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "&", rhs))
            }

            solang_parser::pt::Expression::BitwiseXor(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "^", rhs))
            }

            solang_parser::pt::Expression::BitwiseOr(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "|", rhs))
            }

            solang_parser::pt::Expression::Less(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "<", rhs))
            }

            solang_parser::pt::Expression::More(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, ">", rhs))
            }

            solang_parser::pt::Expression::LessEqual(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "<=", rhs))
            }

            solang_parser::pt::Expression::MoreEqual(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, ">=", rhs))
            }

            solang_parser::pt::Expression::Equal(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "==", rhs))
            }

            solang_parser::pt::Expression::NotEqual(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "!=", rhs))
            }

            solang_parser::pt::Expression::And(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "&&", rhs))
            }

            solang_parser::pt::Expression::Or(loc, lhs, rhs) => {
                Expression::BinaryOperation(self.build_binary_operation(loc, lhs, "||", rhs))
            }

            solang_parser::pt::Expression::ConditionalOperator(
                loc,
                condition,
                true_expression,
                false_expression,
            ) => Expression::Conditional(self.build_conditional(
                loc,
                condition,
                true_expression,
                false_expression,
            )),

            solang_parser::pt::Expression::Assign(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "=", rhs))
            }

            solang_parser::pt::Expression::AssignOr(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "|=", rhs))
            }

            solang_parser::pt::Expression::AssignAnd(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "&=", rhs))
            }

            solang_parser::pt::Expression::AssignXor(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "^=", rhs))
            }

            solang_parser::pt::Expression::AssignShiftLeft(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "<<=", rhs))
            }

            solang_parser::pt::Expression::AssignShiftRight(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, ">>=", rhs))
            }

            solang_parser::pt::Expression::AssignAdd(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "+=", rhs))
            }

            solang_parser::pt::Expression::AssignSubtract(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "-=", rhs))
            }

            solang_parser::pt::Expression::AssignMultiply(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "*=", rhs))
            }

            solang_parser::pt::Expression::AssignDivide(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "/=", rhs))
            }

            solang_parser::pt::Expression::AssignModulo(loc, lhs, rhs) => {
                Expression::Assignment(self.build_assignment(loc, lhs, "%=", rhs))
            }

            solang_parser::pt::Expression::BoolLiteral(_, _)
            | solang_parser::pt::Expression::NumberLiteral(_, _, _, _)
            | solang_parser::pt::Expression::RationalNumberLiteral(_, _, _, _, _)
            | solang_parser::pt::Expression::HexNumberLiteral(_, _, _)
            | solang_parser::pt::Expression::StringLiteral(_)
            | solang_parser::pt::Expression::HexLiteral(_)
            | solang_parser::pt::Expression::AddressLiteral(_, _) => {
                Expression::Literal(self.build_literal(input))
            }

            solang_parser::pt::Expression::Type(loc, _) => {
                Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                    type_name: self.build_type_name(input),
                    argument_types: None,     // TODO
                    is_constant: false,       // TODO
                    is_l_value: false,        // TODO
                    is_pure: false,           // TODO
                    l_value_requested: false, // TODO
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None,     // TODO
                    },
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Expression::Variable(identifier) => {
                Expression::Identifier(Identifier {
                    argument_types: None, // TODO
                    name: identifier.name.clone(),
                    overloaded_declarations: vec![], // TODO
                    referenced_declaration: -1,      // TODO
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None,     // TODO
                    },
                    src: self.loc_to_src(&identifier.loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Expression::List(loc, list) => {
                Expression::TupleExpression(TupleExpression {
                    components: list
                        .iter()
                        .map(|(loc, x)| {
                            x.as_ref().map(|x| {
                                Expression::Identifier(Identifier {
                                    argument_types: None, // TODO
                                    name: x.name.as_ref().map(|x| x.name.clone()).unwrap(),
                                    overloaded_declarations: vec![], // TODO
                                    referenced_declaration: -1,      // TODO
                                    type_descriptions: TypeDescriptions {
                                        type_identifier: None, // TODO
                                        type_string: None,     // TODO
                                    },
                                    src: self.loc_to_src(loc),
                                    id: self.next_node_id(),
                                })
                            })
                        })
                        .collect(),
                    argument_types: None,     // TODO
                    is_inline_array: false,   // TODO
                    is_constant: false,       // TODO
                    is_l_value: false,        // TODO
                    is_pure: false,           // TODO
                    l_value_requested: false, // TODO
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None,     // TODO
                    },
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                })
            }

            solang_parser::pt::Expression::ArrayLiteral(_, _) => todo!(),
        }
    }

    pub fn build_inline_assembly(
        &mut self,
        loc: &solang_parser::pt::Loc,
        dialect: Option<&solang_parser::pt::StringLiteral>,
        flags: Option<&[solang_parser::pt::StringLiteral]>,
        block: &solang_parser::pt::YulBlock,
    ) -> InlineAssembly {
        InlineAssembly {
            ast: Some(self.build_yul_block(block)),
            evm_version: None,           // TODO
            external_references: vec![], // TODO
            operations: None,            // TODO
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_yul_block(&mut self, block: &solang_parser::pt::YulBlock) -> YulBlock {
        YulBlock {
            statements: block
                .statements
                .iter()
                .map(|stmt| self.build_yul_statement(stmt))
                .collect(),
        }
    }

    pub fn build_yul_statement(&mut self, stmt: &solang_parser::pt::YulStatement) -> YulStatement {
        match stmt {
            solang_parser::pt::YulStatement::Assign(_loc, variable_names, value) => {
                YulStatement::YulAssignment(YulAssignment {
                    value: self.build_yul_expression(value),
                    variable_names: variable_names
                        .iter()
                        .map(|x| self.build_yul_identifier(x))
                        .collect(),
                })
            }

            solang_parser::pt::YulStatement::VariableDeclaration(_loc, variables, value) => {
                YulStatement::YulVariableDeclaration(YulVariableDeclaration {
                    value: Some(
                        value
                            .as_ref()
                            .map(|x| self.build_yul_expression(x))
                            .unwrap(),
                    ),
                    variables: variables
                        .iter()
                        .map(|x| self.build_yul_typed_name(x))
                        .collect(),
                })
            }

            solang_parser::pt::YulStatement::If(_loc, condition, body) => {
                YulStatement::YulIf(self.build_yul_if(condition, body))
            }

            solang_parser::pt::YulStatement::For(x) => {
                YulStatement::YulForLoop(self.build_yul_for_loop(x))
            }

            solang_parser::pt::YulStatement::Switch(switch) => {
                YulStatement::YulSwitch(self.build_yul_switch(switch))
            }

            solang_parser::pt::YulStatement::Leave(_) => YulStatement::YulLeave,

            solang_parser::pt::YulStatement::Break(_) => YulStatement::YulBreak,

            solang_parser::pt::YulStatement::Continue(_) => YulStatement::YulContinue,

            solang_parser::pt::YulStatement::Block(_) => todo!(),

            solang_parser::pt::YulStatement::FunctionDefinition(function) => {
                YulStatement::YulFunctionDefinition(self.build_yul_function_definition(function))
            }

            solang_parser::pt::YulStatement::FunctionCall(call) => {
                YulStatement::YulExpressionStatement(YulExpressionStatement {
                    expression: YulExpression::YulFunctionCall(YulFunctionCall {
                        function_name: YulIdentifier {
                            name: call.id.name.clone(),
                        },
                        arguments: call
                            .arguments
                            .iter()
                            .map(|arg| self.build_yul_expression(arg))
                            .collect(),
                    }),
                })
            }

            solang_parser::pt::YulStatement::Error(_) => panic!("Invalid yul statement: {stmt:#?}"),
        }
    }

    pub fn build_yul_identifier(
        &mut self,
        expression: &solang_parser::pt::YulExpression,
    ) -> YulIdentifier {
        match expression {
            solang_parser::pt::YulExpression::Variable(identifier) => YulIdentifier {
                name: identifier.name.clone(),
            },

            _ => panic!("Invalid yul identifier expression: {expression:#?}"),
        }
    }

    pub fn build_yul_typed_name(
        &mut self,
        identifier: &solang_parser::pt::YulTypedIdentifier,
    ) -> YulTypedName {
        YulTypedName {
            r#type: identifier
                .ty
                .as_ref()
                .map(|x| x.name.clone())
                .unwrap_or_else(String::new),
            name: identifier.id.name.clone(),
        }
    }

    pub fn build_yul_if(
        &mut self,
        condition: &solang_parser::pt::YulExpression,
        body: &solang_parser::pt::YulBlock,
    ) -> YulIf {
        YulIf {
            condition: self.build_yul_expression(condition),
            body: self.build_yul_block(body),
        }
    }

    pub fn build_yul_switch(&mut self, switch: &solang_parser::pt::YulSwitch) -> YulSwitch {
        YulSwitch {
            cases: switch
                .cases
                .iter()
                .map(|case| self.build_yul_case(case))
                .collect(),
            expression: self.build_yul_expression(&switch.condition),
        }
    }

    pub fn build_yul_case(&mut self, case: &solang_parser::pt::YulSwitchOptions) -> YulCase {
        match case {
            solang_parser::pt::YulSwitchOptions::Case(_loc, expression, body) => YulCase {
                body: self.build_yul_block(body),
                value: Some(self.build_yul_expression(expression)),
            },

            solang_parser::pt::YulSwitchOptions::Default(_loc, body) => YulCase {
                body: self.build_yul_block(body),
                value: None,
            },
        }
    }

    pub fn build_yul_for_loop(&mut self, input: &solang_parser::pt::YulFor) -> YulForLoop {
        YulForLoop {
            pre: self.build_yul_block(&input.init_block),
            condition: self.build_yul_expression(&input.condition),
            post: self.build_yul_block(&input.post_block),
            body: self.build_yul_block(&input.execution_block),
        }
    }

    pub fn build_yul_function_definition(
        &mut self,
        function: &solang_parser::pt::YulFunctionDefinition,
    ) -> YulFunctionDefinition {
        YulFunctionDefinition {
            name: function.id.name.clone(),
            parameters: Some(
                function
                    .params
                    .iter()
                    .map(|param| self.build_yul_typed_name(param))
                    .collect(),
            ),
            return_parameters: Some(
                function
                    .returns
                    .iter()
                    .map(|param| self.build_yul_typed_name(param))
                    .collect(),
            ),
            body: self.build_yul_block(&function.body),
        }
    }

    pub fn build_yul_expression(
        &mut self,
        expression: &solang_parser::pt::YulExpression,
    ) -> YulExpression {
        match expression {
            solang_parser::pt::YulExpression::BoolLiteral(_, value, _) => {
                YulExpression::YulLiteral(YulLiteral {
                    kind: YulLiteralKind::Bool,
                    value: Some(format!("{value}")),
                    hex_value: None,
                })
            }

            solang_parser::pt::YulExpression::NumberLiteral(_, value, _, _) => {
                YulExpression::YulLiteral(YulLiteral {
                    kind: YulLiteralKind::Number,
                    value: Some(value.clone()),
                    hex_value: None,
                })
            }

            solang_parser::pt::YulExpression::HexNumberLiteral(_, _, _) => todo!(),
            solang_parser::pt::YulExpression::HexStringLiteral(_, _) => todo!(),

            solang_parser::pt::YulExpression::StringLiteral(_, value) => {
                YulExpression::YulLiteral(YulLiteral {
                    kind: YulLiteralKind::String,
                    value: value.as_ref().map(|x| x.name.clone()),
                    hex_value: None,
                })
            }

            solang_parser::pt::YulExpression::Variable(identifier) => {
                YulExpression::YulIdentifier(YulIdentifier {
                    name: identifier.name.clone(),
                })
            }

            solang_parser::pt::YulExpression::FunctionCall(function_call) => {
                YulExpression::YulFunctionCall(YulFunctionCall {
                    function_name: YulIdentifier {
                        name: function_call.id.name.clone(),
                    },
                    arguments: function_call
                        .arguments
                        .iter()
                        .map(|x| self.build_yul_expression(x))
                        .collect(),
                })
            }

            solang_parser::pt::YulExpression::SuffixAccess(_, _, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_builder() {
        let src = std::fs::read_to_string("/Users/camden/Source/solidity-test/contracts/USDC.sol_")
            .unwrap();
        let (input, _comments) = solang_parser::parse(src.as_str(), 0).unwrap();

        let mut builder = AstBuilder::default();
        let source_unit = builder.build_source_unit(&input);

        println!("{:#?}", source_unit);
    }
}
