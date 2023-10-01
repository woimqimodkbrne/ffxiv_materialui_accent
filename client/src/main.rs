use eframe::egui;

extern crate aetherment;

mod cli;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
	cli::handle_cli()?;
	
	// #[cfg(target_os = "windows")]
	// let backends = aetherment::Backends::DX12;
	// #[cfg(target_os = "linux")]
	// let backends = aetherment::Backends::VULKAN;
	// #[cfg(target_os = "macos")]
	// let backends = aetherment::Backends::METAL;
	
	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::Vec2::new(1280.0, 720.0)),
		wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
			// supported_backends: backends,
			// supported_backends: aetherment::Backends::all(),
			..Default::default()
		},
		..Default::default()
	};
	
	eframe::run_native("Aetherment", options, Box::new(|cc| {
		let _backend = cc.wgpu_render_state.as_ref().unwrap().adapter.get_info().backend;
		Box::new(CoreWrapper(aetherment::Core::new(log, cc.egui_ctx.clone(), aetherment::modman::backend::BackendInitializers::None)))
	}))?;
	
	Ok(())
}