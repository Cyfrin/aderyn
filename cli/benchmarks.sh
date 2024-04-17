# This script will bench each individual detectors as well as overall aderyn 
# Note: We Must skip CLOC counting and forge build phase. 

# Copy the existing benchmarks to target/ so aderyn can know the history
cp -r benchmarks/* target/criterion/

# Make the bench
cargo bench

# Replace the benchmarks to the reflect the latest
cp -r target/criterion/* benchmarks/

# Open the benchmarks reports in browser
open benchmarks/report/index.html