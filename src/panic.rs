use core::ffi::c_void;
use core::sync::atomic::{AtomicBool, Ordering};
use gimli::Register;
use unwinding::abi::{
    _Unwind_Backtrace, _Unwind_FindEnclosingFunction, _Unwind_GetGR, _Unwind_GetIP, UnwindContext,
    UnwindReasonCode,
};

static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe extern "Rust" {
        pub fn __panic_handler(info: &core::panic::PanicInfo) -> !;
    }
    unsafe { __panic_handler(info) }
}

#[unsafe(no_mangle)]
pub fn __panic_handler(info: &core::panic::PanicInfo) -> ! {
    if PANIC_IN_PROGRESS.load(Ordering::Acquire) {
        println!(
            "DOUBLE PANIC detected! Reason: {:?}",
            info.message().as_str().unwrap_or("")
        );
        loop {}
    }
    
    PANIC_IN_PROGRESS.store(true, Ordering::Release);
    
    println!(
        "(__panic_handler) panic at {:?}, reason: {:?}",
        info.location().unwrap(),
        info.message().as_str().unwrap_or("")
    );
    print_stack_trace();
    println!("halting, now we reach the end of the program");
    loop {}
}

pub fn print_stack_trace() {
    println!("printing stack trace...");

    struct CallbackData {
        counter: usize,
    }
    extern "C" fn callback(unwind_ctx: &UnwindContext<'_>, arg: *mut c_void) -> UnwindReasonCode {
        println!("callback...");
        let data = unsafe { &mut *(arg as *mut CallbackData) };
        data.counter += 1;
        
        if data.counter > 100 {
            println!("Stack trace limit reached (100 frames), stopping to prevent infinite loop");
            return UnwindReasonCode::END_OF_STACK;
        }
        
        let pc = _Unwind_GetIP(unwind_ctx);
        if pc > 0 {
            let fde_initial_address = _Unwind_FindEnclosingFunction(pc as *mut c_void) as usize;
            println!(
                "{:4}: fn {:#18x} - pc {:#18x} / registers:",
                data.counter, fde_initial_address, pc,
            );
        }
        // Print the first 8 general registers for any architecture. The register number follows
        // the DWARF standard.
        for i in 0..8u16 {
            let reg_i = _Unwind_GetGR(unwind_ctx, i as i32);
            // we are only testing on loongarch64 for now :D
            let reg_name = gimli::LoongArch::register_name(Register(i)).unwrap_or("unknown");
            if i % 4 == 0 {
                print!("\n    ");
            }
            print!(" {} {:#18x};", reg_name, reg_i);
        }
        print!("\n\n");
        UnwindReasonCode::NO_REASON
    }

    let mut data = CallbackData { counter: 0 };
    _Unwind_Backtrace(callback, &mut data as *mut _ as _);
}
