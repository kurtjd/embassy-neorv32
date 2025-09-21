# neorv32-pac
To re-generate this PAC:

```
svd2rust --target riscv --edition=2024 --settings config.yml -i neorv32.svd
rm -rf src
form -i lib.rs -o src
rm lib.rs
cargo fmt
```

## TODO
SVD:
- Split UART DATA reg into fields?
- Consider removing `<interrupt>` fields since svd2rust treats these as external interrupts but are actually core interrupts

svd2rust:
- Not specifying unsafe {...} where needed for 2024 edition (lib.rs setting static mut DEVICE_PERIPHERALS)
- `pac_enum` macro for ExceptionNumer seems to be requiring rt but not for CoreInterruptNumber?
- `pac_enum` docs says it can't be used for CoreInterruptNumber, but svd2rust generates it?
- Unnecessary unsafe for some regs?
