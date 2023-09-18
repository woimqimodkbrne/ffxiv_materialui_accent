use std::{path::PathBuf, rc::Rc, cell::RefCell};
use crate::{render_helper::RendererExtender, modman::meta::*};

pub struct ModMeta {
	name: String,
	path: PathBuf,
	meta: Rc<RefCell<crate::modman::meta::Meta>>,
}

impl ModMeta {
	pub fn new(name: impl Into<String>, path: impl Into<PathBuf>, meta: Rc<RefCell<crate::modman::meta::Meta>>) -> Result<Self, super::BacktraceError> {
		Ok(Self {
			name: name.into(),
			path: path.into(),
			meta,
		})
	}
}

impl super::View for ModMeta {
	fn name(&self) -> &str {
		&self.name
	}
	
	fn path(&self) -> &str {
		"_modmeta"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		let mut meta = self.meta.borrow_mut();
		let org = meta.clone(); // this aint the best way to do it but its easy
		
		ui.label("Name");
		ui.text_edit_singleline(&mut meta.name);
		ui.add_space(10.0);
		
		ui.label("Description");
		ui.text_edit_multiline(&mut meta.description);
		ui.add_space(10.0);
		
		ui.label("Version");
		ui.text_edit_singleline(&mut meta.version);
		ui.add_space(10.0);
		
		ui.label("Author");
		ui.text_edit_singleline(&mut meta.author);
		ui.add_space(10.0);
		
		ui.label("Website");
		ui.text_edit_singleline(&mut meta.website);
		ui.add_space(10.0);
		
		ui.label("Tags");
		let mut delete = None;
		for (i, tag) in meta.tags.iter_mut().enumerate() {
			ui.horizontal(|ui| {
				ui.text_edit_singleline(tag);
				if ui.button("🗑").clicked() {
					delete = Some(i);
				}
			});
		}
		
		if let Some(i) = delete {
			meta.tags.remove(i);
		}
		
		if ui.button("➕ Add tag").clicked() {
			meta.tags.push(String::new());
		}
		ui.add_space(10.0);
		
		ui.label("Dependencies");
		let mut delete = None;
		for (i, tag) in meta.dependencies.iter_mut().enumerate() {
			ui.horizontal(|ui| {
				ui.text_edit_singleline(tag);
				if ui.button("🗑").clicked() {
					delete = Some(i);
				}
			});
		}
		
		if let Some(i) = delete {
			meta.dependencies.remove(i);
		}
		
		if ui.button("➕ Add dependency").clicked() {
			meta.dependencies.push(String::new());
		}
		ui.add_space(10.0);
		
		ui.label("Options");
		let mut delete = None;
		egui_dnd::dnd(ui, "options").show_vec(&mut meta.options, |ui, option, handle, state| {
			ui.dnd_header(handle, |ui| {
				egui::CollapsingHeader::new(format!("({}) {}", state.index, option.name)).id_source(state.index).show(ui, |ui| {
					ui.enum_combo(&mut option.settings, "Option type");
					ui.text_edit_singleline(&mut option.name);
					ui.text_edit_multiline(&mut option.description);
					match &mut option.settings {
						OptionSettings::SingleFiles(v) => render_value_files(ui, v, false),
						OptionSettings::MultiFiles(v) => render_value_files(ui, v, true),
						OptionSettings::Rgb(v) => render_value_rgb(ui, v),
						OptionSettings::Rgba(v) => render_value_rgba(ui, v),
						OptionSettings::Grayscale(v) => render_value_single(ui, v),
						OptionSettings::Opacity(v) => render_value_single(ui, v),
						OptionSettings::Mask(v) => render_value_single(ui, v),
					}
					
					if ui.delete_button("Delete option").clicked() {
						delete = Some(state.index);
					}
				});
			});
		});
		
		if let Some(i) = delete {
			meta.options.remove(i);
		}
		
		if ui.button("➕ Add option").clicked() {
			meta.options.push(Option {
				name: "New option".to_owned(),
				description: String::new(),
				settings: OptionSettings::SingleFiles(Default::default()),
			});
		}
		
		// save on changes
		if *meta != org {
			meta.save(&self.path)?;
		}
		
		Ok(())
	}
}

fn render_value_files(ui: &mut egui::Ui, value: &mut ValueFiles, is_multi: bool) {
	ui.add_space(10.0);
	
	if is_multi {
		ui.label("Defaults");
		for (i, item) in value.options.iter().enumerate() {
			let mut toggled = value.default & (1 << i) != 0;
			if ui.checkbox(&mut toggled, format!("({i}) {}", item.name)).changed() {
				value.default ^= 1 << i;
			}
		}
	} else {
		egui::ComboBox::from_label("Default")
			.selected_text(value.options.iter().enumerate().find(|(i, _)| *i == value.default as usize).map_or("Invalid".to_owned(), |(i, v)| format!("({i}) {}", v.name)))
			.show_ui(ui, |ui| {
				for (i, item) in value.options.iter().enumerate() {
					ui.selectable_value(&mut value.default, i as u32, format!("({i}) {}", item.name));
				}
			});
	}
	
	ui.add_space(10.0);
	
	let mut delete = None;
	egui_dnd::dnd(ui, "suboptions").show_vec(&mut value.options, |ui, option, handle, state| {
		ui.dnd_header(handle, |ui| {
			egui::CollapsingHeader::new(format!("({}) {}", state.index, option.name)).id_source(state.index).show(ui, |ui| {
				ui.text_edit_singleline(&mut option.name);
				ui.text_edit_multiline(&mut option.description);
				ui.label("TODO: files, file swaps, manipulations");
				
				if ui.delete_button("Delete sub option").clicked() {
					delete = Some(state.index);
				}
			});
		});
	});
	
	if let Some(i) = delete {
		value.options.remove(i);
	}
	
	if ui.button("➕ Add sub option").clicked() {
		value.options.push(Default::default());
	}
}

fn render_value_rgb(ui: &mut egui::Ui, value: &mut ValueRgb) {
	// TODO: improve this
	ui.num_multi_edit(&mut value.min, "Minimum");
	ui.num_multi_edit(&mut value.max, "Maximum");
	ui.num_multi_edit_range(&mut value.default, "Default", &[value.min[0]..=value.max[0], value.min[1]..=value.max[1], value.min[2]..=value.max[2]]);
}

fn render_value_rgba(ui: &mut egui::Ui, value: &mut ValueRgba) {
	// TODO: improve this
	ui.num_multi_edit(&mut value.min, "Minimum");
	ui.num_multi_edit(&mut value.max, "Maximum");
	ui.num_multi_edit_range(&mut value.default, "Default", &[value.min[0]..=value.max[0], value.min[1]..=value.max[1], value.min[2]..=value.max[2], value.min[3]..=value.max[3]]);
}

fn render_value_single(ui: &mut egui::Ui, value: &mut ValueSingle) {
	ui.num_edit(&mut value.min, "Minimum");
	ui.num_edit(&mut value.max, "Maximum");
	ui.num_edit_range(&mut value.default, "Default", value.min..=value.max);
}