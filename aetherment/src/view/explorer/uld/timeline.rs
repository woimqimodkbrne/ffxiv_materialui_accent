use noumenon::format::game::uld;
use crate::render_helper::{RendererExtender, EnumTools};

pub fn render_frames(ui: &mut egui::Ui, frames: &mut Vec<uld::FrameData>) {
	let mut delete = None;
	for (i, frame) in frames.iter_mut().enumerate() {
		egui::CollapsingHeader::new(format!("({i}) {}->{}", frame.start_frame, frame.end_frame)).id_source(i).show(ui, |ui| {
		// ui.collapsing(format!("({i}) {}->{}", frame.start_frame, frame.end_frame), |ui| {
			render_framedata(ui, frame);
			ui.horizontal(|ui| {
				if ui.button("🗑").clicked() {
					delete = Some(i);
				}
				ui.label("Delete frame");
			});
		});
	}
	
	if let Some(i) = delete {
		frames.remove(i);
	}
	
	if ui.button("➕ Add new frame").clicked() {
		frames.push(uld::FrameData::default());
	}
}

fn render_framedata(ui: &mut egui::Ui, frame: &mut uld::FrameData) {
	ui.num_edit(&mut frame.start_frame, "Start Frame");
	ui.num_edit(&mut frame.end_frame, "End Frame");
	
	ui.collapsing("Keygroups", |ui| {
		let mut delete = None;
		for (i, keygroup) in frame.keygroups.iter_mut().enumerate() {
			egui::CollapsingHeader::new(format!("({i}) {}", keygroup.usage.to_str())).id_source(i).show(ui, |ui| {
			// ui.collapsing(format!("({i}) {}", keygroup.usage.to_str()), |ui| {
				ui.enum_combo(&mut keygroup.usage, "Usage");
				render_keyframes(ui, &mut keygroup.frames);
				ui.horizontal(|ui| {
					if ui.button("🗑").clicked() {
						delete = Some(i);
					}
					ui.label("Delete keygroup");
				});
			});
		}
		
		if let Some(i) = delete {
			frame.keygroups.remove(i);
		}
		
		if ui.button("➕ Add new keygroup").clicked() {
			frame.keygroups.push(uld::KeyGroup::default());
		}
	});
}

fn render_keyframes(ui: &mut egui::Ui, frames: &mut uld::Keyframes) {
	ui.enum_combo(frames, "Type");
	
	match frames {
		uld::Keyframes::Float1(k) => render_keyframes2(ui, k),
		uld::Keyframes::Float2(k) => render_keyframes2(ui, k),
		uld::Keyframes::Float3(k) => render_keyframes2(ui, k),
		uld::Keyframes::SByte1(k) => render_keyframes2(ui, k),
		uld::Keyframes::SByte2(k) => render_keyframes2(ui, k),
		uld::Keyframes::SByte3(k) => render_keyframes2(ui, k),
		uld::Keyframes::Byte1(k) => render_keyframes2(ui, k),
		uld::Keyframes::Byte2(k) => render_keyframes2(ui, k),
		uld::Keyframes::Byte3(k) => render_keyframes2(ui, k),
		uld::Keyframes::Short1(k) => render_keyframes2(ui, k),
		uld::Keyframes::Short2(k) => render_keyframes2(ui, k),
		uld::Keyframes::Short3(k) => render_keyframes2(ui, k),
		uld::Keyframes::UShort1(k) => render_keyframes2(ui, k),
		uld::Keyframes::UShort2(k) => render_keyframes2(ui, k),
		uld::Keyframes::UShort3(k) => render_keyframes2(ui, k),
		uld::Keyframes::Int1(k) => render_keyframes2(ui, k),
		uld::Keyframes::Int2(k) => render_keyframes2(ui, k),
		uld::Keyframes::Int3(k) => render_keyframes2(ui, k),
		uld::Keyframes::UInt1(k) => render_keyframes2(ui, k),
		uld::Keyframes::UInt2(k) => render_keyframes2(ui, k),
		uld::Keyframes::UInt3(k) => render_keyframes2(ui, k),
		uld::Keyframes::Bool1(k) => render_keyframes2(ui, k),
		uld::Keyframes::Bool2(k) => render_keyframes2(ui, k),
		uld::Keyframes::Bool3(k) => render_keyframes2(ui, k),
		uld::Keyframes::Color(k) => render_keyframes2(ui, k),
		uld::Keyframes::Label(k) => render_keyframes2(ui, k),
	}
}

fn render_keyframes2<R: KeyFrameRenderer + Default>(ui: &mut egui::Ui, frames: &mut Vec<R>) {
	let mut delete = None;
	for (i, frame) in frames.iter_mut().enumerate() {
		let base = frame.get_base();
		egui::CollapsingHeader::new(format!("({i}) {}+{}", base.time, base.offset)).id_source(i).show(ui, |ui| {
		// ui.collapsing(format!("({i}) {}+{}", base.time, base.offset), |ui| {
			frame.render_fully(ui);
			ui.horizontal(|ui| {
				if ui.button("🗑").clicked() {
					delete = Some(i);
				}
				ui.label("Delete keyframe");
			});
		});
	}
	
	if let Some(i) = delete {
		frames.remove(i);
	}
	
	ui.separator();
	
	if ui.button("➕ Add new keyframe").clicked() {
		frames.push(R::default());
	}
}

