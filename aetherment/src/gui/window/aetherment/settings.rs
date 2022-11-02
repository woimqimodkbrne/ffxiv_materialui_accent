use imgui::aeth::{DrawList, F2};
use crate::gui::aeth;

pub struct Tab {
	test: [f32; 2],
}

impl Tab {
	pub fn new(_state: &mut crate::Data) -> Self {
		Tab {
			test: [500.0; 2],
		}
	}
	
	pub fn draw(&mut self, state: &mut crate::Data) {
		state.config.mark_for_changes();
		
		aeth::tab_bar("settings_tabs")
			.dock_top()
			.tab("Generic", || {
				aeth::child("generic", [0.0, -imgui::get_style().item_spacing[1]], false, imgui::WindowFlags::None, || {
					imgui::text("generic");
					
					imgui::slider_float2("area", &mut self.test, 0.0, 3000.0, "%.0f", imgui::SliderFlags::None);
					
					let draw = imgui::get_window_draw_list();
					let pos = imgui::get_cursor_screen_pos();
					let text = r#"Quick six blind smart out burst. Perfectly on furniture dejection determine my depending an to. Add short water court fat. Her bachelor honoured perceive securing but desirous ham required. Questions deficient acuteness to engrossed as. Entirely led ten humoured greatest and yourself. Besides ye country on observe. She continue appetite endeavor she judgment interest the met. For she surrounded motionless fat resolution may.

Why end might ask civil again spoil. She dinner she our horses depend. Remember at children by reserved to vicinity. In affronting unreserved delightful simplicity ye. Law own advantage furniture continual sweetness bed agreeable perpetual. Oh song well four only head busy it. Afford son she had lively living. Tastes lovers myself too formal season our valley boy. Lived it their their walls might to by young.

Moments its musical age explain. But extremity sex now education concluded earnestly her continual. Oh furniture acuteness suspected continual ye something frankness. Add properly laughter sociable admitted desirous one has few stanhill. Opinion regular in perhaps another enjoyed no engaged he at. It conveying he continual ye suspected as necessary. Separate met packages shy for kindness.

Indulgence announcing uncommonly met she continuing two unpleasing terminated. Now busy say down the shed eyes roof paid her. Of shameless collected suspicion existence in. Share walls stuff think but the arise guest. Course suffer to do he sussex it window advice. Yet matter enable misery end extent common men should. Her indulgence but assistance favourable cultivated everything collecting.

Chapter too parties its letters nor. Cheerful but whatever ladyship disposed yet judgment. Lasted answer oppose to ye months no esteem. Branched is on an ecstatic directly it. Put off continue you denoting returned juvenile. Looked person sister result mr to. Replied demands charmed do viewing ye colonel to so. Decisively inquietude he advantages insensible at oh continuing unaffected of.

Good draw knew bred ham busy his hour. Ask agreed answer rather joy nature admire wisdom. Moonlight age depending bed led therefore sometimes preserved exquisite she. An fail up so shot leaf wise in. Minuter highest his arrived for put and. Hopes lived by rooms oh in no death house. Contented direction september but end led excellent ourselves may. Ferrars few arrival his offered not charmed you. Offered anxious respect or he. On three thing chief years in money arise of.

Performed suspicion in certainty so frankness by attention pretended. Newspaper or in tolerably education enjoyment. Extremity excellent certainty discourse sincerity no he so resembled. Joy house worse arise total boy but. Elderly up chicken do at feeling is. Like seen drew no make fond at on rent. Behaviour extremely her explained situation yet september gentleman are who. Is thought or pointed hearing he.

Warmly little before cousin sussex entire men set. Blessing it ladyship on sensible judgment settling outweigh. Worse linen an of civil jokes leave offer. Parties all clothes removal cheered calling prudent her. And residence for met the estimable disposing. Mean if he they been no hold mr. Is at much do made took held help. Latter person am secure of estate genius at.

Yet remarkably appearance get him his projection. Diverted endeavor bed peculiar men the not desirous. Acuteness abilities ask can offending furnished fulfilled sex. Warrant fifteen exposed ye at mistake. Blush since so in noisy still built up an again. As young ye hopes no he place means. Partiality diminution gay yet entreaties admiration. In mr it he mention perhaps attempt pointed suppose. Unknown ye chamber of warrant of norland arrived.

Improve ashamed married expense bed her comfort pursuit mrs. Four time took ye your as fail lady. Up greatest am exertion or marianne. Shy occasional terminated insensible and inhabiting gay. So know do fond to half on. Now who promise was justice new winding. In finished on he speaking suitable advanced if. Boy happiness sportsmen say prevailed offending concealed nor was provision. Provided so as doubtful on striking required. Waiting we to compass assured."#;
					draw.add_text_area(pos, 0xFFFFFFFF, text, self.test);
					draw.add_rect(pos, pos.add(self.test), 0xFFFFFFFF, 0.0, imgui::DrawFlags::None, 2.0);
				});
			})
			.tab("Advanced", || {
				aeth::child("advanced", [0.0, -imgui::get_style().item_spacing[1]], false, imgui::WindowFlags::None, || {
					imgui::input_text("Local Mod Directory", &mut state.config.local_path, imgui::InputTextFlags::None);
					imgui::checkbox("File Explorer", &mut state.config.tab_explorer);
					imgui::checkbox("Mod Development", &mut state.config.tab_moddev);
				});
			})
			.finish();
		
		state.config.save().unwrap();
	}
}