#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - TWD_FIRQ"]
    TWD_FIRQ = 0,
    #[doc = "1 - CFS_FIRQ"]
    CFS_FIRQ = 1,
    #[doc = "2 - UART0_FIRQ"]
    UART0_FIRQ = 2,
    #[doc = "3 - UART1_FIRQ"]
    UART1_FIRQ = 3,
    #[doc = "5 - TRACER_FIRQ"]
    TRACER_FIRQ = 5,
    #[doc = "6 - SPI_FIRQ"]
    SPI_FIRQ = 6,
    #[doc = "7 - TWI_FIRQ"]
    TWI_FIRQ = 7,
    #[doc = "8 - GPIO_FIRQ"]
    GPIO_FIRQ = 8,
    #[doc = "9 - NEOLED_FIRQ"]
    NEOLED_FIRQ = 9,
    #[doc = "10 - DMA_FIRQ"]
    DMA_FIRQ = 10,
    #[doc = "11 - SDI_FIRQ"]
    SDI_FIRQ = 11,
    #[doc = "12 - GPTMR_FIRQ"]
    GPTMR_FIRQ = 12,
    #[doc = "13 - ONEWIRE_FIRQ"]
    ONEWIRE_FIRQ = 13,
    #[doc = "14 - SLINK_FIRQ"]
    SLINK_FIRQ = 14,
    #[doc = "15 - TRNG_FIRQ"]
    TRNG_FIRQ = 15,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::TWD_FIRQ),
            1 => Ok(Interrupt::CFS_FIRQ),
            2 => Ok(Interrupt::UART0_FIRQ),
            3 => Ok(Interrupt::UART1_FIRQ),
            5 => Ok(Interrupt::TRACER_FIRQ),
            6 => Ok(Interrupt::SPI_FIRQ),
            7 => Ok(Interrupt::TWI_FIRQ),
            8 => Ok(Interrupt::GPIO_FIRQ),
            9 => Ok(Interrupt::NEOLED_FIRQ),
            10 => Ok(Interrupt::DMA_FIRQ),
            11 => Ok(Interrupt::SDI_FIRQ),
            12 => Ok(Interrupt::GPTMR_FIRQ),
            13 => Ok(Interrupt::ONEWIRE_FIRQ),
            14 => Ok(Interrupt::SLINK_FIRQ),
            15 => Ok(Interrupt::TRNG_FIRQ),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [unsafe (no_mangle)] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [unsafe (no_mangle)] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
