use noumenon::format::game::uld;
use crate::render_helper::RendererExtender;

pub fn render_component(ui: &mut egui::Ui, component: &mut uld::Component) {
	match component {
		uld::Component::Custom(c) => {
			ui.num_multi_edit(c, "Unknown");
		}
		
		uld::Component::Button(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Window(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::CheckBox(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::RadioButton(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Gauge(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			ui.checkbox(&mut c.is_vertical, "Vertical");
			ui.num_edit(&mut c.vertical_margin, "Vertical margin");
			ui.num_edit(&mut c.horizontal_margin, "Horizontal margin");
		}
		
		uld::Component::Slider(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			ui.checkbox(&mut c.is_vertical, "Vertical");
			ui.num_edit(&mut c.left_offset, "Left offset");
			ui.num_edit(&mut c.right_offset, "Right offset");
		}
		
		uld::Component::TextInput(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			
			ui.horizontal(|ui| {
				ui.color_edit_button_srgba(unsafe{std::mem::transmute(&mut c.color)});
				ui.label("Color");
			});
			
			ui.horizontal(|ui| {
				ui.color_edit_button_srgba(unsafe{std::mem::transmute(&mut c.ime_color)});
				ui.label("IME Color");
			});
		}
		
		uld::Component::NumericInput(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			
			ui.horizontal(|ui| {
				// TODO: this color picker is kinda shit, it doesnt even support manual input
				ui.color_edit_button_srgba(unsafe{std::mem::transmute(&mut c.color)});
				ui.label("Color");
			});
		}
		
		uld::Component::List(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			ui.num_edit(&mut c.wrap, "Wrap");
			ui.num_edit(&mut c.orientation, "Orientation");
		}
		
		uld::Component::DropDown(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Tabbed(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::TreeList(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			
			ui.num_edit(&mut c.wrap, "Wrap");
			ui.num_edit(&mut c.orientation, "Orientation");
		}
		
		uld::Component::ScrollBar(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			ui.checkbox(&mut c.is_vertical, "Vertical");
			ui.num_edit(&mut c.margin, "Margin");
		}
		
		uld::Component::ListItem(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Icon(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::IconWithText(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::DragDrop(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::LeveCard(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::NineGridText(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Journal(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
			ui.num_edit(&mut c.margin, "Margin");
			ui.num_edit(&mut c.unk1, "Unk1");
			ui.num_edit(&mut c.unk2, "Unk2");
		}
		
		uld::Component::Multipurpose(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Map(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Preview(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
		
		uld::Component::Unknown25(c) => {
			ui.num_multi_edit(&mut c.unk, "Unknown");
		}
	}
}