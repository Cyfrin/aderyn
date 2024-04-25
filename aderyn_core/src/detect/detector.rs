use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    ast::NodeID,
    context::workspace_context::WorkspaceContext,
    detect::{
        high::{
            ArbitraryTransferFromDetector, AvoidAbiEncodePackedDetector,
            BlockTimestampDeadlineDetector, DelegateCallInLoopDetector,
            UnprotectedInitializerDetector, UnsafeCastingDetector,
        },
        low::{
            CentralizationRiskDetector, ConstantDefinedMultipleTimesDetector,
            ConstantsInsteadOfLiteralsDetector, ContractsWithTodosDetector,
            DeprecatedOZFunctionsDetector, DivisionBeforeMultiplicationDetector, EcrecoverDetector,
            EmptyBlockDetector, InconsistentTypeNamesDetector, LargeLiteralValueDetector,
            NonReentrantBeforeOthersDetector, PushZeroOpcodeDetector, RequireWithStringDetector,
            RevertsAndRequiresInLoopsDetector, SolmateSafeTransferLibDetector,
            UnindexedEventsDetector, UnsafeERC20FunctionsDetector, UnsafeERC721MintDetector,
            UnspecificSolidityPragmaDetector, UselessErrorDetector,
            UselessInternalFunctionDetector, UselessModifierDetector,
            UselessPublicFunctionDetector, ZeroAddressCheckDetector,
        },
    },
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
        Box::<ConstantDefinedMultipleTimesDetector>::default(),
    ]
}

pub fn get_all_detectors_names() -> Vec<String> {
    get_all_issue_detectors().iter().map(|d| d.name()).collect()
}

// Note to maintainers: DO NOT CHANGE THE ORDER OF THESE DERIVE ATTRIBUTES
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum IssueDetectorNamePool {
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
    ConstantDefinedMultipleTimes,
    // NOTE: `Undecided` will be the default name (for new bots).
    // If it's accepted, a new variant will be added to this enum before normalizing it in aderyn
    Undecided,
}

pub fn request_issue_detector_by_name(detector_name: &str) -> Option<Box<dyn IssueDetector>> {
    // Expects a valid detector_name
    let detector_name = IssueDetectorNamePool::from_str(detector_name).ok()?;
    match detector_name {
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
        IssueDetectorNamePool::ConstantDefinedMultipleTimes => {
            Some(Box::<ConstantDefinedMultipleTimesDetector>::default())
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
}
