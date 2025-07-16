use embedded_alloc::LlffHeap as Heap;
use htif::println;

use spin::Once;


#[cfg_attr(feature = "no-jolt", global_allocator)]
static HEAP: Heap = Heap::empty();

// Allocate heap storage once, safely
static HEAP_MEM: Once<&'static mut [u8]> = Once::new();

pub fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024 * 1024 * 10; // 100MB instead of 50MB

    let heap = HEAP_MEM.call_once(|| {
        // Allocate a static uninitialized buffer
        static mut RAW_MEM: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

        unsafe {
            let ptr = RAW_MEM.as_mut_ptr() as *mut u8;
            let slice = core::slice::from_raw_parts_mut(ptr, HEAP_SIZE);
            slice
        }
    });

    unsafe {
        HEAP.init(heap.as_ptr() as usize, heap.len());
    }
}

pub fn exit(code: u32) -> ! {
    htif::exit(code)
}

fn panic_dump(info: &core::panic::PanicInfo) -> ! {
    let message = info.message();

    if let Some(location) = info.location() {
        println!(
            "PANIC: {} at {}:{}:{}",
            message,
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        htif::println!("PANIC: {}", message);
    }

    exit(1);
    loop {
        core::hint::spin_loop();
    }
}

#[cfg_attr(feature = "no-jolt", panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    panic_dump(_info);

    exit(1);
    loop {
        core::hint::spin_loop();
    }
}

// Default entry point for bare-metal RISC-V programs
// Users should implement their own main() function that returns i32
#[cfg(feature = "no-jolt")]
#[no_mangle]
pub extern "C" fn start() -> ! {
    // Initialize the heap
    init_heap();
    htif::read_fromhost();
    // Call the user's main function and exit with its return value
    extern "C" {
        fn main() -> i32;
    }
    let exit_code = unsafe { main() };
    exit(exit_code as u32)
}

#[cfg(not(test))]
use core::arch::global_asm;

#[cfg(not(test))]
global_asm!(
    r#"
    .section .text.boot
    .globl _start
_start:
    # Load stack pointer from linker symbol
    la sp, _STACK_PTR
    # Call Rust entry point
    call start
    # Should never return, but just in case
1:
    j 1b
    "#
);
