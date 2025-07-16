#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
#![feature(alloc_error_handler)]

extern crate alloc;

use buddy_system_allocator::LockedHeap;
use loongArch64::register::*;
use test::{test_panic, test_vtimer};

#[macro_use]
mod print;
mod panic;
mod test;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::new();

use print::uart_init;

const DMW_DA_BITS: usize = 48;
const CSR_DMW0_PLV0: usize = 1 << 0;
const CSR_DMW0_VSEG: usize = 0x8000;
const CSR_DMW0_BASE: usize = CSR_DMW0_VSEG << DMW_DA_BITS;
const CSR_DMW0_INIT: usize = CSR_DMW0_BASE | CSR_DMW0_PLV0;
const CSR_DMW1_PLV0: usize = 1 << 0;
const CSR_DMW1_MAT: usize = 1 << 4;
const CSR_DMW1_VSEG: usize = 0x9000;
const CSR_DMW1_BASE: usize = CSR_DMW1_VSEG << DMW_DA_BITS;
const CSR_DMW1_INIT: usize = CSR_DMW1_BASE | CSR_DMW1_PLV0 | CSR_DMW1_MAT;

#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text.start")]
pub extern "C" fn _start() -> ! {
    #[allow(unused_unsafe)]
    unsafe {
        core::arch::naked_asm!(
            "
                li.d        $r12, {CSR_DMW0_INIT} // 0x8
                csrwr       $r12, {LOONGARCH_CSR_DMW0}
                li.d        $r12, {CSR_DMW1_INIT} // 0x9
                csrwr       $r12, {LOONGARCH_CSR_DMW1}
    
                // first JUMP_VIRT_ADDR
                li.d        $r12, {CSR_DMW1_BASE}
                pcaddi      $r13, 0
                or          $r12, $r12, $r13
                jirl        $zero, $r12, 0xc
                // end of JUMP_VIRT_ADDR
    
                li.w		$r12, 0xb0		    // PLV=0, IE=0, PG=1
                csrwr		$r12, {LOONGARCH_CSR_CRMD}
                li.w		$r12, 0x04		    // PLV=0, PIE=1, PWE=0
                csrwr		$r12, {LOONGARCH_CSR_PRMD}
                li.w		$r12, 0x00		    // FPE=0, SXE=0, ASXE=0, BTE=0
                csrwr		$r12, {LOONGARCH_CSR_EUEN}
    
                csrrd       $r4, {CSR_CPUID}
                la.pcrel    $r12, __stack_end
                move        $r3, $r12
                ibar        0
                dbar        0
                bl          {rust_main}
            ",
            CSR_DMW0_INIT = const CSR_DMW0_INIT,
            CSR_DMW1_INIT = const CSR_DMW1_INIT,
            LOONGARCH_CSR_CRMD = const 0x0,
            LOONGARCH_CSR_PRMD = const 0x1,
            LOONGARCH_CSR_EUEN = const 0x2,
            LOONGARCH_CSR_DMW0 = const 0x180,
            LOONGARCH_CSR_DMW1 = const 0x181,
            CSR_DMW1_BASE = const 0x9000000000000000usize,
            rust_main = sym crate::rust_main,
            CSR_CPUID = const 0x20,
        );
    }
}

const HV_HEAP_SIZE: usize = 4096;

fn heap_init() {
    const MACHINE_ALIGN: usize = core::mem::size_of::<usize>();
    const HEAP_BLOCK: usize = HV_HEAP_SIZE / MACHINE_ALIGN;
    static mut HEAP: [usize; HEAP_BLOCK] = [0; HEAP_BLOCK];
    #[allow(static_mut_refs)]
    let heap_start = {
        let heap_ptr = unsafe { HEAP.as_ptr() };
        heap_ptr as usize
    };
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(heap_start, HEAP_BLOCK * MACHINE_ALIGN);
    }
    println!(
        "heap allocator initialization finished: {:#x?}",
        heap_start..heap_start + HV_HEAP_SIZE
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let busy_loop_wait = 1e7 as usize;
    let mut x = 0;
    for i in 0..busy_loop_wait {
        x = x + 1;
    }
    uart_init();
    heap_init();
    println!("hello world! {}", 42);
    // test_panic();

    // set global trap handler
    set_global_trap_handler(trap_handler);

    test_vtimer();
    println!("going to idle loop");
    loop {
        // Idle loop
    }
}

#[unsafe(link_section = ".text.eentry")]
#[unsafe(no_mangle)]
pub extern "C" fn trap_handler() -> ! {
    let ecode = estat::read().ecode();
    let is = estat::read().is();
    println!("trap handler: ecode: {:#x}, is: {:#x}", ecode, is);
    ticlr::clear_timer_interrupt();
    crmd::set_ie(true);
    loop {
        // Idle loop
    }
}

fn set_global_trap_handler(handler: extern "C" fn() -> !) {
    ecfg::set_vs(0);
    ecfg::set_lie(ecfg::LineBasedInterrupt::TIMER);
    eentry::set_eentry(handler as usize);
    crmd::set_ie(true);
}
