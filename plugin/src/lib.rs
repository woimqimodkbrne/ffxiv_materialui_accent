#![allow(improper_ctypes_definitions)]

use std::mem::transmute;
use egui::epaint::ahash::HashMap;

extern crate aetherment;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod imgui;
// mod handle;
mod wndproc;
mod texture;

static mut LOG: fn(u8, *mut i8, usize) = |_, _, _| {};
fn log(typ: aetherment::LogType, msg: String) {
	use std::ffi::CString;
	let len = msg.len();
	unsafe{crate::LOG(typ as _, CString::into_raw(CString::new(msg).unwrap()), len)};
}

extern "C" {fn GetKeyboardState(key_states: *mut [u8; 256]);}
fn get_keyboard_state() -> [bool; 256] {
	let mut key_states = [0; 256];
	unsafe{GetKeyboardState(&mut key_states)};
	let key_states: [bool; 256] = key_states.into_iter().map(|v| v > 1).collect::<Vec<bool>>().try_into().unwrap();
	key_states
}

pub struct State {
	// egui
	ctx: egui::Context,
	start: std::time::Instant,
	last_key_states: [bool; 256],
	
	// scuffed texture handler
	textures: HashMap<u64, (texture::Texture, Vec<u8>)>,
	free_textures: Vec::<egui::TextureId>,
	blank_texture: texture::Texture,
	
	// // wgpu
	// _handle: handle::HWNDHandle,
	// device: wgpu::Device,
	// queue: wgpu::Queue,
	// bind_group_layout: wgpu::BindGroupLayout,
	// samplers: HashMap<epaint::textures::TextureOptions, wgpu::Sampler>,
	// textures: HashMap<egui::TextureId, (Option<wgpu::Texture>, wgpu::BindGroup)>,
	// free_textures: Vec::<egui::TextureId>,
	
	// other
	visible: bool,
	core: aetherment::Core,
}

#[repr(packed)]
pub struct Initializers {
	// log: fn(u8, *mut u8, usize, usize),
	log: fn(u8, *mut i8, usize),
	
	create_texture: fn(texture::TextureOptions) -> usize,
	drop_texture: fn(usize),
	pin_texture: fn(usize) -> *mut u8,
	unpin_texture: fn(usize),
}

#[no_mangle]
pub extern fn initialize(init: Initializers) -> *mut State {
	std::panic::set_hook(Box::new(|info| {
		log(aetherment::LogType::Error, format!("{}", info));
	}));
	
	match std::panic::catch_unwind(|| {
		unsafe {
			LOG = init.log;
			
			texture::CREATE = init.create_texture;
			texture::DROP = init.drop_texture;
			texture::PIN = init.pin_texture;
			texture::UNPIN = init.unpin_texture;
		};
		
		wndproc::hook();
		
		let ctx = egui::Context::default();
		Box::into_raw(Box::new(State {
			ctx: ctx.clone(),
			start: std::time::Instant::now(),
			last_key_states: get_keyboard_state(),
			
			textures: HashMap::default(),
			free_textures: Vec::new(),
			blank_texture: {
				// null pointer doesnt work as "no texture", so this it is
				let mut tex = texture::Texture::new(texture::TextureOptions {
					width: 1,
					height: 1,
					format: texture::TextureFormat::R8g8b8a8Unorm,
					usage: texture::TextureUsage::Dynamic,
					cpu_access_flags: texture::TextureCpuFlags::Write,
				});
				
				_ = tex.draw_to(&[255, 255, 255, 255]);
				
				tex
			},
			
			visible: true,
			core: aetherment::Core::new(log, ctx, aetherment::Backends::DX12),
		}))
	}) {
		Ok(v) => v,
		Err(_) => 0 as *mut _,
	}
}

// #[no_mangle]
// pub extern fn destroy_string(ptr: *mut u8, len: usize, cap: usize) {
// 	unsafe{String::from_raw_parts(ptr, len, cap)};
// }

#[no_mangle]
pub extern fn destroy(state: *mut State) {
	wndproc::revert();
	
	_ = unsafe{Box::from_raw(state)};
}

#[no_mangle]
pub extern fn destroy_string(ptr: *mut i8) {
	_ = unsafe{std::ffi::CString::from_raw(ptr)};
}

#[no_mangle]
pub extern fn command(state: *mut State, args: &str) {
	let state = unsafe{&mut *state};
	
	match args {
		_ => state.visible = !state.visible,
	}
}

