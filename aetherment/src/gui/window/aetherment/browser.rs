use std::{sync::{Arc, Mutex}, thread, collections::HashSet, cell::RefCell};
use imgui::aeth::{self, F2, DrawList, Texture, TextureOptions};
use serde::Deserialize;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use crate::{CLIENT, SERVER, SERVERCDN, server::structs::*};

#[derive(Debug, PartialEq, Eq, Clone, AsRefStr, EnumIter)]
pub enum SearchOrderType {
	Release,
	Update,
	Likes,
	Downloads,
	Favourites,
}

#[derive(Debug, PartialEq, Eq, Clone, AsRefStr, EnumIter)]
pub enum SearchOrderDirection {
	Ascending,
	Descending,
}

#[derive(Debug, PartialEq, Eq, Clone, AsRefStr, EnumIter)]
pub enum SearchTimespan {
	Hour,
	Day,
	Week,
	Month,
	Year,
	Alltime,
}

#[derive(Debug, Clone)]
pub struct SearchRequest {
	query: String,
	// tags: String,
	tags: HashSet<usize>,
	page: i32,
	order: SearchOrderType,
	direction: SearchOrderDirection,
	timespan: SearchTimespan,
	clear: bool,
}

struct ModNode {
	id: i32,
	name: String,
	tags: Vec<i16>,
	thumbnail: Arc<Mutex<Texture>>,
	content_rating: u8,
	author: IdName
}

#[derive(Deserialize)]
struct Version {
	version: u32,
	size: u64,
	patch: bool,
	date: chrono::DateTime<chrono::Utc>,
	description: String,
	file: String,
}

struct ModPage {
	id: i32,
	name: String,
	description: String,
	tags: Vec<i16>,
	previews: Vec<Arc<Mutex<Texture>>>,
	content_rating: u8,
	author: IdNameImg,
	contributors: Vec<IdNameImg>,
	dependencies: Vec<IdNameImg>,
	versions: Vec<Version>,
	
	current_preview: i32,
}

const NODESIZE: [f32; 2] = [212.0, 200.0];
const THUMBNAILSIZE: [f32; 2] = [210.0, 140.0];
const RATIO: f32 = 2.0 / 3.0;

// ---------------------------------------- //

pub struct Tab {
	fetching: Arc<Mutex<bool>>,
	query: Arc<Mutex<SearchRequest>>,
	mods_search: Arc<Mutex<Vec<ModNode>>>,
	show_tags: bool,
	
	mod_pages: RefCell<Vec<(IdName, Arc<Mutex<Option<ModPage>>>)>>,
}

impl Tab {
	pub fn new(_state: &mut crate::Data) -> Self {
		let mut t = Tab {
			fetching: Arc::new(Mutex::new(false)),
			query: Arc::new(Mutex::new(SearchRequest {
				query: String::with_capacity(128),
				tags: HashSet::new(),
				page: 0,
				order: SearchOrderType::Downloads,
				direction: SearchOrderDirection::Descending,
				timespan: SearchTimespan::Week,
				clear: false,
			})),
			mods_search: Arc::new(Mutex::new(Vec::new())),
			show_tags: false,
			
			mod_pages: RefCell::new(Vec::new()),
		};
		
		t.search();
		
		t
	}
	
	pub fn draw(&mut self, _state: &mut crate::Data) {
		let mut bar = aeth::tab_bar("browser_tabs")
			.dock_top()
			.tab("Search", || {
				aeth::child("search", [0.0, -imgui::get_style().item_spacing.y()], false, imgui::WindowFlags::None, || {
					self.draw_search_header();
					
					imgui::begin_child("modlist", [0.0, 0.0], false, imgui::WindowFlags::None);
					self.draw_search_list();
					
					// TODO: make footer fancier
					let footer_size = 50.0;
					imgui::dummy([0.0, footer_size]);
					
					if *self.fetching.lock().unwrap() {
						imgui::text("searching");
					} else {
						if self.query.lock().unwrap().page != -1 {
							if imgui::get_scroll_y() >= imgui::get_scroll_max_y() - footer_size {
								self.query.lock().unwrap().page += 1;
								self.search();
							}
						} else {
							imgui::text("thats it");
						}
					}
					imgui::end_child();
				});
			});
		
		let pages = self.mod_pages.borrow();
		for (idname, m) in pages.iter() {
			if let Some(m) = m.lock().unwrap().as_mut() {
				bar = bar.tab(&idname.name, || {
					imgui::push_id_i32(m.id);
					self.draw_mod_page(m);
					imgui::pop_id();
				});
			} else {
				bar = bar.tab(&idname.name, || {
					imgui::text("loading")
				});
			}
		}
		
		bar.finish();
	}
	
