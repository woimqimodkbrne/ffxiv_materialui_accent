pub trait RendererExtender {
	fn texture(&mut self, img: egui::TextureHandle, max_size: impl Into<egui::Vec2>, uv: impl Into<egui::Rect>) -> egui::Response;
	fn num_edit<Num: egui::emath::Numeric>(&mut self, value: &mut Num, label: impl Into<egui::WidgetText>) -> egui::Response;
	fn num_edit_range<Num: egui::emath::Numeric>(&mut self, value: &mut Num, label: impl Into<egui::WidgetText>, range: std::ops::RangeInclusive<Num>) -> egui::Response;
	fn num_multi_edit<Num: egui::emath::Numeric>(&mut self, values: &mut [Num], label: impl Into<egui::WidgetText>) -> egui::Response;
	fn num_multi_edit_range<Num: egui::emath::Numeric>(&mut self, values: &mut [Num], label: impl Into<egui::WidgetText>, range: &[std::ops::RangeInclusive<Num>]) -> egui::Response;
	fn enum_combo<Enum: EnumTools + PartialEq>(&mut self, value: &mut Enum, label: impl Into<egui::WidgetText>);
	fn enum_combo_id<Enum: EnumTools + PartialEq>(&mut self, value: &mut Enum, id: impl std::hash::Hash);
	fn helptext(&mut self, text: impl Into<egui::WidgetText>);
	fn delete_button(&mut self, label: impl Into<egui::WidgetText>) -> egui::Response;
	fn dnd_header(&mut self, handle: egui_dnd::Handle, content: impl FnOnce(&mut egui::Ui));
}

impl RendererExtender for egui::Ui {
	fn texture(&mut self, img: egui::TextureHandle, max_size: impl Into<egui::Vec2>, uv: impl Into<egui::Rect>) -> egui::Response {
		let max_size = max_size.into();
		let uv: egui::Rect = uv.into();
		let size = img.size_vec2();
		let width = size.x * (uv.max.x - uv.min.x);
		let height = size.y * (uv.max.y - uv.min.y);
		let scale = (max_size.x / width).min(max_size.y / height);
		self.add(egui::Image::new(img.id(), egui::vec2(width * scale, height * scale)).uv(uv))
	}
	
	fn num_edit<Num: egui::emath::Numeric>(&mut self, value: &mut Num, label: impl Into<egui::WidgetText>) -> egui::Response {
		self.horizontal(|ui| {
			let resp = ui.add(create_drag(value));
			ui.label(label.into());
			resp
		}).inner
	}
	
	fn num_edit_range<Num: egui::emath::Numeric>(&mut self, value: &mut Num, label: impl Into<egui::WidgetText>, range: std::ops::RangeInclusive<Num>) -> egui::Response {
		self.horizontal(|ui| {
			let resp = ui.add(create_drag(value).clamp_range(range));
			ui.label(label.into());
			resp
		}).inner
	}
	
	fn num_multi_edit<Num: egui::emath::Numeric>(&mut self, values: &mut [Num], label: impl Into<egui::WidgetText>) -> egui::Response {
		self.horizontal(|ui| {
			let mut resp = ui.add(create_drag(&mut values[0]));
			for value in values.iter_mut().skip(1) {
				resp |= ui.add(create_drag(value));
			}
			ui.label(label.into());
			resp
		}).inner
	}
	
	fn num_multi_edit_range<Num: egui::emath::Numeric>(&mut self, values: &mut [Num], label: impl Into<egui::WidgetText>, range: &[std::ops::RangeInclusive<Num>]) -> egui::Response {
		self.horizontal(|ui| {
			let mut resp = ui.add(create_drag(&mut values[0]).clamp_range(range[0].clone()));
			for (i, value) in values.iter_mut().skip(1).enumerate() {
				resp |= ui.add(create_drag(value).clamp_range(range[i].clone()));
			}
			ui.label(label.into());
			resp
		}).inner
	}
	
	fn enum_combo<Enum: EnumTools + PartialEq>(&mut self, value: &mut Enum, label: impl Into<egui::WidgetText>) {
		egui::ComboBox::from_label(label)
			.selected_text(value.to_str())
			.show_ui(self, |ui| {
				for item in Enum::iter() {
					let name = item.to_str();
					ui.selectable_value(value, item, name);
				}
			});
	}
	
	fn enum_combo_id<Enum: EnumTools + PartialEq>(&mut self, value: &mut Enum, id: impl std::hash::Hash) {
		egui::ComboBox::from_id_source(id)
			.selected_text(value.to_str())
			.show_ui(self, |ui| {
				for item in Enum::iter() {
					let name = item.to_str();
					ui.selectable_value(value, item, name);
				}
			});
	}
	
	fn helptext(&mut self, text: impl Into<egui::WidgetText>) {
		self.label("❓").on_hover_text(text);
	}
	
	fn delete_button(&mut self, label: impl Into<egui::WidgetText>) -> egui::Response {
		self.horizontal(|ui| {
			let resp = ui.button("🗑");
			ui.label(label);
			resp
		}).inner
	}
	
	fn dnd_header(&mut self, handle: egui_dnd::Handle, content: impl FnOnce(&mut egui::Ui)) {
		let pos = self.next_widget_position();
		// let text = label.into().into_galley(self, Some(false), 99999.0, egui::TextStyle::Button);
		// let height = text.size().y + self.spacing().button_padding.y * 2.0;
		let height = <&str as Into<egui::WidgetText>>::into("Hi o/").into_galley(self, Some(false), 99999.0, egui::TextStyle::Button).size().y + self.spacing().button_padding.y * 2.0;
		if self.rect_contains_pointer(egui::Rect{min: pos, max: egui::pos2(pos.x + self.available_width(), pos.y + height)}) {
			handle.ui(self, content);
		} else {
			self.scope(content);
		}
	}
}

fn create_drag<Num: egui::emath::Numeric>(value: &mut Num) -> egui::DragValue {
	if Num::INTEGRAL {
		egui::DragValue::new(value)
	} else {
		egui::DragValue::new(value)
			.max_decimals(3)
			.speed(0.01)
	}
}

pub trait EnumTools {
	type Iterator: core::iter::Iterator<Item = Self>;
	
	fn to_str(&self) -> &'static str;
	fn to_string(&self) -> String {self.to_str().to_string()}
	fn iter() -> Self::Iterator;
}