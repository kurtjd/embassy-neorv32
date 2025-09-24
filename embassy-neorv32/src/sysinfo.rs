//! SysInfo

/// Processor boot mode
pub enum BootMode {
    Bootloader,
    CustomAddress,
    ImemImage,
    Unknown,
}

impl BootMode {
    /// Returns the boot made as a static string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bootloader => "Bootloader",
            Self::CustomAddress => "Custom Address",
            Self::ImemImage => "IMEM Image",
            Self::Unknown => "Unknown",
        }
    }
}

impl From<u8> for BootMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Bootloader,
            1 => Self::CustomAddress,
            2 => Self::ImemImage,
            _ => Self::Unknown,
        }
    }
}

/// SoC Configuration
pub struct SocConfig(u32);
impl SocConfig {
    /// Returns raw 32-bit SoC config
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Returns true if processor-internal bootloader is implemented
    pub fn bootloader(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Returns true if external bus interface (XBUS) is implemented
    pub fn xbus(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Returns true if processor-internal IMEM is implemented
    pub fn imem(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Returns true if processor-internal DMEM is implemented
    pub fn dmem(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Returns true if on-chip debugger is implemented
    pub fn ocd(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Returns true if processor-internal instruction cache is implemented
    pub fn icache(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Returns true if processor-internal data cache is implemented
    pub fn dcache(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Returns true if on-chip debugger authentication is implemented
    pub fn ocd_auth(&self) -> bool {
        self.0 & (1 << 11) != 0
    }

    /// Returns true if processor-internal IMEM is implemented as pre-initialized ROM
    pub fn imem_as_rom(&self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// Returns true if TWD is implemented
    pub fn twd(&self) -> bool {
        self.0 & (1 << 13) != 0
    }

    /// Returns true if DMA is implemented
    pub fn dma(&self) -> bool {
        self.0 & (1 << 14) != 0
    }

    /// Returns true if GPIO is implemented
    pub fn gpio(&self) -> bool {
        self.0 & (1 << 15) != 0
    }

    /// Returns true if CLINT is implemented
    pub fn clint(&self) -> bool {
        self.0 & (1 << 16) != 0
    }

    /// Returns true if UART0 is implemented
    pub fn uart0(&self) -> bool {
        self.0 & (1 << 17) != 0
    }

    /// Returns true if SPI is implemented
    pub fn spi(&self) -> bool {
        self.0 & (1 << 18) != 0
    }

    /// Returns true if TWI is implemented
    pub fn twi(&self) -> bool {
        self.0 & (1 << 19) != 0
    }

    /// Returns true if PWM is implemented
    pub fn pwm(&self) -> bool {
        self.0 & (1 << 20) != 0
    }

    /// Returns true if WDT is implemented
    pub fn wdt(&self) -> bool {
        self.0 & (1 << 21) != 0
    }

    /// Returns true if CFS is implemented
    pub fn cfs(&self) -> bool {
        self.0 & (1 << 22) != 0
    }

    /// Returns true if TRNG is implemented
    pub fn trng(&self) -> bool {
        self.0 & (1 << 23) != 0
    }

    /// Returns true if SDI is implemented
    pub fn sdi(&self) -> bool {
        self.0 & (1 << 24) != 0
    }

    /// Returns true if UART1 is implemented
    pub fn uart1(&self) -> bool {
        self.0 & (1 << 25) != 0
    }

    /// Returns true if NEOLED is implemented
    pub fn neoled(&self) -> bool {
        self.0 & (1 << 26) != 0
    }

    /// Returns true if TRACER is implemented
    pub fn tracer(&self) -> bool {
        self.0 & (1 << 27) != 0
    }

    /// Returns true if GPTMR is implemented
    pub fn gptmr(&self) -> bool {
        self.0 & (1 << 28) != 0
    }

    /// Returns true if SLINK is implemented
    pub fn slink(&self) -> bool {
        self.0 & (1 << 29) != 0
    }

    /// Returns true if ONEWIRE is implemented
    pub fn onewire(&self) -> bool {
        self.0 & (1 << 30) != 0
    }
}

/// SysInfo Driver
pub struct SysInfo {
    reg: &'static pac::sysinfo::RegisterBlock,
}

impl SysInfo {
    /// Returns a SysInfo instance
    pub fn new<T: Instance>(_instance: T) -> Self {
        Self { reg: T::reg() }
    }

    /// Returns the main CPU clock frequency (Hz)
    #[inline(always)]
    pub fn clock_freq(&self) -> u32 {
        self.reg.clk().read().bits()
    }

    /// Sets the main CPU clock frequency (Hz)
    #[inline(always)]
    pub fn set_clock_freq(&self, freq_hz: u32) {
        self.reg.clk().write(|w| unsafe { w.bits(freq_hz) });
    }

    /// Returns the IMEM size in bytes
    #[inline(always)]
    pub fn imem_size(&self) -> u32 {
        // Read value is log2, so do inverse log for actual value
        1 << self.reg.mem().read().sysinfo_misc_imem().bits() as u32
    }

    /// Returns the DMEM size in bytes
    #[inline(always)]
    pub fn dmem_size(&self) -> u32 {
        // Read value is log2, so do inverse log for actual value
        1 << self.reg.mem().read().sysinfo_misc_dmem().bits() as u32
    }

    /// Returns the number of harts (cores)
    #[inline(always)]
    pub fn num_harts(&self) -> u8 {
        self.reg.mem().read().sysinfo_misc_hart().bits()
    }

    /// Returns the boot mode configuration
    #[inline(always)]
    pub fn boot_mode(&self) -> BootMode {
        let raw = self.reg.mem().read().sysinfo_misc_boot().bits();
        BootMode::from(raw)
    }

    /// Returns the number of bus timeout cycles
    #[inline(always)]
    pub fn bus_timeout_cycles(&self) -> u8 {
        self.reg.mem().read().sysinfo_misc_btmo().bits()
    }

    /// Returns the SoC config
    ///
    /// Additional methods can be called to check if SoC features are implemented
    #[inline(always)]
    pub fn soc_config(&self) -> SocConfig {
        SocConfig(self.reg.soc().read().bits())
    }
}

/// A valid SysInfo peripheral
#[allow(private_bounds)]
pub trait Instance: crate::Sealed {
    fn reg() -> &'static pac::sysinfo::RegisterBlock;
}

impl crate::Sealed for pac::Sysinfo {}
impl Instance for pac::Sysinfo {
    fn reg() -> &'static pac::sysinfo::RegisterBlock {
        unsafe { &*pac::Sysinfo::ptr() }
    }
}