	fn draw_search_header(&mut self) {
		let style = imgui::get_style();
		let mut query = self.query.lock().unwrap();
		let mut dosearch = false;
		
		let dropdown_w = 150.0;
		let w = aeth::width_left();
		
		// Search query
		imgui::set_next_item_width(w - dropdown_w * 2.0 - style.item_spacing.x() * 3.0 - aeth::frame_height());
		dosearch |= imgui::input_text_with_hint("##searchquery", "Search", &mut query.query, imgui::InputTextFlags::None);
		
		// Sort order
		imgui::same_line();
		imgui::set_next_item_width(dropdown_w);
		let pos = imgui::get_cursor_pos();
		if imgui::begin_combo("##sortorder", "", imgui::ComboFlags::None) {
			for order in SearchOrderType::iter() {
				let pos = imgui::get_cursor_pos();
				if imgui::selectable(&format!("##{}_asc", order.as_ref()), query.order == order && query.direction == SearchOrderDirection::Ascending, imgui::SelectableFlags::None, [0.0, 0.0]) {
					query.order = order.clone();
					query.direction = SearchOrderDirection::Ascending;
					dosearch = true;
				}
				imgui::set_cursor_pos(pos.add(style.frame_padding));
				aeth::icon(""); // fa-angle-up
				imgui::same_line();
				imgui::text(order.as_ref());
				
				let pos = imgui::get_cursor_pos();
				if imgui::selectable(&format!("##{}_desc", order.as_ref()), query.order == order && query.direction == SearchOrderDirection::Descending, imgui::SelectableFlags::None, [0.0, 0.0]) {
					query.order = order.clone();
					query.direction = SearchOrderDirection::Descending;
					dosearch = true;
				}
				imgui::set_cursor_pos(pos.add(style.frame_padding));
				aeth::icon(""); // fa-angle-down
				imgui::same_line();
				imgui::text(order.as_ref());
			}
			imgui::end_combo();
		}
		imgui::same_line();
		let p = imgui::get_cursor_pos();
		imgui::set_cursor_pos(pos.add(style.frame_padding));
		aeth::icon(if query.direction == SearchOrderDirection::Ascending {""} else {""});
		imgui::same_line();
		imgui::text(query.order.as_ref());
		
		// Timespan
		imgui::set_cursor_pos(p);
		imgui::set_next_item_width(dropdown_w);
		if imgui::begin_combo("##timespan", query.timespan.as_ref(), imgui::ComboFlags::None) {
			for timespan in SearchTimespan::iter() {
				if imgui::selectable(timespan.as_ref(), query.timespan == timespan, imgui::SelectableFlags::None, [0.0, 0.0]) {
					query.timespan = timespan;
					dosearch = true;
				}
			}
			imgui::end_combo();
		}
		
		// Tags
		imgui::same_line();
		if aeth::button_icon("") { // fa-cog
			self.show_tags = !self.show_tags;
		}
		
		if self.show_tags {
			for (category, tags) in crate::creator::tags::TAGS_SORTED.iter() {
				if let Some(category) = category {
					imgui::text(category);
				}
				
				imgui::indent();
				for (i, (tag_index, tag)) in tags.iter().enumerate() {
					imgui::same_line();
					let size = imgui::calc_text_size(tag, false, -1.0);
					if i == 0 || size.x() + style.frame_padding.x() + style.item_spacing.x() > aeth::width_left() {
						imgui::new_line();
					}
					
					if imgui::selectable(tag, query.tags.contains(&tag_index), imgui::SelectableFlags::None, size) {
						if !query.tags.remove(&tag_index) {
							query.tags.insert(*tag_index);
						}
						dosearch = true;
					}
				}
				imgui::unindent();
				aeth::offset([0.0, 8.0]);
			}
		}
		
		if dosearch {
			query.page = 0;
			query.clear = true;
			drop(query);
			self.search();
		}
	}
	
