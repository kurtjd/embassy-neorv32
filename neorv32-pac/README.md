# neorv32-pac
To re-generate this PAC:

```
svd2rust --target riscv --edition=2024 -i neorv32.svd
rm -rf src
form -i lib.rs -o src
cargo fmt
```

## TODO
- Figure out warning about legacy interrupt rendering
- Add target settings for more detail?
- Split UART DATA reg into fields
- Unnecessary unsafe for some regs?