use eframe::egui;

extern crate aetherment;

fn log(typ: aetherment::LogType, msg: String) {
	let typ = match typ {
		aetherment::LogType::Log => "LOG",
		aetherment::LogType::Error => "ERROR",
		aetherment::LogType::Fatal => "FATAL",
	};
	
	println!("[{typ}] {msg}");
}

struct CoreWrapper(aetherment::Core);

impl eframe::App for CoreWrapper {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().frame(egui::Frame {
			inner_margin: egui::Margin::same(0.0),
			outer_margin: egui::Margin::same(0.0),
			rounding: egui::Rounding::none(),
			shadow: egui::epaint::Shadow::NONE,
			fill: egui::Color32::TRANSPARENT,
			stroke: egui::Stroke::NONE,
		}).show(&ctx, |ui| self.0.draw(ui));
	}
 }

fn main() -> eframe::Result<()> {
	let options = eframe::NativeOptions {
		..Default::default()
	};
	
	eframe::run_native("Aetherment", options, Box::new(|cc| Box::new(CoreWrapper(aetherment::Core::new(log, cc.egui_ctx.clone())))))
}