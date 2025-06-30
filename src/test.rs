use alloc::boxed::Box;
use unwinding::panic::begin_panic;

pub fn test_panic() {
    println!("testing loongarch64 baremetal unwinding...");
    let payload1 = Box::new(1234);
    println!("payload1: {:?}", payload1);
    begin_panic(payload1);
}
