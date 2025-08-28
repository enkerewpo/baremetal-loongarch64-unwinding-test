use alloc::boxed::Box;
use loongArch64::{
    register::tcfg,
    time::{Time, get_timer_freq},
};
use unwinding::panic::begin_panic;

pub fn test_panic_level0() {
    println!("testing loongarch64 baremetal panic level 0...");
    test_panic_level1();
}

pub fn test_panic_level1() {
    println!("testing loongarch64 baremetal panic level 1...");
    test_panic_level2();
}

pub fn test_panic_level2() {
    println!("testing loongarch64 baremetal panic level 2...");
    test_panic_level3();
}

pub fn test_panic_level3() {
    println!("testing loongarch64 baremetal panic level 3...");
    panic!("test panic for baremetal application using unwinding library");
}

pub fn test_panic() {
    test_panic_level0();
}

pub fn test_vtimer() {
    println!("testing loongarch64 vtimer...");
    let freq = get_timer_freq();
    println!("timer frequency: {}", freq);

    // test timer interrupt
    tcfg::set_periodic(true);
    let hz = 100;
    let interval = freq / hz;
    println!("interval: {}", interval);
    tcfg::set_init_val(interval);
    tcfg::set_en(true);
    loop {
        // Idle loop
    }
}
