use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumString};

use crate::{
    ast::NodeID,
    context::workspace_context::WorkspaceContext,
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
            ConstantsInsteadOfLiteralsDetector, NonReentrantBeforeOthersDetector,
            RequireWithStringDetector, UnindexedEventsDetector, UselessPublicFunctionDetector,
            ZeroAddressCheckDetector,
        },
    },
};
use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

pub fn get_all_detectors() -> Vec<Box<dyn Detector>> {
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
    ]
}

pub fn get_all_detectors_names() -> Vec<String> {
    get_all_detectors().iter().map(|d| d.name()).collect()
}

// Note to maintainers: DO NOT CHANGE THE ORDER OF THESE DERIVE ATTRIBUTES
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum DetectorNamePool {
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
    // NOTE: `Undecided` will be the default name (for new bots).
    // If it's accepted, a new variant will be added to this enum before normalizing it in aderyn
    Undecided,
}

pub fn get_detector_by_name(detector_name: &str) -> Box<dyn Detector> {
    // Expects a valid detector_name
    let detector_name = DetectorNamePool::from_str(detector_name).unwrap();
    match detector_name {
        DetectorNamePool::DelegateCallInLoop => Box::<DelegateCallInLoopDetector>::default(),
        DetectorNamePool::CentralizationRisk => Box::<CentralizationRiskDetector>::default(),
        DetectorNamePool::SolmateSafeTransferLib => {
            Box::<SolmateSafeTransferLibDetector>::default()
        }
        DetectorNamePool::AvoidAbiEncodePacked => Box::<AvoidAbiEncodePackedDetector>::default(),
        DetectorNamePool::Ecrecover => Box::<EcrecoverDetector>::default(),
        DetectorNamePool::DeprecatedOzFunctions => Box::<DeprecatedOZFunctionsDetector>::default(),
        DetectorNamePool::UnsafeERC20Functions => Box::<UnsafeERC20FunctionsDetector>::default(),
        DetectorNamePool::UnspecificSolidityPragma => {
            Box::<UnspecificSolidityPragmaDetector>::default()
        }
        DetectorNamePool::ZeroAddressCheck => Box::<ZeroAddressCheckDetector>::default(),
        DetectorNamePool::UselessPublicFunction => Box::<UselessPublicFunctionDetector>::default(),
        DetectorNamePool::ConstantsInsteadOfLiterals => {
            Box::<ConstantsInsteadOfLiteralsDetector>::default()
        }
        DetectorNamePool::UnindexedEvents => Box::<UnindexedEventsDetector>::default(),
        DetectorNamePool::RequireWithString => Box::<RequireWithStringDetector>::default(),
        DetectorNamePool::NonReentrantBeforeOthers => {
            Box::<NonReentrantBeforeOthersDetector>::default()
        }
        DetectorNamePool::BlockTimestampDeadline => {
            Box::<BlockTimestampDeadlineDetector>::default()
        }
        DetectorNamePool::UnsafeOzERC721Mint => Box::<UnsafeERC721MintDetector>::default(),
        DetectorNamePool::PushZeroOpcode => Box::<PushZeroOpcodeDetector>::default(),
        DetectorNamePool::ArbitraryTransferFrom => Box::<ArbitraryTransferFromDetector>::default(),
        DetectorNamePool::Undecided => panic!("Undecided bots should't be invoked"),
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumCount, Clone)]
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

pub trait Detector {
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
        format!("{}", DetectorNamePool::Undecided)
    }

    // Keys are source file name and line number
    // Value is ASTNode NodeID
    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        BTreeMap::new()
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
