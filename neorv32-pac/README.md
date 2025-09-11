# neorv32-pac
To re-generate this PAC:

```
svd2rust --target riscv --edition=2024 -i neorv32.svd
rm -rf src
form -i lib.rs -o src
cargo fmt
```