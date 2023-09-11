use std::sync::Mutex;
use windows::{
	s,
	Win32::{
		Foundation::{HWND, WPARAM, LPARAM, LRESULT},
		UI::{WindowsAndMessaging::{SetWindowLongPtrA, GetWindowLongPtrA, CallWindowProcA, FindWindowExA, GWLP_WNDPROC, WM_CHAR, WM_SYSCHAR, WM_KEYDOWN, WM_SYSKEYDOWN, WM_KEYUP, WM_SYSKEYUP, WM_NCACTIVATE, WM_SETFOCUS, WM_MOUSEMOVE, /*WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP,*/ WM_MOUSEWHEEL, WHEEL_DELTA, WM_MOUSEHWHEEL}, Controls::WM_MOUSELEAVE}
	}
};

static mut ORGWNDPROC: Option<unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT> = None;
static SURROGATE: Mutex<Option<u16>> = Mutex::new(None);
pub static EVENTS: Mutex<Vec<egui::Event>> = Mutex::new(Vec::new());
pub static MODIFIERS: Mutex<egui::Modifiers> = Mutex::new(egui::Modifiers{alt: false, ctrl: false, shift: false, mac_cmd: false, command: false});
pub static POS: Mutex<egui::Pos2> = Mutex::new(egui::pos2(0.0, 0.0));

// TODO: move all events here
fn convert_key(key: u16) -> Option<egui::Key> {
	use egui::Key::*;
	match key {
		0x08 => Some(Backspace),
		0x09 => Some(Tab),
		0x0D => Some(Enter),
		0x18 => Some(Escape),
		0x20 => Some(Space),
		
		0x21 => Some(PageUp),
		0x22 => Some(PageDown),
		0x24 => Some(Home),
		0x23 => Some(End),
		0x2D => Some(Insert),
		0x2E => Some(Delete),
		
		0x25 => Some(ArrowLeft),
		0x26 => Some(ArrowUp),
		0x27 => Some(ArrowRight),
		0x28 => Some(ArrowDown),
		
		0x30 => Some(Num0),
		0x31 => Some(Num1),
		0x32 => Some(Num2),
		0x33 => Some(Num3),
		0x34 => Some(Num4),
		0x35 => Some(Num5),
		0x36 => Some(Num6),
		0x37 => Some(Num7),
		0x38 => Some(Num8),
		0x39 => Some(Num9),
		
		0x41 => Some(A),
		0x42 => Some(B),
		0x43 => Some(C),
		0x44 => Some(D),
		0x45 => Some(E),
		0x46 => Some(F),
		0x47 => Some(G),
		0x48 => Some(H),
		0x49 => Some(I),
		0x4A => Some(J),
		0x4B => Some(K),
		0x4C => Some(L),
		0x4D => Some(M),
		0x4E => Some(N),
		0x4F => Some(O),
		0x50 => Some(P),
		0x51 => Some(Q),
		0x52 => Some(R),
		0x53 => Some(S),
		0x54 => Some(T),
		0x55 => Some(U),
		0x56 => Some(V),
		0x57 => Some(W),
		0x58 => Some(X),
		0x59 => Some(Y),
		0x5A => Some(Z),
		
		0x70 => Some(F1),
		0x71 => Some(F2),
		0x72 => Some(F3),
		0x73 => Some(F4),
		0x74 => Some(F5),
		0x75 => Some(F6),
		0x76 => Some(F7),
		0x77 => Some(F8),
		0x78 => Some(F9),
		0x79 => Some(F10),
		0x7A => Some(F11),
		0x7B => Some(F12),
		0x7C => Some(F13),
		0x7D => Some(F14),
		0x7E => Some(F15),
		0x7F => Some(F16),
		0x80 => Some(F17),
		0x81 => Some(F18),
		0x82 => Some(F19),
		0x83 => Some(F20),
		
		0xBD => Some(Minus),
		0xBB => Some(PlusEquals),
		_ => None,
	}
}

// fn key_event(button: egui::PointerButton, pressed: bool) {
// 	EVENTS.lock().unwrap().push(egui::Event::PointerButton {
// 		pos: POS.lock().unwrap().clone(),
// 		button,
// 		pressed,
// 		modifiers: MODIFIERS.lock().unwrap().clone(),
// 	});
// }

// https://github.com/emilk/egui/blob/9478e50d012c5138551c38cbee16b07bc1fcf283/crates/egui-winit/src/lib.rs#L719
fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}' <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';
	
    !is_in_private_use_area && !chr.is_ascii_control()
}

