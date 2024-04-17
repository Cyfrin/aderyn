cargo publish -p aderyn_core && echo "Aderyn Core is published" &&
cargo publish -p aderyn_driver && echo "Aderyn Driver is published" &&
cargo publish -p aderyn_py && echo "Aderyn Py is published" &&
cargo publish -p aderyn && echo "Aderyn is published" ||
echo "Aderyn is not ready to be published"