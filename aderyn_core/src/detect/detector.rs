use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    ast::NodeID,
    context::workspace_context::WorkspaceContext,
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
        Box::<DelegateCallInLoopDetector>::default(),
        Box::<CentralizationRiskDetector>::default(),
        Box::<SolmateSafeTransferLibDetector>::default(),
        Box::<AvoidAbiEncodePackedDetector>::default(),
        Box::<EcrecoverDetector>::default(),
        Box::<DeprecatedOZFunctionsDetector>::default(),
        Box::<UnsafeERC20FunctionsDetector>::default(),
        Box::<UnspecificSolidityPragmaDetector>::default(),
        Box::<ZeroAddressCheckDetector>::default(),
        Box::<UselessPublicFunctionDetector>::default(),
        Box::<ConstantsInsteadOfLiteralsDetector>::default(),
        Box::<UnindexedEventsDetector>::default(),
        Box::<RequireWithStringDetector>::default(),
        Box::<NonReentrantBeforeOthersDetector>::default(),
        Box::<BlockTimestampDeadlineDetector>::default(),
        Box::<UnsafeERC721MintDetector>::default(),
        Box::<PushZeroOpcodeDetector>::default(),
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<UselessModifierDetector>::default(),
        Box::<EmptyBlockDetector>::default(),
        Box::<LargeLiteralValueDetector>::default(),
        Box::<UselessInternalFunctionDetector>::default(),
        Box::<ContractsWithTodosDetector>::default(),
        Box::<InconsistentTypeNamesDetector>::default(),
        Box::<UnprotectedInitializerDetector>::default(),
        Box::<UselessErrorDetector>::default(),
        Box::<RevertsAndRequiresInLoopsDetector>::default(),
        Box::<DivisionBeforeMultiplicationDetector>::default(),
        Box::<UnsafeCastingDetector>::default(),
        Box::<EnumerableLoopRemovalDetector>::default(),
        Box::<ExperimentalEncoderDetector>::default(),
        Box::<IncorrectShiftOrderDetector>::default(),
        Box::<StorageArrayEditWithMemoryDetector>::default(),
        Box::<MultipleConstructorsDetector>::default(),
        Box::<ReusedContractNameDetector>::default(),
        Box::<NestedStructInMappingDetector>::default(),
        Box::<SelfdestructIdentifierDetector>::default(),
        Box::<DynamicArrayLengthAssignmentDetector>::default(),
        Box::<UninitializedStateVariableDetector>::default(),
        Box::<IncorrectUseOfCaretOperatorDetector>::default(),
        Box::<YulReturnDetector>::default(),
        Box::<StateVariableShadowingDetector>::default(),
        Box::<UncheckedSendDetector>::default(),
        Box::<MisusedBooleanDetector>::default(),
        Box::<SendEtherNoChecksDetector>::default(),
        Box::<DelegateCallOnUncheckedAddressDetector>::default(),
        Box::<TautologicalCompareDetector>::default(),
        Box::<RTLODetector>::default(),
        Box::<UncheckedReturnDetector>::default(),
        Box::<DangerousUnaryOperatorDetector>::default(),
        Box::<TautologyOrContraditionDetector>::default(),
        Box::<DangerousStrictEqualityOnBalanceDetector>::default(),
        Box::<StorageSignedIntegerArrayDetector>::default(),
        Box::<RedundantStatementsDetector>::default(),
        Box::<PublicVariableReadInExternalContextDetector>::default(),
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
        Box::<CostlyOperationsInsideLoopsDetector>::default(),
        Box::<ConstantFunctionChangingStateDetector>::default(),
        Box::<BuiltinSymbolShadowDetector>::default(),
        Box::<VoidConstructorDetector>::default(),
        Box::<FunctionSelectorCollisionDetector>::default(),
        Box::<MissingInheritanceDetector>::default(),
        Box::<UnusedImportDetector>::default(),
        Box::<UncheckedLowLevelCallDetector>::default(),
        Box::<FucntionPointerInConstructorDetector>::default(),
        Box::<StateVariableCouldBeConstantDetector>::default(),
        Box::<StateVariableChangesWithoutEventDetector>::default(),
        Box::<MultiplePlaceholdersDetector>::default(),
    ]
}

pub fn get_all_detectors_names() -> Vec<String> {
    get_all_issue_detectors().iter().map(|d| d.name()).collect()
}

