#[link_section = ".tohost"]
#[used]
#[no_mangle]
pub static mut TOHOST: u64 = 0;

#[link_section = ".fromhost"]
#[used]
#[no_mangle]
pub static mut FROMHOST: u64 = 0;

const HTIF_DEV_CONSOLE: u64 = 1;
const HTIF_DEV_SYSCALL: u64 = 2;
const HTIF_DEV_EXIT: u64 = 0xFFFF;

const HTIF_CMD_WRITE: u64 = 1;
const HTIF_CMD_EXIT: u64 = 0;
const HTIF_CMD_SYSCALL: u64 = 0;

#[inline(always)]
fn htif_send(device: u64, cmd: u64, payload: u64) {
    unsafe {
        core::ptr::write_volatile(&raw mut TOHOST, ((device & 0xFFFF) << 48) | ((cmd & 0xFFFF) << 32) | (payload & 0xFFFF_FFFF));
    }
}

/// Exit to host with the given code (0 = success)
pub fn htif_exit(code: u32) -> ! {
    htif_send(HTIF_DEV_EXIT, HTIF_CMD_EXIT, code as u64);
    loop {
        // Avoid compiler optimizing away the exit loop
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

/// Write a single byte to the HTIF console
pub fn htif_console_putchar(ch: u8) {
    htif_send(HTIF_DEV_CONSOLE, HTIF_CMD_WRITE, ch as u64);
}

/// Perform a syscall with the given payload
/// (the payload layout must match expected syscall ABI)
pub fn htif_syscall(payload: u64) {
    htif_send(HTIF_DEV_SYSCALL, HTIF_CMD_SYSCALL, payload);
}
