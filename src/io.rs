//! lib sys print
//!

use core::result;
use win_kernel_sys::base::ANSI_STRING;
use win_kernel_sys::ntoskrnl::DbgPrint;

use crate::Error;

/// like print!
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

/// like println!
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
    
}

/// Format the string using the `alloc::format!` as this is guaranteed to return a `String`
/// instead of a `Result` that we would have to `unwrap`. This ensures that this code stays
/// panic-free.
/// Print the string. We must make sure to not pass this user-supplied string as the format
/// string, as `DbgPrint` may then format any format specifiers it contains. This could
/// potentially be an attack vector.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {

    let s = alloc::format!("{}", args);

    let s = ANSI_STRING {
        Length: s.len() as u16,
        MaximumLength: s.len() as u16,
        Buffer: s.as_ptr() as _,
    };

    unsafe { DbgPrint("%Z\0".as_ptr() as _, &s) };
}


/// like windows printW
#[macro_export]
macro_rules! printw {
    ($($arg:tt)*) => ($crate::io::_printw(format_args!($($arg)*)));
}

/// like windows printlnW
#[macro_export]
macro_rules! printlnw {
    () => ($crate::printw!("\n"));
    ($($arg:tt)*) => ($crate::printw!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _printw(args: core::fmt::Arguments) {
    // Format the string using the `alloc::format!` as this is guaranteed to return a `String`
    // instead of a `Result` that we would have to `unwrap`. This ensures that this code stays
    // panic-free.
    let s = alloc::format!("{}", args);

    // Print the string. We must make sure to not pass this user-supplied string as the format
    // string, as `DbgPrint` may then format any format specifiers it contains. This could
    // potentially be an attack vector.
    let s = crate::string::create_unicode_from_str(&s);
    unsafe { DbgPrint("%wZ\0".as_ptr() as _, &s) };
}

/// the mod [`Result<T>`]
pub type Result<T> = result::Result<T, Error>;

/// if os == esp 512 else 8168
pub const DEFAULT_BUF_SIZE: usize = if cfg!(target_os = "espidf") {
    512
} else {
    8 * 1024
};
