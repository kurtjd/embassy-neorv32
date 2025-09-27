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

#[riscv_rt::core_interrupt(crate::pac::interrupt::CoreInterrupt::MachineTimer)]
fn machine_timer_handler() {
    DRIVER.on_interrupt()
}

fn clint() -> crate::pac::Clint {
    // SAFETY: TODO
    unsafe { crate::pac::Clint::steal() }
}

struct MtimerDriver {
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

impl MtimerDriver {
    fn on_interrupt(&self) {
        // Disable the mtimer interrupt
        riscv::interrupt::disable_interrupt(crate::pac::interrupt::CoreInterrupt::MachineTimer);

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

        clint().mtimer().mtimecmp0().write(ts);

        // Timestamp is in the past
        let t = self.now();
        if ts <= t {
            false
        } else {
            // SAFETY: TODO
            unsafe {
                riscv::interrupt::enable_interrupt(
                    crate::pac::interrupt::CoreInterrupt::MachineTimer,
                );
            }
            true
        }
    }
}

impl Driver for MtimerDriver {
    fn now(&self) -> u64 {
        clint().mtimer().mtime().read()
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
