use std::collections::HashMap;
use noumenon::format::game::{uld, Uld as GameUld, Tex as GameTex};
use crate::{NOUMENON, resource_loader::load_file, render_helper::{RendererExtender, EnumTools}};

mod enum_tools;
mod component;
mod node;
mod timeline;

pub struct Uld {
	name: String,
	path: String,
	real_path: Option<String>,
	data: Option<GameUld>,
	views: egui_dock::Tree<Box<dyn Tab>>,
	resources: HashMap<String, Option<egui::TextureHandle>>,
}

impl Uld {
	pub fn new(path: &str, real_path: Option<&str>) -> Result<Self, super::BacktraceError> {
		let mut views: egui_dock::Tree<Box<dyn Tab>> = egui_dock::Tree::new(vec![
			Box::new(Preview),
		]);
		
		views.split_right(egui_dock::NodeIndex::root(), 0.6, vec![
			Box::new(Asset{add: String::new()}),
			Box::new(Part),
			Box::new(Component),
			Box::new(Timeline),
			Box::new(Widget),
		]);
		
		let mut v = Self {
			name: path.split("/").last().unwrap().to_owned(),
			path: path.to_owned(),
			real_path: real_path.map(|v| v.to_owned()),
			data: None,
			views,
			resources: HashMap::new(),
		};
		
		v.load_data()?;
		
		Ok(v)
	}
	
	fn load_data(&mut self) -> Result<(), super::BacktraceError> {
		self.data = None;
		self.data = Some(load_file(&self.path, self.real_path.as_deref())?);
		
		Ok(())
	}
}

impl super::View for Uld {
	fn name(&self) -> &str {
		&self.name
	}
	
	fn path(&self) -> &str {
		&self.path
	}
	
	fn exts(&self) -> Vec<&str> {
		vec![self.name.split(".").last().unwrap()]
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		if let Some(data) = &mut self.data {
			egui_dock::DockArea::new(&mut self.views)
				.id(egui::Id::new(&self.name))
				.style(egui_dock::Style::from_egui(ui.style().as_ref()))
				.show_close_buttons(false)
				.tab_context_menus(false)
				.show_inside(ui, &mut Tabs{resources: &mut self.resources, data});
		} 
		
		Ok(())
	}
	
	fn export(&self, _ext: &str, mut writer: Box<dyn super::Writer>) -> Result<(), super::BacktraceError> {
		if let Some(data) = &self.data {
			data.write(&mut writer)?;
			Ok(())
		} else {
			Err(super::ExplorerError::Data.into())
		}
	}
}

fn get_resource(resources: &mut HashMap<String, Option<egui::TextureHandle>>, path: &str, ctx: egui::Context) -> Option<egui::TextureHandle> {
	resources.entry(path.to_owned()).or_insert_with(|| {
		load_file::<GameTex>(&path, None).ok().map(|data| {
			let img = egui::epaint::image::ColorImage::from_rgba_unmultiplied([data.header.width as usize, data.header.height as usize], &data.data);
			ctx.load_texture(path, img, Default::default())
		})
	}).clone()
}

fn path_name(path: &str) -> &str {
	path.split("/").last().unwrap().split(".").next().unwrap()
}

fn longest_common_substring<'a>(a: &'a str, b: &'a str) -> &'a str {
	let mut max = "";
	for i in 0..a.len() {
		for j in 0..b.len() {
			let mut k = 0;
			while i + k < a.len() && j + k < b.len() && a.chars().nth(i + k) == b.chars().nth(j + k) {
				k += 1;
			}
			if k > max.len() {
				max = &a[i..i + k];
			}
		}
	}
	
	max
}

fn longest_common_substring_all<'a>(a: &[&'a str]) -> &'a str {
	if a.len() == 0 {
		return "";
	}
	
	const MINLEN: usize = 5;
	let mut max = a[0];
	for i in 1..a.len() {
		let sub = longest_common_substring(max, a[i]);
		if sub.len() >= MINLEN {
			max = sub;
		}
	}
	
	max.trim_matches(|c: char| !c.is_alphanumeric())
}