#[no_mangle]
pub extern fn draw(state: *mut State) {
	let state = state as usize;
	
	std::panic::catch_unwind(|| {
		let state = unsafe{&mut *(state as *mut State)};
		
		let key_states = get_keyboard_state();
		let mut procevents = wndproc::EVENTS.lock().unwrap();
		let mut events = procevents.clone();
		procevents.clear();
		
		let mut pos = [0.0; 2];
		let mut size = [0.0; 2];
		
		unsafe {
			imgui::igSetNextWindowSize(transmute([1280.0f32, 720.0]), imgui::ImGuiCond_FirstUseEver);
			// imgui::igSetNextWindowPos(transmute([0.0f32, 0.0]), imgui::ImGuiCond_Appearing, transmute([0.0f32, 0.0]));
			let name = std::ffi::CString::new("Aetherment").unwrap();
			imgui::igBegin(name.as_ptr(), &mut state.visible, 0);
			
			imgui::igGetCursorScreenPos(&mut pos as *mut _ as *mut _);
			imgui::igGetContentRegionAvail(&mut size as *mut _ as *mut _);
			
			imgui::igInvisibleButton(name.as_ptr(), *imgui::ImVec2_ImVec2_Float(size[0], size[1]), 0);
		}
		
		let modifiers = wndproc::MODIFIERS.lock().unwrap().clone();
		let mouse_pos = wndproc::POS.lock().unwrap().clone();
		let down = key_states[0x01];
		if down != state.last_key_states[0x01] {
			events.push(egui::Event::PointerButton {
				pos: mouse_pos,
				button: egui::PointerButton::Primary,
				pressed: down,
				modifiers,
			});
		}
		
		let down = key_states[0x02];
		if down != state.last_key_states[0x02] {
			events.push(egui::Event::PointerButton {
				pos: mouse_pos,
				button: egui::PointerButton::Secondary,
				pressed: down,
				modifiers,
			});
		}
		
		let down = key_states[0x04];
		if down != state.last_key_states[0x04] {
			events.push(egui::Event::PointerButton {
				pos: mouse_pos,
				button: egui::PointerButton::Middle,
				pressed: down,
				modifiers,
			});
		}
		
		let down = key_states[0x05];
		if down != state.last_key_states[0x05] {
			events.push(egui::Event::PointerButton {
				pos: mouse_pos,
				button: egui::PointerButton::Extra1,
				pressed: down,
				modifiers,
			});
		}
		
		let down = key_states[0x06];
		if down != state.last_key_states[0x06] {
			events.push(egui::Event::PointerButton {
				pos: mouse_pos,
				button: egui::PointerButton::Extra2,
				pressed: down,
				modifiers,
			});
		}
		
		state.last_key_states = key_states;
		
		let now = std::time::Instant::now();
		let raw_input = egui::RawInput {
			screen_rect: Some(egui::Rect {
				min: egui::pos2(pos[0], pos[1]),
				max: egui::pos2(pos[0], pos[1]) + egui::vec2(size[0], size[1]),
			}),
			pixels_per_point: Some(1.0),
			max_texture_side: None,
			time: Some(now.duration_since(state.start).as_secs_f64()),
			predicted_dt: 0.0, // egui calculates dt as we provide time
			modifiers,
			events,
			hovered_files: Vec::new(),
			dropped_files: Vec::new(),
			focused: true,
		};
		
		// TODO: apply imgui style to egui
		let out = state.ctx.run(raw_input, |ctx| {
			egui::CentralPanel::default().frame(egui::Frame {
				inner_margin: egui::Margin::same(0.0),
				outer_margin: egui::Margin::same(0.0),
				rounding: egui::Rounding::none(),
				shadow: egui::epaint::Shadow::NONE,
				fill: egui::Color32::TRANSPARENT,
				stroke: egui::Stroke::NONE,
			}).show(&ctx, |ui| state.core.draw(ui));
		});
		
		// Handle textures
		for id in &state.free_textures {
			if let egui::TextureId::Managed(id) = id {
				state.textures.remove(id);
			}
		}
		state.free_textures = out.textures_delta.free;
		
		for (id, img_delta) in &out.textures_delta.set {
			// img_delta.options
			if let egui::TextureId::Managed(id) = id {
				let (data, size) = match &img_delta.image {
					egui::ImageData::Color(img) => {
						(img.pixels.iter().flat_map(|v| v.to_array()).collect::<Vec<u8>>(), img.size)
					}
					
					egui::ImageData::Font(img) => {
						log(aetherment::LogType::Log, format!("font texture was touched {:?}", img.size));
						const GAMMA: f32 = 0.55;
						// let data: Vec<u8> = img.pixels.iter().map(|v| (v.powf(GAMMA) * 255.0 + 0.5).floor() as u8).collect();
						(img.pixels.iter().flat_map(|v| [255, 255, 255, (v.powf(GAMMA) * 255.0 + 0.5).floor() as u8]).collect::<Vec<u8>>(), img.size)
						// let data: Vec<u8> = img.pixels.iter().flat_map(|v| {
						// 	let a = (v.powf(GAMMA) * 255.0 + 0.5).floor() as u8;
						// 	[a, a, a, a]
						// }).collect();
					}
				};
				
				let (tex, tex_data) = match state.textures.get_mut(id) {
					Some(tex) => tex,
					None => {
						if img_delta.pos.is_some() {
							log(aetherment::LogType::Error, String::from("Texture doesn't exist but wishes to update"));
							continue;
						}
						
						let tex = texture::Texture::new(texture::TextureOptions {
							width: size[0] as i32,
							height: size[1] as i32,
							format: texture::TextureFormat::R8g8b8a8Unorm,
							usage: texture::TextureUsage::Dynamic,
							cpu_access_flags: texture::TextureCpuFlags::Write,
						});
						
						state.textures.insert(*id, (tex, data.clone()));
						state.textures.get_mut(id).unwrap()
					}
				};
				
				if let Err(err) = if let Some(pos) = img_delta.pos {
					log(aetherment::LogType::Log, String::from("draw to section"));
					
					let byte_count = tex.format.byte_count();
					let bytes_per_line = size[0] * byte_count;
					
					for (i, v) in data.into_iter().enumerate() {
						let curx = i % bytes_per_line;
						let cury = i / bytes_per_line;
						tex_data[pos[0] * byte_count + curx + (pos[1] + cury) * tex.width * byte_count] = v;
					}
					
					tex.draw_to(&tex_data)
				} else {
					log(aetherment::LogType::Log, String::from("draw to full"));
					tex.draw_to(&data)
				} {
					log(aetherment::LogType::Error, String::from(err));
				}
			}
		}
		
		// draw egui as imgui primitives
		for prim in state.ctx.tessellate(out.shapes) {
			match prim.primitive {
				egui::epaint::Primitive::Callback(_) => log(aetherment::LogType::Error, String::from("Callback is unsupported")),
				egui::epaint::Primitive::Mesh(mesh) => unsafe {
					let drawlist = imgui::igGetWindowDrawList();
					
					// doing this mostly fixes a bug caused by the fact we cant set (or atleast im too dumb to) sampler state
					// so that we can have filter modes other than linear, thing like separator dont show
					// TODO: find a proper solution as this is real nasty fix
					let mut max_uv = 0f32;
					for vertex in mesh.vertices.iter().skip(1) {
						max_uv = max_uv.max(vertex.uv.x.max(vertex.uv.y));
					}
					
					imgui::ImDrawList_PushTextureID(drawlist, if max_uv == 0.0 {
						state.blank_texture.resource()
					} else {
						match mesh.texture_id {
							egui::TextureId::Managed(id) => state.textures.get(&id).map_or(state.blank_texture.resource(), |v| v.0.resource()),
							egui::TextureId::User(id) => id as usize,
						}
					} as _);
					// imgui::ImDrawList_PushTextureID(drawlist, state.blank_texture.resource() as _);
					
					imgui::ImDrawList_PrimReserve(drawlist, mesh.indices.len() as i32, mesh.vertices.len() as i32);
					let offset = (*drawlist)._VtxCurrentIdx;
					for vertex in mesh.vertices {
						imgui::ImDrawList_PrimWriteVtx(drawlist, transmute(vertex.pos), transmute(vertex.uv), ((vertex.color.a() as u32) << 24) + ((vertex.color.b() as u32) << 16) + ((vertex.color.g() as u32) << 8) + (vertex.color.r() as u32)); // abgr
					}
					
					for index in mesh.indices {
						imgui::ImDrawList_PrimWriteIdx(drawlist, (offset + index) as u16);
					}
					
					imgui::ImDrawList_PopTextureID(drawlist);
				},
			}
		}
		
		unsafe{imgui::igEnd()};
		
		// unsafe {
		// 	let w = 2048.0;
		// 	let h = 256.0;
		// 	
		// 	let drawlist = imgui::igGetForegroundDrawList_Nil();
		// 	imgui::ImDrawList_PushTextureID(drawlist, state.textures.get(&0).unwrap().0.resource() as _);
		// 	imgui::ImDrawList_PrimReserve(drawlist, 6i32, 4i32);
		// 	let offset = (*drawlist)._VtxCurrentIdx;
		// 	imgui::ImDrawList_PrimWriteVtx(drawlist, imgui::ImVec2{x: 0.0, y: 0.0}, imgui::ImVec2{x: 0.0, y: 0.0}, 0xFFFFFFFF);
		// 	imgui::ImDrawList_PrimWriteVtx(drawlist, imgui::ImVec2{x: w, y: 0.0}, imgui::ImVec2{x: 1.0, y: 0.0}, 0xFFFFFFFF);
		// 	imgui::ImDrawList_PrimWriteVtx(drawlist, imgui::ImVec2{x: 0.0, y: h}, imgui::ImVec2{x: 0.0, y: 1.0}, 0xFFFFFFFF);
		// 	imgui::ImDrawList_PrimWriteVtx(drawlist, imgui::ImVec2{x: w, y: h}, imgui::ImVec2{x: 1.0, y: 1.0}, 0xFFFFFFFF);
		// 	imgui::ImDrawList_PrimWriteIdx(drawlist, (offset + 0) as u16);
		// 	imgui::ImDrawList_PrimWriteIdx(drawlist, (offset + 1) as u16);
		// 	imgui::ImDrawList_PrimWriteIdx(drawlist, (offset + 2) as u16);
		// 	imgui::ImDrawList_PrimWriteIdx(drawlist, (offset + 1) as u16);
		// 	imgui::ImDrawList_PrimWriteIdx(drawlist, (offset + 3) as u16);
		// 	imgui::ImDrawList_PrimWriteIdx(drawlist, (offset + 2) as u16);
		// 	imgui::ImDrawList_PopTextureID(drawlist);
		// }
	}).ok();
}