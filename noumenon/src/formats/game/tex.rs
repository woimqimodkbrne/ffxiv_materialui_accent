#![allow(dead_code)]

use std::{io::{Cursor, Read, Seek, Write, SeekFrom}, borrow::Cow};
use binrw::{BinRead, BinReaderExt, BinWrite, binrw};
use image::{codecs::png::PngEncoder, ImageEncoder, ColorType};
use ironworks::file::File;
use crate::formats::{game::Result, external::{dds::{Dds, Format as DFormat}, png::Png}};

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
#[derive(Copy, Clone, Debug)]
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
#[repr(C)]
pub struct Header {
	pub flags: u32,
	pub format: Format,
	pub width: u16,
	pub height: u16,
	pub depths: u16,
	pub mip_levels: u16,
	pub lod_offsets: [u32; 3],
	pub mip_offsets: [u32; 13],
}

#[repr(C)]
pub struct Tex {
	pub header: Header,
	pub data: Vec<u8>,
}

// used to load from spack using ironworks
impl File for Tex {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(Tex::read(&mut Cursor::new(&data.into())))
	}
}

impl Tex {
	pub fn as_pixels(&self) -> &[Pixel] {
		unsafe { ::std::slice::from_raw_parts(self.data.as_ptr() as *const _, self.data.len() / 4) }
	}
	
	pub fn as_pixels_mut(&mut self) -> &mut [Pixel] {
		unsafe { ::std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut _, self.data.len() / 4) }
	}
	
	pub fn overlay_onto(&self, target: &mut Tex) {
		let pixels = self.as_pixels();
		target.as_pixels_mut().iter_mut().enumerate().for_each(|(i, pixel)| {
			let ar = pixel.a as f32 / 255.0;
			let ao = pixels[i].a as f32 / 255.0;
			let a = ao + ar * (1.0 - ao);
			
			pixel.b = ((pixels[i].b as f32 * ao + pixel.b as f32 * ar * (1.0 - ao)) / a) as u8;
			pixel.g = ((pixels[i].g as f32 * ao + pixel.g as f32 * ar * (1.0 - ao)) / a) as u8;
			pixel.r = ((pixels[i].r as f32 * ao + pixel.r as f32 * ar * (1.0 - ao)) / a) as u8;
			pixel.a = (a * 255.0) as u8;
		});
	}
	
	// this wont work properloy with compressed textures since those are only decompressed for a single slice
	// TODO: fix that
	pub fn slice(&self, miplevel: u16, depth: u16) -> (u16, u16, &[u8]) {
		let factor = 0.5f32.powi(miplevel as i32);
		let (w, h) = (self.header.width as f32 * factor, self.header.height as f32 * factor);
		let slicesize = (w * h * 4.0) as usize;
		let offset = ((self.header.mip_offsets[miplevel as usize] - 80) * 4 / (DFormat::from(self.header.format).bitcount() / 8))
			as usize + slicesize * depth as usize;
		(w as u16, h as u16, &self.data[offset..(offset + slicesize)])
	}
	
	pub fn read<T>(reader: &mut T) -> Self where T: Read + Seek {
		// unwrap cuz ? doesn't seem to like it and cba figuring out why or using match
		let header = <Header as BinRead>::read(reader).unwrap();
		
		reader.seek(SeekFrom::End(0)).unwrap();
		let mut data = Vec::with_capacity(reader.stream_position().unwrap() as usize);
		reader.seek(SeekFrom::Start(80)).unwrap();
		reader.read_to_end(&mut data).unwrap();
		
		Tex {
			data: DFormat::from(header.format).convert_from(header.width as usize, header.height as usize, &data).unwrap(),
			header
		}
	}
	
	pub fn write<T>(&self, writer: &mut T) where T: Write + Seek {
		self.header.write_to(writer).unwrap();
		writer.write_all(&DFormat::from(self.header.format).convert_to(self.header.width as usize, self.header.height as usize, &self.data).unwrap()).unwrap();
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
			data: format.convert_from(width as usize, height as usize, &data).unwrap(),
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
	
	// TODO: gotta use those results...
	fn write<T>(&self, writer: &mut T) where T: Write + Seek {
		let format = DFormat::from(self.header.format);
		
		"DDS ".as_bytes().write_to(writer).unwrap();
		124u32.write_to(writer).unwrap();
		(format.flags() | if self.header.mip_levels > 1 {0x2000} else {0}).write_to(writer).unwrap();
		(self.header.height as u32).write_to(writer).unwrap();
		(self.header.width as u32).write_to(writer).unwrap();
		0u32.write_to(writer).unwrap(); // most software calculate the pitch itself, so eh fuck it
		0u32.write_to(writer).unwrap();
		(self.header.mip_levels as u32).write_to(writer).unwrap();
		"Noumenon v1".as_bytes().write_to(writer).unwrap(); // combines with the one below should total 44 bytes (reserved)
		[0u8; 33].write_to(writer).unwrap();
		32u32.write_to(writer).unwrap();
		format.flags2().write_to(writer).unwrap();
		format.fourcc().write_to(writer).unwrap();
		format.bitcount().write_to(writer).unwrap();
		let (b, g, r, a) = format.masks();
		r.write_to(writer).unwrap();
		g.write_to(writer).unwrap();
		b.write_to(writer).unwrap();
		a.write_to(writer).unwrap();
		0x1000u32.write_to(writer).unwrap(); // TODO: the 2 other flags
		[0u32; 4].write_to(writer).unwrap();
		format.convert_to(self.header.width as usize, self.header.height as usize, &self.data).unwrap().write_to(writer).unwrap();
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
			&self.data[0..(self.header.width as usize * self.header.height as usize * 4)]
				.chunks_exact(4)
				.flat_map(|p| [p[2], p[1], p[0], p[3]])
				.collect::<Vec<u8>>(),
			self.header.width as u32,
			self.header.height as u32,
			ColorType::Rgba8
		).unwrap();
	}
}