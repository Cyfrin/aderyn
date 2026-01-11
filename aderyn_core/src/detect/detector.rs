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
        Box::<ArbitraryTransferFromDetector>::default(),
        Box::<AssertStateChangeDetector>::default(),
        Box::<AvoidAbiEncodePackedDetector>::default(),
        Box::<BlockTimestampDeadlineDetector>::default(),
        Box::<BooleanEqualityDetector>::default(),
        Box::<BuiltinSymbolShadowingDetector>::default(),
        Box::<CacheArrayLengthDetector>::default(),
        Box::<CentralizationRiskDetector>::default(),
        Box::<ConstantFunctionChangesStateDetector>::default(),
        Box::<ConstantFunctionContainsAssemblyDetector>::default(),
        Box::<ContractLocksEtherDetector>::default(),
        Box::<CostlyLoopDetector>::default(),
        Box::<DangerousStrictEqualityOnBalanceDetector>::default(),
        Box::<DangerousUnaryOperatorDetector>::default(),
        Box::<DeadCodeDetector>::default(),
        Box::<DelegatecallInLoopDetector>::default(),
        Box::<DelegateCallUncheckedAddressDetector>::default(),
        Box::<DeletionNestedMappingDetector>::default(),
        Box::<DeprecatedOZFunctionDetector>::default(),
        Box::<DivisionBeforeMultiplicationDetector>::default(),
        Box::<DynamicArrayLengthAssignmentDetector>::default(),
        Box::<EcrecoverDetector>::default(),
        Box::<EmptyBlockDetector>::default(),
        Box::<EmptyRequireRevertDetector>::default(),
        Box::<EnumerableLoopRemovalDetector>::default(),
        Box::<ExperimentalEncoderDetector>::default(),
        Box::<FunctionInitializingStateDetector>::default(),
        Box::<FunctionPointerInConstructorDetector>::default(),
        Box::<FunctionSelectorCollisionDetector>::default(),
        Box::<InconsistentTypeNamesDetector>::default(),
        Box::<IncorrectERC20InterfaceDetector>::default(),
        Box::<IncorrectERC721InterfaceDetector>::default(),
        Box::<IncorrectShiftOrderDetector>::default(),
        Box::<IncorrectUseOfCaretOperatorDetector>::default(),
        Box::<IncorrectUseOfModifierDetector>::default(),
        Box::<InternalFunctionUsedOnceDetector>::default(),
        Box::<LargeLiteralValueDetector>::default(),
        Box::<LiteralsInsteadOfConstantsDetector>::default(),
        Box::<LocalVariableShadowingDetector>::default(),
        Box::<MissingInheritanceDetector>::default(),
        Box::<MisusedBooleanDetector>::default(),
        Box::<ModifierUsedOnlyOnceDetector>::default(),
        Box::<MsgValueUsedInLoopDetector>::default(),
        Box::<MultipleConstructorsDetector>::default(),
        Box::<MultiplePlaceholdersDetector>::default(),
        Box::<NestedStructInMappingDetector>::default(),
        Box::<NonReentrantBeforeOthersDetector>::default(),
        Box::<OutOfOrderRetryableDetector>::default(),
        Box::<PreDeclaredLocalVariableUsageDetector>::default(),
        Box::<PushZeroOpcodeDetector>::default(),
        Box::<RedundantStatementDetector>::default(),
        Box::<ReentrancyStateChangeDetector>::default(),
        Box::<RequireRevertInLoopDetector>::default(),
        Box::<ReturnBombDetector>::default(),
        Box::<ReusedContractNameDetector>::default(),
        Box::<RTLODetector>::default(),
        Box::<SelfdestructDetector>::default(),
        Box::<SendEtherNoChecksDetector>::default(),
        Box::<SolmateSafeTransferLibDetector>::default(),
        Box::<StateNoAddressCheckDetector>::default(),
        Box::<StateVariableChangesWithoutEventDetector>::default(),
        Box::<StateVariableCouldBeConstantDetector>::default(),
        Box::<StateVariableCouldBeImmutableDetector>::default(),
        Box::<StateVariableInitOrderDetector>::default(),
        Box::<StateVariableReadExternalDetector>::default(),
        Box::<StateVariableShadowingDetector>::default(),
        Box::<StorageArrayMemoryEditDetector>::default(),
        Box::<StorageSignedIntegerArrayDetector>::default(),
        Box::<TautologicalCompareDetector>::default(),
        Box::<TautologyOrContraditionDetector>::default(),
        Box::<TodoDetector>::default(),
        Box::<TxOriginUsedForAuthDetector>::default(),
        Box::<UncheckedLowLevelCallDetector>::default(),
        Box::<UncheckedReturnDetector>::default(),
        Box::<UncheckedSendDetector>::default(),
        Box::<UninitializedLocalVariableDetector>::default(),
        Box::<UnprotectedInitializerDetector>::default(),
        Box::<UnsafeCastingDetector>::default(),
        Box::<UnsafeERC20OperationDetector>::default(),
        Box::<UnsafeERC721MintDetector>::default(),
        Box::<UnspecificSolidityPragmaDetector>::default(),
        Box::<UnusedErrorDetector>::default(),
        Box::<UnusedImportDetector>::default(),
        Box::<UnusedPublicFunctionDetector>::default(),
        Box::<UnusedStateVariablesDetector>::default(),
        Box::<VoidConstructorDetector>::default(),
        Box::<WeakRandomnessDetector>::default(),
        Box::<YulReturnDetector>::default(),
    ]
}