	fn draw_search_list(&mut self) {
		let width_left = aeth::width_left();
		let padding = imgui::get_style().item_spacing.x();
		let nodewp = NODESIZE.x() + padding;
		let nodes_per_row = (width_left / nodewp + padding / nodewp).floor().max(1.0) as usize;
		let offset = (width_left - (nodewp * nodes_per_row as f32 - padding)) / 2.0;
		aeth::offset([offset, 0.0]);
		for (i, m) in self.mods_search.lock().unwrap().iter().enumerate() {
			self.draw_mod_node(m);
			
			if (i + 1) % nodes_per_row != 0 {
				imgui::same_line();
			} else {
				aeth::offset([offset, 0.0]);
			}
		}
	}
	
	fn draw_mod_node(&self, m: &ModNode) {
		let mut draw = imgui::get_window_draw_list();
		
		let size = THUMBNAILSIZE;
		let mut pos = imgui::get_cursor_screen_pos();
		let rounding = imgui::get_style().frame_rounding;
		imgui::dummy(NODESIZE);
		if imgui::is_item_clicked(imgui::MouseButton::Left) {
			self.open_mod_page(m);
		}
		
		// Frame
		draw.add_rect_filled(pos, pos.add(NODESIZE), imgui::get_color(imgui::Col::FrameBg), rounding, imgui::DrawFlags::RoundCornersAll);
		pos = pos.add([1.0, 1.0]);
		
		// Thumbnail
		draw.push_texture_id(m.thumbnail.lock().unwrap().resource());
		draw.add_rect_rounded(pos.add([1.0; 2]), pos.add(size).sub([1.0; 2]), [0.0; 2], [1.0; 2], 0xFFFFFFFF, rounding, imgui::DrawFlags::RoundCornersTop);
		draw.pop_texture_id();
		
		// content rating
		if m.content_rating != 0 {
			let rating = content_rating(m.content_rating);
			draw.add_rect_filled(pos.add([1.0, size.y() * 0.1]), pos.add([5.0, size.y() * 0.1 + 4.0]).add(imgui::calc_text_size2(rating)), aeth::RED, rounding, imgui::DrawFlags::RoundCornersRight);
			draw.add_text(pos.add([3.0, size.y() * 0.1 + 2.0]), imgui::get_color(imgui::Col::Text), rating);
		}
		
		pos = pos.add([5.0, size.y() + 1.0]);
		
		// Title
		let d = NODESIZE.y() - size.y() - imgui::get_font_size() - 1.0 - 1.0 - 1.0 - 6.0;
		draw.add_text_area(pos, imgui::get_color(imgui::Col::Text), &m.name, [size.x() - 1.0 - 5.0 - 1.0, d]);
		
		// Author
		draw.add_text(pos.add([10.0, d + 1.0]), imgui::get_color(imgui::Col::TextDisabled), &format!("by {}", m.author.name));
	}

