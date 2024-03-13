const aderyn_driver = require(".");

aderyn_driver.drive("../tests/contract-playground", "report_jsbindings.json", false, [], [])

aderyn_driver.drive_with(
    "../tests/contract-playground", 
    "report_jsbindings_config.json",
    false,
    [],
    ["Counter.sol", "Contract.sol"],
    ["push-zero-opcode","useless-public-function"],
);