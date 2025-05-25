use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    ast::NodeID,
    context::workspace::WorkspaceContext,
    detect::{high::*, low::*},
};

use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

pub fn get_all_issue_detectors() -> Vec<Box<dyn IssueDetector>> {
    vec![
        Box::<DelegatecallInLoopDetector>::default(),
        Box::<CentralizationRiskDetector>::default(),
        Box::<SolmateSafeTransferLibDetector>::default(),
        Box::<AvoidAbiEncodePackedDetector>::default(),
        Box::<EcrecoverDetector>::default(),
        Box::<DeprecatedOZFunctionDetector>::default(),
        Box::<UnsafeERC20OperationDetector>::default(),
        Box::<UnspecificSolidityPragmaDetector>::default(),
        Box::<StateNoAddressCheckDetector>::default(),
        Box::<UnusedPublicFunctionDetector>::default(),
        Box::<LiteralsInsteadOfConstantsDetector>::default(),
        Box::<EmptyRequireRevertDetector>::default(),
        Box::<NonReentrantBeforeOthersDetector>::default(),
        Box::<BlockTimestampDeadlineDetector>::default(),
        Box::<UnsafeERC721MintDetector>::default(),
        Box::<PushZeroOpcodeDetector>::default(),
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<ModifierUsedOnlyOnceDetector>::default(),
        Box::<EmptyBlockDetector>::default(),
        Box::<LargeLiteralValueDetector>::default(),
        Box::<InternalFunctionUsedOnceDetector>::default(),
        Box::<TodoDetector>::default(),
        Box::<InconsistentTypeNamesDetector>::default(),
        Box::<UnprotectedInitializerDetector>::default(),
        Box::<UnusedErrorDetector>::default(),
        Box::<RequireRevertInLoopDetector>::default(),
        Box::<DivisionBeforeMultiplicationDetector>::default(),
        Box::<UnsafeCastingDetector>::default(),
        Box::<EnumerableLoopRemovalDetector>::default(),
        Box::<ExperimentalEncoderDetector>::default(),
        Box::<IncorrectShiftOrderDetector>::default(),
        Box::<StorageArrayMemoryEditDetector>::default(),
        Box::<MultipleConstructorsDetector>::default(),
        Box::<ReusedContractNameDetector>::default(),
        Box::<NestedStructInMappingDetector>::default(),
        Box::<SelfdestructDetector>::default(),
        Box::<DynamicArrayLengthAssignmentDetector>::default(),
        Box::<IncorrectUseOfCaretOperatorDetector>::default(),
        Box::<YulReturnDetector>::default(),
        Box::<StateVariableShadowingDetector>::default(),
        Box::<UncheckedSendDetector>::default(),
        Box::<MisusedBooleanDetector>::default(),
        Box::<SendEtherNoChecksDetector>::default(),
        Box::<DelegateCallUncheckedAddressDetector>::default(),
        Box::<TautologicalCompareDetector>::default(),
        Box::<RTLODetector>::default(),
        Box::<DangerousUnaryOperatorDetector>::default(),
        Box::<TautologyOrContraditionDetector>::default(),
        Box::<DangerousStrictEqualityOnBalanceDetector>::default(),
        Box::<StorageSignedIntegerArrayDetector>::default(),
        Box::<RedundantStatementDetector>::default(),
        Box::<StateVariableReadExternalDetector>::default(),
        Box::<WeakRandomnessDetector>::default(),
        Box::<PreDeclaredLocalVariableUsageDetector>::default(),
        Box::<DeletionNestedMappingDetector>::default(),
        Box::<UnusedStateVariablesDetector>::default(),
        Box::<ConstantFunctionContainsAssemblyDetector>::default(),
        Box::<BooleanEqualityDetector>::default(),
        Box::<TxOriginUsedForAuthDetector>::default(),
        Box::<MsgValueUsedInLoopDetector>::default(),
        Box::<ContractLocksEtherDetector>::default(),
        Box::<LocalVariableShadowingDetector>::default(),
        Box::<IncorrectERC721InterfaceDetector>::default(),
        Box::<IncorrectERC20InterfaceDetector>::default(),
        Box::<UninitializedLocalVariableDetector>::default(),
        Box::<ReturnBombDetector>::default(),
        Box::<OutOfOrderRetryableDetector>::default(),
        Box::<FunctionInitializingStateDetector>::default(),
        Box::<DeadCodeDetector>::default(),
        Box::<CacheArrayLengthDetector>::default(),
        Box::<AssertStateChangeDetector>::default(),
        Box::<CostlyLoopDetector>::default(),
        Box::<ConstantFunctionChangesStateDetector>::default(),
        Box::<BuiltinSymbolShadowingDetector>::default(),
        Box::<VoidConstructorDetector>::default(),
        Box::<FunctionSelectorCollisionDetector>::default(),
        Box::<MissingInheritanceDetector>::default(),
        Box::<UnusedImportDetector>::default(),
        Box::<UncheckedLowLevelCallDetector>::default(),
        Box::<FunctionPointerInConstructorDetector>::default(),
        Box::<StateVariableCouldBeConstantDetector>::default(),
        Box::<StateVariableChangesWithoutEventDetector>::default(),
        Box::<StateVariableCouldBeImmutableDetector>::default(),
        Box::<MultiplePlaceholdersDetector>::default(),
        Box::<ReentrancyStateChangeDetector>::default(),
        Box::<IncorrectUseOfModifierDetector>::default(),
        Box::<UncheckedReturnDetector>::default(),
    ]
}

