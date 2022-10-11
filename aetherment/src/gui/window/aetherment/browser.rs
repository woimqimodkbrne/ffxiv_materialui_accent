use std::{sync::{Arc, Mutex}, thread, collections::HashSet};
use imgui::aeth::{self, F2, DrawList};
use serde::Deserialize;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use crate::{CLIENT, SERVER};

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

#[derive(Debug, Deserialize)]
struct Mod {
	id: i32,
	name: String,
	tags: Vec<i16>,
	thumbnail: Option<String>,
	nsfw: bool,
	author: i32,
	author_name: String,
}

// ---------------------------------------- //

pub struct Tab {
	query: Arc<Mutex<SearchRequest>>,
	fetching: Arc<Mutex<bool>>,
	mods: Arc<Mutex<Vec<Mod>>>,
	show_tags: bool,
}

impl Tab {
	pub fn new(_state: &mut crate::Data) -> Self {
		let mut t = Tab {
			query: Arc::new(Mutex::new(SearchRequest {
				query: String::with_capacity(128),
				tags: HashSet::new(),
				page: 0,
				order: SearchOrderType::Downloads,
				direction: SearchOrderDirection::Descending,
				timespan: SearchTimespan::Week,
				clear: false,
			})),
			fetching: Arc::new(Mutex::new(false)),
			mods: Arc::new(Mutex::new(Vec::new())),
			show_tags: false,
		};
		
		t.search();
		
		t
	}
	
	pub fn draw(&mut self, _state: &mut crate::Data) {
		aeth::tab_bar("browser_tabs")
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
			})
			.finish();
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
	
	const NODESIZE: [f32; 2] = [212.0, 200.0];
	const THUMBNAILSIZE: [f32; 2] = [210.0, 140.0];
	
	fn draw_search_list(&mut self) {
		let width_left = aeth::width_left();
		let padding = imgui::get_style().item_spacing.x();
		let nodewp = Self::NODESIZE.x() + padding;
		let nodes_per_row = (width_left / nodewp + padding / nodewp).floor().max(1.0) as usize;
		let offset = (width_left - (nodewp * nodes_per_row as f32 - padding)) / 2.0;
		aeth::offset([offset, 0.0]);
		for (i, m) in self.mods.lock().unwrap().iter().enumerate() {
			self.draw_mod_node(m);
			
			if (i + 1) % nodes_per_row != 0 {
				imgui::same_line();
			} else {
				aeth::offset([offset, 0.0]);
			}
		}
	}
	
	fn draw_mod_node(&self, m: &Mod) {
		let draw = imgui::get_window_draw_list();
		
		let mut pos = imgui::get_cursor_screen_pos();
		let rounding = imgui::get_style().frame_rounding;
		imgui::dummy(Self::NODESIZE);
		
		// Frame
		draw.add_rect_filled(pos, pos.add(Self::NODESIZE), imgui::get_color(imgui::Col::FrameBg), rounding, imgui::DrawFlags::None);
		pos = pos.add([1.0, 1.0]);
		
		// Thumbnail
		// TOOD: thumbnail, rework upload first
		draw.add_rect_filled(pos, pos.add(Self::THUMBNAILSIZE), 0xFF101010, rounding, imgui::DrawFlags::RoundCornersTop);
		pos = pos.add([5.0, Self::THUMBNAILSIZE.y() + 1.0]);
		
		// the old stuff
		// let pos = pos.add([1.0, 1.0]);
		// let preview_size = [size.x() - 2.0, (size.x() - 2.0) / 3.0 * 2.0];
		
		// draw.add_rect_filled(pos, pos.add(preview_size), 0xFF101010, rounding, imgui::DrawFlags::None);
		// let preview = self.get_preview(m);
		// let scale = (preview_size.x() / preview.width as f32).min(preview_size.y() / preview.height as f32);
		// let (w, h) = (preview.width as f32 * scale, preview.height as f32 * scale);
		// let preview_pos = pos.add([(preview_size.x() - w) / 2.0, (preview_size.y() - h) / 2.0]);
		// draw.add_image_rounded(preview.resource(),
		// 	preview_pos,
		// 	preview_pos.add([w, h]),
		// 	[0.0, 0.0],
		// 	[1.0, 1.0],
		// 	0xFFFFFFFF,
		// 	rounding - rounding.min((preview_size.x() - w).max(preview_size.y() - h) / 2.0),
		// 	imgui::DrawFlags::None
		// );
		
		// Title
		let d = Self::NODESIZE.y() - Self::THUMBNAILSIZE.y() - imgui::get_font_size() - 1.0 - 1.0 - 1.0 - 6.0;
		draw.add_text_area(pos, imgui::get_color(imgui::Col::Text), &m.name, [Self::THUMBNAILSIZE.x() - 1.0 - 5.0 - 1.0, d]);
		
		// Author
		draw.add_text(pos.add([10.0, d + 1.0]), imgui::get_color(imgui::Col::TextDisabled), &format!("by {}", m.author_name));
	}
	
	fn search(&mut self) {
		let query = self.query.clone();
		let mods = self.mods.clone();
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
				let m: Vec<Mod> = match CLIENT.get(format!("{}/api/search", SERVER)).query(&[
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
				
				mods.lock().unwrap().extend(m);
				
				let qc = query.lock().unwrap();
				if qc.query == q.query && qc.tags == q.tags && qc.order == q.order && qc.direction == q.direction && qc.timespan == q.timespan && qc.page == q.page {
					break;
				}
			}
			
			*fetching.lock().unwrap() = false;
		});
	}
}