fn wndproc(hwnd: HWND, umsg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	match umsg {
		WM_NCACTIVATE => {
			if wparam.0 != 0 {
				*MODIFIERS.lock().unwrap() = egui::Modifiers::default();
			}
		}
		
		WM_SETFOCUS => {
			*MODIFIERS.lock().unwrap() = egui::Modifiers::default();
		}
		
		WM_MOUSEMOVE => {
			let mut pos = POS.lock().unwrap();
			pos.x = (lparam.0 as u32 & 0xFFFF) as f32;
			pos.y = ((lparam.0 as u32 >> 16) & 0xFFFF) as f32;
			EVENTS.lock().unwrap().push(egui::Event::PointerMoved(pos.clone()));
		}
		
		WM_MOUSELEAVE => {
			EVENTS.lock().unwrap().push(egui::Event::PointerGone);
		}
		
		// can't use these as dalamud captures them and doesn't pass them along
		// WM_LBUTTONDOWN => key_event(egui::PointerButton::Primary, true),
		// WM_LBUTTONUP => key_event(egui::PointerButton::Primary, false),
		// WM_RBUTTONDOWN => key_event(egui::PointerButton::Secondary, true),
		// WM_RBUTTONUP => key_event(egui::PointerButton::Secondary, false),
		// WM_MBUTTONDOWN => key_event(egui::PointerButton::Middle, true),
		// WM_MBUTTONUP => key_event(egui::PointerButton::Middle, false),
		// WM_XBUTTONDOWN => {
		// 	match (wparam.0 as u32 >> 16) & 0xFFFF {
		// 		1 => key_event(egui::PointerButton::Extra1, true),
		// 		2 => key_event(egui::PointerButton::Extra2, true),
		// 		_ => {},
		// 	}
		// }
		// WM_XBUTTONUP => {
		// 	match (wparam.0 as u32 >> 16) & 0xFFFF {
		// 		1 => key_event(egui::PointerButton::Extra1, false),
		// 		2 => key_event(egui::PointerButton::Extra2, false),
		// 		_ => {},
		// 	}
		// }
		
		WM_MOUSEWHEEL => {
			crate::log(aetherment::LogType::Log, format!("wheel"));
			EVENTS.lock().unwrap().push(egui::Event::MouseWheel {
				unit: egui::MouseWheelUnit::Line,
				delta: egui::vec2(0.0, ((wparam.0 >> 16) & 0xFFFF) as f32 / WHEEL_DELTA as f32),
				modifiers: MODIFIERS.lock().unwrap().clone(),
			});
		}
		
		WM_MOUSEHWHEEL => {
			EVENTS.lock().unwrap().push(egui::Event::MouseWheel {
				unit: egui::MouseWheelUnit::Line,
				delta: egui::vec2(-(((wparam.0 >> 16) & 0xFFFF) as f32) / WHEEL_DELTA as f32, 0.0),
				modifiers: MODIFIERS.lock().unwrap().clone(),
			});
		}
		
		WM_KEYDOWN | WM_SYSKEYDOWN => {
			// if (lparam.0 >> 30) & 1 == 0 {
				let mut mods = MODIFIERS.lock().unwrap();
				match wparam.0 as u16 {
					0x12 => mods.alt = true,
					0x11 => {mods.ctrl = true; mods.command = true},
					0x10 => mods.shift = true,
					key => {
						if let Some(key) = convert_key(key) {
							EVENTS.lock().unwrap().push(egui::Event::Key {
								key,
								pressed: true,
								// repeat: false,
								// it says to just put this to false as it manually does it but thats just a lie
								repeat: (lparam.0 >> 30) & 1 == 0,
								modifiers: mods.clone(),
							});
						}
					},
				}
			// }
		}
		
		WM_KEYUP | WM_SYSKEYUP => {
			// if (lparam.0 >> 30) & 1 == 0 {
				let mut mods = MODIFIERS.lock().unwrap();
				match wparam.0 as u16 {
					0x12 => mods.alt = false,
					0x11 => {mods.ctrl = false; mods.command = false},
					0x10 => mods.shift = false,
					key => {
						if let Some(key) = convert_key(key) {
							EVENTS.lock().unwrap().push(egui::Event::Key {
								key,
								pressed: false,
								repeat: false,
								modifiers: mods.clone(),
							});
						}
					},
				}
			// }
		}
		
		WM_CHAR | WM_SYSCHAR => {
			// https://github.com/rust-windowing/winit/blob/640c51fe6f05b790ff1825eedc18703cbc4c5175/src/platform_impl/windows/event_loop.rs#L1256
			let is_high_surrogate = (0xD800..=0xDBFF).contains(&wparam.0);
			let is_low_surrogate = (0xDC00..=0xDFFF).contains(&wparam.0);
			
			if is_high_surrogate {
				*SURROGATE.lock().unwrap() = Some(wparam.0 as u16)
			} else if is_low_surrogate {
				let high_surrogate = SURROGATE.lock().unwrap().take();
				
				if let Some(high_surrogate) = high_surrogate {
					let pair = [high_surrogate, wparam.0 as u16];
					if let Some(Ok(chr)) = char::decode_utf16(pair.iter().copied()).next() {
						if is_printable_char(chr) {
							EVENTS.lock().unwrap().push(egui::Event::Text(chr.to_string()))
						}
					}
				}
			} else {
				*SURROGATE.lock().unwrap() = None;
				
				if let Some(chr) = char::from_u32(wparam.0 as u32) {
					if is_printable_char(chr) {
						EVENTS.lock().unwrap().push(egui::Event::Text(chr.to_string()))
					}
				}
			}
		}
		
		// TODO: IME/character composition!!
		// egui::Event::CompositionStart
		// egui::Event::CompositionUpdate
		// egui::Event::CompositionEnd
		
		_ => {}
	}
	
	unsafe{CallWindowProcA(ORGWNDPROC, hwnd, umsg, wparam, lparam)}
}

fn get_hwnd() -> HWND {
	// let id = std::process::id();
	unsafe{FindWindowExA(None, HWND(0), s!("FFXIVGAME"), None)}
}

pub fn hook() {
	unsafe {
		let hwnd = get_hwnd();
		ORGWNDPROC = Some(std::mem::transmute(GetWindowLongPtrA(hwnd, GWLP_WNDPROC) as *const ()));
		SetWindowLongPtrA(hwnd, GWLP_WNDPROC, wndproc as isize);
	}
}

pub fn revert() {
	unsafe {
		if let Some(ptr) = ORGWNDPROC.take() {
			SetWindowLongPtrA(get_hwnd(), GWLP_WNDPROC, ptr as isize);
		}
	}
}