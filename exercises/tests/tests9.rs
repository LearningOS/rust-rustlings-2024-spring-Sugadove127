// tests9.rs



#[no_mangle]
pub extern "Rust" fn my_demo_function(a: u32) -> u32 {
    foo::my_demo_function(a)
}

#[no_mangle]
pub extern "Rust" fn my_demo_function_alias(a: u32) -> u32 {
    foo::my_demo_function(a)
}

mod foo {
    // No `extern` equals `extern "Rust"`.
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}

