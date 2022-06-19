#![allow(dead_code)]

use std::{borrow::Cow, io::{Cursor, Read, Seek, Write}};
use binrw::{binrw, BinRead};
use ironworks::file::File;
use crate::{formats::game::Result, NullReader};

#[binrw]
#[brw(little)]
struct Data {
	sig: u32,
	size: u16,
	colorset_data_size: u16,
	strings_size: u16,
	shader_offset: u16,
	texture_count: u8,
	uvset_count: u8,
	colorset_count: u8,
	unk_size: u8,
	
	#[br(count = texture_count)]
	textures: Vec<Offset>,
	#[br(count = uvset_count)]
	uvsets: Vec<Offset>,
	#[br(count = colorset_count)]
	colorsets: Vec<Offset>,
	
	#[br(count = strings_size)]
	strings: Vec<u8>,
	
	// 4 bytes for (most?) human based things (skin,hair,equipment,weapons)
	// many for others (world,housing)
	// TODO: look into shpk files (shader)
	#[br(count = unk_size)]
	unk: Vec<u8>,
	
	#[br(if(colorset_data_size > 0))]
	colorset_datas: [ColorsetRow; 16],
	
	// TODO: this, cba for now (https://github.com/TexTools/FFXIV_TexTools_UI/blob/924b1fd4b401d7d1ae98051f7bc3eff1b0d3191a/FFXIV_TexTools/ViewModels/ColorsetEditorViewModel.cs#L141)
	#[br(if(colorset_data_size == 544))]
	colorsetdye_datas: [u16; 16],
	
	shader_values_size: u16,
	shader_keys_count: u16,
	constant_count: u16,
	sampler_count: u16,
	flags: u32,
	
	#[br(count = shader_keys_count)]
	shader_keys: Vec<[u8; 8]>, //Vec<(u32, u32)>,
	#[br(count = constant_count)]
	constants: Vec<(u32, u16, u16)>,
	#[br(count = sampler_count)]
	samplers: Vec<[u32; 3]>,
	#[br(count = shader_values_size / 4)]
	shader_values: Vec<f32>,
}

#[binrw]
#[brw(little)]
struct Offset {
	name_offset: u16,
	special: u16,
}

#[binrw]
#[brw(little)]
#[derive(Default, Clone)]
pub struct ColorsetRow {
	// not actually u16, but theres no native f16 and we dont really need it on the rust side
	diffuse_r: u16,
	diffuse_g: u16,
	diffuse_b: u16,
	specular_strength: u16,
	specular_r: u16,
	specular_g: u16,
	specular_b: u16,
	gloss_strength: u16,
	emissive_r: u16,
	emissive_g: u16,
	emissive_b: u16,
	material: u16,
	material_repeat_x: u16,
	material_skew_x: u16,
	material_skew_y: u16,
	material_repeat_y: u16,
}

#[repr(packed)]
pub struct Sampler {
	typ: u32,
	flags: u32,
	path: String,
}

#[repr(packed)]
pub struct Constant {
	typ: u32,
	offset: u16,
	size: u16,
}

#[repr(packed)]
pub struct Mtrl {
	pub shader: String,
	// pub textures: Vec<String>,
	pub uvsets: Vec<String>,
	pub colorsets: Vec<String>,
	pub unk: Vec<u8>,
	pub colorset_datas: Vec<ColorsetRow>,
	pub samplers: Vec<Sampler>,
	pub constants: Vec<Constant>,
}

impl File for Mtrl {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(Mtrl::read(&mut Cursor::new(&data.into())))
	}
}

impl Mtrl {
	pub fn read<T>(reader: &mut T) -> Self where T: Read + Seek {
		let data = <Data as BinRead>::read(reader).unwrap();
		
		// make it big enough so we can modify it on the c# side, idk if good idea but fuck it
		// let mut textures = Vec::with_capacity(8);
		// for o in data.textures {textures.push(data.strings[o.name_offset as usize..].null_terminated().unwrap())}
		
		let mut uvsets = Vec::with_capacity(8);
		for o in data.uvsets {uvsets.push(data.strings[o.name_offset as usize..].null_terminated().unwrap())}
		
		let mut colorsets = Vec::with_capacity(8);
		for o in data.colorsets {colorsets.push(data.strings[o.name_offset as usize..].null_terminated().unwrap())}
		
		let mut samplers = Vec::with_capacity(8);
		for s in data.samplers {
			samplers.push(Sampler {
				typ: s[0],
				flags: s[1],
				path: data.strings[data.textures[s[2] as usize].name_offset as usize..].null_terminated().unwrap(),
			})
		}
		
		Mtrl {
			shader: data.strings[data.shader_offset as usize..].null_terminated().unwrap(),
			// textures,
			uvsets,
			colorsets,
			unk: data.unk,
			colorset_datas: data.colorset_datas.to_vec(),
			samplers,
			constants: data.constants.iter().map(|c| Constant {
				typ: c.0, 
				offset: c.1,
				size: c.2,
			}).collect(),
		}
	}
	
	pub fn write<T>(&self, _writer: &mut T) where T: Write + Seek {
		todo!();
	}
}