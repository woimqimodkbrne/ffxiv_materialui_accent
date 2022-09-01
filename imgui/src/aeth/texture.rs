#[repr(C)]
pub struct TextureOptions {
	pub width: i32,
	pub height: i32,
	pub format: i32,
	pub usage: i32,
	pub cpu_access_flags: i32,
}

pub static mut CREATE: fn(TextureOptions) -> usize = |_| {0};
pub static mut CREATEDATA: fn(TextureOptions, Vec<u8>) -> usize = |_, _| {0};
pub static mut DROP: fn(usize) = |_| {};
pub static mut PIN: fn(usize) -> *mut u8 = |_| {0 as *mut u8};
pub static mut UNPIN: fn(usize) = |_| {};

pub struct Texture {
	pub width: usize,
	pub height: usize,
	ptr: usize,
}

impl Texture {
	pub fn new(options: TextureOptions) -> Self {
		Texture {
			width: options.width as usize,
			height: options.height as usize,
			ptr: unsafe{CREATE(options)},
		}
	}
	
	pub fn empty() -> Self {
		Texture {
			width: 0,
			height: 0,
			ptr: 0,
		}
	}
	
	pub fn with_data(options: TextureOptions, data: &[u8]) -> Self {
		Texture {
			width: options.width as usize,
			height: options.height as usize,
			ptr: unsafe{CREATEDATA(options, data.to_vec())},
		}
	}
	
	pub fn draw_to(&mut self, data: &[u8]) -> Result<(), &'static str> {
		if data.len() != self.width * self.height * 4 {
			return Err("Invalid data size for texture dimensions");
		}
		
		unsafe {
			let dataptr = PIN(self.ptr);
			for (i, v) in data.into_iter().enumerate() {
				*dataptr.offset(i as isize) = *v;
			}
			UNPIN(self.ptr);
		}
		
		Ok(())
	}
	
	pub fn resource(&self) -> usize {
		self.ptr
	}
}

impl Drop for Texture {
	fn drop(&mut self) {
		if self.ptr != 0 {
			unsafe{DROP(self.ptr)};
		}
	}
}