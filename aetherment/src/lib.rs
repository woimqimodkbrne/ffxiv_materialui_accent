#![allow(improper_ctypes_definitions)]

#[no_mangle]
pub extern fn free_object(s: *mut ()) {
	unsafe { Box::from_raw(s); }
}

#[no_mangle]
pub extern fn cool_test(s: &str) -> *mut String {
	Box::into_raw(Box::new(format!("cool str! {}, this was send from rust", s)))
}