# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.2.1] - 2026-02-18

### Changed

- Fix UART `write_byte` so it does a write, not modify of data reg
- Check the duty cycle for PWM in embedded-hal implementation
- Change DMA transfer size comments from 23 bits to 24 bits
- Fix dual-hart CS acquire to return mie, not mstatus
- Have TWI driver shift address before ORing R/W bit
- Fix UART TX active
- Fix GPIO `is_low`

## [v0.2.0] - 2026-02-15

### Added

- Add instructions for building `image_gen` to README
- Implemented RX-buffered UART
- Created a CHANGELOG

### Changed

- Change image-gen parameters in example scripts
- Changed WDT timeout in example
- Update to NEORV32 v1.12.8

### Removed

- Removed temporary critical section bug workaround
