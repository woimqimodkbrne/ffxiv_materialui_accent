use std::{io::{Write, Cursor}, fs::File, collections::{HashMap, BTreeMap}};
use noumenon::formats::game::{mtrl, tex::Tex};
use crate::{GAME, gui::{window::aetherment::explorer::load_file, aeth::{self, F2}}, apply::penumbra::PenumbraFile};
use super::Viewer;

lazy_static!{
	static ref TILES: aeth::Texture = {
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
		(mtrl::ShaderKeyId::Skin, BTreeMap::from([(735790577, "Normal"), (1476344676, "Furry")])),
		(mtrl::ShaderKeyId::Unk3054951514, BTreeMap::from([(502437980, "502437980"), (1556481461, "1556481461"), (1611594207, "1611594207"), (2484609214, "2484609214"), (3835352875, "3835352875")])),
		(mtrl::ShaderKeyId::Unk3531043187, BTreeMap::from([(1480746461, "1480746461"), (4083110193, "4083110193")])),
		(mtrl::ShaderKeyId::Unk4176438622, BTreeMap::from([(138432195, "138432195"), (3869682983, "3869682983")])),
	]);
}

pub struct Mtrl {
	ext: String,
	gamepath: String,
	mtrl: mtrl::Mtrl,
	// colorset_row: Option<usize>,
	colorset_row_copy: Option<(mtrl::ColorsetRow, Option<mtrl::ColorsetDyeRow>)>,
}

