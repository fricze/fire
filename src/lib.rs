use std::ffi::{CString, c_char, c_double};

// Rust function: Adds two numbers
#[unsafe(no_mangle)]
pub extern "C" fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

#[repr(C)] // Ensures compatibility with C-style memory layout
pub struct CharVec {
    str: *mut c_char,
    byte_len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn give_string() -> CharVec {
    let rust_string = "Hello from Rust!".to_string();
    let c_string =
        CString::new(rust_string).expect("CString::new failed: string contains null bytes");

    let byte_len = c_string.count_bytes();
    // .into_raw() consumes the CString and returns the raw pointer.
    // The memory is now "leaked" from Rust's perspective and owned by the caller (Java).

    CharVec {
        str: c_string.into_raw(),
        byte_len,
    }
}
