use std::{sync::{Arc, Mutex, RwLock}, thread, time::Instant, collections::HashMap, io::Cursor, panic::catch_unwind};
use crate::{gui::{imgui, aeth::{F2 as _, self}}, server::Mod, CLIENT, SERVER};

pub struct Tab {
	fetching: Arc<Mutex<bool>>,
	mods: Arc<RwLock<Vec<Mod>>>,
	query: Arc<RwLock<String>>,
	tags: Arc<RwLock<Vec<i16>>>,
	page: Arc<RwLock<i32>>,
	previews: Arc<RwLock<HashMap<String, Preview>>>,
}

struct Preview {
	texture: Arc<aeth::Texture>,
	last_access: Instant,
}

const NODESIZE: [f32; 2] = [300.0, 300.0];

impl Tab {
	pub fn new(_state: &mut crate::Data) -> Self {
		let mut t = Tab {
			fetching: Arc::new(Mutex::new(false)),
			mods: Arc::new(RwLock::new(Vec::new())),
			query: Arc::new(RwLock::new(String::with_capacity(64))),
			tags: Arc::new(RwLock::new(Vec::new())),
			page: Arc::new(RwLock::new(0)),
			previews: Arc::new(RwLock::new(HashMap::new())),
		};
		
		t.search();
		t
	}
	
	pub fn draw(&mut self, _state: &mut crate::Data) {
		aeth::tab_bar("browser_tabs")
			.dock_top()
			.tab("Main", || {
				
			})
			.tab("Search", || {
				aeth::child("search", [0.0, -imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None, || {
					self.draw_search();
				});
			})
			.finish();
	}
	
	fn draw_search(&mut self) {
		if imgui::input_text("Search", &mut *self.query.write().unwrap(), imgui::InputTextFlags::None) {
			*self.page.write().unwrap() = 0;
			self.search();
		}
		
		let width_left = imgui::get_column_width(-1);
		let padding = imgui::get_style().item_spacing.x();
		let nodewp = NODESIZE.x() + padding;
		let nodes_per_row = (width_left / nodewp + padding / nodewp).floor().max(1.0) as usize;
		let offset = (width_left - (nodewp * nodes_per_row as f32 - padding)) / 2.0;
		aeth::offset([offset, 0.0]);
		for (i, m) in (&*self.mods.read().unwrap()).iter().enumerate() {
			self.draw_mod_node(m, NODESIZE);
			
			if (i + 1) % nodes_per_row != 0 {
				imgui::same_line();
			} else {
				aeth::offset([offset, 0.0]);
			}
		}
		
		imgui::new_line();
		imgui::dummy([0.0, 50.0]);
		
		if *self.fetching.lock().unwrap() {
			imgui::text("Searching");
		} else if *self.page.read().unwrap() != -1 {
			if imgui::get_scroll_y() >= imgui::get_scroll_max_y() - 50.0 {
				*self.page.write().unwrap() += 1;
				self.search();
			}
		} else {
			imgui::text("Thats it");
		}
	}
	
	fn draw_mod_node(&self, m: &Mod, size: [f32; 2]) {
		let draw = imgui::get_window_draw_list();
		let pos = imgui::get_cursor_screen_pos();
		let rounding = imgui::get_style().frame_rounding;
		imgui::dummy(size);
		
		// Frame
		draw.add_rect_filled(pos, pos.add(size), imgui::get_color(imgui::Col::FrameBg), rounding, imgui::DrawFlags::None);
		
		// Preview
		let pos = pos.add([1.0, 1.0]);
		let preview_size = [size.x() - 2.0, (size.x() - 2.0) / 3.0 * 2.0];
		
		draw.add_rect_filled(pos, pos.add(preview_size), 0xFF101010, rounding, imgui::DrawFlags::None);
		let preview = self.get_preview(m);
		let scale = (preview_size.x() / preview.width as f32).min(preview_size.y() / preview.height as f32);
		let (w, h) = (preview.width as f32 * scale, preview.height as f32 * scale);
		let preview_pos = pos.add([(preview_size.x() - w) / 2.0, (preview_size.y() - h) / 2.0]);
		draw.add_image_rounded(preview.resource(),
			preview_pos,
			preview_pos.add([w, h]),
			[0.0, 0.0],
			[1.0, 1.0],
			0xFFFFFFFF,
			rounding - rounding.min((preview_size.x() - w).max(preview_size.y() - h) / 2.0),
			imgui::DrawFlags::None
		);
		
		// Title
		draw.add_text(pos.add([10.0, preview_size.y() + 4.0]), imgui::get_color(imgui::Col::Text), &m.name);
	}
	
	fn get_preview<'a>(&'a self, m: &Mod) -> Arc<aeth::Texture> {
		let modid = m.id.clone();
		let id = (if let Some(p) = m.previews.get(0) {p} else {&m.id}).clone();
		let mut previews = self.previews.write().unwrap();
		let preview = previews.entry(id.clone()).or_insert_with(|| {
			if m.previews.len() > 0 {
				let previews = Arc::clone(&self.previews);
				thread::spawn(move || {
					let img = image::io::Reader::new(Cursor::new(CLIENT.get(format!("{}/mod/{}/{}", SERVER, modid, id))
						.send()
						.unwrap()
						.bytes()
						.unwrap()
						.to_vec()))
						.with_guessed_format()
						.unwrap()
						.decode()
						.unwrap()
						.into_rgba8();
					
					previews.write().unwrap().entry(id).and_modify(|p| {
						p.texture = Arc::new(aeth::Texture::with_data(aeth::TextureOptions {
							width: img.width() as i32,
							height: img.height() as i32,
							format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
							usage: 1, // D3D11_USAGE_IMMUTABLE
							cpu_access_flags: 0,
						}, &img.as_raw()));
					});
				});
			}
			
			Preview {
				texture: Arc::new(aeth::Texture::empty()),
				last_access: Instant::now(),
			}
		});
		
		preview.last_access = Instant::now();
		preview.texture.clone()
	}
	
