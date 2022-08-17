use std::io::{Write, Cursor};
use noumenon::formats::game::mtrl;
use crate::{GAME, gui::{window::aetherment::explorer::load_file, aeth::{self, F2}}};
use super::Viewer;

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
	
	fn save_changes(&self, conf: Option<super::Conf>) {
		
	}
}

impl Viewer for Mtrl {
	fn valid_imports(&self) -> Vec<String> {
		vec![self.ext.to_owned()]
	}
	
	fn valid_exports(&self) -> Vec<String> {
		vec![self.ext.to_owned()]
	}
	
	fn draw(&mut self, _state: &mut crate::Data, _conf: Option<super::Conf>) {
		// imgui::text(&format!("shader: {}", self.mtrl.shader));
		// imgui::text(&format!("colorsets: {:?}", self.mtrl.colorsets));
		// imgui::text(&format!("colorset_datas: {:?}", self.mtrl.colorset_datas));
		// imgui::text(&format!("unk: {:?}", self.mtrl.unk));
		// imgui::text(&format!("shader_params: {:?}", self.mtrl.shader_params));
		// imgui::text(&format!("samplers: {:?}", self.mtrl.samplers));
		// imgui::text(&format!("shader keys: {:?}", self.mtrl.shader_keys));
		
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
			for (id, param) in &mut self.mtrl.shader.inner().params {
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
		}
		
		if imgui::collapsing_header("Shader Keys", imgui::TreeNodeFlags::SpanAvailWidth) {
			imgui::text("This is still very much unknown and wip. Set 940355280 to 1476344676 for hrothgar body things");
			for (id, val) in &mut self.mtrl.shader_keys {
				imgui::push_id_i32(*id as i32);
				imgui::text(&id.to_string());
				
				imgui::same_line();
				aeth::offset([150.0 - imgui::get_cursor_pos().x(), 0.0]);
				imgui::set_next_item_width(100.0);
				let mut sval = val.to_string();
				imgui::input_text("##val", &mut sval, imgui::InputTextFlags::CharsDecimal);
				if let Ok(sval) = sval.parse::<u32>() {
					*val = sval;
				}
				imgui::pop_id()
			}
		}
		
		if imgui::collapsing_header("Textures", imgui::TreeNodeFlags::SpanAvailWidth) {
			// for sampler in &mut self.mtrl.samplers {
			for (id, sampler) in &mut self.mtrl.shader.inner().samplers {
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
		}
		
		if let Some(colorset) = &mut self.mtrl.colorset_datas && imgui::collapsing_header("Colorset", imgui::TreeNodeFlags::SpanAvailWidth) {
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
				imgui::input_int("Material", &mut row.material, 0, 0, imgui::InputTextFlags::None);
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
				
				imgui::end_child();
			}
		}
		
		imgui::text(&format!("flags: {:#032b}", self.mtrl.flags));
		imgui::text(&format!("uvsets: {:?}", self.mtrl.uvsets));
		imgui::text(&format!("unk: {:?}", self.mtrl.unk));
	}
	
	fn save(&self, _ext: &str, writer: &mut Vec<u8>) {
		writer.write_all(&GAME.file::<Vec<u8>>(&self.gamepath).unwrap()).unwrap();
	}
}