// Note to maintainers: DO NOT CHANGE THE ORDER OF THESE DERIVE ATTRIBUTES
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum IssueDetectorNamePool {
    MultiplePlaceholders,
    StateVariableChangesWithoutEvents,
    MissingInheritance,
    UnusedImport,
    VoidConstructor,
    UncheckedLowLevelCall,
    FunctionPointerInConstructor,
    DeadCode,
    FunctionSelectorCollision,
    CacheArrayLength,
    AssertStateChange,
    CostlyOperationsInsideLoops,
    ConstantFunctionChangingState,
    BuiltinSymbolShadow,
    IncorrectERC721Interface,
    FunctionInitializingState,
    DelegateCallInLoop,
    CentralizationRisk,
    SolmateSafeTransferLib,
    AvoidAbiEncodePacked,
    Ecrecover,
    DeprecatedOzFunctions,
    UnsafeERC20Functions,
    UnspecificSolidityPragma,
    ZeroAddressCheck,
    UselessPublicFunction,
    ConstantsInsteadOfLiterals,
    UnindexedEvents,
    RequireWithString,
    NonReentrantBeforeOthers,
    BlockTimestampDeadline,
    UnsafeOzERC721Mint,
    PushZeroOpcode,
    ArbitraryTransferFrom,
    UselessModifier,
    UselessError,
    LargeNumericLiteral,
    UselessInternalFunction,
    EmptyBlock,
    ContractWithTodos,
    InconsistentTypeNames,
    UnprotectedInitializer,
    RevertsAndRequiresInLoops,
    DivisionBeforeMultiplication,
    UnsafeCastingDetector,
    EnumerableLoopRemoval,
    ExperimentalEncoder,
    IncorrectShiftOrder,
    StorageArrayEditWithMemory,
    MultipleConstructors,
    ReusedContractName,
    NestedStructInMapping,
    SelfdestructIdentifier,
    DynamicArrayLengthAssignment,
    UninitializedStateVariable,
    IncorrectCaretOperator,
    YulReturn,
    StateVariableShadowing,
    UncheckedSend,
    MisusedBoolean,
    SendEtherNoChecks,
    DelegateCallUncheckedAddress,
    TautologicalCompare,
    #[allow(clippy::upper_case_acronyms)]
    RTLO,
    UncheckedReturn,
    DangerousUnaryOperator,
    TautologyOrContradiction,
    DangerousStrictEquailtyOnContractBalance,
    SignedStorageArray,
    RedundantStatements,
    PublicVariableReadInExternalContext,
    WeakRandomness,
    PreDeclaredLocalVariableUsage,
    DeleteNestedMapping,
    UnusedStateVariable,
    ConstantFunctionsAssembly,
    BooleanEquality,
    TxOriginUsedForAuth,
    MsgValueInLoop,
    ContractLocksEther,
    LocalVariableShadowing,
    IncorrectERC20Interface,
    UninitializedLocalVariable,
    ReturnBomb,
    OutOfOrderRetryable,
    StateVariableCouldBeDeclaredConstant,
    // NOTE: `Undecided` will be the default name (for new bots).
    // If it's accepted, a new variant will be added to this enum before normalizing it in aderyn
    Undecided,
}