	fn draw_mod_page(&self, m: &mut ModPage) {
		let style = imgui::get_style();
		let frame_h = aeth::frame_height();
		
		aeth::child("sidebar", [300.0 + style.frame_padding.x() * 2.0, -style.item_spacing.y()], false, imgui::WindowFlags::None, || {
			let sidebar_w = 300.0;
			let img_h = imgui::get_font_size() * 2.0;
			
			// Header
			aeth::frame_sized(|| {
				// Name
				// TODO: custom big font for non ugly
				aeth::wrapped_text(&m.name, [sidebar_w, 0.0], aeth::TextAlign::Center);
				
				// Tags
				aeth::offset([0.0, frame_h]);
				let mut tags = String::from("Tags: ");
				let mut line_w = imgui::calc_text_size2(&tags).x();
				for (i, tag) in m.tags.iter().enumerate() {
					let name = &crate::creator::tags::TAGS[*tag as usize].name;
					let name_w = imgui::calc_text_size2(&format!("{name}, ")).x();
					
					line_w += name_w;
					if line_w > sidebar_w {
						line_w = name_w;
						tags.push_str("\n");
					}
					
					tags.push_str(&name);
					if i < m.tags.len() - 1 {
						tags.push_str(", ");
					}
				}
				imgui::text_unformatted(&tags);
				
				// Dates
				// TODO: format these to the users timezone and mby locale
				aeth::offset([0.0, frame_h]);
				imgui::text_unformatted(&format!("Posted: {}", m.versions.first().unwrap().date));
				imgui::text_unformatted(&format!("Updated: {}", m.versions.last().unwrap().date));
			});
			
			// Author
			aeth::offset([0.0, frame_h]);
			aeth::frame_sized(|| {
				aeth::wrapped_text("Author", [sidebar_w, 0.0], aeth::TextAlign::Center);
				
				aeth::image(m.author.img.lock().unwrap().resource(), [img_h; 2], [0.0; 2], [1.0; 2], 0xFFFFFFFF);
				imgui::same_line();
				imgui::text_unformatted(&m.author.name);
			});
			
			// Contributors
			if m.contributors.len() > 0 {
				aeth::offset([0.0, frame_h]);
				aeth::frame_sized(|| {
					aeth::wrapped_text("Contributors", [sidebar_w, 0.0], aeth::TextAlign::Center);
					
					for c in &m.contributors {
						aeth::image(c.img.lock().unwrap().resource(), [img_h; 2], [0.0; 2], [1.0; 2], 0xFFFFFFFF);
						imgui::same_line();
						imgui::text_unformatted(&c.name);
					}
				});
			}
			
			// Dependencies
			if m.dependencies.len() > 0 {
				aeth::offset([0.0, frame_h]);
				aeth::frame_sized(|| {
					aeth::wrapped_text("Dependencies", [sidebar_w, 0.0], aeth::TextAlign::Center);
					
					for d in &m.dependencies {
						aeth::image(d.img.lock().unwrap().resource(), [img_h / RATIO, img_h], [0.0; 2], [1.0; 2], 0xFFFFFFFF);
						imgui::same_line();
						imgui::text_unformatted(&d.name);
					}
				});
			}
			
			// TODO: download and version select here
		});
		
		// Content
		imgui::same_line();
		aeth::child("content", [0.0, -style.item_spacing.y()], false, imgui::WindowFlags::None, || {
			if m.previews.len() > 0 {
				let avail = imgui::get_content_region_avail();
				let w = avail.x().min((avail.y() - frame_h - style.item_spacing.y()) / RATIO);
				let h = w * RATIO;
				aeth::offset([(avail.x() - w) / 2.0, 0.0]);
				
				if let Some(preview) = m.previews.get(m.current_preview as usize) {
					let mut draw = imgui::get_window_draw_list();
					let pos = imgui::get_cursor_screen_pos();
					draw.push_texture_id(preview.lock().unwrap().resource());
					draw.add_rect_rounded(pos, pos.add([w, h]), [0.0; 2], [1.0; 2], 0xFFFFFFFF, style.frame_rounding, imgui::DrawFlags::RoundCornersAll);
					draw.pop_texture_id();
				}
				
				imgui::dummy([avail.x(), h]);
				
				let radio_w = m.previews.len() as f32 * (frame_h + style.item_spacing.x()) - style.item_spacing.x();
				aeth::offset([(avail.x() - radio_w) / 2.0, 0.0]);
				for i in 0..(m.previews.len() as i32) {
					if i > 0 {imgui::same_line()}
					imgui::push_id_i32(i);
					imgui::radio_button("##previewradio", &mut m.current_preview, i);
					imgui::pop_id();
				}
				
				aeth::offset([0.0, frame_h]);
			}
			
			aeth::wrapped_text(&m.description, [0.0; 2], aeth::TextAlign::Left);
		});
	}
	
