#![allow(dead_code)]

use std::io::{Cursor, Read, Seek, Write, SeekFrom};
use binrw::{BinRead, BinReaderExt, BinWrite, binrw};
use image::{codecs::png::PngEncoder, ImageEncoder, ColorType};
use ironworks::{file::File, Error};
use crate::formats::external::{dds::{Dds, Format as DFormat}, png::Png};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(C)]
pub struct Pixel {
	pub b: u8,
	pub g: u8,
	pub r: u8,
	pub a: u8,
}

#[binrw]
#[brw(little, repr = u32)]
#[repr(u32)]
#[derive(Copy, Clone)]
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
	Argb16 = 0x2460,
	// Rgba16F = 0x2460,
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

impl From<Format> for DFormat {
	fn from(format: Format) -> Self {
		match format {
			Format::L8     => DFormat::L8,
			Format::A8     => DFormat::A8,
			Format::Argb4  => DFormat::A4R4G4B4,
			Format::A1rgb5 => DFormat::A1R5G5B5,
			Format::Argb8  => DFormat::A8R8G8B8,
			Format::Xrgb8  => DFormat::X8R8G8B8,
			Format::Dxt1   => DFormat::Dxt1,
			Format::Dxt3   => DFormat::Dxt3,
			Format::Dxt5   => DFormat::Dxt5,
			Format::Argb16 => DFormat::A16B16G16R16,
			_              => DFormat::Unknown,
		}
	}
}

impl From<DFormat> for Format {
	fn from(format: DFormat) -> Self {
		match format {
			DFormat::L8           => Format::L8,
			DFormat::A8           => Format::A8,
			DFormat::A4R4G4B4     => Format::Argb4,
			DFormat::A1R5G5B5     => Format::A1rgb5,
			DFormat::A8R8G8B8     => Format::Argb8,
			DFormat::X8R8G8B8     => Format::Xrgb8,
			DFormat::Dxt1         => Format::Dxt1,
			DFormat::Dxt3         => Format::Dxt3,
			DFormat::Dxt5         => Format::Dxt5,
			DFormat::A16B16G16R16 => Format::Argb16,
			DFormat::Unknown      => Format::Unknown,
		}
	}
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
	pub header: Header,
	pub data: Vec<u8>,
}

// used to load from spack using ironworks
impl File for Tex {
	fn read(data: Vec<u8>) -> Result<Self> {
		Ok(Tex::read(&mut Cursor::new(&data)))
	}
}

impl Tex {
	pub fn as_pixels(&self) -> &[Pixel] {
		unsafe { ::std::slice::from_raw_parts(self.data.as_ptr() as *const _, self.data.len() / 4) }
	}
	
	pub fn as_pixels_mut(&mut self) -> &mut [Pixel] {
		unsafe { ::std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut _, self.data.len() / 4) }
	}
	
	pub fn read<T>(reader: &mut T) -> Self where T: Read + Seek {
		// unwrap cuz ? doesn't seem to like it and cba figuring out why or using match
		let header = <Header as BinRead>::read(reader).unwrap();
		
		reader.seek(SeekFrom::End(0)).unwrap();
		let mut data = Vec::with_capacity(reader.stream_position().unwrap() as usize);
		reader.seek(SeekFrom::Start(80)).unwrap();
		reader.read_to_end(&mut data).unwrap();
		
		Tex {
			data: DFormat::from(header.format).convert_from(&data).unwrap(),
			header
		}
	}
	
	pub fn write<T>(&self, writer: &mut T) where T: Write + Seek {
		self.header.write_to(writer).unwrap();
		writer.write_all(&DFormat::from(self.header.format).convert_to(&self.data).unwrap()).unwrap();
	}
}

impl Dds for Tex {
	fn read<T>(reader: &mut T) -> Self where T: Read + Seek {
		// TODO: dont unwrap, return a result
		reader.seek(SeekFrom::Start(12)).unwrap();
		let height = reader.read_le::<u32>().unwrap() as u16;
		let width = reader.read_le::<u32>().unwrap() as u16;
		reader.seek(SeekFrom::Current(4)).unwrap();
		let depths = 1.max(reader.read_le::<u32>().unwrap() as u16);
		let mip_levels = reader.read_le::<u32>().unwrap() as u16;
		
		let format = DFormat::get(reader);
		
		// im sure theres some fancier way but w/e
		let mut mip_offsets = [0u32; 13];
		let mut mip_offset = 80;
		for i in 0..13 {
			mip_offsets[i] = if (i as u16) < mip_levels {
				mip_offset
			} else {
				0
			};
			
			mip_offset += ((width as u32 * height as u32 * 4) as f32 * (0.25f32.powi(i as i32))) as u32;
		}
		
		reader.seek(SeekFrom::End(0)).unwrap();
		let mut data = Vec::with_capacity(reader.stream_position().unwrap() as usize);
		reader.seek(SeekFrom::Start(128)).unwrap();
		reader.read_to_end(&mut data).unwrap();
		
		Tex {
			data: format.convert_from(&data).unwrap(),
			header: Header {
				flags: 0x00800000, // TODO: care about other stuff like 3d textures
				format: Format::from(format),
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

impl Png for Tex {
	fn read<T>(_reader: &mut T) -> Self where T: Read + Seek {
		// let png = PngDecoder::new(reader).unwrap();
		// let data = Vec::with_capacity(png.total_bytes())
		todo!();
	}
	
	fn write<T>(&self, writer: &mut T) where T: Write + Seek {
		let png = PngEncoder::new(writer);
		// TODO: possibly convert to a different colortype based on header format, idk
		png.write_image(
			&self.data.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0], p[3]]).collect::<Vec<u8>>(),
			self.header.width as u32,
			self.header.height as u32,
			ColorType::Rgba8
		).unwrap();
	}
}