// used for wgpu in the past (i think?) keeping it here for possible future usage

use windows::{
	s,
	Win32::{
		Foundation::{HWND, HINSTANCE},
		UI::WindowsAndMessaging::{FindWindowExA, GetWindowLongPtrA, GWL_HINSTANCE}
	},
};

pub struct HWNDHandle {
	window_handle: raw_window_handle::RawWindowHandle,
	display_handle: raw_window_handle::RawDisplayHandle,
}

unsafe impl raw_window_handle::HasRawWindowHandle for HWNDHandle {
	fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
		self.window_handle
	}
}

unsafe impl raw_window_handle::HasRawDisplayHandle for HWNDHandle {
	fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
		self.display_handle
	}
}

unsafe fn get_worker() -> HWND {
	FindWindowExA(None, HWND(0), s!("FFXIVGAME"), None)
}

unsafe fn get_hinstance(hwnd: HWND) -> HINSTANCE {
	HINSTANCE(GetWindowLongPtrA(hwnd, GWL_HINSTANCE))
}

impl HWNDHandle {
	pub fn get_handle() -> Self {
		unsafe {
			let hwnd = get_worker();
			
			Self {
				window_handle: raw_window_handle::RawWindowHandle::Win32({
					let mut handle = raw_window_handle::Win32WindowHandle::empty();
					handle.hwnd = hwnd.0 as _;
					handle.hinstance = get_hinstance(hwnd).0 as _;
					handle
				}),
				display_handle: raw_window_handle::RawDisplayHandle::Windows(raw_window_handle::WindowsDisplayHandle::empty())
			}
		}
	}
}