fn names_from_parts_list<'a>(parts: &uld::UldPartsList, assets: &'a [uld::UldTexture]) -> Vec<&'a str> {
	parts.parts.iter().map(|v| assets.iter().find(|v2| v2.id == v.texture_id).map_or("Invalid", |v2| path_name(&v2.path))).collect()
}

fn name_from_part<'a>(part: &uld::UldPart, assets: &'a [uld::UldTexture]) -> &'a str {
	assets.iter().find(|v| v.id == part.texture_id).map_or("Invalid", |v| path_name(&v.path))
}

// ---------------------------------------- //

struct Tabs<'a> {
	resources: &'a mut HashMap<String, Option<egui::TextureHandle>>,
	data: &'a mut GameUld,
}

impl<'a> egui_dock::TabViewer for Tabs<'a> {
	type Tab = Box<dyn Tab>;
	
	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.name().into()
	}
	
	fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
		if let Err(err) = tab.render(ui, self) {
			super::render_error(ui, &err);
		}
	}
}

// ---------------------------------------- //

trait Tab {
	fn name(&self) -> &str;
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError>;
}

struct Preview;
impl Tab for Preview {
	fn name(&self) -> &str {
		"Preview"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, _data: &mut Tabs) -> Result<(), super::BacktraceError> {
		ui.label("WARNING: This editor is still very wip and not all that user friendly, I recommend just changing things in the 'Widget' tab and finding the correct element using '/xldev ai' to find the element to change.\n\nTODO: Preview");
		
		Ok(())
	}
}

