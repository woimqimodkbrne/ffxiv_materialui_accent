use std::collections::HashMap;

use crate::render_helper::RendererExtender;

struct Mod {
	meta: crate::modman::meta::Meta,
	settings: HashMap<String, crate::modman::settings::Settings>,
}

pub struct Main {
	import_dialog: Option<egui_file::FileDialog>,
	
	active_collection: String,
	collections: Vec<String>,
	mod_list: Vec<Mod>,
}

impl Main {
	pub fn new() -> Self {
		let mut v = Self {
			import_dialog: None,
			
			active_collection: String::new(),
			collections: Vec::new(),
			mod_list: Vec::new(),
		};
		
		v.refresh();
		
		v
	}
}

impl Main {
	fn refresh(&mut self) {
		let backend = crate::backend();
		
		self.collections = backend.get_collections();
		self.active_collection = self.collections.first().unwrap_or(&String::new()).to_owned();
		self.mod_list = backend.get_mods().into_iter().filter_map(|mod_id| {
			backend.get_aeth_meta(&mod_id).map(|meta| Mod {
				// settings: self.collections.iter().map(|col| (col.to_owned(), crate::modman::settings::Settings::open(&meta, col))).collect(),
				meta,
				settings: HashMap::new(),
			})
		}).collect();
	}
}

impl super::View for Main {
	fn name(&self) -> &'static str {
		&"Main"
	}
	
	fn render(&mut self, ui: &mut egui::Ui) {
		if ui.button("Download Paths").clicked() {
			crate::view::explorer::tree::update_paths()
		}
		
		if ui.button("Import").clicked() {
			let mut dialog = egui_file::FileDialog::open_file(Some(crate::config().config.file_dialog_path.clone()))
				.filter(Box::new(|p| p.extension().map(|e| e == "aeth").unwrap_or(false)))
				.title("Import mod");
			dialog.open();
			self.import_dialog = Some(dialog);
		}
		
		if let Some(dialog) = &mut self.import_dialog {
			if dialog.show(ui.ctx()).selected() {
				if let Some(path) = dialog.path() {
					let path = path.to_owned();
					
					if let Some(parent) = path.parent() {
						crate::config().config.file_dialog_path = parent.to_owned();
						_ = crate::config().save_forced();
					}
					
					match crate::backend().install_mod(&path) {
						Ok(v) => log!("Successfully installed mod {v}"),
						Err(e) => log!(err, "Failed to install mod: {e}"),
					}
				}
			}
		}
		
		ui.add_space(30.0);
		ui.label("Settings");
		
		if ui.button("refresh mod list").clicked() {
			self.refresh();
		}
		
		egui::ComboBox::from_label("Collection")
			.selected_text(&self.active_collection)
			.show_ui(ui, |ui| {
				for col in &self.collections {
					ui.selectable_value(&mut self.active_collection, col.to_owned(), col);
				}
			});
		
		if self.collections.contains(&self.active_collection) {
			ui.add_space(10.0);
			
			for m in &mut self.mod_list {
				ui.collapsing(&m.meta.name, |ui| {
					ui.label(&m.meta.description);
					ui.add_space(10.0);
					
					use crate::modman::{meta::OptionSettings, settings::Value::*};
					let meta = &m.meta;
					let settings = m.settings.entry(self.active_collection.clone()).or_insert_with(|| crate::modman::settings::Settings::open(meta, &self.active_collection));
					for (setting_id, val) in settings.iter_mut() {
						let Some(option) = meta.options.iter().find(|o| o.name == *setting_id) else {continue};
						
						match val {
							SingleFiles(val) => {
								let OptionSettings::SingleFiles(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								egui::ComboBox::from_label(&option.name)
									.selected_text(o.options.get(*val as usize).map_or("Invalid", |v| &v.name))
									.show_ui(ui, |ui| {
										for (i, sub) in o.options.iter().enumerate() {
											ui.selectable_value(val, i as u32, &sub.name);
										}
									});
							}
							
							MultiFiles(val) => {
								let OptionSettings::MultiFiles(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								ui.label(&option.name);
								ui.indent(setting_id, |ui| {
									for (i, sub) in o.options.iter().enumerate() {
										let mut toggled = *val & (1 << i) != 0;
										if ui.checkbox(&mut toggled, &sub.name).changed() {
											*val ^= 1 << i;
										}
									}
								});
							}
							
							Rgb(val) => {
								let OptionSettings::Rgb(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								ui.num_multi_edit_range(val, &option.name, &[o.min[0]..=o.max[0], o.min[1]..=o.max[1], o.min[2]..=o.max[2]]);
							}
							
							Rgba(val) => {
								let OptionSettings::Rgba(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								ui.num_multi_edit_range(val, &option.name, &[o.min[0]..=o.max[0], o.min[1]..=o.max[1], o.min[2]..=o.max[2], o.min[3]..=o.max[3]]);
							}
							
							Grayscale(val) => {
								let OptionSettings::Grayscale(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								ui.num_edit_range(val, &option.name, o.min..=o.max);
							}
							
							Opacity(val) => {
								let OptionSettings::Opacity(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								ui.num_edit_range(val, &option.name, o.min..=o.max);
							}
							
							Mask(val) => {
								let OptionSettings::Mask(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								ui.num_edit_range(val, &option.name, o.min..=o.max);
							}
							
							Path(val) => {
								let OptionSettings::Path(o) = &option.settings else {ui.label(format!("Invalid setting type for {setting_id}")); continue};
								egui::ComboBox::from_label(&option.name)
									.selected_text(o.options.get(*val as usize).map_or("Invalid", |v| &v.0))
									.show_ui(ui, |ui| {
										for (i, (name, _)) in o.options.iter().enumerate() {
											ui.selectable_value(val, i as u32, name);
										}
									});
							}
						}
					}
					
					ui.add_space(5.0);
					if ui.button("Apply").clicked() {
						match crate::backend().apply_mod_settings(&m.meta.name, &self.active_collection, Some(settings)) {
							Ok(_) => log!("Successfully applied settings for {}", m.meta.name),
							Err(e) => log!(err, "Failed to apply settings for {}: {e}", m.meta.name),
						}
					}
				});
			}
		}
	}
}