use crate::gui::imgui;

pub struct Divider<'a> {
	id: &'a str,
	columns: Vec<(imgui::TableColumnFlags, f32, Box<dyn FnOnce() + 'a>)>,
}

impl<'a> Divider<'a> {
	pub fn column<F>(mut self, flags: imgui::TableColumnFlags, size: f32, func: F) -> Self where F: FnOnce() + 'a {
		self.columns.push((flags, size, Box::new(func)));
		self
	}
	
	pub fn finish(self) {
		imgui::begin_table(self.id, self.columns.len() as i32, imgui::TableFlags::Resizable, [0.0, 0.0], 0.0);
		self.columns.iter().enumerate().for_each(|(i, c)| imgui::table_setup_column(&format!("##{}", i), c.0, c.1, 0));
		imgui::table_next_row(imgui::TableRowFlags::None, 0.0);
		self.columns.into_iter().for_each(|c| {
			imgui::table_next_column();
			c.2();
		});
		imgui::end_table();
	}
}

pub fn divider(id: &str) -> Divider {
	Divider {
		id: id,
		columns: Vec::new(),
	}
}