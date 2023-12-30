//! This file contains library functions for playing with FFI

use libc::c_char;
use std::ffi::{c_void, CStr, CString};
use std::ptr;

/// This is a macro which lets you more conveniently convert C strings to Rust Strings.
macro_rules! cstr_to_string {
    ($a:ident) => {
        if $a.is_null() {
            None
        } else {
            let __in_cstr: &CStr = unsafe { CStr::from_ptr($a) };
            match __in_cstr.to_str() {
                Ok(s) => Some(String::from(s)),
                Err(e) => panic!("ACK! Can't convert to string: {e:?}"),
            }
        }
    };
}

/// Add two numbers

fn rs_add_two(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
pub extern "C" fn add_two(a: u32, b: u32) -> u32 {
    rs_add_two(a, b)
}

/// Upper-case a string

fn rs_upper_string(input: String) -> String {
    input.to_uppercase()
}

#[no_mangle]
pub extern "C" fn upper_string(ffi_input: *const c_char) -> *mut c_char {
    if let Some(rs_string) = cstr_to_string!(ffi_input) {
        eprintln!("RS: Got a Rust string: «{rs_string}»");

        let output = rs_upper_string(rs_string);

        eprintln!("RS: It's upper-case now: «{output}»");

        let res = match CString::new(output) {
            Ok(cs) => cs,
            Err(e) => panic!("Failed to return the String to CString format: {:#?}", e),
        };

        res.into_raw()
    } else {
        ptr::null_mut()
    }
}

/// Demonstration of returning an opaque pointer

#[derive(Debug)]
pub struct OpaqueKeyValue {
    pub key: String,
    pub value: Option<String>,
}

pub type COpaqueKeyValue = c_void;

#[no_mangle]
pub extern "C" fn make_kvp(in_key: *const c_char, in_value: *const c_char) -> *mut COpaqueKeyValue {
    if in_key.is_null() {
        return ptr::null_mut();
    }

    Box::into_raw(Box::new(OpaqueKeyValue {
        key: cstr_to_string!(in_key).unwrap(),
        value: cstr_to_string!(in_value),
    })) as *mut COpaqueKeyValue
}

#[no_mangle]
pub extern "C" fn print_kvp(in_kvp: *mut COpaqueKeyValue) {
    let kvp = unsafe { &*(in_kvp as *mut OpaqueKeyValue) };

    println!("RS: KVP is: {kvp:#?}");
}

// Doing the async bits
// async fn wait
