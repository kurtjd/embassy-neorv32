//! Pulse Width Modulation (PWM)
use crate::sysinfo::SysInfo;
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};

const MAX_DUTY: u8 = u8::MAX;

/// PWM period in microseconds.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Microseconds(pub u32);

impl From<Microseconds> for u32 {
    fn from(micros: Microseconds) -> Self {
        micros.0
    }
}

impl From<u32> for Microseconds {
    fn from(value: u32) -> Self {
        Microseconds(value)
    }
}

/// PWM channel driver.
///
/// **Note**: This will disable PWM channel when dropped.
pub struct Pwm<'d> {
    cfg: &'static crate::pac::pwm::ChannelCfg0,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Pwm<'d> {
    fn enable(&mut self) {
        self.cfg.modify(|_, w| w.pwm_cfg_en().set_bit());
    }

    fn disable(&mut self) {
        self.cfg.modify(|_, w| w.pwm_cfg_en().clear_bit());
    }

    fn get_duty(&self) -> u8 {
        self.cfg.read().pwm_cfg_duty().bits()
    }

    fn set_freq(&mut self, pwm_freq: u32) {
        let cpu_freq = SysInfo::clock_freq() as u64;
        let denom = pwm_freq as u64 * 256 * 2;

        // The provided frequency is zero, or too large to be represented
        assert!(denom > 0 && denom <= cpu_freq);

        let mut cdiv = (cpu_freq / denom) - 1;
        let mut psc = 0;

        // Calculate prescaler and divider similar to UART
        // See: https://github.com/stnolting/neorv32/blob/main/sw/lib/source/neorv32_uart.c#L47
        while cdiv > 0x3ff {
            if psc == 2 || psc == 4 {
                cdiv >>= 3;
            } else {
                cdiv >>= 1;
            }
            psc += 1;
        }

        // The provided frequency is too small to be represented
        assert!(psc <= 7);

        // SAFETY: We are writing a valid 3 bit pattern for the correct prescaler
        self.cfg
            .modify(|_, w| unsafe { w.pwm_cfg_prsc().bits(psc) });

        // SAFETY: The chosen clock divider is ensured to fit in 10 bits
        self.cfg
            .modify(|_, w| unsafe { w.pwm_cfg_cdiv().bits(cdiv as u16) });
    }

    fn get_freq(&self) -> u32 {
        let psc = match self.cfg.read().pwm_cfg_prsc().bits() {
            0b000 => 2,
            0b001 => 4,
            0b010 => 8,
            0b011 => 64,
            0b100 => 128,
            0b101 => 1024,
            0b110 => 2048,
            0b111 => 4096,
            // The register is physically only 3 bits wide
            _ => unreachable!(),
        };

        let cdiv = self.cfg.read().pwm_cfg_cdiv().bits() as u64;
        let cpu_freq = SysInfo::clock_freq() as u64;

        (cpu_freq / (256 * psc * (1 + cdiv))) as u32
    }

    /// Create a new instance of a PWM channel driver with given frequency and enables it.
    ///
    /// Depending on main clock frequency, not all pwm frequencies can be represented exactly.
    ///
    /// # Panics
    ///
    /// Panics if `pwm_freq == 0`, or `pwm_freq` is too small or too large to be represented,
    /// which is dependent on system main clock frequency.
    pub fn new<T: Instance>(_instance: Peri<'d, T>, pwm_freq: u32, invert_polarity: bool) -> Self {
        let mut pwm = Self {
            cfg: T::cfg(),
            _phantom: PhantomData,
        };

        pwm.set_freq(pwm_freq);

        // Invert polarity if desired
        if invert_polarity {
            T::cfg().modify(|_, w| w.pwm_cfg_pol().set_bit());
        }

        pwm.enable();
        pwm
    }

    /// Set the PWM channel duty cycle in raw form.
    pub fn set_duty_cycle(&mut self, duty: u8) {
        // SAFETY: Any u8 duty cycle is valid
        self.cfg
            .modify(|_, w| unsafe { w.pwm_cfg_duty().bits(duty) });
    }

    /// Set the PWM channel duty cycle in percent. Caller must ensure
    /// `percent <= 100`.
    ///
    /// # Panics
    ///
    /// Panics if `percent > 100`
    pub fn set_duty_cycle_percent(&mut self, percent: u8) {
        assert!(percent <= 100);

        // Round to nearest integer
        let duty = (u16::from(percent) * MAX_DUTY as u16 + 50) / 100;
        self.set_duty_cycle(duty as u8);
    }
}

