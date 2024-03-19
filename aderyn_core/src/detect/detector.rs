use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    ast::NodeID,
    context::workspace_context::{ASTNode, WorkspaceContext},
    detect::{
        high::{ArbitraryTransferFromDetector, DelegateCallInLoopDetector},
        low::{
            AvoidAbiEncodePackedDetector, DeprecatedOZFunctionsDetector, EcrecoverDetector,
            PushZeroOpcodeDetector, UnsafeERC20FunctionsDetector, UnspecificSolidityPragmaDetector,
        },
        medium::{
            BlockTimestampDeadlineDetector, CentralizationRiskDetector,
            SolmateSafeTransferLibDetector, UnsafeERC721MintDetector,
        },
        nc::{
            ConstantsInsteadOfLiteralsDetector, LargeLiteralValueDetector,
            NonReentrantBeforeOthersDetector, RequireWithStringDetector, UnindexedEventsDetector,
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
        Box::<LargeLiteralValueDetector>::default(),
        Box::<UselessInternalFunctionDetector>::default(),
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
    LargeNumericLiteral,
    UselessInternalFunction,
    // NOTE: `Undecided` will be the default name (for new bots).
    // If it's accepted, a new variant will be added to this enum before normalizing it in aderyn
    Undecided,
}

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum ResuableDetectorNamePool {
    IdentifiersThatReferenceAFunction,
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
        IssueDetectorNamePool::Undecided => None,
    }
}

pub fn get_issue_detector_by_name(detector_name: &str) -> Box<dyn IssueDetector> {
    request_issue_detector_by_name(detector_name).unwrap()
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumCount, Clone, EnumIter)]
pub enum IssueSeverity {
    NC,
    Low,
    Medium,
    High,
    Critical,
}

impl Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let issue_description = match self {
            IssueSeverity::NC => "NC (Non Critical)",
            IssueSeverity::Low => "Low",
            IssueSeverity::Medium => "Medium",
            IssueSeverity::High => "High",
            IssueSeverity::Critical => "Critical",
        };
        write!(f, "{}", issue_description).unwrap();
        Ok(())
    }
}

pub trait IssueDetector: Send + Sync + 'static {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Medium
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

pub trait ReusableDetector {
    fn detect(
        &mut self,
        _context: &WorkspaceContext,
        _using: &[ASTNode],
        _within: &[ASTNode],
    ) -> Result<&[ASTNode], Box<dyn Error>> {
        Ok(&[])
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::Undecided)
    }
}

pub mod detector_test_helpers {
    use std::path::PathBuf;

    use crate::{
        context::workspace_context::WorkspaceContext, framework::foundry::read_foundry_output_file,
        read_file_to_string, visitor::ast_visitor::Node,
    };

    pub fn load_contract(filepath: &str) -> WorkspaceContext {
        let path_buf_filepath = std::path::PathBuf::from(filepath);
        let mut context = WorkspaceContext::default();
        let foundry_output = read_foundry_output_file(path_buf_filepath.to_str().unwrap()).unwrap();
        let mut ast = foundry_output.ast.clone();
        // Get the path of the source file
        let mut new_path = PathBuf::new();
        for component in path_buf_filepath.components() {
            if component.as_os_str() == "out" {
                break;
            }
            new_path.push(component);
        }
        new_path.push(ast.absolute_path.as_ref().unwrap());
        match read_file_to_string(&new_path) {
            Ok(content) => {
                println!(
                    "Loaded Solidity source file: {}",
                    new_path.to_str().unwrap()
                );

                ast.source = Some(content);
            }
            Err(err) => {
                eprintln!(
                    "Error reading Solidity source file: {}",
                    new_path.to_str().unwrap()
                );
                eprintln!("{:?}", err);
            }
        }
        ast.accept(&mut context).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error loading Hardhat AST into WorkspaceContext");
            eprintln!("{:?}", err);
        });
        context
    }
}
