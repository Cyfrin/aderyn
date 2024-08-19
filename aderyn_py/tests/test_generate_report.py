import subprocess
import pytest
from aderynpy import generate_report

@pytest.mark.parametrize("root, report", [
    ("../tests/contract-playground", "../reports/report.md"),
    ("../tests/2024-05-Sablier", "../reports/sablier-aderyn-toml-nested-root.md"),
    ("../tests/adhoc-sol-files", "../reports/adhoc-sol-files-report.md"),
    ("../tests/foundry-nft-f23", "../reports/nft-report.md"),
    ("../tests/prb-math", "../reports/prb-math-report.md"),
])
def test_generate_report(root, report):
    # Define output file path
    out_file = f"./{root.split('../')[-1]}-workflow.md"
    
    # Call the generate_report function
    generate_report(root, out_file)
    
    # Run the diff command to compare the generated report with the original report
    result = subprocess.run(
        ["diff", str(out_file), report],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    
    # Check if diff command found any differences (result.returncode == 0 means no differences)
    assert result.returncode == 1
