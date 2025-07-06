use alloc::boxed::Box;
use loongArch64::time::{Time, get_timer_freq};
use unwinding::panic::begin_panic;

pub fn test_panic() {
    println!("testing loongarch64 baremetal unwinding...");
    let payload1 = Box::new(1234);
    println!("payload1: {:?}", payload1);
    begin_panic(payload1);
}

pub fn test_vtimer() {
    println!("testing loongarch64 vtimer...");
    let freq = get_timer_freq();
    println!("timer frequency: {}", freq);
    let mut last_time = Time::read();
    let mut counter = 0;
    loop {
        let current_time = Time::read();
        if current_time - last_time > freq {
            counter += 1;
            println!(
                "time ticked, counter: {}, current_time: {}, last_time: {}, diff: {}",
                counter,
                current_time,
                last_time,
                current_time - last_time
            );
            last_time = current_time;
            if counter >= 10 {
                break;
            }
        }
    }
    println!("test_vtimer done");
}
