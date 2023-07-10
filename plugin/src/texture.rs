#![allow(non_upper_case_globals)]
#![allow(dead_code)]

// TODO: convert str errors to enum type errors

#[repr(C)]
pub struct TextureOptions {
	pub width: i32,
	pub height: i32,
	pub format: TextureFormat,
	pub usage: TextureUsage,
	pub cpu_access_flags: TextureCpuFlags,
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum TextureFormat {
	Unknown = 0,
	A8Unorm = 65,
	R8g8b8a8Unorm = 28,
	B8g8r8a8Unorm = 87,
}

impl TextureFormat {
	pub fn byte_count(&self) -> usize {
		match self {
			TextureFormat::Unknown => 0,
			TextureFormat::A8Unorm => 1,
			TextureFormat::R8g8b8a8Unorm => 4,
			TextureFormat::B8g8r8a8Unorm => 4,
		}
	}
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum TextureUsage {
	Default = 0,
	Immutable = 1,
	Dynamic = 2,
	Staging = 3,
}

bitflags::bitflags!{
	#[derive(Debug, Copy, Clone)]
	pub struct TextureCpuFlags: i32 {
		const None = 0;
		const Write = 0x10000;
		const Read = 0x20000;
	}
}

pub static mut CREATE: fn(TextureOptions) -> usize = |_| {0};
pub static mut DROP: fn(usize) = |_| {};
pub static mut PIN: fn(usize) -> *mut u8 = |_| {0 as *mut u8};
pub static mut UNPIN: fn(usize) = |_| {};

#[derive(Debug)]
pub struct Texture {
	pub width: usize,
	pub height: usize,
	format: TextureFormat,
	usage: TextureUsage,
	cpu_access_flags: TextureCpuFlags,
	ptr: usize,
}

impl Texture {
	pub fn new(options: TextureOptions) -> Self {
		Texture {
			width: options.width as usize,
			height: options.height as usize,
			format: options.format,
			usage: options.usage,
			cpu_access_flags: options.cpu_access_flags,
			ptr: unsafe{CREATE(options)},
		}
	}
	
	pub const fn empty() -> Self {
		Texture {
			width: 0,
			height: 0,
			format: TextureFormat::Unknown,
			usage: TextureUsage::Default,
			cpu_access_flags: TextureCpuFlags::None,
			ptr: 0,
		}
	}
	
	pub fn draw_to(&mut self, data: &[u8]) -> Result<(), &'static str> {
		if data.len() != self.width * self.height * self.format.byte_count() {
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
	
	pub fn draw_to_section(&mut self, x: usize, y: usize, w: usize, h: usize, data: &[u8]) -> Result<(), &'static str> {
		let byte_count = self.format.byte_count();
		let bytes_per_line = w * byte_count;
		
		if data.len() != w * h * byte_count {
			return Err("Invalid data size for texture dimensions");
		} else if x + w > self.width || y + h > self.height {
			return Err("Section extends beyond texture borders");
		}
		
		unsafe {
			let dataptr = PIN(self.ptr);
			for (i, v) in data.into_iter().enumerate() {
				*dataptr.offset((y * bytes_per_line + x * byte_count + i % bytes_per_line + i / bytes_per_line * (self.width * byte_count)) as isize) = *v;
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