	fn open_mod_page(&self, m: &ModNode) {
		let mut pages = self.mod_pages.borrow_mut();
		if pages.iter().any(|v| v.0.id == m.id) {return}
		
		let id = m.id;
		let page = Arc::new(Mutex::new(None));
		pages.push((IdName{name: m.name.clone(), id}, page.clone()));
		
		thread::spawn(move || {
			#[derive(Deserialize)]
			struct Resp {
				id: i32,
				name: String,
				description: String,
				tags: Vec<i16>,
				previews: Vec<String>,
				content_rating: u8,
				author: IdNameImg,
				contributors: Vec<IdNameImg>,
				dependencies: Vec<IdNameImg>,
				versions: Vec<Version>,
			}
			
			let m: Resp = match CLIENT.get(format!("{SERVER}/api/mod/{id}")).send() {
				Ok(v) => match v.json() {
					Ok(v) => v,
					Err(e) => {
						log!(err, "mod page json conversion failed {:?}", e);
						return;
					},
				},
				Err(e) => {
					log!(err, "mod page failed {:?}", e);
					return;
				},
			};
			
			let previews = m.previews.iter().map(|_| Arc::new(Mutex::new(Texture::empty()))).collect::<Vec<_>>();
			for (i, preview) in previews.iter().enumerate() {
				let preview = preview.clone();
				let path = format!("{SERVERCDN}{}", m.previews[i]);
				thread::spawn(move || {
					let img = image::io::Reader::new(std::io::Cursor::new(CLIENT.get(path).send().unwrap().bytes().unwrap()))
						.with_guessed_format()
						.unwrap()
						.decode()
						.unwrap();
					
					*preview.lock().unwrap() = Texture::with_data(TextureOptions {
						width: img.width() as i32,
						height: img.height() as i32,
						format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
						usage: 1, // D3D11_USAGE_IMMUTABLE
						cpu_access_flags: 0,
					}, &img.into_rgba8());
				});
			}
			
			*page.lock().unwrap() = Some(ModPage {
				id: m.id,
				name: m.name,
				description: m.description,
				tags: m.tags,
				previews: previews,
				content_rating: m.content_rating,
				author: m.author,
				contributors: m.contributors,
				dependencies: m.dependencies,
				versions: m.versions,
				
				current_preview: 0,
			});
		});
	}
	
	fn search(&mut self) {
		let query = self.query.clone();
		let mods = self.mods_search.clone();
		let fetching = self.fetching.clone();
		
		*fetching.lock().unwrap() = true;
		
		thread::spawn(move || {
			loop {
				let mut q2 = query.lock().unwrap();
				let clear = q2.clear;
				q2.clear = false;
				
				let q = q2.clone();
				drop(q2);
				
				log!("search");
				
				#[derive(Deserialize)]
				struct Resp {
					id: i32,
					name: String,
					tags: Vec<i16>,
					thumbnail: String,
					content_rating: u8,
					author: IdName,
				}
				let m: Vec<Resp> = match CLIENT.get(format!("{SERVER}/api/search")).query(&[
					("query", &q.query[..]),
					("tags", &q.tags.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")[..]),
					("page", &q.page.to_string()[..]),
					("order", q.order.as_ref()),
					("direction", q.direction.as_ref()),
					("timespan", q.timespan.as_ref()),
				]).send() {
					Ok(v) => match v.json() {
						Ok(v) => v,
						Err(e) => {
							log!(err, "search json conversion failed {:?}", e);
							query.lock().unwrap().page = -1;
							break;
						},
					},
					Err(e) => {
						log!(err, "search failed {:?}", e);
						query.lock().unwrap().page = -1;
						break;
					},
				};
				
				if clear {
					mods.lock().unwrap().clear();
				}
				
				if m.len() == 0 {
					query.lock().unwrap().page = -1;
					break;
				}
				
				let mut mods = mods.lock().unwrap();
				for mo in m {
					let thumbnail = Arc::new(Mutex::new(Texture::empty()));
					
					mods.push(ModNode {
						id: mo.id,
						name: mo.name,
						tags: mo.tags,
						thumbnail: thumbnail.clone(),
						content_rating: mo.content_rating,
						author: mo.author,
					});
					
					// TODO: blur thumbnail or hide mod depending on user settings and mod content rating
					thread::spawn(move || {
						let t = Texture::with_data(TextureOptions {
							width: 135,
							height: 90,
							format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
							usage: 1, // D3D11_USAGE_IMMUTABLE
							cpu_access_flags: 0,
						}, &image::io::Reader::new(std::io::Cursor::new(crate::get_resource(&mo.thumbnail)))
							.with_guessed_format()
							.unwrap()
							.decode()
							.unwrap()
							.into_rgba8()
						);
						
						*thumbnail.lock().unwrap() = t;
					});
				}
				
				let qc = query.lock().unwrap();
				if qc.query == q.query && qc.tags == q.tags && qc.order == q.order && qc.direction == q.direction && qc.timespan == q.timespan && qc.page == q.page {
					break;
				}
			}
			
			*fetching.lock().unwrap() = false;
		});
	}
}

fn content_rating(rating: u8) -> &'static str {
	match rating {
		0 => "SFW",
		1 => "NSFW",
		2 => "NSFL",
		_ => "????",
	}
}