pub fn request_issue_detector_by_name(detector_name: &str) -> Option<Box<dyn IssueDetector>> {
    // Expects a valid detector_name
    let detector_name = IssueDetectorNamePool::from_str(detector_name).ok()?;
    match detector_name {
        IssueDetectorNamePool::MultiplePlaceholders => {
            Some(Box::<MultiplePlaceholdersDetector>::default())
        }
        IssueDetectorNamePool::StateVariableChangesWithoutEvents => {
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
        IssueDetectorNamePool::StateVariableCouldBeDeclaredConstant => {
            Some(Box::<StateVariableCouldBeConstantDetector>::default())
        }
        IssueDetectorNamePool::FunctionPointerInConstructor => {
            Some(Box::<FucntionPointerInConstructorDetector>::default())
        }
        IssueDetectorNamePool::DeadCode => Some(Box::<DeadCodeDetector>::default()),
        IssueDetectorNamePool::FunctionSelectorCollision => {
            Some(Box::<FunctionSelectorCollisionDetector>::default())
        }
        IssueDetectorNamePool::CacheArrayLength => Some(Box::<CacheArrayLengthDetector>::default()),
        IssueDetectorNamePool::AssertStateChange => {
            Some(Box::<AssertStateChangeDetector>::default())
        }
        IssueDetectorNamePool::CostlyOperationsInsideLoops => {
            Some(Box::<CostlyOperationsInsideLoopsDetector>::default())
        }
        IssueDetectorNamePool::ConstantFunctionChangingState => {
            Some(Box::<ConstantFunctionChangingStateDetector>::default())
        }
        IssueDetectorNamePool::BuiltinSymbolShadow => {
            Some(Box::<BuiltinSymbolShadowDetector>::default())
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
        IssueDetectorNamePool::DelegateCallInLoop => {
            Some(Box::<DelegateCallInLoopDetector>::default())
        }
        IssueDetectorNamePool::CentralizationRisk => {
            Some(Box::<CentralizationRiskDetector>::default())
        }
        IssueDetectorNamePool::SolmateSafeTransferLib => {
            Some(Box::<SolmateSafeTransferLibDetector>::default())
        }
        IssueDetectorNamePool::AvoidAbiEncodePacked => {
            Some(Box::<AvoidAbiEncodePackedDetector>::default())
        }
        IssueDetectorNamePool::Ecrecover => Some(Box::<EcrecoverDetector>::default()),
        IssueDetectorNamePool::DeprecatedOzFunctions => {
            Some(Box::<DeprecatedOZFunctionsDetector>::default())
        }
        IssueDetectorNamePool::UnsafeERC20Functions => {
            Some(Box::<UnsafeERC20FunctionsDetector>::default())
        }
        IssueDetectorNamePool::UnspecificSolidityPragma => {
            Some(Box::<UnspecificSolidityPragmaDetector>::default())
        }
        IssueDetectorNamePool::ZeroAddressCheck => Some(Box::<ZeroAddressCheckDetector>::default()),
        IssueDetectorNamePool::UselessPublicFunction => {
            Some(Box::<UselessPublicFunctionDetector>::default())
        }
        IssueDetectorNamePool::ConstantsInsteadOfLiterals => {
            Some(Box::<ConstantsInsteadOfLiteralsDetector>::default())
        }
        IssueDetectorNamePool::UnindexedEvents => Some(Box::<UnindexedEventsDetector>::default()),
        IssueDetectorNamePool::RequireWithString => {
            Some(Box::<RequireWithStringDetector>::default())
        }
        IssueDetectorNamePool::NonReentrantBeforeOthers => {
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
        IssueDetectorNamePool::UselessModifier => Some(Box::<UselessModifierDetector>::default()),
        IssueDetectorNamePool::LargeNumericLiteral => {
            Some(Box::<LargeLiteralValueDetector>::default())
        }
        IssueDetectorNamePool::UselessInternalFunction => {
            Some(Box::<UselessInternalFunctionDetector>::default())
        }
        IssueDetectorNamePool::EmptyBlock => Some(Box::<EmptyBlockDetector>::default()),
        IssueDetectorNamePool::ContractWithTodos => {
            Some(Box::<ContractsWithTodosDetector>::default())
        }
        IssueDetectorNamePool::InconsistentTypeNames => {
            Some(Box::<InconsistentTypeNamesDetector>::default())
        }
        IssueDetectorNamePool::UnprotectedInitializer => {
            Some(Box::<UnprotectedInitializerDetector>::default())
        }
        IssueDetectorNamePool::RevertsAndRequiresInLoops => {
            Some(Box::<RevertsAndRequiresInLoopsDetector>::default())
        }
        IssueDetectorNamePool::UselessError => Some(Box::<UselessErrorDetector>::default()),
        IssueDetectorNamePool::DivisionBeforeMultiplication => {
            Some(Box::<DivisionBeforeMultiplicationDetector>::default())
        }
        IssueDetectorNamePool::UnsafeCastingDetector => {
            Some(Box::<UnsafeCastingDetector>::default())
        }
        IssueDetectorNamePool::EnumerableLoopRemoval => {
            Some(Box::<EnumerableLoopRemovalDetector>::default())
        }
        IssueDetectorNamePool::ExperimentalEncoder => {
            Some(Box::<ExperimentalEncoderDetector>::default())
        }
        IssueDetectorNamePool::IncorrectShiftOrder => {
            Some(Box::<IncorrectShiftOrderDetector>::default())
        }
        IssueDetectorNamePool::StorageArrayEditWithMemory => {
            Some(Box::<StorageArrayEditWithMemoryDetector>::default())
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
        IssueDetectorNamePool::SelfdestructIdentifier => {
            Some(Box::<SelfdestructIdentifierDetector>::default())
        }
        IssueDetectorNamePool::DynamicArrayLengthAssignment => {
            Some(Box::<DynamicArrayLengthAssignmentDetector>::default())
        }

        IssueDetectorNamePool::UninitializedStateVariable => {
            Some(Box::<UninitializedStateVariableDetector>::default())
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
        IssueDetectorNamePool::SendEtherNoChecks => {
            Some(Box::<SendEtherNoChecksDetector>::default())
        }
        IssueDetectorNamePool::DelegateCallUncheckedAddress => {
            Some(Box::<DelegateCallOnUncheckedAddressDetector>::default())
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
        IssueDetectorNamePool::DangerousStrictEquailtyOnContractBalance => {
            Some(Box::<DangerousStrictEqualityOnBalanceDetector>::default())
        }
        IssueDetectorNamePool::SignedStorageArray => {
            Some(Box::<StorageSignedIntegerArrayDetector>::default())
        }
        IssueDetectorNamePool::RedundantStatements => {
            Some(Box::<RedundantStatementsDetector>::default())
        }
        IssueDetectorNamePool::PublicVariableReadInExternalContext => {
            Some(Box::<PublicVariableReadInExternalContextDetector>::default())
        }
        IssueDetectorNamePool::WeakRandomness => Some(Box::<WeakRandomnessDetector>::default()),
        IssueDetectorNamePool::PreDeclaredLocalVariableUsage => {
            Some(Box::<PreDeclaredLocalVariableUsageDetector>::default())
        }
        IssueDetectorNamePool::DeleteNestedMapping => {
            Some(Box::<DeletionNestedMappingDetector>::default())
        }
        IssueDetectorNamePool::ConstantFunctionsAssembly => {
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
