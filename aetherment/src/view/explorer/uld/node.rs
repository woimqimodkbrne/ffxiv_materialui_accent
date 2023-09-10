use std::collections::HashMap;
use noumenon::format::game::uld;
use crate::render_helper::RendererExtender;
use super::{names_from_parts_list, name_from_part, longest_common_substring_all, get_resource};

// TODO: this should be more user friendly

pub fn render_nodedata(ui: &mut egui::Ui, node: &mut uld::NodeData, resources: &mut HashMap<String, Option<egui::TextureHandle>>, assets: &Vec<uld::UldTexture>, parts: &Vec<uld::UldPartsList>) {
	ui.enum_combo(&mut node.node, "Node Type");
	// TODO: make this a dropdown to select node from the list -current one
	if let uld::Node::Component(node) = &mut node.node {
		ui.num_edit(&mut node.component_id, "Component ID");
	}
	
	ui.separator();
	
	ui.num_edit(&mut node.node_id, "Node ID");
	ui.num_edit(&mut node.parent_id, "Parent ID");
	ui.num_edit(&mut node.next_sibling_id, "Next Sibling ID");
	ui.num_edit(&mut node.prev_sibling_id, "Previous Sibling ID");
	ui.num_edit(&mut node.child_node_id, "Child Node ID");
	ui.num_edit(&mut node.tab_index, "Tab Index");
	
	ui.num_multi_edit(&mut node.unk1, "Unknown");
	
	ui.num_edit(&mut node.x, "X");
	ui.num_edit(&mut node.y, "Y");
	ui.num_edit(&mut node.w, "Width");
	ui.num_edit(&mut node.h, "Height");
	ui.num_edit(&mut node.rotation, "Rotation");
	ui.num_edit(&mut node.scale_x, "Scale X");
	ui.num_edit(&mut node.scale_y, "Scale Y");
	ui.num_edit(&mut node.origin_x, "Origin X");
	ui.num_edit(&mut node.origin_y, "Origin Y");
	ui.num_edit(&mut node.priority, "Priority");
	
	ui.checkbox(&mut node.visible, "Visible");
	ui.checkbox(&mut node.enabled, "Enabled");
	ui.checkbox(&mut node.clip, "Clip");
	ui.checkbox(&mut node.fill, "Fill");
	ui.checkbox(&mut node.anchor_top, "Anchor Top");
	ui.checkbox(&mut node.anchor_bottom, "Anchor Bottom");
	ui.checkbox(&mut node.anchor_left, "Anchor Left");
	ui.checkbox(&mut node.anchor_right, "Anchor Right");
	
	ui.num_edit(&mut node.unk2, "Unknown 2");
	ui.num_edit(&mut node.multiply_red, "Multiply Red");
	ui.num_edit(&mut node.multiply_green, "Multiply Green");
	ui.num_edit(&mut node.multiply_blue, "Multiply Blue");
	ui.num_edit(&mut node.add_red, "Add Red");
	ui.num_edit(&mut node.add_green, "Add Green");
	ui.num_edit(&mut node.add_blue, "Add Blue");
	ui.num_edit(&mut node.alpha, "Alpha");
	ui.num_edit(&mut node.clip_count, "Clip Count");
	ui.num_edit(&mut node.timeline_id, "Timeline ID");
	
	ui.separator();
	
	render_node(ui, &mut node.node, resources, assets, parts);
}