	fn search(&mut self) {
		if *self.fetching.lock().unwrap() || *self.page.read().unwrap() == -1 {
			return;
		}
		
		*self.fetching.lock().unwrap() = true;
		
		let fetching = Arc::clone(&self.fetching);
		let mods = Arc::clone(&self.mods);
		let query = Arc::clone(&self.query);
		let tags = Arc::clone(&self.tags);
		let page = Arc::clone(&self.page);
		
		thread::spawn(move || {
			match std::panic::catch_unwind(|| {
				let mut searched_query;
				let mut searched_tags;
				let mut searched_page;
				
				loop {
					searched_query = query.read().unwrap().clone();
					searched_tags = tags.read().unwrap().clone();
					searched_page = page.read().unwrap().clone();
					
					if searched_page == 0 {
						mods.write().unwrap().clear();
					}
					
					let tags_comma = tags.read().unwrap().iter().map(|e| e.to_string()).collect::<Vec<String>>().join(",");
					let url = format!("{}/search.json?query={}&tags={}&page={}", SERVER, query.read().unwrap(), tags_comma, page.read().unwrap());
					log!(log, "searching {}", url);
					
					let mut m = CLIENT.get(url)
						.send()
						.unwrap()
						.json::<Vec<Mod>>()
						.unwrap();
					
					if m.len() == 0 {
						*page.write().unwrap() = -1;
						break;
					}
					
					mods.write().unwrap().append(&mut m);
					
					if *query.read().unwrap() == searched_query && *tags.read().unwrap() == searched_tags && *page.read().unwrap() == searched_page {
						break;
					}
				}
			}) {
				Ok(_) => {},
				Err(_) => {
					*page.write().unwrap() = -1;
				}
			}
			
			*fetching.lock().unwrap() = false;
		});
	}
}