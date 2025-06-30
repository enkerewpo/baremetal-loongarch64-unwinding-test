use core::ffi::c_void;
use gimli::Register;
use spin::Mutex;
use unwinding::abi::{
    _Unwind_Backtrace, _Unwind_FindEnclosingFunction, _Unwind_GetGR, _Unwind_GetIP, UnwindContext,
    UnwindReasonCode,
};
pub use unwinding::panic::{begin_panic, catch_unwind};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe extern "Rust" {
        pub fn __panic_handler(info: &core::panic::PanicInfo) -> !;
    }
    unsafe { __panic_handler(info) }
}

#[unsafe(no_mangle)]
pub fn __panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {:?}", info);
    print_stack_trace();
    loop {}
}

pub fn print_stack_trace() {
    /// We acquire a global lock to prevent the frames in the stack trace from
    /// interleaving. The spin lock is used merely for its simplicity.
    static BACKTRACE_PRINT_LOCK: Mutex<()> = Mutex::new(());
    let _lock = BACKTRACE_PRINT_LOCK.lock();

    println!("Printing stack trace:");

    struct CallbackData {
        counter: usize,
    }
    extern "C" fn callback(unwind_ctx: &UnwindContext<'_>, arg: *mut c_void) -> UnwindReasonCode {
        let data = unsafe { &mut *(arg as *mut CallbackData) };
        data.counter += 1;
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
