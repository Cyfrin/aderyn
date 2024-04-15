# This script will bench each individual detectors as well as overall aderyn 
# Note: We Must skip CLOC counting and forge build phase. 

# Copy the existing benchmarks to target/ so aderyn can know the history
cp -r benchmarks/* target/criterion/

export ADERYN_SKIP_BUILD=1
export ADERYN_CLOC_SKIP=1

# Make the bench
cargo bench

unset ADERYN_SKIP_BUILD
unset ADERYN_CLOC_SKIP

# Replace the benchmarks to the reflect the latest
cp -r target/criterion/* benchmarks/

# Open the benchmarks reports in browser
open benchmarks/report/index.html