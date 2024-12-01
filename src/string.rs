//! string mod

use alloc::string::String;

use widestring::U16CString;

use win_kernel_sys::base::UNICODE_STRING;


/// create a [UNICODE_STRING] from a `&[u16]`
pub fn create_unicode_string(s: &[u16]) -> UNICODE_STRING {
    let len = s.len();

    let n = if len > 0 && s[len - 1] == 0 {
        len - 1
    } else {
        len
    };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: s.as_ptr() as _,
    }
}


/// create a [UNICODE_STRING] from a `String`
pub fn create_unicode_from_str(name: &str) -> UNICODE_STRING {
    let name = U16CString::from_str(name).unwrap();
    create_unicode_string(name.as_slice())
}

/// string from a unicode string
pub fn from_unicode_string(s: &UNICODE_STRING) -> String {
    unsafe { U16CString::from_ptr_str(s.Buffer).to_string_lossy() }
}
