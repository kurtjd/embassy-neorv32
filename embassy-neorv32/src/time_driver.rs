//! Extremely basic and very likely not sound
//! Just a simple PoC for now
use core::cell::RefCell;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

embassy_time_driver::time_driver_impl!(static DRIVER: MtimerDriver = MtimerDriver {
    queue: Mutex::new(RefCell::new(Queue::new()))
});

#[riscv_rt::core_interrupt(riscv::interrupt::Interrupt::MachineTimer)]
fn mtimer_handler() {
    DRIVER.on_interrupt()
}

fn clint() -> &'static neorv32_pac::clint::RegisterBlock {
    // SAFETY: TODO
    unsafe { &*neorv32_pac::Clint::ptr() }
}

struct MtimerDriver {
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

impl MtimerDriver {
    fn on_interrupt(&self) {
        // Disable the mtimer interrupt
        riscv::interrupt::disable_interrupt(riscv::interrupt::Interrupt::MachineTimer);

        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            let mut next = queue.next_expiration(self.now());
            while !self.set_alarm(next) {
                next = queue.next_expiration(self.now());
            }
        });
    }

    fn set_alarm(&self, ts: u64) -> bool {
        // Timestamp is in the past, so can't set the alarm
        let t = self.now();
        if ts <= t {
            return false;
        }

        // SAFETY: Pac might be unnecessarily marking these unsafe
        clint()
            .mtimecmp0_hi()
            .write(|w| unsafe { w.bits((ts >> 32) as u32) });

        // SAFETY: Pac might be unnecessarily marking these unsafe
        clint()
            .mtimecmp0_low()
            .write(|w| unsafe { w.bits(ts as u32) });

        // Timestamp is in the past
        let t = self.now();
        if ts <= t {
            false
        } else {
            // SAFETY: TODO
            unsafe {
                riscv::interrupt::enable_interrupt(riscv::interrupt::Interrupt::MachineTimer);
            }
            true
        }
    }
}

impl Driver for MtimerDriver {
    fn now(&self) -> u64 {
        // Read the 64 bit mtime register
        // Unfortunately neorv32 does not mirror this to `time` so can't use `riscv::register::time`
        loop {
            let mtime_hi = clint().mtime_hi().read().bits() as u64;
            let mtime_lo = clint().mtime_low().read().bits() as u64;
            let mtime_hi2 = clint().mtime_hi().read().bits() as u64;

            // Need to check for rollover of lo in between hi and lo read
            // If there is rollover, repeat
            if mtime_hi == mtime_hi2 {
                return (mtime_hi << 32) | mtime_lo;
            }
        }
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(next) {
                    next = queue.next_expiration(self.now());
                }
            }
        })
    }
}
