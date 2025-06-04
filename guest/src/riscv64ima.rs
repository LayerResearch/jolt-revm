#![no_std]

use embedded_alloc::LlffHeap as Heap;
use htif::{htif_exit, htif_receive};

#[cfg_attr(feature = "no-jolt", global_allocator)]
static HEAP: Heap = Heap::empty();

// Initialize the heap
pub fn init_heap() {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 1024 * 1; // 1MB
        static  HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
}

pub fn exit(code: u32) -> ! {
    htif_exit(code)
}

#[cfg_attr(feature = "no-jolt", panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1);
}

// Default entry point for bare-metal RISC-V programs
// Users should implement their own main() function that returns i32
#[cfg(feature = "no-jolt")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the heap
    init_heap();
    htif_receive();
    // Call the user's main function and exit with its return value
    extern "C" {
        fn main() -> i32;
    }
    let exit_code = unsafe { main() };
    exit(exit_code as u32)
}
