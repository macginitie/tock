use riscvimac;
use riscvimac::plic;
use kernel;
use gpio;
use interrupts;
use uart;

pub struct E310x(());

impl E310x {
    pub unsafe fn new() -> E310x {
        E310x(())
    }
}

impl kernel::Chip for E310x {
    type MPU = ();
    type SysTick = ();

    fn mpu(&self) -> &Self::MPU {
        &self.0
    }

    fn systick(&self) -> &Self::SysTick {
        &self.0
    }

    fn service_pending_interrupts(&mut self) {
        unsafe {
            while let Some(interrupt) = plic::next_pending() {
                // debug_gpio!(0, toggle);

                if interrupt == 1 {
                    debug_gpio!(0, toggle);
                }

                match interrupt {
                    interrupts::WATCHDOG => { /* Not sure why this interrupt is happening. */}
                    interrupts::UART0 => uart::UART0.handle_interrupt(),
                    index @ interrupts::GPIO0..interrupts::GPIO31 => gpio::PORT[index as usize].handle_interrupt(),
                    _ => debug!("PLIC index not supported by Tock"),
                }
                plic::clear_pending(interrupt);
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { plic::has_pending() }
    }

    fn sleep(&self) {
        unsafe {
            // riscvimac::support::wfi();
            riscvimac::support::nop();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        riscvimac::support::atomic(f)
    }
}
