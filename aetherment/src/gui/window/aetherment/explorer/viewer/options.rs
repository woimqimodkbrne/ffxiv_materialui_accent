use crate::{apply::penumbra::{MetaOption, TypRgb, TypRgba, TypSingle, TypPenumbra, PenumbraOption, MetaOptionUnique}, gui::aeth};
use super::Viewer;

pub struct Options {
	
}

impl Options {
	pub fn new() -> Self {
		Options {
			
		}
	}
}

impl Viewer for Options {
	fn valid_imports(&self) -> Vec<&str> {
		vec![]
	}
	
	fn valid_exports(&self) -> Vec<&str> {
		vec![]
	}
	
	fn draw(&mut self, _state: &mut crate::Data, conf: Option<super::Conf>) {
		if conf.is_none() {return}
		
		let conf = conf.unwrap();
		let options = &mut conf.datas.penumbra.as_mut().unwrap().options;
		let mut rem = None;
		aeth::orderable_list2("options", [0.0; 2], options, |i, _| {
			if imgui::button("Remove", [0.0, 0.0]) {
				rem = Some(i);
			}
		}, |i, option| {
			imgui::begin_group();
			if imgui::collapsing_header(&format!("{} ({})###{}", option.name(), option.type_name(), i), imgui::TreeNodeFlags::SpanAvailWidth) {
				let single = matches!(option.deref(), MetaOption::Single(_));
				
				// TODO: fix reserve, it doesnt work since imgui has an internal buffer with capacity that only updates on refocus
				match option.deref_mut() {
					MetaOption::Rgb(opt) => {
						// opt.id.reserve(16);
						opt.name.reserve(16);
						opt.description.reserve(16);
						
						// imgui::input_text("ID", &mut opt.id, imgui::InputTextFlags::None);
						imgui::input_text("Name", &mut opt.name, imgui::InputTextFlags::None);
						imgui::input_text("Description", &mut opt.description, imgui::InputTextFlags::None);
						imgui::color_edit3("Default Value", &mut opt.default, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs);
					},
					MetaOption::Rgba(opt) => {
						// opt.id.reserve(16);
						opt.name.reserve(16);
						opt.description.reserve(16);
						
						// imgui::input_text("ID", &mut opt.id, imgui::InputTextFlags::None);
						imgui::input_text("Name", &mut opt.name, imgui::InputTextFlags::None);
						imgui::input_text("Description", &mut opt.description, imgui::InputTextFlags::None);
						imgui::color_edit4("Default Value", &mut opt.default, imgui::ColorEditFlags::PickerHueWheel | imgui::ColorEditFlags::NoInputs | imgui::ColorEditFlags::AlphaBar | imgui::ColorEditFlags::AlphaPreviewHalf);
					},
					MetaOption::Grayscale(opt) | MetaOption::Opacity(opt) => {
						// opt.id.reserve(16);
						opt.name.reserve(16);
						opt.description.reserve(16);
						
						// imgui::input_text("ID", &mut opt.id, imgui::InputTextFlags::None);
						imgui::input_text("Name", &mut opt.name, imgui::InputTextFlags::None);
						imgui::input_text("Description", &mut opt.description, imgui::InputTextFlags::None);
						let mut v = opt.default * 255.0;
						imgui::drag_float("Default Value", &mut v, 0.0, 0.0, 255.0, "%.0f", imgui::SliderFlags::NoRoundToFormat);
						opt.default = v / 255.0;
					},
					MetaOption::Mask(opt) => {
						// opt.id.reserve(16);
						opt.name.reserve(16);
						opt.description.reserve(16);
						
						// imgui::input_text("ID", &mut opt.id, imgui::InputTextFlags::None);
						imgui::input_text("Name", &mut opt.name, imgui::InputTextFlags::None);
						imgui::input_text("Description", &mut opt.description, imgui::InputTextFlags::None);
						let mut v = opt.default * 100.0;
						imgui::drag_float("Default Value", &mut v, 0.0, 0.0, 100.0, "%.1f%%", imgui::SliderFlags::NoRoundToFormat);
						opt.default = v / 100.0;
					},
					MetaOption::Single(opt) | MetaOption::Multi(opt) => {
						opt.name.reserve(16);
						opt.description.reserve(16);
						
						let convert = if single {imgui::button("Convert to Multi", [0.0, 0.0])}
						              else {imgui::button("Convert to Single", [0.0, 0.0])};
						
						imgui::input_text("Name", &mut opt.name, imgui::InputTextFlags::None);
						imgui::input_text("Description", &mut opt.description, imgui::InputTextFlags::None);
						
						if imgui::collapsing_header("Sub Options", imgui::TreeNodeFlags::SpanAvailWidth) {
							let mut rem = None;
							aeth::orderable_list("sub", &mut opt.options, |i, _| {
								if imgui::button("Remove", [0.0, 0.0]) {
									rem = Some(i);
								}
							}, |_, sub| {
								imgui::text(&sub.name);
							});
							
							if let Some(rem) = rem {
								opt.options.remove(rem);
							}
							
							if aeth::button_icon("") { // fa-plus
								opt.options.push(PenumbraOption::default())
							}
							imgui::same_line();
							imgui::text("New Sub Option");
						}
						
						if convert {
							*option.deref_mut() = if single {MetaOption::Multi(opt.clone())} else {MetaOption::Single(opt.clone())}
						}
					},
				}
			}
			imgui::end_group();
		});
		
		if let Some(rem) = rem {
			options.remove(rem);
		}
		
		if aeth::button_icon("") { // fa-plus
			imgui::open_popup("newopt", imgui::PopupFlags::None);
		}
		imgui::same_line();
		imgui::text("New Option");
		
		aeth::popup("newopt", imgui::WindowFlags::None, || {
			if imgui::selectable("Rgb", false, imgui::SelectableFlags::None, [0.0, 0.0]) {options.push(MetaOptionUnique::new(MetaOption::Rgb(TypRgb::default())))}
			if imgui::selectable("Rgba", false, imgui::SelectableFlags::None, [0.0, 0.0]) {options.push(MetaOptionUnique::new(MetaOption::Rgba(TypRgba::default())))}
			if imgui::selectable("Grayscale", false, imgui::SelectableFlags::None, [0.0, 0.0]) {options.push(MetaOptionUnique::new(MetaOption::Grayscale(TypSingle::default())))}
			if imgui::selectable("Opacity", false, imgui::SelectableFlags::None, [0.0, 0.0]) {options.push(MetaOptionUnique::new(MetaOption::Opacity(TypSingle::default())))}
			if imgui::selectable("Mask", false, imgui::SelectableFlags::None, [0.0, 0.0]) {options.push(MetaOptionUnique::new(MetaOption::Mask(TypSingle::default())))}
			if imgui::selectable("Single", false, imgui::SelectableFlags::None, [0.0, 0.0]) {options.push(MetaOptionUnique::new(MetaOption::Single(TypPenumbra::default())))}
			if imgui::selectable("Multi", false, imgui::SelectableFlags::None, [0.0, 0.0]) {options.push(MetaOptionUnique::new(MetaOption::Multi(TypPenumbra::default())))}
		});
		
		if imgui::button("Save", [0.0, 0.0]) {
			conf.save();
		}
	}
	
	fn save(&self, _ext: &str, _writer: &mut Vec<u8>) {
		todo!()
	}
}