impl<'d> Drop for Pwm<'d> {
    fn drop(&mut self) {
        self.disable();
    }
}

trait SealedInstance {
    // All channels share the pwm::Register block,
    // but each has it's own dedicated register.
    //
    // All ChannelCfgN are alias for ChannelCfg0,
    // so we can use that as return type,
    // but fortunately each channel_cfgN
    // returns the address to its register.
    fn cfg() -> &'static crate::pac::pwm::ChannelCfg0;
}

/// A valid PWM peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}

macro_rules! impl_pwm {
    ($periph:ident, $ch:ident) => {
        impl SealedInstance for crate::peripherals::$periph {
            fn cfg() -> &'static crate::pac::pwm::ChannelCfg0 {
                // SAFETY: We own the PWM peripheral and use it safely
                unsafe { &*crate::pac::Pwm::ptr() }.$ch()
            }
        }
        impl Instance for crate::peripherals::$periph {}
    };
}

impl_pwm!(PWM0, channel_cfg0);
impl_pwm!(PWM1, channel_cfg1);
impl_pwm!(PWM2, channel_cfg2);
impl_pwm!(PWM3, channel_cfg3);
impl_pwm!(PWM4, channel_cfg4);
impl_pwm!(PWM5, channel_cfg5);
impl_pwm!(PWM6, channel_cfg6);
impl_pwm!(PWM7, channel_cfg7);
impl_pwm!(PWM8, channel_cfg8);
impl_pwm!(PWM9, channel_cfg9);
impl_pwm!(PWM10, channel_cfg10);
impl_pwm!(PWM11, channel_cfg11);
impl_pwm!(PWM12, channel_cfg12);
impl_pwm!(PWM13, channel_cfg13);
impl_pwm!(PWM14, channel_cfg14);
impl_pwm!(PWM15, channel_cfg15);

impl<'d> embedded_hal_02::Pwm for Pwm<'d> {
    // An instance of `Pwm` already represents a specific channel
    type Channel = ();
    type Duty = u8;
    type Time = Microseconds;

    fn enable(&mut self, _channel: Self::Channel) {
        self.enable();
    }

    fn disable(&mut self, _channel: Self::Channel) {
        self.disable();
    }

    fn get_period(&self) -> Self::Time {
        Microseconds(1_000_000 / self.get_freq())
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>,
    {
        self.set_freq(1_000_000 / u32::from(period.into()));
    }

    fn get_duty(&self, _channel: Self::Channel) -> Self::Duty {
        self.get_duty()
    }

    fn get_max_duty(&self) -> Self::Duty {
        MAX_DUTY
    }

    fn set_duty(&mut self, _channel: Self::Channel, duty: Self::Duty) {
        self.set_duty_cycle(duty);
    }
}

impl<'d> embedded_hal_02::PwmPin for Pwm<'d> {
    type Duty = u8;

    fn enable(&mut self) {
        self.enable();
    }

    fn disable(&mut self) {
        self.disable();
    }

    fn get_duty(&self) -> Self::Duty {
        self.get_duty()
    }

    fn get_max_duty(&self) -> Self::Duty {
        MAX_DUTY
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.set_duty_cycle(duty);
    }
}

impl<'d> embedded_hal_1::pwm::ErrorType for Pwm<'d> {
    type Error = core::convert::Infallible;
}

impl<'d> embedded_hal_1::pwm::SetDutyCycle for Pwm<'d> {
    fn max_duty_cycle(&self) -> u16 {
        MAX_DUTY as u16
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        assert!(duty <= MAX_DUTY as u16);
        self.set_duty_cycle(duty as u8);
        Ok(())
    }

    fn set_duty_cycle_percent(&mut self, percent: u8) -> Result<(), Self::Error> {
        self.set_duty_cycle_percent(percent);
        Ok(())
    }
}
