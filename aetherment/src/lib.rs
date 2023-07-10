#[repr(u8)]
pub enum LogType {
	Log = 0,
	Error = 1,
	Fatal = 255,
}

static mut LOG: fn(LogType, String) = |_, _| {};
#[macro_export]
macro_rules! log {
	(ftl, $($e:tt)*) => {unsafe{crate::LOG(LogType::Fatal, format!($($e)*))}};
	(log, $($e:tt)*) => {unsafe{crate::LOG(LogType::Log, format!($($e)*))}};
	(err, $($e:tt)*) => {unsafe{crate::LOG(LogType::Error, format!($($e)*))}};
	($($e:tt)*) => {unsafe{crate::LOG(LogType::Log, format!($($e)*))}};
}

pub struct Core {
	texture: Option<egui::TextureHandle>,
	slider: i32,
	text: String,
}

impl Core {
	pub fn new(log: fn(LogType, String)) -> Self {
		unsafe {LOG = log};
		
		log!("hello!");
		
		Self {
			texture: None,
			slider: 11,
			text: String::from("hello"),
		}
	}
	
	pub fn draw(&mut self, ui: &mut egui::Ui) {
		let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
			// Load the texture only once.
			ui.ctx().load_texture(
				"my-image",
				egui::ColorImage::example(),
				Default::default()
			)
		});
		
		// Show the image:
		ui.image(texture, texture.size_vec2());
		
		ui.label("Hello there!");
		if ui.button("am button").hovered() {
			ui.label("hover");
		}
		ui.add(egui::Slider::new(&mut self.slider, 0..=100));
		ui.text_edit_singleline(&mut self.text);
		
		if ui.ctx().is_using_pointer() {
			ui.label("using");
		}
		
		if ui.ui_contains_pointer() {
			ui.label("contains");
		}
	}
}