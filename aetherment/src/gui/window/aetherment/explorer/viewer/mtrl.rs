use std::{io::{Write, Cursor}, fs::File, collections::{HashMap, BTreeMap}};
use noumenon::formats::game::{mtrl, tex::Tex};
use crate::{GAME, gui::{window::aetherment::explorer::load_file, aeth::{self, F2}}, apply::penumbra::PenumbraFile};
use super::Viewer;

lazy_static!{
	static ref MATERIALS: aeth::Texture = {
		let mut data = Vec::with_capacity(32 * 2048 * 4);
		let tex_d = GAME.file::<Tex>("chara/common/texture/-tile_low_d.tex").unwrap();
		let tex_n = GAME.file::<Tex>("chara/common/texture/-tile_low_n.tex").unwrap();
		
		for i in 0..64 {
			let (_, _, diff) = tex_d.slice(0, i);
			let (_, _, norm) = tex_n.slice(0, i);
			for y in 0..32 {
				for x in 0..(32 - y) {
					let a = diff[y * 32 * 4 + x * 4 + 3];
					data.push(a);
					data.push(a);
					data.push(a);
					data.push(255);
				}
				
				for x in (32 - y)..32 {
					let o = y * 32 * 4 + x * 4;
					data.push(norm[o]);
					data.push(norm[o + 1]);
					data.push(norm[0 + 2]);
					data.push(255);
				}
			}
		}
		
		aeth::Texture::with_data(aeth::TextureOptions {
			width: 32,
			height: 2048,
			format: 87, // DXGI_FORMAT_B8G8R8A8_UNORM
			usage: 1, // D3D11_USAGE_IMMUTABLE
			cpu_access_flags: 0,
		}, &data)
	};
	
	static ref SHADERKEYPRESETS: HashMap<mtrl::ShaderKeyId, BTreeMap<u32, &'static str>> = HashMap::from([
		(mtrl::ShaderKeyId::Unk940355280, BTreeMap::from([(735790577, "Normal Skin"), (1476344676, "Furry Skin")])),
		(mtrl::ShaderKeyId::Unk3054951514, BTreeMap::from([(502437980, "502437980"), (1556481461, "1556481461"), (1611594207, "1611594207"), (2484609214, "2484609214"), (3835352875, "3835352875")])),
		(mtrl::ShaderKeyId::Unk3531043187, BTreeMap::from([(1480746461, "1480746461"), (4083110193, "4083110193")])),
		(mtrl::ShaderKeyId::Unk4176438622, BTreeMap::from([(138432195, "138432195"), (3869682983, "3869682983")])),
	]);
}

pub struct Mtrl {
	ext: String,
	gamepath: String,
	mtrl: mtrl::Mtrl,
	colorset_row: Option<usize>,
}

impl Mtrl {
	pub fn new(gamepath: String, conf: Option<super::Conf>) -> Self {
		Mtrl {
			ext: format!(".{}", gamepath.split('.').last().unwrap()),
			mtrl: mtrl::Mtrl::read(&mut Cursor::new(load_file(&conf, &gamepath))),
			gamepath,
			colorset_row: None,
		}
	}
	
	fn save_changes(&self, conf: &mut super::Conf) {
		let mut buf = Vec::new();
		self.mtrl.write(&mut Cursor::new(&mut buf));
		let hash = crate::hash_str(blake3::hash(&buf));
		let path = conf.path.join("files").join(&hash);
		File::create(path).unwrap().write_all(&buf).unwrap();
		
		let file = PenumbraFile::new_simple(&format!("files/{hash}"));
		conf.datas.penumbra.as_mut().unwrap().update_file(&conf.option, &conf.sub_option, &self.gamepath, Some(file));
		conf.save();
		conf.reload_penumbra();
	}
}

impl Viewer for Mtrl {
	fn valid_imports(&self) -> Vec<String> {
		vec![self.ext.to_owned()]
	}
	
	fn valid_exports(&self) -> Vec<String> {
		vec![self.ext.to_owned()]
	}
	