trait KeyFrameRenderer {
	fn render(&mut self, ui: &mut egui::Ui);
	fn get_base(&mut self) -> &mut uld::BaseKeyframeData;
	fn render_fully(&mut self, ui: &mut egui::Ui) {
		let base = self.get_base();
		ui.num_edit(&mut base.time, "Time");
		ui.num_edit(&mut base.offset, "Offset");
		ui.num_edit(&mut base.interpolation, "Interpolation");
		ui.num_edit(&mut base.unk1, "Unknown 1");
		ui.num_edit(&mut base.acceleration, "Acceleration");
		ui.num_edit(&mut base.deceleration, "Deceleration");
		self.render(ui);
	}
}

macro_rules! impl_renderer {
	(s, $type:ty) => {
		impl KeyFrameRenderer for $type {
			fn get_base(&mut self) -> &mut uld::BaseKeyframeData {&mut self.keyframe}
			fn render(&mut self, ui: &mut egui::Ui) {
				ui.num_edit(&mut self.value, "Value");
			}
		}
	};
	
	(m, $type:ty) => {
		impl KeyFrameRenderer for $type {
			fn get_base(&mut self) -> &mut uld::BaseKeyframeData {&mut self.keyframe}
			fn render(&mut self, ui: &mut egui::Ui) {
				ui.num_multi_edit(&mut self.value, "Value");
			}
		}
	};
}

impl_renderer!(s, uld::Float1Keyframe);
impl_renderer!(m, uld::Float2Keyframe);
impl_renderer!(m, uld::Float3Keyframe);
impl_renderer!(s, uld::SByte1Keyframe);
impl_renderer!(m, uld::SByte2Keyframe);
impl_renderer!(m, uld::SByte3Keyframe);
impl_renderer!(s, uld::Byte1Keyframe);
impl_renderer!(m, uld::Byte2Keyframe);
impl_renderer!(m, uld::Byte3Keyframe);
impl_renderer!(s, uld::Short1Keyframe);
impl_renderer!(m, uld::Short2Keyframe);
impl_renderer!(m, uld::Short3Keyframe);
impl_renderer!(s, uld::UShort1Keyframe);
impl_renderer!(m, uld::UShort2Keyframe);
impl_renderer!(m, uld::UShort3Keyframe);
impl_renderer!(s, uld::Int1Keyframe);
impl_renderer!(m, uld::Int2Keyframe);
impl_renderer!(m, uld::Int3Keyframe);
impl_renderer!(s, uld::UInt1Keyframe);
impl_renderer!(m, uld::UInt2Keyframe);
impl_renderer!(m, uld::UInt3Keyframe);

impl KeyFrameRenderer for uld::Bool1Keyframe {
	fn get_base(&mut self) -> &mut uld::BaseKeyframeData {&mut self.keyframe}
	fn render(&mut self, ui: &mut egui::Ui) {
		ui.checkbox(&mut self.value, "Value");
	}
}

impl KeyFrameRenderer for uld::Bool2Keyframe {
	fn get_base(&mut self) -> &mut uld::BaseKeyframeData {&mut self.keyframe}
	fn render(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.add(egui::Checkbox::without_text(&mut self.value[0]));
			ui.checkbox(&mut self.value[1], "Value");
		});
	}
}

impl KeyFrameRenderer for uld::Bool3Keyframe {
	fn get_base(&mut self) -> &mut uld::BaseKeyframeData {&mut self.keyframe}
	fn render(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.add(egui::Checkbox::without_text(&mut self.value[0]));
			ui.add(egui::Checkbox::without_text(&mut self.value[1]));
			ui.checkbox(&mut self.value[2], "Value");
		});
	}
}

impl KeyFrameRenderer for uld::ColorKeyframe {
	fn get_base(&mut self) -> &mut uld::BaseKeyframeData {&mut self.keyframe}
	fn render(&mut self, ui: &mut egui::Ui) {
		ui.num_edit(&mut self.multiply_red, "Multiply Red");
		ui.num_edit(&mut self.multiply_green, "Multiply Green");
		ui.num_edit(&mut self.multiply_blue, "Multiply Blue");
		ui.num_edit(&mut self.add_red, "Add Red");
		ui.num_edit(&mut self.add_green, "Add Green");
		ui.num_edit(&mut self.add_blue, "Add Blue");
	}
}

impl KeyFrameRenderer for uld::LabelKeyframe {
	fn get_base(&mut self) -> &mut uld::BaseKeyframeData {&mut self.keyframe}
	fn render(&mut self, ui: &mut egui::Ui) {
		ui.num_edit(&mut self.label_id, "Label ID");
		ui.num_edit(&mut self.label_command, "Label Command");
		ui.num_edit(&mut self.jump_id, "Jump ID");
	}
}