pub fn get_all_detectors_names() -> Vec<String> {
    get_all_issue_detectors().iter().map(|d| d.name()).collect()
}

// Note to maintainers: DO NOT CHANGE THE ORDER OF THESE DERIVE ATTRIBUTES
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum IssueDetectorNamePool {
    IncorrectUseOfModifier,
    ReentrancyStateChange,
    StateVariableCouldBeImmutable,
    MultiplePlaceholders,
    StateChangeWithoutEvent,
    MissingInheritance,
    UnusedImport,
    VoidConstructor,
    UncheckedLowLevelCall,
    FunctionPointerInConstructor,
    DeadCode,
    FunctionSelectorCollision,
    StorageArrayLengthNotCached,
    AssertStateChange,
    CostlyLoop,
    ConstantFunctionChangesState,
    BuiltinSymbolShadowing,
    IncorrectERC721Interface,
    FunctionInitializingState,
    DelegatecallInLoop,
    CentralizationRisk,
    SolmateSafeTransferLib,
    AbiEncodePackedHashCollision,
    Ecrecover,
    DeprecatedOzFunction,
    UnsafeERC20Operation,
    UnspecificSolidityPragma,
    StateNoAddressCheck,
    UnusedPublicFunction,
    EmptyRequireRevert,
    NonReentrantNotFirst,
    BlockTimestampDeadline,
    LiteralInsteadOfConstant,
    UnsafeOzERC721Mint,
    PushZeroOpcode,
    ArbitraryTransferFrom,
    ModifierUsedOnlyOnce,
    UnusedError,
    LargeNumericLiteral,
    InternalFunctionUsedOnce,
    EmptyBlock,
    Todo,
    InconsistentTypeNames,
    UnprotectedInitializer,
    RequireRevertInLoop,
    DivisionBeforeMultiplication,
    UnsafeCasting,
    EnumerableLoopRemoval,
    ExperimentalEncoder,
    IncorrectShiftOrder,
    StorageArrayMemoryEdit,
    MultipleConstructors,
    ReusedContractName,
    NestedStructInMapping,
    Selfdestruct,
    DynamicArrayLengthAssignment,
    IncorrectCaretOperator,
    YulReturn,
    StateVariableShadowing,
    UncheckedSend,
    MisusedBoolean,
    EthSendUncheckedAddress,
    DelegateCallUncheckedAddress,
    TautologicalCompare,
    #[allow(clippy::upper_case_acronyms)]
    RTLO,
    UncheckedReturn,
    DangerousUnaryOperator,
    TautologyOrContradiction,
    StrictEqualityContractBalance,
    SignedIntegerStorageArray,
    RedundantStatement,
    StateVariableReadExternal,
    WeakRandomness,
    PreDeclaredLocalVariableUsage,
    DeleteNestedMapping,
    UnusedStateVariable,
    ConstantFunctionContainsAssembly,
    BooleanEquality,
    TxOriginUsedForAuth,
    MsgValueInLoop,
    ContractLocksEther,
    LocalVariableShadowing,
    IncorrectERC20Interface,
    UninitializedLocalVariable,
    ReturnBomb,
    OutOfOrderRetryable,
    StateVariableCouldBeConstant,
    // NOTE: `Undecided` will be the default name (for new bots).
    // If it's accepted, a new variant will be added to this enum before normalizing it in aderyn
    Undecided,
}