impl Mtrl {
	pub fn new(gamepath: String, conf: Option<super::Conf>) -> Self {
		Mtrl {
			ext: format!(".{}", gamepath.split('.').last().unwrap()),
			mtrl: mtrl::Mtrl::read(&mut Cursor::new(load_file(&conf, &gamepath))),
			gamepath,
			// colorset_row: None,
			colorset_row_copy: None,
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
				aeth::offset([180.0 - imgui::get_cursor_pos().x(), 0.0]);
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
				aeth::offset([180.0 - imgui::get_cursor_pos().x(), 0.0]);
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
				aeth::offset([180.0 - imgui::get_cursor_pos().x(), 0.0]);
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
			let mut state = self.mtrl.colorset_rows.is_some();
			if imgui::checkbox("##colorset_enabled", &mut state) {
				match state {
					true => self.mtrl.colorset_rows = Some(Default::default()),
					false => self.mtrl.colorset_rows = None,
				}
			}
			if imgui::is_item_hovered() {
				imgui::set_tooltip("Colorsets")
			}
			
			if self.mtrl.colorset_rows.is_some() {
				imgui::same_line();
				let mut state = self.mtrl.colorsetdye_rows.is_some();
				if imgui::checkbox("##colorsetdye_enabled", &mut state) {
					match state {
						true => self.mtrl.colorsetdye_rows = Some(Default::default()),
						false => self.mtrl.colorsetdye_rows = None,
					}
				}
				if imgui::is_item_hovered() {
					imgui::set_tooltip("Colorset Dyes")
				}
			}
			
			imgui::same_line();
			if let Some(colorset) = &mut self.mtrl.colorset_rows && imgui::collapsing_header("Colorset", imgui::TreeNodeFlags::SpanAvailWidth) {
				imgui::indent();
				
				let h = aeth::frame_height();
				let row_width = h + 10.0 + h + h + 10.0 + (h + 1.0) * 3.0 + 60.0 + 1.0 + 60.0 + 10.0 + h + (1.0 + 60.0) * 4.0 +
					if self.mtrl.colorsetdye_rows.is_some() {10.0 + 60.0 + (h + 1.0) * 5.0} else {0.0};
				let draw = imgui::get_window_draw_list();
				
				let clrframe = imgui::get_color(imgui::Col::FrameBg);
				let rounding = imgui::get_style().frame_rounding;
				
				for (i, row) in colorset.iter_mut().enumerate() {
					let pos = imgui::get_cursor_screen_pos();
					draw.add_rect_filled(pos, pos.add([row_width, h]), imgui::get_color(imgui::Col::PopupBg), imgui::get_style().frame_rounding, imgui::DrawFlags::RoundCornersAll);
					
					let num = &(i + 1).to_string();
					draw.add_text(pos.add([h, h].sub(imgui::calc_text_size(num, false, 0.0)).div([2.0, 2.0])), imgui::get_color(imgui::Col::Text), num);
					
					imgui::begin_group();
					imgui::push_style_var2(imgui::StyleVar::ItemSpacing, [0.0, 0.0]);
					imgui::push_style_var(imgui::StyleVar::FrameRounding, 0.0);
					imgui::push_style_color(imgui::Col::FrameBg, 0);
					imgui::push_style_color(imgui::Col::FrameBgActive, 0);
					imgui::push_style_color(imgui::Col::FrameBgHovered, 0);
					imgui::push_id_i32(i as i32);
					
					// copy row
					aeth::offset([h + 10.0, 0.0]);
					if aeth::button_icon("") { // fa-copy
						self.colorset_row_copy = Some((
							row.clone(),
							if let Some(dyes) = &self.mtrl.colorsetdye_rows {Some(dyes[i].clone())} else {None},
						))
					}
					aeth::tooltip("Copy");
					
					// paste copied row
					imgui::same_line();
					if aeth::button_icon_state("", self.colorset_row_copy.is_some()) && let Some(copy) = &self.colorset_row_copy { // fa-paste
						*row = copy.0.clone();
						if let Some(dye) = &copy.1 {
							self.mtrl.colorsetdye_rows.as_mut().unwrap()[i] = dye.clone();
						}
						// self.colorset_row_copy = None;
					}
					aeth::tooltip("Paste");
					
					// diffuse
					imgui::same_line();
					aeth::offset([10.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					if imgui::invisible_button("diffuse", [h, h], imgui::ButtonFlags::MouseButtonLeft) {imgui::open_popup("diffuse", imgui::PopupFlags::None)}
					aeth::tooltip("Diffuse");
					let col = 0xFF000000 + (((row.diffuse[2].min(1.0) * 255.0) as u32) << 16) + (((row.diffuse[1].min(1.0) * 255.0) as u32) << 8) + ((row.diffuse[0].min(1.0) * 255.0) as u32);
					draw.add_rect_filled(pos, pos.add([h, h]), col, 0.0, imgui::DrawFlags::None);
					
					// emissive
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					if imgui::invisible_button("emissive", [h, h], imgui::ButtonFlags::MouseButtonLeft) {imgui::open_popup("emissive", imgui::PopupFlags::None)}
					aeth::tooltip("Emissive");
					let col = 0xFF000000 + (((row.emissive[2].min(1.0) * 255.0) as u32) << 16) + (((row.emissive[1].min(1.0) * 255.0) as u32) << 8) + ((row.emissive[0].min(1.0) * 255.0) as u32);
					draw.add_rect_filled(pos, pos.add([h, h]), col, 0.0, imgui::DrawFlags::None);
					
					// specular
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					if imgui::invisible_button("specular", [h, h], imgui::ButtonFlags::MouseButtonLeft) {imgui::open_popup("specular", imgui::PopupFlags::None)}
					aeth::tooltip("Specular");
					let col = 0xFF000000 + (((row.specular[2].min(1.0) * 255.0) as u32) << 16) + (((row.specular[1].min(1.0) * 255.0) as u32) << 8) + ((row.specular[0].min(1.0) * 255.0) as u32);
					draw.add_rect_filled(pos, pos.add([h, h]), col, 0.0, imgui::DrawFlags::None);
					
					// specular strength
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					draw.add_rect_filled(pos, pos.add([60.0, h]), clrframe, 0.0, imgui::DrawFlags::None);
					imgui::set_next_item_width(60.0);
					imgui::input_float("##specular_strength", &mut row.specular_strength, 0.0, 0.0, "%.2f", imgui::InputTextFlags::None);
					aeth::tooltip("Specular Strength");
					
					// gloss strength
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					draw.add_rect_filled(pos, pos.add([60.0, h]), clrframe, rounding, imgui::DrawFlags::RoundCornersRight);
					imgui::set_next_item_width(60.0);
					imgui::input_float("##gloss_strength", &mut row.gloss_strength, 0.0, 0.0, "%.2f", imgui::InputTextFlags::None);
					aeth::tooltip("Gloss Strength");
					
					// tile
					imgui::same_line();
					aeth::offset([10.0, 0.0]);
					imgui::image(TILES.resource(), [h, h], [0.0, row.tile_index as f32 / 64.0], [1.0, row.tile_index as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
					if imgui::is_item_clicked(imgui::MouseButton::Left) {imgui::open_popup("tile", imgui::PopupFlags::None)}
					if imgui::is_item_hovered() {
						imgui::begin_tooltip();
						imgui::text(&format!("Tile Index ({})", row.tile_index));
						imgui::image(TILES.resource(), [128.0, 128.0], [0.0, row.tile_index as f32 / 64.0], [1.0, row.tile_index as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
						imgui::end_tooltip();
					}
					
					// tile repeat x
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					draw.add_rect_filled(pos, pos.add([60.0, h]), clrframe, 0.0, imgui::DrawFlags::None);
					imgui::set_next_item_width(60.0);
					imgui::input_float("##tile_repeat_x", &mut row.tile_repeat_x, 0.0, 0.0, "%.2f", imgui::InputTextFlags::None);
					aeth::tooltip("Tile Repeat X");
					
					// tile repeat y
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					draw.add_rect_filled(pos, pos.add([60.0, h]), clrframe, 0.0, imgui::DrawFlags::None);
					imgui::set_next_item_width(60.0);
					imgui::input_float("##tile_repeat_y", &mut row.tile_repeat_y, 0.0, 0.0, "%.2f", imgui::InputTextFlags::None);
					aeth::tooltip("Tile Repeat Y");
					
					// tile skew x
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					draw.add_rect_filled(pos, pos.add([60.0, h]), clrframe, 0.0, imgui::DrawFlags::None);
					imgui::set_next_item_width(60.0);
					imgui::input_float("##tile_skew_x", &mut row.tile_skew_x, 0.0, 0.0, "%.2f", imgui::InputTextFlags::None);
					aeth::tooltip("Tile Skew X");
					
					// tile skew y
					imgui::same_line();
					aeth::offset([1.0, 0.0]);
					let pos = imgui::get_cursor_screen_pos();
					draw.add_rect_filled(pos, pos.add([60.0, h]), clrframe, rounding, imgui::DrawFlags::RoundCornersRight);
					imgui::set_next_item_width(60.0);
					imgui::input_float("##tile_skew_y", &mut row.tile_skew_y, 0.0, 0.0, "%.2f", imgui::InputTextFlags::None);
					aeth::tooltip("Tile Skew Y");
					
					if let Some(dye) = &mut self.mtrl.colorsetdye_rows {
						let dye = &mut dye[i];
						
						imgui::same_line();
						aeth::offset([10.0, 0.0]);
						let pos = imgui::get_cursor_screen_pos();
						imgui::set_next_item_width(60.0);
						draw.add_rect_filled(pos, pos.add([60.0, h]), clrframe, rounding, imgui::DrawFlags::RoundCornersLeft);
						imgui::input_int("##dye_template", &mut dye.template, 0, 0, imgui::InputTextFlags::None);
						aeth::tooltip("Dye Template");
						
						imgui::same_line();
						aeth::offset([1.0, 0.0]);
						let pos = imgui::get_cursor_screen_pos();
						draw.add_rect_filled(pos, pos.add([h, h]), clrframe, 0.0, imgui::DrawFlags::None);
						imgui::checkbox("##dye_diffuse", &mut dye.diffuse);
						aeth::tooltip("Dye Diffuse");
						
						imgui::same_line();
						aeth::offset([1.0, 0.0]);
						let pos = imgui::get_cursor_screen_pos();
						draw.add_rect_filled(pos, pos.add([h, h]), clrframe, 0.0, imgui::DrawFlags::None);
						imgui::checkbox("##dye_emisive", &mut dye.emisive);
						aeth::tooltip("Dye Emisive");
						
						imgui::same_line();
						aeth::offset([1.0, 0.0]);
						let pos = imgui::get_cursor_screen_pos();
						draw.add_rect_filled(pos, pos.add([h, h]), clrframe, 0.0, imgui::DrawFlags::None);
						imgui::checkbox("##dye_specular", &mut dye.specular);
						aeth::tooltip("Dye Specular");
						
						imgui::same_line();
						aeth::offset([1.0, 0.0]);
						let pos = imgui::get_cursor_screen_pos();
						draw.add_rect_filled(pos, pos.add([h, h]), clrframe, 0.0, imgui::DrawFlags::None);
						imgui::checkbox("##dye_specular_strength", &mut dye.specular_strength);
						aeth::tooltip("Dye Specular Strength");
						
						imgui::same_line();
						aeth::offset([1.0, 0.0]);
						let pos = imgui::get_cursor_screen_pos();
						draw.add_rect_filled(pos, pos.add([h, h]), clrframe, rounding, imgui::DrawFlags::RoundCornersRight);
						imgui::checkbox("##dye_gloss", &mut dye.gloss);
						aeth::tooltip("Dye Gloss");
					}
					
					//
					imgui::pop_style_var(2);
					imgui::pop_style_color(3);
					imgui::end_group();
					
					// draw these at the end so it isnt affected by the style changes
					if imgui::begin_popup("diffuse", imgui::WindowFlags::None) {
						imgui::color_picker3("diffuse", &mut row.diffuse, imgui::ColorEditFlags::PickerHueWheel);
						imgui::end_popup();
					}
					
					if imgui::begin_popup("emissive", imgui::WindowFlags::None) {
						imgui::color_picker3("emissive", &mut row.emissive, imgui::ColorEditFlags::PickerHueWheel);
						imgui::end_popup();
					}
					
					if imgui::begin_popup("specular", imgui::WindowFlags::None) {
						imgui::color_picker3("specular", &mut row.specular, imgui::ColorEditFlags::PickerHueWheel);
						imgui::end_popup();
					}
					
					if imgui::begin_popup("tile", imgui::WindowFlags::None) {
						for i in 0..64 {
							imgui::begin_group();
							imgui::image(TILES.resource(), [32.0, 32.0], [0.0, i as f32 / 64.0], [1.0, i as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
							if imgui::is_item_clicked(imgui::MouseButton::Left) {
								row.tile_index = i;
								imgui::close_current_popup();
							}
							aeth::offset([0.0, -imgui::get_style().item_spacing.y()]);
							imgui::text(&(i + 1).to_string());
							imgui::end_group();
							if imgui::is_item_hovered() {
								imgui::begin_tooltip();
								imgui::image(TILES.resource(), [128.0, 128.0], [0.0, i as f32 / 64.0], [1.0, i as f32 / 64.0 + 1.0 / 64.0], [1.0; 4], [0.0; 4]);
								imgui::end_tooltip();
							}
							
							if (i + 1) % 8 != 0 {
								imgui::same_line();
							}
						}
						
						imgui::end_popup();
					}
					
					imgui::pop_id();
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