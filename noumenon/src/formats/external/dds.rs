#![allow(dead_code)]

use std::io::{Read, Seek, Write};

pub trait Dds {
	fn read<T>(reader: &mut T) -> Self where T: Read + Seek;
	fn write<T>(&self, writer: &mut T) where T: Write + Seek;
}

// https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide
// b g r a
// ---------------------------------------- //

pub fn convert_from_l8(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(1)
		.flat_map(|p| {
			let v = p[0];
			[v, v, v, 255]
		}).collect::<Vec<u8>>()
}

pub fn convert_to_l8(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(4)
		.flat_map(|p| {
			[p[0]]
		}).collect::<Vec<u8>>()
}

// ---------------------------------------- //

pub fn convert_from_a8(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(1)
		.flat_map(|p| {
			[0, 0, 0, p[0]]
		}).collect::<Vec<u8>>()
}

pub fn convert_to_a8(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(4)
		.flat_map(|p| {
			[p[0]]
		}).collect::<Vec<u8>>()
}

// ---------------------------------------- //

pub fn convert_from_a4r4g4b4(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(2)
		.flat_map(|p| {
			let v = ((p[1] as u16) << 8) + p[0] as u16;
			[
				(v & 0x000F << 4) as u8,
				(v & 0x00F0     ) as u8,
				(v & 0x0F00 >> 4) as u8,
				(v & 0xF000 >> 8) as u8,
			]
		}).collect::<Vec<u8>>()
}

pub fn convert_to_a4r4g4b4(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(4)
		.flat_map(|p| {
			[
				(p[0] >> 4) + p[1],
				(p[2] >> 4) + p[3],
			]
		}).collect::<Vec<u8>>()
}

// ---------------------------------------- //

pub fn convert_from_a1r5g5b5(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(2)
		.flat_map(|p| {
			let v = ((p[1] as u16) << 8) + p[0] as u16;
			[
				(v & 0x001F << 3) as u8,
				(v & 0x03E0 >> 2) as u8,
				(v & 0x7C00 >> 7) as u8,
				(v & 0x8000 >> 8) as u8,
			]
		}).collect::<Vec<u8>>()
}

pub fn convert_to_a1r5g5b5(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(4)
		.flat_map(|p| {
			[
				(p[0] >> 3) + ((p[1] << 2) & 0xE0),
				(p[1] >> 6) + ((p[2] >> 1) & 0x7C) + p[3] & 0x80,
			]
		}).collect::<Vec<u8>>()
}

// ---------------------------------------- //

pub fn convert_from_x8r8g8b8(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(4)
		.flat_map(|p| {
			[p[0], p[1], p[2], 255]
		}).collect::<Vec<u8>>()
}

pub fn convert_to_x8r8g8b8(pixel_data: &[u8]) -> Vec<u8> {
	pixel_data
		.chunks_exact(4)
		.flat_map(|p| {
			[p[0], p[1], p[2], 0]
		}).collect::<Vec<u8>>()
}

// ---------------------------------------- //

// pub fn convert_from_dxt1<T>(width: u32, pixel_data: &[u8]) -> Vec<u8> {
	
// }