fn render_node(ui: &mut egui::Ui, node: &mut uld::Node, resources: &mut HashMap<String, Option<egui::TextureHandle>>, assets: &Vec<uld::UldTexture>, parts_lists: &Vec<uld::UldPartsList>) {
	match node {
		uld::Node::Image(node) => {
			render_part_id_selectors(ui, &mut node.part_list_id, &mut node.part_id, resources, assets, parts_lists);
			ui.checkbox(&mut node.flip_h, "Flip Horizontal");
			ui.checkbox(&mut node.flip_v, "Flip Vertical");
			ui.num_edit(&mut node.wrap, "Wrap");
			ui.num_edit(&mut node.unk1, "Unknown 1");
		}
		
		uld::Node::Text(node) => {
			// TODO: text select from the Addon sheet, perhabs even allow custom text by injecting it into the addon sheet or replace the text at runtime
			ui.num_edit(&mut node.text_id, "Text ID");
			ui.num_edit(&mut node.color, "Color");
			ui.num_edit(&mut node.alignment, "Alignment");
			ui.enum_combo(&mut node.font, "Font");
			ui.num_edit(&mut node.font_size, "Font Size");
			ui.num_edit(&mut node.edge_color, "Edge Color");
			ui.enum_combo(&mut node.sheet_type, "Sheet Type");
			ui.num_edit(&mut node.char_spacing, "Character Spacing");
			ui.num_edit(&mut node.line_spacing, "Line Spacing");
			ui.num_edit(&mut node.unk2, "Unknown 2");
			
			ui.checkbox(&mut node.bold, "Bold");
			ui.checkbox(&mut node.italic, "Italic");
			ui.checkbox(&mut node.edge, "Edge");
			ui.checkbox(&mut node.glare, "Glare");
			ui.checkbox(&mut node.multiline, "Multiline");
			ui.checkbox(&mut node.ellipsis, "Ellipsis");
			ui.checkbox(&mut node.paragraph, "Paragraph");
			ui.checkbox(&mut node.emboss, "Emboss");
		}
		
		uld::Node::NineGrid(node) => {
			render_part_id_selectors(ui, &mut node.part_list_id, &mut node.part_id, resources, assets, parts_lists);
			ui.enum_combo(&mut node.grid_parts_type, "Grid Parts Type");
			ui.enum_combo(&mut node.grid_render_type, "Grid Render Type");
			ui.num_edit(&mut node.top_offset, "Top Offset");
			ui.num_edit(&mut node.bottom_offset, "Bottom Offset");
			ui.num_edit(&mut node.left_offset, "Left Offset");
			ui.num_edit(&mut node.right_offset, "Right Offset");
			ui.num_edit(&mut node.unk1, "Unknown 1");
		}
		
		uld::Node::Counter(node) => {
			let mut part_id = node.part_id as u32;
			render_part_id_selectors(ui, &mut node.part_list_id, &mut part_id, resources, assets, parts_lists);
			node.part_id = part_id as u8;
			ui.num_edit(&mut node.number_width, "Number Width");
			ui.num_edit(&mut node.comma_width, "Comma Width");
			ui.num_edit(&mut node.space_width, "Space Width");
			ui.num_edit(&mut node.alignment, "Alignment");
			ui.num_edit(&mut node.unk1, "Unknown 1");
		}
		
		uld::Node::Collision(node) => {
			ui.enum_combo(&mut node.collision_type, "Collision Type");
			ui.num_edit(&mut node.unk1, "Unknown 1");
			ui.num_edit(&mut node.x, "X");
			ui.num_edit(&mut node.y, "Y");
			ui.num_edit(&mut node.radius, "Radius");
		}
		
		uld::Node::Component(node) => {
			ui.num_edit(&mut node.index, "Index");
			ui.num_edit(&mut node.up, "Up");
			ui.num_edit(&mut node.down, "Down");
			ui.num_edit(&mut node.left, "Left");
			ui.num_edit(&mut node.right, "Right");
			ui.num_edit(&mut node.cursor, "Cursor");
			ui.num_edit(&mut node.unk5, "Unknown 5");
			ui.num_edit(&mut node.offset_x, "Offset X");
			ui.num_edit(&mut node.offset_y, "Offset Y");
			
			ui.checkbox(&mut node.repeat_up, "Repeat Up");
			ui.checkbox(&mut node.repeat_down, "Repeat Down");
			ui.checkbox(&mut node.repeat_left, "Repeat Left");
			ui.checkbox(&mut node.repeat_right, "Repeat Right");
			ui.checkbox(&mut node.unk1, "Unknown 1");
			ui.checkbox(&mut node.unk2, "Unknown 2");
			ui.checkbox(&mut node.unk3, "Unknown 3");
			ui.checkbox(&mut node.unk4, "Unknown 4");
			
			ui.separator();
			
			render_componentnode(ui, &mut node.component_node_data);
		}
		
		uld::Node::Unknown(node) => {
			ui.label(format!("Unknown, bytes: {}", node.len()));
		}
		
		uld::Node::Other(_node) => {
			ui.label("Other");
		}
	}
}

