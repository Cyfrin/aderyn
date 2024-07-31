from aderynpy import generate_report

generate_report("../tests/contract-playground", "./report.md")

# generate_report("../tests/contract-playground", "./report.md", 
#     path_includes=["src/eth2", "src/inheritance"],
# )