	fn draw(&mut self, _state: &mut crate::Data, conf: Option<super::Conf>) {
		let curshader = self.mtrl.shader.shader_name();
		if imgui::begin_combo("Shader", curshader, imgui::ComboFlags::None) {
			for shader in mtrl::Shader::shader_names() {
				if imgui::selectable(shader, shader == curshader, imgui::SelectableFlags::None, [0.0, 0.0]) {
					self.mtrl.shader = mtrl::Shader::new(shader).unwrap();
				}
			}
			imgui::end_combo();
		}
		
		if imgui::collapsing_header("Shader Parameters", imgui::TreeNodeFlags::SpanAvailWidth) {
			imgui::indent();
			for (id, param) in &mut self.mtrl.shader.inner_mut().params {
				let param_id = &id.to_string();
				imgui::push_id(param_id);
				imgui::checkbox("##enabled", &mut param.enabled);
				imgui::same_line();
				imgui::text(param_id);
				
				imgui::same_line();
				aeth::offset([150.0 - imgui::get_cursor_pos().x(), 0.0]);
				for (i, val) in param.vals.iter_mut().enumerate() {
					imgui::push_id_i32(i as i32);
					imgui::set_next_item_width(60.0);
					imgui::input_float("##val", val, 0.0, 0.0, "%.4f", imgui::InputTextFlags::None);
					imgui::same_line();
					imgui::pop_id();
				}
				imgui::new_line();
				imgui::pop_id();
			}
			imgui::unindent();
		}
		
		if imgui::collapsing_header("Shader Keys", imgui::TreeNodeFlags::SpanAvailWidth) {
			imgui::indent();
			for (id, key) in &mut self.mtrl.shader.inner_mut().keys {
				imgui::push_id_i32(*id as i32);
				imgui::checkbox("##enabled", &mut key.enabled);
				imgui::same_line();
				imgui::text(&id.to_string());
				
				imgui::same_line();
				aeth::offset([150.0 - imgui::get_cursor_pos().x(), 0.0]);
				if let Some(presets) = SHADERKEYPRESETS.get(id) {
					imgui::set_next_item_width(200.0);
					if imgui::begin_combo("##presetselect", presets.get(&key.val).unwrap(), imgui::ComboFlags::None) {
						for (val, name) in presets {
							if imgui::selectable(name, *val == key.val, imgui::SelectableFlags::None, [0.0, 0.0]) {
								key.val = *val;
							}
						}
						imgui::end_combo();
					}
				} else {
					imgui::text(&key.val.to_string());
				}
				imgui::pop_id()
			}
			imgui::unindent();
		}
		
		if imgui::collapsing_header("Textures", imgui::TreeNodeFlags::SpanAvailWidth) {
			imgui::indent();
			for (id, sampler) in &mut self.mtrl.shader.inner_mut().samplers {
				let sampler_id = &id.to_string();
				imgui::push_id(sampler_id);
				imgui::checkbox("##enabled", &mut sampler.enabled);
				imgui::same_line();
				imgui::text(sampler_id);
				
				imgui::same_line();
				aeth::offset([150.0 - imgui::get_cursor_pos().x(), 0.0]);
				imgui::set_next_item_width(500.0);
				imgui::input_text("##path", &mut sampler.path, imgui::InputTextFlags::None);
				
				imgui::same_line();
				imgui::set_next_item_width(100.0);
				let mut sflags = sampler.flags.to_string();
				imgui::input_text("##flags", &mut sflags, imgui::InputTextFlags::CharsDecimal);
				if let Ok(sflags) = sflags.parse::<u32>() {
					sampler.flags = sflags;
				}
				imgui::pop_id()
			}
			imgui::unindent();
		}
		
		if self.mtrl.shader.inner().unk.len() > 0 && imgui::collapsing_header("Unknown", imgui::TreeNodeFlags::SpanAvailWidth) {
			imgui::indent();
			imgui::text("None of this is known, if you figure out what this does let me know!");
			for val in &mut self.mtrl.shader.inner_mut().unk {
				imgui::set_next_item_width(100.0);
				let mut sval = val.to_string();
				imgui::input_text("##flags", &mut sval, imgui::InputTextFlags::CharsDecimal);
				if let Ok(sval) = sval.parse::<u32>() {
					*val = sval;
				}
			}
			imgui::unindent();
		}
		
		{ // colorsets
			let mut state = self.mtrl.colorset_datas.is_some();
			if imgui::checkbox("##colorset_enabled", &mut state) {
				match state {
					true => self.mtrl.colorset_datas = Some(Default::default()),
					false => self.mtrl.colorset_datas = None,
				}
			}
			if imgui::is_item_hovered() {
				imgui::set_tooltip("Colorsets")
			}
			
			if self.mtrl.colorset_datas.is_some() {
				imgui::same_line();
				let mut state = self.mtrl.colorsetdye_datas.is_some();
				if imgui::checkbox("##colorsetdye_enabled", &mut state) {
					match state {
						true => self.mtrl.colorsetdye_datas = Some(Default::default()),
						false => self.mtrl.colorsetdye_datas = None,
					}
				}
				if imgui::is_item_hovered() {
					imgui::set_tooltip("Colorset Dyes")
				}
			}
			
			imgui::same_line();
			if let Some(colorset) = &mut self.mtrl.colorset_datas && imgui::collapsing_header("Colorset", imgui::TreeNodeFlags::SpanAvailWidth) {
				imgui::indent();
				let h = aeth::frame_height();
				imgui::begin_child("rows", [h * 5.0, h * 16.0], false, imgui::WindowFlags::None);
				let draw = imgui::get_window_draw_list();
				let spos = imgui::get_cursor_screen_pos();
				let pos = imgui::get_cursor_pos();
				for (i, row) in colorset.iter().enumerate() {
					// imgui::button(&i.to_string(), [0.0, 0.0]);
					imgui::push_id_i32(i as i32);
					imgui::set_cursor_pos(pos.add([0.0, h * i as f32]));
					if imgui::invisible_button("##row", [h * 4.0, h], imgui::ButtonFlags::MouseButtonLeft) {
						self.colorset_row = Some(i);
					}
					imgui::pop_id();
					
					let num = &(i + 1).to_string();
					draw.add_text(spos.add([0.0, h * i as f32]).add([h, h].sub(imgui::calc_text_size(num, false, 0.0)).div([2.0, 2.0])), 0xFFFFFFFF, num);
					
					let col = 0xFF000000 + (((row.diffuse[2].min(1.0) * 255.0) as u32) << 16) + (((row.diffuse[1].min(1.0) * 255.0) as u32) << 8) + ((row.diffuse[0].min(1.0) * 255.0) as u32);
					draw.add_rect_filled(spos.add([h, h * i as f32]), spos.add([h * 2.0, h * i as f32 + h]), col, 0.0, imgui::DrawFlags::None);
					
					let col = 0xFF000000 + (((row.specular[2].min(1.0) * 255.0) as u32) << 16) + (((row.specular[1].min(1.0) * 255.0) as u32) << 8) + ((row.specular[0].min(1.0) * 255.0) as u32);
					draw.add_rect_filled(spos.add([h * 2.0, h * i as f32]), spos.add([h * 3.0, h * i as f32 + h]), col, 0.0, imgui::DrawFlags::None);
					
					let col = 0xFF000000 + (((row.emissive[2].min(1.0) * 255.0) as u32) << 16) + (((row.emissive[1].min(1.0) * 255.0) as u32) << 8) + ((row.emissive[0].min(1.0) * 255.0) as u32);
					draw.add_rect_filled(spos.add([h * 3.0, h * i as f32]), spos.add([h * 4.0, h * i as f32 + h]), col, 0.0, imgui::DrawFlags::None);
					
					if imgui::is_item_hovered() || self.colorset_row == Some(i) {
						draw.add_rect(spos.add([0.0, h * i as f32]), spos.add([h * 4.0, h * i as f32 + h]), 0xFF000000, 0.0, imgui::DrawFlags::None, 2.0);
					}
				}
				imgui::end_child();
				if let Some(rowi) = self.colorset_row {
					let row = self.mtrl.colorset_datas.as_mut().unwrap().get_mut(rowi).unwrap();
					
					imgui::same_line();
					imgui::begin_child("row", [0.0, h * 16.0], false, imgui::WindowFlags::None);
					
					imgui::color_edit3("Diffuse", &mut row.diffuse, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs);
					imgui::color_edit3("Specular", &mut row.specular, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs);
					imgui::input_float("Specular Strength", &mut row.specular_strength, 0.0, 0.0, "%.4f", imgui::InputTextFlags::None);
					imgui::color_edit3("Emissive", &mut row.emissive, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs);
					imgui::input_float("Gloss Strength", &mut row.gloss_strength, 0.0, 0.0, "%.4f", imgui::InputTextFlags::None);
					imgui::image(MATERIALS.resource(), [32.0, 32.0], [0.0, row.material as f32 / 64.0], [1.0, row.material as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
					if imgui::is_item_clicked(imgui::MouseButton::Left) {
						imgui::open_popup("materialselect", imgui::PopupFlags::MouseButtonLeft)
					}
					if imgui::is_item_hovered() {
						imgui::begin_tooltip();
						imgui::image(MATERIALS.resource(), [128.0, 128.0], [0.0, row.material as f32 / 64.0], [1.0, row.material as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
						imgui::end_tooltip();
					}
					imgui::same_line();
					imgui::text(&format!("Material ({})", row.material + 1));
					imgui::input_float("Material Repeat X", &mut row.material_repeat_x, 0.0, 0.0, "%.4f", imgui::InputTextFlags::None);
					imgui::input_float("Material Repeat Y", &mut row.material_repeat_y, 0.0, 0.0, "%.4f", imgui::InputTextFlags::None);
					imgui::input_float("Material Skew X", &mut row.material_skew_x, 0.0, 0.0, "%.4f", imgui::InputTextFlags::None);
					imgui::input_float("Material Skew Y", &mut row.material_skew_y, 0.0, 0.0, "%.4f", imgui::InputTextFlags::None);
					
					if let Some(dyedatas) = &mut self.mtrl.colorsetdye_datas {
						let dyerow = &mut dyedatas[rowi];
						
						aeth::offset([0.0, 10.0]);
						imgui::text("Dye settings");
						imgui::input_int("Template", &mut dyerow.template, 0, 0, imgui::InputTextFlags::None);
						imgui::checkbox("Apply to Diffiuse", &mut dyerow.diffuse);
						imgui::checkbox("Apply to Specular", &mut dyerow.specular);
						imgui::checkbox("Apply to Emisive", &mut dyerow.emisive);
						imgui::checkbox("Apply to Gloss", &mut dyerow.gloss);
						imgui::checkbox("Apply to Specular Strength", &mut dyerow.specular_strength);
					}
					
					if imgui::begin_popup("materialselect", imgui::WindowFlags::None) {
						for i in 0..64 {
							imgui::begin_group();
							imgui::image(MATERIALS.resource(), [32.0, 32.0], [0.0, i as f32 / 64.0], [1.0, i as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
							if imgui::is_item_clicked(imgui::MouseButton::Left) {
								row.material = i;
								imgui::close_current_popup();
							}
							aeth::offset([0.0, -imgui::get_style().item_spacing.y()]);
							imgui::text(&(i + 1).to_string());
							imgui::end_group();
							if imgui::is_item_hovered() {
								imgui::begin_tooltip();
								imgui::image(MATERIALS.resource(), [128.0, 128.0], [0.0, i as f32 / 64.0], [1.0, i as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
								imgui::end_tooltip();
							}
							
							if (i + 1) % 8 != 0 {
								imgui::same_line();
							}
						}
						
						imgui::end_popup();
					}
					
					imgui::end_child();
				}
				imgui::unindent();
			} else {
				imgui::text("Colorsets")
			}
		}
		
		imgui::text(&format!("flags: {:#032b}", self.mtrl.flags));
		// imgui::text(&format!("uvsets: {:?}", self.mtrl.uvsets));
		// imgui::text(&format!("colorsets: {:?}", self.mtrl.colorsets));
		// imgui::text(&format!("unk: {:?}", self.mtrl.unk));
		
		if let Some(mut conf) = conf && imgui::button("Save", [0.0, 0.0]) {
			self.save_changes(&mut conf);
		}
	}
	
	fn save(&self, _ext: &str, writer: &mut Vec<u8>) {
		writer.write_all(&GAME.file::<Vec<u8>>(&self.gamepath).unwrap()).unwrap();
	}
}