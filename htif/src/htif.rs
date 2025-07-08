#[link_section = ".tohost"]
#[used]
#[no_mangle]
pub static mut tohost: u64 = 0;

#[link_section = ".fromhost"]
#[used]
#[no_mangle]
pub static mut fromhost: u64 = 0;

const HTIF_DEVICE_HALT: u64 = 0x0000; // Halt device
const HTIF_DEVICE_CONSOLE: u64 = 0x0001; // Console output device
const HTIF_DEVICE_SYSCALL: u64 = 0x0002; // Syscall device

const HTIF_CMD_HALT: u64 = 0; // Command for HALT
const HTIF_CMD_GETCHAR: u64 = 0; // Command for GETCHAR
const HTIF_CMD_PUTCHAR: u64 = 1; // Command for PUTCHAR
const HTIF_CMD_SYSCALL: u64 = 0; // Command for syscall (if used)

#[inline(always)]
fn htif_send(device: u64, cmd: u64, payload: u64) {
    unsafe {
        core::ptr::write_volatile(
            &raw mut tohost,
            ((device & 0xFF) << 56) | ((cmd & 0xFF) << 48) | (payload & 0xFFFF_FFFF_FFFF),
        );
    }
}

/// Read a response from the host
#[inline(always)]
pub fn htif_receive() -> u64 {
    unsafe { core::ptr::read_volatile(&raw const fromhost) }
}

/// Exit to host with the given code (0 = success)
pub fn htif_exit(code: u32) -> ! {
    htif_send(HTIF_DEVICE_HALT, HTIF_CMD_HALT, ((code << 1) | 1) as u64);
    loop {
        core::hint::spin_loop();
    }
}

/// Write a single byte to the HTIF console
pub fn htif_console_putchar(ch: u8) {
    htif_send(HTIF_DEVICE_CONSOLE, HTIF_CMD_PUTCHAR, ch as u64);
    // Wait for the host to process the command (tohost will be cleared)
    htif_wait_for_completion();
}


fn htif_wait_for_completion() {
    unsafe {
        // Wait until tohost is cleared by the host
        while core::ptr::read_volatile(&raw const tohost) != 0 {
            // Busy wait or yield if in a cooperative environment
            core::hint::spin_loop();
        }
    }
}

/// Perform a syscall with the given payload
/// (the payload layout must match expected syscall ABI)
pub fn htif_syscall(payload: u64) {
    htif_send(HTIF_DEVICE_SYSCALL, HTIF_CMD_SYSCALL, payload);
}

/// Write directly to tohost (for test failure signaling)
pub fn htif_write_tohost(value: u64) {
    unsafe {
        core::ptr::write_volatile(&raw mut tohost, value);
    }
}