fn render_componentnode(ui: &mut egui::Ui, component_node: &mut uld::ComponentNodeNode) {
	match component_node {
		uld::ComponentNodeNode::Button(node) => {
			ui.num_edit(&mut node.text_id, "Text ID");
		}
		
		uld::ComponentNodeNode::Window(node) => {
			ui.num_edit(&mut node.title_text_id, "Title Text ID");
			ui.num_edit(&mut node.subtitle_text_id, "Subtitle Text ID");
			ui.checkbox(&mut node.close_button, "Close Button");
			ui.checkbox(&mut node.config_button, "Config Button");
			ui.checkbox(&mut node.help_button, "Help Button");
			ui.checkbox(&mut node.header, "Header");
		}
		
		uld::ComponentNodeNode::CheckBox(node) => {
			ui.num_edit(&mut node.text_id, "Text ID");
		}
		
		uld::ComponentNodeNode::RadioButton(node) => {
			ui.num_edit(&mut node.text_id, "Text ID");
			ui.num_edit(&mut node.group_id, "Group ID");
		}
		
		uld::ComponentNodeNode::Gauge(node) => {
			ui.num_edit(&mut node.indicator, "Indicator");
			ui.num_edit(&mut node.min, "Min");
			ui.num_edit(&mut node.max, "Max");
			ui.num_edit(&mut node.value, "Value");
		}
		
		uld::ComponentNodeNode::Slider(node) => {
			ui.num_edit(&mut node.min, "Min");
			ui.num_edit(&mut node.max, "Max");
			ui.num_edit(&mut node.step, "Step");
		}
		
		uld::ComponentNodeNode::TextInput(node) => {
			ui.num_edit(&mut node.max_width, "Max Width");
			ui.num_edit(&mut node.max_line, "Max Line");
			ui.num_edit(&mut node.max_s_byte, "Max S Byte");
			ui.num_edit(&mut node.max_char, "Max Char");
			ui.num_edit(&mut node.charset, "Charset");
			ui.num_multi_edit(&mut node.charset_extras, "Charset Extras");
			
			ui.checkbox(&mut node.capitalize, "Capitalize");
			ui.checkbox(&mut node.mask, "Mask");
			ui.checkbox(&mut node.auto_translate_enabled, "Auto Translate Enabled");
			ui.checkbox(&mut node.history_enabled, "History Enabled");
			ui.checkbox(&mut node.ime_enabled, "IME Enabled");
			ui.checkbox(&mut node.escape_clears, "Escape Clears");
			ui.checkbox(&mut node.caps_allowed, "Caps Allowed");
			ui.checkbox(&mut node.lower_allowed, "Lower Allowed");
			
			ui.checkbox(&mut node.numbers_allowed, "Numbers Allowed");
			ui.checkbox(&mut node.symbols_allowed, "Symbols Allowed");
			ui.checkbox(&mut node.word_wrap, "Word Wrap");
			ui.checkbox(&mut node.multiline, "Multiline");
			ui.checkbox(&mut node.auto_max_width, "Auto Max Width");
		}
		
		uld::ComponentNodeNode::NumericInput(node) => {
			ui.num_edit(&mut node.value, "Value");
			ui.num_edit(&mut node.max, "Max");
			ui.num_edit(&mut node.min, "Min");
			ui.num_edit(&mut node.add, "Add");
			ui.checkbox(&mut node.comma, "Comma");
			
			ui.num_edit(&mut node.unk1, "Unknown 1");
			ui.num_multi_edit(&mut node.unk2, "Unknown 2");
		}
		
		uld::ComponentNodeNode::List(node) => {
			ui.num_edit(&mut node.row_num, "Row Num");
			ui.num_edit(&mut node.column_num, "Column Num");
		}
		
		uld::ComponentNodeNode::Tabbed(node) => {
			ui.num_edit(&mut node.text_id, "Text ID");
			ui.num_edit(&mut node.group_id, "Group ID");
		}
		
		uld::ComponentNodeNode::ListItem(node) => {
			ui.checkbox(&mut node.toggle, "Toggle");
			
			ui.num_multi_edit(&mut node.unk1, "Unknown 1");
		}
		
		uld::ComponentNodeNode::NineGridText(node) => {
			ui.num_edit(&mut node.text_id, "Text ID");
		}
		
		uld::ComponentNodeNode::None => {}
	}
}

fn render_part_id_selectors(ui: &mut egui::Ui, part_list_id: &mut u32, part_id: &mut u32, resources: &mut HashMap<String, Option<egui::TextureHandle>>, assets: &Vec<uld::UldTexture>, parts_lists: &Vec<uld::UldPartsList>) {
	let parts_list = parts_lists.iter().find(|p| p.id == *part_list_id);
	let parts_list_text = parts_list.map(|p| longest_common_substring_all(&names_from_parts_list(p, &assets))).unwrap_or("None");
	egui::ComboBox::from_label("Parts list")
		.selected_text(parts_list_text)
		.show_ui(ui, |ui| {
			for parts in parts_lists {
				let part_text = longest_common_substring_all(&names_from_parts_list(parts, &assets));
				ui.selectable_value(part_list_id, parts.id, &format!("({}) {}", parts.id, part_text));
			}
		});
	
	if let Some(parts_list) = parts_list {
		let part = &parts_list.parts[*part_id as usize];
		egui::ComboBox::from_label("Part")
			.selected_text(format!("({}) {}", part_id, name_from_part(part, &assets)))
			.show_ui(ui, |ui| {
				for (i, part) in parts_list.parts.iter().enumerate() {
					ui.selectable_value(part_id, i as u32, format!("({i}) {}", name_from_part(part, &assets)));
				}
			});
		
		if let Some(asset) = assets.iter().find(|v| v.id == part.texture_id) {
			if let Some(resource) = get_resource(resources, &asset.path, ui.ctx().clone()) {
				let width = ui.available_size().x;
				let size = resource.size_vec2();
				ui.texture(resource, egui::vec2(width, width / 4.0), egui::Rect{min: egui::pos2(part.u as f32 / size.x, part.v as f32 / size.y), max: egui::pos2((part.u + part.w) as f32 / size.x, (part.v + part.h) as f32 / size.y)});
			}
		}
	}
}