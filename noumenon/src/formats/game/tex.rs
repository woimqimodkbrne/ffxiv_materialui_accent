#![allow(dead_code)]

use std::io::{Cursor, Read, Seek, Write, SeekFrom};
use binrw::{BinRead, BinReaderExt, BinWrite, binrw};
use ironworks::{file::File, Error};
use crate::formats::external::dds::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Pixel {
	b: u8,
	g: u8,
	r: u8,
	a: u8,
}

#[binrw]
#[brw(little, repr = u32)]
#[repr(u32)]
pub enum Format {
	Unknown = 0x0,
	
	L8 = 0x1130,
	A8 = 0x1131,
	
	Argb4 = 0x1440,
	// Rgba4 = 0x1440,
	A1rgb5 = 0x1441,
	// Rgb5a1 = 0x1441,
	Argb8 = 0x1450,
	Xrgb8 = 0x1451,
	// Rgbx8 = 0x1451,
	Argb82 = 0x1452,
	
	R32F = 0x2150,
	Rg16F = 0x2250,
	Rgba16F = 0x2460,
	Rgba32F = 0x2470,
	
	Dxt1 = 0x3420,
	Dxt3 = 0x3430,
	Dxt5 = 0x3431,
	
	D16 = 0x4140,
	D24S8 = 0x4250,
	Rgba8 = 0x4401,
	
	Null = 0x5100,
	Shadow16 = 0x5140,
	Shadow24 = 0x5150,
}

#[binrw]
#[brw(little)]
pub struct Header {
	flags: u32,
	format: Format,
	width: u16,
	height: u16,
	depths: u16,
	mip_levels: u16,
	lod_offsets: [u32; 3],
	mip_offsets: [u32; 13],
}

pub struct Tex {
	header: Header,
	data: Vec<u8>,
}

// used to load from spack using ironworks
impl File for Tex {
	fn read(data: Vec<u8>) -> Result<Self> {
		Ok(Tex::read(&mut Cursor::new(&data)))
	}
}

impl Tex {
	pub fn as_pixels(&self) -> &[Pixel] {
		unsafe { ::std::slice::from_raw_parts(self.data[0] as *const _, self.data.len() / 4) }
	}
	
	pub fn as_pixels_mut(&mut self) -> &mut [Pixel] {
		unsafe { ::std::slice::from_raw_parts_mut(self.data[0] as *mut _, self.data.len() / 4) }
	}
	
	fn read<T>(reader: &mut T) -> Self where T: Read + Seek {
		// unwrap cuz ? doesn't seem to like it and cba figuring out why or using match
		let header = <Header as BinRead>::read(reader).unwrap();
		
		reader.seek(SeekFrom::End(0)).unwrap();
		let mut data = Vec::with_capacity(reader.stream_position().unwrap() as usize);
		reader.seek(SeekFrom::Start(80)).unwrap();
		reader.read_to_end(&mut data).unwrap();
		
		Tex {
			data: match header.format {
				Format::L8     => convert_from_l8(&data),
				Format::A8     => convert_from_a8(&data),
				Format::Argb4  => convert_from_a4r4g4b4(&data),
				Format::A1rgb5 => convert_from_a1r5g5b5(&data),
				Format::Argb8  => data,
				Format::Xrgb8  => convert_from_x8r8g8b8(&data),
				_ => Vec::new(), // TODO: return error instead
			},
			header
		}
	}
	
	fn write<T>(&self, writer: &mut T) where T: Write + Seek {
		// todo!();
		self.header.write_to(writer).unwrap();
		match self.header.format {
			Format::L8     => writer.write_all(&convert_to_l8(&self.data)),
			Format::A8     => writer.write_all(&convert_to_a8(&self.data)),
			Format::Argb4  => writer.write_all(&convert_to_a4r4g4b4(&self.data)),
			Format::A1rgb5 => writer.write_all(&convert_to_a1r5g5b5(&self.data)),
			Format::Argb8  => writer.write_all(&self.data),
			Format::Xrgb8  => writer.write_all(&convert_to_x8r8g8b8(&self.data)),
			_ => return,
		}.unwrap();
	}
}

impl Dds for Tex {
	fn read<T>(reader: &mut T) -> Self where T: Read + Seek {
		// TODO: dont unwrap, return a result
		reader.seek(SeekFrom::Start(12)).unwrap();
		let height = reader.read_le::<u32>().unwrap() as u16;
		let width = reader.read_le::<u32>().unwrap() as u16;
		reader.seek(SeekFrom::Current(4)).unwrap();
		let depths = reader.read_le::<u32>().unwrap() as u16;
		let mip_levels = reader.read_le::<u32>().unwrap() as u16;
		reader.seek(SeekFrom::Start(84)).unwrap();
		let cc: u32 = reader.read_le().unwrap();
		reader.seek(SeekFrom::Start(92)).unwrap();
		let rmask: u32 = reader.read_le().unwrap();
		reader.seek(SeekFrom::Current(8)).unwrap();
		let amask: u32 = reader.read_le().unwrap();
		
		let format = match (cc, rmask, amask) { // eh, good enough
			// (0x33545844, 0,          0         ) => Format::Dxt3,
			// (0x31545844, 0,          0         ) => Format::Dxt1,
			// (0x35545844, 0,          0         ) => Format::Dxt3,
			// (113,        0,          0         ) => Format::Rgba16F,
			(0,          0xFF,       0         ) => Format::L8,
			(0,          0,          0xFF      ) => Format::A8,
			(0,          0x0F00,     0xF000    ) => Format::Argb4,
			(0,          0x7C00,     0x8000    ) => Format::A1rgb5,
			(0,          0x00FF0000, 0xFF000000) => Format::Argb8,
			(0,          0x00FF0000, 0         ) => Format::Xrgb8,
			_ => Format::Unknown,
		};
		
		// im sure theres some fancier way but w/e
		let mut mip_offsets = [0u32; 13];
		for i in 0..13 {
			mip_offsets[i] = if (i as u16) < mip_levels {
				80 + ((width as u32 * height as u32 * 4) as f32 * (0.25f32.powi(i as i32))) as u32 
			} else {
				0
			}
		}
		
		reader.seek(SeekFrom::End(0)).unwrap();
		let mut data = Vec::with_capacity(reader.stream_position().unwrap() as usize);
		reader.seek(SeekFrom::Start(128)).unwrap();
		reader.read_to_end(&mut data).unwrap();
		
		Tex {
			data: match format {
				Format::L8     => convert_from_l8(&data),
				Format::A8     => convert_from_a8(&data),
				Format::Argb4  => convert_from_a4r4g4b4(&data),
				Format::A1rgb5 => convert_from_a1r5g5b5(&data),
				Format::Argb8  => data,
				Format::Xrgb8  => convert_from_x8r8g8b8(&data),
				_ => Vec::new(), // TODO: return error instead
			},
			header: Header {
				flags: 0x00800000, // TODO: care about other stuff like 3d textures
				format,
				width,
				height,
				depths,
				mip_levels,
				lod_offsets: [0u32, 1u32, 2u32],
				mip_offsets,
			}
		}
	}
	
	fn write<T>(&self, _writer: &mut T) where T: Write + Seek {
		todo!();
	}
}