struct Asset{add: String}
impl Tab for Asset {
	fn name(&self) -> &str {
		"Assets"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError> {
		let resources = &mut data.resources;
		let data = &mut data.data;
		
		let mut delete = None;
		for asset in &mut data.assets {
			ui.collapsing(format!("({}) {}", asset.id, path_name(&asset.path)), |ui| {
				ui.label(&asset.path);
				
				if let Some(resource) = get_resource(resources, &asset.path, ui.ctx().clone()) {
					let width = ui.available_size().x;
					ui.texture(resource, egui::vec2(width, width), egui::Rect{min: egui::pos2(0.0, 0.0), max: egui::pos2(1.0, 1.0)});
				}
				
				ui.num_edit(&mut asset.icon, "Icon");
				if let Some(unk1) = &mut asset.unk1 {
					ui.num_edit(unk1, "Unknown");
				}
				
				ui.horizontal(|ui| {
					if ui.button("🗑").clicked() {
						delete = Some(asset.id);
					}
					ui.label("Delete");
				});
			});
		}
		
		if let Some(id) = delete {
			data.assets.retain(|v| v.id != id);
		}
		
		ui.horizontal(|ui| {
			if ui.button("➕ Add new asset").clicked() {
				if NOUMENON.as_ref().unwrap().file::<GameTex>(&self.add).is_ok() && data.assets.iter().find(|v| v.path.to_ascii_lowercase() == self.add.to_ascii_lowercase()).is_none() {
					data.assets.push(uld::UldTexture {
						id: data.assets.iter().map(|v| v.id).max().unwrap_or(0) + 1,
						path: self.add.clone(),
						icon: 0,
						unk1: if data.assets_header.version[3] >= 1 {Some(0)} else {None},
					});
					self.add.clear();
				}
			}
			ui.text_edit_singleline(&mut self.add);
		});
		
		Ok(())
	}
}

struct Part;
impl Tab for Part {
	fn name(&self) -> &str {
		"Parts"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError> {
		let resources = &mut data.resources;
		let data = &mut data.data;
		
		let mut delete = None;
		for parts_list in &mut data.parts_lists {
			egui::CollapsingHeader::new(format!("({}) {}", parts_list.id, longest_common_substring_all(&names_from_parts_list(parts_list, &data.assets)))).id_source(parts_list.id).show(ui, |ui| {
				let mut delete_part = None;
				for (i, part) in parts_list.parts.iter_mut().enumerate() {
					let asset = data.assets.iter().find(|v| v.id == part.texture_id);
					let selected_text = format!("({}) {}", part.texture_id, path_name(&asset.map_or("Invalid", |v| &v.path)));
					egui::ComboBox::from_id_source(i)
						.selected_text(selected_text)
						.show_ui(ui, |ui| {
							for asset in &data.assets {
								ui.selectable_value(&mut part.texture_id, asset.id, &format!("({}) {}", asset.id, path_name(&asset.path)));
							}
						});
					
					if let Some(asset) = asset {
						let mut max_x = 4096;
						let mut max_y = 4096;
						
						if let Some(resource) = get_resource(resources, &asset.path, ui.ctx().clone()) {
							let width = ui.available_size().x;
							let size = resource.size_vec2();
							max_x = size.x as u16;
							max_y = size.y as u16;
							if part.w > 0 && part.h > 0 {
								ui.texture(resource, egui::vec2(width, width / 4.0), egui::Rect{min: egui::pos2(part.u as f32 / size.x, part.v as f32 / size.y), max: egui::pos2((part.u + part.w) as f32 / size.x, (part.v + part.h) as f32 / size.y)});
							}
						}
						
						ui.num_edit_range(&mut part.u, "U", 0..=max_x);
						ui.num_edit_range(&mut part.v, "V", 0..=max_y);
						ui.num_edit_range(&mut part.w, "W", 0..=max_x);
						ui.num_edit_range(&mut part.h, "H", 0..=max_y);
					}
						
					ui.horizontal(|ui| {
						if ui.button("🗑").clicked() {
							delete_part = Some(i);
						}
						ui.label("Delete part");
					});
					
					ui.separator();
				}
				
				if let Some(i) = delete_part {
					parts_list.parts.remove(i);
				}
				
				if ui.button("➕ Add new part").clicked() {
					parts_list.parts.push(uld::UldPart {
						texture_id: 0,
						u: 0,
						v: 0,
						w: 0,
						h: 0,
					});
				}
				
				ui.horizontal(|ui| {
					if ui.button("🗑").clicked() {
						delete = Some(parts_list.id);
					}
					ui.label("Delete parts list");
				});
			});
		}
		
		if let Some(id) = delete {
			data.parts_lists.retain(|v| v.id != id);
		}
		
		if ui.button("➕ Add new parts list").clicked() {
			data.parts_lists.push(uld::UldPartsList {
				id: data.parts_lists.iter().map(|v| v.id).max().unwrap_or(0) + 1,
				parts: Vec::new(),
			});
		}
		
		Ok(())
	}
}

struct Component;
impl Tab for Component {
	fn name(&self) -> &str {
		"Components"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError> {
		let resources = &mut data.resources;
		let data = &mut data.data;
		
		let mut delete = None;
		for (i, comp) in data.components.iter_mut().enumerate() {
			egui::CollapsingHeader::new(format!("({}) {}", comp.id, comp.component.get_type().to_str())).id_source(comp.id).show(ui, |ui| {
				ui.enum_combo(&mut comp.component, "Type");
				ui.checkbox(&mut comp.ignore_input, "Ignore input");
				ui.checkbox(&mut comp.drag_arrow, "Drag arrow");
				ui.checkbox(&mut comp.drop_arrow, "Drop arrow");
				
				ui.collapsing("Data", |ui| {
					component::render_component(ui, &mut comp.component);
				});
				
				ui.collapsing("Nodes", |ui| {
					let mut delete_node = None;
					for (j, node) in comp.nodes.iter_mut().enumerate() {
						egui::CollapsingHeader::new(format!("({}) {}", node.node_id, node.node.to_str())).id_source(node.node_id).show(ui, |ui| {
							node::render_nodedata(ui, node, resources, &data.assets, &data.parts_lists);
							
							ui.separator();
							ui.horizontal(|ui| {
								if ui.button("🗑").clicked() {
									delete_node = Some(j);
								}
								ui.label("Delete node");
							});
						});
					}
					
					if let Some(j) = delete_node {
						comp.nodes.remove(j);
					}
					
					if ui.button("➕ Add new node").clicked() {
						comp.nodes.push(uld::NodeData::default());
					}
				});
				
				ui.horizontal(|ui| {
					if ui.button("🗑").clicked() {
						delete = Some(i);
					}
					ui.label("Delete component");
				});
			});
		}
		
		if let Some(i) = delete {
			data.components.remove(i);
		}
		
		if ui.button("➕ Add new component").clicked() {
			data.components.push(uld::UldComponent {
				id: data.components.iter().map(|v| v.id).max().unwrap_or(1000) + 1,
				ignore_input: false,
				drag_arrow: false,
				drop_arrow: false,
				component: uld::Component::default(),
				nodes: Vec::new(),
			});
		}
		
		Ok(())
	}
}

struct Timeline;
impl Tab for Timeline {
	fn name(&self) -> &str {
		"Timelines"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError> {
		// TODO: perhabs a timeline like used in many other software?
		// let resources = &mut data.resources;
		let data = &mut data.data;
		
		let mut delete = None;
		for (i, time) in data.timelines.iter_mut().enumerate() {
			egui::CollapsingHeader::new(format!("({})", time.id)).id_source(time.id).show(ui, |ui| {
				ui.num_edit(&mut time.id, "ID");
				
				ui.collapsing("Frames 1", |ui| {
					timeline::render_frames(ui, &mut time.frames1);
				});
				
				ui.collapsing("Frames 2", |ui| {
					timeline::render_frames(ui, &mut time.frames2);
				});
				
				ui.horizontal(|ui| {
					if ui.button("🗑").clicked() {
						delete = Some(i);
					}
					ui.label("Delete timeline");
				});
			});
		}
		
		if let Some(i) = delete {
			data.timelines.remove(i);
		}
		
		if ui.button("➕ Add new timeline").clicked() {
			data.timelines.push(uld::UldTimeline {
				id: data.timelines.iter().map(|v| v.id).max().unwrap_or(0) + 1,
				frames1: Vec::new(),
				frames2: Vec::new(),
			});
		}
		
		Ok(())
	}
}

struct Widget;
impl Tab for Widget {
	fn name(&self) -> &str {
		"Widgets"
	}
	