pub fn request_issue_detector_by_name(detector_name: &str) -> Option<Box<dyn IssueDetector>> {
    // Expects a valid detector_name
    let detector_name = IssueDetectorNamePool::from_str(detector_name).ok()?;
    match detector_name {
        IssueDetectorNamePool::ReentrancyStateChange => {
            Some(Box::<ReentrancyStateChangeDetector>::default())
        }
        IssueDetectorNamePool::IncorrectUseOfModifier => {
            Some(Box::<IncorrectUseOfModifierDetector>::default())
        }
        IssueDetectorNamePool::StateVariableCouldBeImmutable => {
            Some(Box::<StateVariableCouldBeImmutableDetector>::default())
        }
        IssueDetectorNamePool::MultiplePlaceholders => {
            Some(Box::<MultiplePlaceholdersDetector>::default())
        }
        IssueDetectorNamePool::StateChangeWithoutEvent => {
            Some(Box::<StateVariableChangesWithoutEventDetector>::default())
        }
        IssueDetectorNamePool::MissingInheritance => {
            Some(Box::<MissingInheritanceDetector>::default())
        }
        IssueDetectorNamePool::LocalVariableShadowing => {
            Some(Box::<LocalVariableShadowingDetector>::default())
        }
        IssueDetectorNamePool::UnusedImport => Some(Box::<UnusedImportDetector>::default()),
        IssueDetectorNamePool::VoidConstructor => Some(Box::<VoidConstructorDetector>::default()),
        IssueDetectorNamePool::StateVariableCouldBeConstant => {
            Some(Box::<StateVariableCouldBeConstantDetector>::default())
        }
        IssueDetectorNamePool::LiteralInsteadOfConstant => {
            Some(Box::<LiteralsInsteadOfConstantsDetector>::default())
        }
        IssueDetectorNamePool::FunctionPointerInConstructor => {
            Some(Box::<FunctionPointerInConstructorDetector>::default())
        }
        IssueDetectorNamePool::DeadCode => Some(Box::<DeadCodeDetector>::default()),
        IssueDetectorNamePool::FunctionSelectorCollision => {
            Some(Box::<FunctionSelectorCollisionDetector>::default())
        }
        IssueDetectorNamePool::StorageArrayLengthNotCached => {
            Some(Box::<CacheArrayLengthDetector>::default())
        }
        IssueDetectorNamePool::AssertStateChange => {
            Some(Box::<AssertStateChangeDetector>::default())
        }
        IssueDetectorNamePool::CostlyLoop => Some(Box::<CostlyLoopDetector>::default()),
        IssueDetectorNamePool::ConstantFunctionChangesState => {
            Some(Box::<ConstantFunctionChangesStateDetector>::default())
        }
        IssueDetectorNamePool::BuiltinSymbolShadowing => {
            Some(Box::<BuiltinSymbolShadowingDetector>::default())
        }
        IssueDetectorNamePool::IncorrectERC721Interface => {
            Some(Box::<IncorrectERC721InterfaceDetector>::default())
        }
        IssueDetectorNamePool::OutOfOrderRetryable => {
            Some(Box::<OutOfOrderRetryableDetector>::default())
        }
        IssueDetectorNamePool::FunctionInitializingState => {
            Some(Box::<FunctionInitializingStateDetector>::default())
        }
        IssueDetectorNamePool::IncorrectERC20Interface => {
            Some(Box::<IncorrectERC20InterfaceDetector>::default())
        }
        IssueDetectorNamePool::UninitializedLocalVariable => {
            Some(Box::<UninitializedLocalVariableDetector>::default())
        }
        IssueDetectorNamePool::ReturnBomb => Some(Box::<ReturnBombDetector>::default()),
        IssueDetectorNamePool::UnusedStateVariable => {
            Some(Box::<UnusedStateVariablesDetector>::default())
        }
        IssueDetectorNamePool::DelegatecallInLoop => {
            Some(Box::<DelegatecallInLoopDetector>::default())
        }
        IssueDetectorNamePool::CentralizationRisk => {
            Some(Box::<CentralizationRiskDetector>::default())
        }
        IssueDetectorNamePool::SolmateSafeTransferLib => {
            Some(Box::<SolmateSafeTransferLibDetector>::default())
        }
        IssueDetectorNamePool::AbiEncodePackedHashCollision => {
            Some(Box::<AvoidAbiEncodePackedDetector>::default())
        }
        IssueDetectorNamePool::Ecrecover => Some(Box::<EcrecoverDetector>::default()),
        IssueDetectorNamePool::DeprecatedOzFunction => {
            Some(Box::<DeprecatedOZFunctionDetector>::default())
        }
        IssueDetectorNamePool::UnsafeERC20Operation => {
            Some(Box::<UnsafeERC20OperationDetector>::default())
        }
        IssueDetectorNamePool::UnspecificSolidityPragma => {
            Some(Box::<UnspecificSolidityPragmaDetector>::default())
        }
        IssueDetectorNamePool::StateNoAddressCheck => {
            Some(Box::<StateNoAddressCheckDetector>::default())
        }
        IssueDetectorNamePool::UnusedPublicFunction => {
            Some(Box::<UnusedPublicFunctionDetector>::default())
        }
        IssueDetectorNamePool::EmptyRequireRevert => {
            Some(Box::<EmptyRequireRevertDetector>::default())
        }
        IssueDetectorNamePool::NonReentrantNotFirst => {
            Some(Box::<NonReentrantBeforeOthersDetector>::default())
        }
        IssueDetectorNamePool::BlockTimestampDeadline => {
            Some(Box::<BlockTimestampDeadlineDetector>::default())
        }
        IssueDetectorNamePool::UnsafeOzERC721Mint => {
            Some(Box::<UnsafeERC721MintDetector>::default())
        }
        IssueDetectorNamePool::PushZeroOpcode => Some(Box::<PushZeroOpcodeDetector>::default()),
        IssueDetectorNamePool::ArbitraryTransferFrom => {
            Some(Box::<ArbitraryTransferFromDetector>::default())
        }
        IssueDetectorNamePool::ModifierUsedOnlyOnce => {
            Some(Box::<ModifierUsedOnlyOnceDetector>::default())
        }
        IssueDetectorNamePool::LargeNumericLiteral => {
            Some(Box::<LargeLiteralValueDetector>::default())
        }
        IssueDetectorNamePool::InternalFunctionUsedOnce => {
            Some(Box::<InternalFunctionUsedOnceDetector>::default())
        }
        IssueDetectorNamePool::EmptyBlock => Some(Box::<EmptyBlockDetector>::default()),
        IssueDetectorNamePool::Todo => Some(Box::<TodoDetector>::default()),
        IssueDetectorNamePool::InconsistentTypeNames => {
            Some(Box::<InconsistentTypeNamesDetector>::default())
        }
        IssueDetectorNamePool::UnprotectedInitializer => {
            Some(Box::<UnprotectedInitializerDetector>::default())
        }
        IssueDetectorNamePool::RequireRevertInLoop => {
            Some(Box::<RequireRevertInLoopDetector>::default())
        }
        IssueDetectorNamePool::UnusedError => Some(Box::<UnusedErrorDetector>::default()),
        IssueDetectorNamePool::DivisionBeforeMultiplication => {
            Some(Box::<DivisionBeforeMultiplicationDetector>::default())
        }
        IssueDetectorNamePool::UnsafeCasting => Some(Box::<UnsafeCastingDetector>::default()),
        IssueDetectorNamePool::EnumerableLoopRemoval => {
            Some(Box::<EnumerableLoopRemovalDetector>::default())
        }
        IssueDetectorNamePool::ExperimentalEncoder => {
            Some(Box::<ExperimentalEncoderDetector>::default())
        }
        IssueDetectorNamePool::IncorrectShiftOrder => {
            Some(Box::<IncorrectShiftOrderDetector>::default())
        }
        IssueDetectorNamePool::StorageArrayMemoryEdit => {
            Some(Box::<StorageArrayMemoryEditDetector>::default())
        }
        IssueDetectorNamePool::MultipleConstructors => {
            Some(Box::<MultipleConstructorsDetector>::default())
        }
        IssueDetectorNamePool::ReusedContractName => {
            Some(Box::<ReusedContractNameDetector>::default())
        }
        IssueDetectorNamePool::NestedStructInMapping => {
            Some(Box::<NestedStructInMappingDetector>::default())
        }
        IssueDetectorNamePool::Selfdestruct => Some(Box::<SelfdestructDetector>::default()),
        IssueDetectorNamePool::DynamicArrayLengthAssignment => {
            Some(Box::<DynamicArrayLengthAssignmentDetector>::default())
        }

        IssueDetectorNamePool::IncorrectCaretOperator => {
            Some(Box::<IncorrectUseOfCaretOperatorDetector>::default())
        }
        IssueDetectorNamePool::YulReturn => Some(Box::<YulReturnDetector>::default()),
        IssueDetectorNamePool::StateVariableShadowing => {
            Some(Box::<StateVariableShadowingDetector>::default())
        }
        IssueDetectorNamePool::UncheckedSend => Some(Box::<UncheckedSendDetector>::default()),
        IssueDetectorNamePool::MisusedBoolean => Some(Box::<MisusedBooleanDetector>::default()),
        IssueDetectorNamePool::EthSendUncheckedAddress => {
            Some(Box::<SendEtherNoChecksDetector>::default())
        }
        IssueDetectorNamePool::DelegateCallUncheckedAddress => {
            Some(Box::<DelegateCallUncheckedAddressDetector>::default())
        }
        IssueDetectorNamePool::TautologicalCompare => {
            Some(Box::<TautologicalCompareDetector>::default())
        }
        IssueDetectorNamePool::RTLO => Some(Box::<RTLODetector>::default()),
        IssueDetectorNamePool::UncheckedReturn => Some(Box::<UncheckedReturnDetector>::default()),
        IssueDetectorNamePool::DangerousUnaryOperator => {
            Some(Box::<DangerousUnaryOperatorDetector>::default())
        }
        IssueDetectorNamePool::TautologyOrContradiction => {
            Some(Box::<TautologyOrContraditionDetector>::default())
        }
        IssueDetectorNamePool::StrictEqualityContractBalance => {
            Some(Box::<DangerousStrictEqualityOnBalanceDetector>::default())
        }
        IssueDetectorNamePool::SignedIntegerStorageArray => {
            Some(Box::<StorageSignedIntegerArrayDetector>::default())
        }
        IssueDetectorNamePool::RedundantStatement => {
            Some(Box::<RedundantStatementDetector>::default())
        }
        IssueDetectorNamePool::StateVariableReadExternal => {
            Some(Box::<StateVariableReadExternalDetector>::default())
        }
        IssueDetectorNamePool::WeakRandomness => Some(Box::<WeakRandomnessDetector>::default()),
        IssueDetectorNamePool::PreDeclaredLocalVariableUsage => {
            Some(Box::<PreDeclaredLocalVariableUsageDetector>::default())
        }
        IssueDetectorNamePool::DeleteNestedMapping => {
            Some(Box::<DeletionNestedMappingDetector>::default())
        }
        IssueDetectorNamePool::ConstantFunctionContainsAssembly => {
            Some(Box::<ConstantFunctionContainsAssemblyDetector>::default())
        }
        IssueDetectorNamePool::BooleanEquality => Some(Box::<BooleanEqualityDetector>::default()),
        IssueDetectorNamePool::TxOriginUsedForAuth => {
            Some(Box::<TxOriginUsedForAuthDetector>::default())
        }
        IssueDetectorNamePool::MsgValueInLoop => Some(Box::<MsgValueUsedInLoopDetector>::default()),
        IssueDetectorNamePool::ContractLocksEther => {
            Some(Box::<ContractLocksEtherDetector>::default())
        }
        IssueDetectorNamePool::UncheckedLowLevelCall => {
            Some(Box::<UncheckedLowLevelCallDetector>::default())
        }
        IssueDetectorNamePool::Undecided => None,
    }
}

pub fn get_issue_detector_by_name(detector_name: &str) -> Box<dyn IssueDetector> {
    request_issue_detector_by_name(detector_name).unwrap()
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumCount, Clone, EnumIter)]
pub enum IssueSeverity {
    Low,
    High,
}

impl Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let issue_description = match self {
            IssueSeverity::Low => "Low",
            IssueSeverity::High => "High",
        };
        write!(f, "{}", issue_description).unwrap();
        Ok(())
    }
}

impl dyn IssueDetector {
    pub fn skeletal_clone(&self) -> Box<dyn IssueDetector> {
        request_issue_detector_by_name(self.name().as_str()).unwrap()
    }
}

pub trait IssueDetector: Send + Sync + 'static {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn description(&self) -> String {
        String::from("Description")
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::Undecided)
    }

    // Keys are source file name, line number and source location
    // Value is ASTNode NodeID
    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        BTreeMap::new()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        BTreeMap::new()
    }
}
