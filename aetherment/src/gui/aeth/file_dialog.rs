#[repr(u8)]
pub enum FileDialogMode {
	OpenFile = 0,
	SaveFile = 1,
}

#[repr(u8)]
pub enum FileDialogStatus {
	Failed = 0,
	Success = 1,
	Busy = 2,
}

pub enum FileDialogResult {
	Failed,
	Success(String),
	Busy,
}

pub(in crate) static mut FILEDIALOG: fn(FileDialogMode, String, String, String, &mut String) -> FileDialogStatus = |_, _, _, _, _| {FileDialogStatus::Failed};
pub fn file_dialog(mode: FileDialogMode, title: String, name: String, extensions: Vec<String>) -> FileDialogResult {
	let mut path = String::with_capacity(256);
	match unsafe{FILEDIALOG(mode, title, name, extensions.join(","), &mut path)} {
		FileDialogStatus::Failed => FileDialogResult::Failed,
		FileDialogStatus::Success => FileDialogResult::Success(path),
		FileDialogStatus::Busy => FileDialogResult::Busy
	}
}

pub fn file_picker<S>(mode: FileDialogMode, title: S, name: S, extensions: Vec<String>, path: &mut String) -> bool
where S: Into<String> {
	let title = title.into();
	let picking = imgui::get_state_storage().i32(imgui::get_id(&title), 0);
	imgui::push_id(&title);
	// imgui::input_text(&title, path, imgui::InputTextFlags::ReadOnly);
	super::button_icon("");
	imgui::pop_id();
	if imgui::is_item_clicked(imgui::MouseButton::Left) {
		log!("click");
		*picking = 1;
	}
	if *picking == 1 {
		match file_dialog(mode, title, name.into(), extensions.into()) {
			FileDialogResult::Failed => {
				*picking = 0;
				false
			},
			FileDialogResult::Success(p) => {
				*picking = 0;
				*path = p;
				true
			},
			FileDialogResult::Busy => {
				false
			},
		}
	} else {false}
}