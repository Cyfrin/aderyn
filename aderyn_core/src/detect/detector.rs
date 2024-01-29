use crate::{
    context::loader::ContextLoader,
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
use std::{collections::BTreeMap, error::Error};

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

#[derive(Debug, PartialEq)]
pub enum IssueSeverity {
    NC,
    Low,
    Medium,
    High,
    Critical,
}

pub trait Detector {
    fn detect(&mut self, _loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
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

    // Keys are source file name and line number
    // Value is ASTNode.src
    fn instances(&self) -> BTreeMap<(String, usize), String> {
        BTreeMap::new()
    }
}

pub mod detector_test_helpers {
    use std::path::PathBuf;

    use crate::{
        context::loader::ContextLoader, framework::foundry::read_foundry_output_file,
        read_file_to_string, visitor::ast_visitor::Node,
    };

    pub fn load_contract(filepath: &str) -> ContextLoader {
        let path_buf_filepath = std::path::PathBuf::from(filepath);
        let mut context_loader = ContextLoader::default();
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
        ast.accept(&mut context_loader).unwrap_or_else(|err| {
            // Exit with a non-zero exit code
            eprintln!("Error loading Hardhat AST into ContextLoader");
            eprintln!("{:?}", err);
        });
        context_loader
    }
}
