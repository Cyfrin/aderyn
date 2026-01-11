#![allow(clippy::upper_case_acronyms)]
use crate::{
    ast::NodeID,
    context::workspace::WorkspaceContext,
    detect::{high::*, low::*},
};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};
use strum::{Display, EnumCount, EnumIter, EnumString};

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumCount, Clone, EnumIter)]
pub enum IssueSeverity {
    Low,
    High,
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

macro_rules! define_detectors {
    ($($variant:ident),* $(,)?) => {
        paste! {
            #[remain::sorted]
            #[derive(Debug, PartialEq, EnumString, Display)]
            #[strum(serialize_all = "kebab-case")]
            pub enum IssueDetectorNamePool {
                $(
                    #[allow(clippy::upper_case_acronyms)]
                    $variant
                ),*
            }

            pub fn get_all_issue_detectors() -> Vec<Box<dyn IssueDetector>> {
                vec![$(Box::<[<$variant Detector>]>::default()),*]
            }

            pub fn request_issue_detector_by_name(detector_name: &str) -> Option<Box<dyn IssueDetector>> {
                let name = IssueDetectorNamePool::from_str(detector_name).ok()?;
                match name {
                    $(IssueDetectorNamePool::$variant => Some(Box::<[<$variant Detector>]>::default())),*
                }
            }
        }
    };
}

pub fn get_all_detectors_names() -> Vec<String> {
    get_all_issue_detectors().iter().map(|d| d.name()).collect()
}

pub fn get_issue_detector_by_name(detector_name: &str) -> Box<dyn IssueDetector> {
    request_issue_detector_by_name(detector_name).unwrap()
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

define_detectors! {
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
    DelegateCallUncheckedAddress,
    DelegatecallInLoop,
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