pub fn get_all_detectors_names() -> Vec<String> {
    get_all_issue_detectors().iter().map(|d| d.name()).collect()
}

// Note to maintainers: DO NOT CHANGE THE ORDER OF THESE DERIVE ATTRIBUTES
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum IssueDetectorNamePool {
    AbiEncodePackedHashCollision,
    ArbitraryTransferFrom,
    AssertStateChange,
    BlockTimestampDeadline,
    BooleanEquality,
    BuiltinSymbolShadowing,
    CentralizationRisk,
    ConstantFunctionChangesState,
    ConstantFunctionContainsAssembly,
    ContractLocksEther,
    CostlyLoop,
    DangerousUnaryOperator,
    DeadCode,
    DelegatecallInLoop,
    DelegateCallUncheckedAddress,
    DeleteNestedMapping,
    DeprecatedOzFunction,
    DivisionBeforeMultiplication,
    DynamicArrayLengthAssignment,
    Ecrecover,
    EmptyBlock,
    EmptyRequireRevert,
    EnumerableLoopRemoval,
    EthSendUncheckedAddress,
    ExperimentalEncoder,
    FunctionInitializingState,
    FunctionPointerInConstructor,
    FunctionSelectorCollision,
    InconsistentTypeNames,
    IncorrectCaretOperator,
    IncorrectERC20Interface,
    IncorrectERC721Interface,
    IncorrectShiftOrder,
    IncorrectUseOfModifier,
    InternalFunctionUsedOnce,
    LargeNumericLiteral,
    LiteralInsteadOfConstant,
    LocalVariableShadowing,
    MissingInheritance,
    MisusedBoolean,
    ModifierUsedOnlyOnce,
    MsgValueInLoop,
    MultipleConstructors,
    MultiplePlaceholders,
    NestedStructInMapping,
    NonReentrantNotFirst,
    OutOfOrderRetryable,
    PreDeclaredLocalVariableUsage,
    PushZeroOpcode,
    RedundantStatement,
    ReentrancyStateChange,
    RequireRevertInLoop,
    ReturnBomb,
    ReusedContractName,
    #[allow(clippy::upper_case_acronyms)]
    RTLO,
    Selfdestruct,
    SignedIntegerStorageArray,
    SolmateSafeTransferLib,
    StateChangeWithoutEvent,
    StateNoAddressCheck,
    StateVariableCouldBeConstant,
    StateVariableCouldBeImmutable,
    StateVariableInitOrder,
    StateVariableReadExternal,
    StateVariableShadowing,
    StorageArrayLengthNotCached,
    StorageArrayMemoryEdit,
    StrictEqualityContractBalance,
    TautologicalCompare,
    TautologyOrContradiction,
    Todo,
    TxOriginUsedForAuth,
    UncheckedLowLevelCall,
    UncheckedReturn,
    UncheckedSend,
    UninitializedLocalVariable,
    UnprotectedInitializer,
    UnsafeCasting,
    UnsafeERC20Operation,
    UnsafeOzERC721Mint,
    UnspecificSolidityPragma,
    UnusedError,
    UnusedImport,
    UnusedPublicFunction,
    UnusedStateVariable,
    VoidConstructor,
    WeakRandomness,
    YulReturn,
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
        IssueDetectorNamePool::StateVariableInitOrder => {
            Some(Box::<StateVariableInitOrderDetector>::default())
        }
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
    /// Runs the detection algorithm and return true if instances were found.
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>>;

    /// Specify High or Low severity.
    fn severity(&self) -> IssueSeverity;

    /// Title of the issue.
    fn title(&self) -> String;

    /// Description of the issue.
    fn description(&self) -> String;

    /// Name of the detector.
    fn name(&self) -> String;

    /// Collection of all instances where the issue was discovered.
    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID>;

    /// Optionally include special messages crafter for individual messages.
    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        BTreeMap::new()
    }
}
