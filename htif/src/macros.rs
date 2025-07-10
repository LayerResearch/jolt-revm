// Print macros for HTIF console output
#[macro_export]
macro_rules! println {
    // Empty println!()
    () => {
        $crate::putchar(b'\n');
    };

    // String literals - no allocation needed
    ($lit:literal) => {
        {
            for byte in $lit.bytes() {
                $crate::putchar(byte);
            }
            $crate::putchar(b'\n');
        }
    };

    // Everything else - use format! just like std::println!
    ($($arg:tt)*) => {
        {
            extern crate alloc;
            let formatted = alloc::format!($($arg)*);
            for byte in formatted.bytes() {
                $crate::putchar(byte);
            }
            $crate::putchar(b'\n');
        }
    };
}

#[macro_export]
macro_rules! eprintln {
    // Empty eprintln!()
    () => {
        $crate::putchar(b'\n');
    };

    // String literals - no allocation needed
    ($lit:literal) => {
        {
            for byte in $lit.bytes() {
                $crate::putchar(byte);
            }
            $crate::putchar(b'\n');
        }
    };

    // Everything else - use format! just like std::eprintln!
    ($($arg:tt)*) => {
        {
            extern crate alloc;
            let formatted = alloc::format!($($arg)*);
            for byte in formatted.bytes() {
                $crate::putchar(byte);
            }
            $crate::putchar(b'\n');
        }
    };
}
