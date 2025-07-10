#[link_section = ".tohost"]
#[used]
#[no_mangle]
pub static mut tohost: u64 = 0;

#[link_section = ".fromhost"]
#[used]
#[no_mangle]
pub static mut fromhost: u64 = 0;

const HTIF_DEVICE_SYSCALL: u64 = 0x0000; // Syscall device
const HTIF_DEVICE_CONSOLE: u64 = 0x0001; // Console output device

const HTIF_CMD_SYSCALL: u64 = 0; // Command for SYSCALL
const HTIF_CMD_GETCHAR: u64 = 0; // Command for GETCHAR
const HTIF_CMD_PUTCHAR: u64 = 1; // Command for PUTCHAR

#[inline(always)]
fn write_tohost(device: u64, cmd: u64, payload: u64) {
    unsafe {
        core::ptr::write_volatile(
            &raw mut tohost,
            ((device & 0xFF) << 56) | ((cmd & 0xFF) << 48) | (payload & 0xFFFF_FFFF_FFFF),
        );
    }
}

/// Read a response from the host
#[inline(always)]
pub fn read_fromhost() -> u64 {
    unsafe { core::ptr::read_volatile(&raw const fromhost) }
}

/// Exit to host with the given code (0 = success)
pub fn exit(code: u32) -> ! {
    write_tohost(
        HTIF_DEVICE_SYSCALL,
        HTIF_CMD_SYSCALL,
        ((code << 1) | 1) as u64,
    );
    loop {
        core::hint::spin_loop();
    }
}

fn wait_tohost_ack() {
    unsafe {
        // Wait until tohost is cleared by the host
        while core::ptr::read_volatile(&raw const tohost) != 0 {
            // Busy wait or yield if in a cooperative environment
            core::hint::spin_loop();
        }
    }
}

/// Write a single byte to the HTIF console
pub fn putchar(ch: u8) {
    write_tohost(HTIF_DEVICE_CONSOLE, HTIF_CMD_PUTCHAR, ch as u64);
    // Wait for the host to process the command (tohost will be cleared)
    wait_tohost_ack();
}

pub fn getchar() -> u8 {
    write_tohost(HTIF_DEVICE_CONSOLE, HTIF_CMD_GETCHAR, 0);
    wait_tohost_ack();

    let response = read_fromhost();

    loop {
        // Check if valid character received (implementation dependent)
        if response & 0x100 != 0 {
            // Example: bit 8 indicates valid char
            return (response & 0xFF) as u8;
        }
        core::hint::spin_loop();
    }
}

/// Perform a syscall with the given payload
/// (the payload layout must match expected syscall ABI)
pub fn syscall(payload: u64) {
    // FIXME:
    write_tohost(HTIF_DEVICE_SYSCALL, HTIF_CMD_SYSCALL, payload);
}
