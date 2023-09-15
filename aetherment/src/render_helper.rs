pub trait RendererExtender {
	fn texture(&mut self, img: egui::TextureHandle, max_size: impl Into<egui::Vec2>, uv: impl Into<egui::Rect>) -> egui::Response;
	fn num_edit<Num: egui::emath::Numeric>(&mut self, value: &mut Num, label: impl Into<egui::WidgetText>) -> egui::Response;
	fn num_edit_range<Num: egui::emath::Numeric>(&mut self, value: &mut Num, label: impl Into<egui::WidgetText>, range: std::ops::RangeInclusive<Num>) -> egui::Response;
	fn num_multi_edit<Num: egui::emath::Numeric>(&mut self, values: &mut [Num], label: impl Into<egui::WidgetText>) -> egui::Response;
	fn enum_combo<Enum: EnumTools + PartialEq>(&mut self, value: &mut Enum, label: impl Into<egui::WidgetText>);
	fn helptext(&mut self, text: impl Into<egui::WidgetText>);
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
			let resp = ui.add(egui::DragValue::new(value));
			ui.label(label.into());
			resp
		}).inner
	}
	
	fn num_edit_range<Num: egui::emath::Numeric>(&mut self, value: &mut Num, label: impl Into<egui::WidgetText>, range: std::ops::RangeInclusive<Num>) -> egui::Response {
		self.horizontal(|ui| {
			let resp = ui.add(egui::DragValue::new(value).clamp_range(range));
			ui.label(label.into());
			resp
		}).inner
	}
	
	fn num_multi_edit<Num: egui::emath::Numeric>(&mut self, values: &mut [Num], label: impl Into<egui::WidgetText>) -> egui::Response {
		self.horizontal(|ui| {
			let mut resp = ui.add(egui::DragValue::new(&mut values[0]));
			for value in values.iter_mut().skip(1) {
				resp |= ui.add(egui::DragValue::new(value));
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
	
	fn helptext(&mut self, text: impl Into<egui::WidgetText>) {
		self.label("❓").on_hover_text(text);
	}
}

pub trait EnumTools {
	type Iterator: core::iter::Iterator<Item = Self>;
	
	fn to_str(&self) -> &'static str;
	fn to_string(&self) -> String {self.to_str().to_string()}
	fn iter() -> Self::Iterator;
}