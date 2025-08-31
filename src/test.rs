use alloc::boxed::Box;
use loongArch64::{
    register::tcfg,
    time::{Time, get_timer_freq},
};
use unwinding::panic::begin_panic;

pub fn test_panic() {
    // panic!("test panic");
    begin_panic(Box::new("test panic"));
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