	fn render(&mut self, ui: &mut egui::Ui, data: &mut Tabs) -> Result<(), super::BacktraceError> {
		// TODO: render a tree with draggable widgets, should be able to get rid of all 4 id fields
		let resources = &mut data.resources;
		let data = &mut data.data;
		
		let mut delete = None;
		for (i, widget) in data.widgets.iter_mut().enumerate() {
			egui::CollapsingHeader::new(format!("({})", widget.id)).id_source(widget.id).show(ui, |ui| {
				ui.num_edit(&mut widget.id, "ID");
				ui.enum_combo(&mut widget.alignment_type, "Alignment");
				ui.num_edit(&mut widget.x, "X");
				ui.num_edit(&mut widget.y, "Y");
				
				ui.collapsing("Nodes", |ui| {
					let mut delete_node = None;
					for (j, node) in widget.nodes.iter_mut().enumerate() {
						egui::CollapsingHeader::new(format!("({}) {}", node.node_id, node.node.to_str())).id_source(node.node_id).show(ui, |ui| {
							node::render_nodedata(ui, node, resources, &data.assets, &data.parts_lists);
							
							ui.separator();
							ui.horizontal(|ui| {
								if ui.button("🗑").clicked() {
									delete_node = Some(j);
								}
								ui.label("Delete node");
							});
						});
					}
					
					if let Some(j) = delete_node {
						widget.nodes.remove(j);
					}
					
					if ui.button("➕ Add new node").clicked() {
						widget.nodes.push(uld::NodeData::default());
					}
				});
				
				ui.horizontal(|ui| {
					if ui.button("🗑").clicked() {
						delete = Some(i);
					}
					ui.label("Delete widget");
				});
			});
		}
		
		if let Some(i) = delete {
			data.widgets.remove(i);
		}
		
		if ui.button("➕ Add new widget").clicked() {
			data.widgets.push(uld::WidgetData {
				id: data.widgets.iter().map(|v| v.id).max().unwrap_or(0) + 1,
				alignment_type: uld::AlignmentType::default(),
				x: 0,
				y: 0,
				nodes: Vec::new(),
			});
		}
		
		Ok(())
	}
}