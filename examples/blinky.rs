#![feature(proc_macro)]
#![no_std]

extern crate onebitsy;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;

use onebitsy::led::{self, Orange};

use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

app! {
    device: onebitsy::stm32f41x,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: toggle,
            resources: [ON],
        },
    },
}

fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(p.GPIOA, p.RCC);

    // Configure the system timer to generate periodic events at 1 Hz rate
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000); // Period = 1s
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS

// Toggle the state of the LED
fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.ON = !**r.ON;

    if **r.ON {
        Orange.on();
    } else {
        Orange.off();
    }
}
