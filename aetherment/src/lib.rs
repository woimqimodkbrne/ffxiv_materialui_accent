#![allow(improper_ctypes_definitions)]

#[no_mangle]
pub extern fn free_object(s: *mut ()) {
	unsafe { Box::from_raw(s); }
}

#[macro_export]
macro_rules! rtn {
	($e:expr) => {
		Box::into_raw(Box::new($e))
	};
}

#[no_mangle]
pub extern fn cool_test(s: &str) -> *mut String {
	rtn!(format!("cool str! {}, this was send from rust", s))
}

#[no_mangle]
pub extern fn cool_test2(s: &[&str]) -> *mut Vec<String> {
	rtn!(s.into_iter().map(|e| (e.parse::<i32>().unwrap() * -2).to_string()).collect())
}