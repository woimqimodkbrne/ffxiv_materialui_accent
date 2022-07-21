use std::{collections::BTreeMap, rc::Rc};
use crate::gui::aeth;

pub struct Tree {
	selected: usize,
	nodes: Vec<Node>,
}

impl Tree {
	pub fn new<S>(name: S) -> Self where
	S: Into<String> {
		Tree {
			selected: usize::MAX,
			nodes: vec![Node {
				name: name.into(),
				is_folder: true,
				is_enabled: true,
				parent: None,
				children: BTreeMap::new(),
			}],
		}
	}
	
	pub fn add_node(&mut self, path: &str) {
		let mut curnode = 0;
		for seg in path.split('/') {
			curnode = if !self.nodes[curnode].children.contains_key(seg) {
				let i = self.nodes.len();
				let node = Node {
					name: seg.to_owned(),
					is_folder: true,
					is_enabled: true,
					parent: Some(curnode),
					children: BTreeMap::new(),
				};
				self.nodes[curnode].children.insert(Rc::from(node.name.as_str()), i);
				self.nodes.push(node);
				i
			} else {
				*self.nodes[curnode].children.get(seg).unwrap()
			}
		}
		self.nodes[curnode].is_folder = false;
	}
	
	pub fn node_state(&mut self, path: &str, state: bool) {
		let mut curnode = 0;
		for seg in path.split('/') {
			curnode = *self.nodes[curnode].children.get(seg).unwrap();
		}
		self.node_state_i(curnode, state);
	}
	
	fn node_state_i(&mut self, node: usize, state: bool) {
		let mut node = self.nodes.get_mut(node).unwrap();
		if node.is_enabled != state && node.parent != None {
			node.is_enabled = state;
			
			let pi = node.parent.unwrap();
			let p = self.nodes.get(pi).unwrap();
			if p.children.values().any(|n| self.nodes[*n].is_enabled != p.is_enabled) {
				self.node_state_i(pi, state);
			}
		}
	}
	
	pub fn node_state_all(&mut self, state: bool) {
		let mut iter = self.nodes.iter_mut();
		iter.next(); // skip root node
		iter.for_each(|n| n.is_enabled = state);
	}
	
	pub fn draw(&mut self) -> Option<(imgui::MouseButton, String)> {
		let mut r = None;
		if let Some((button, id)) = self.draw_node(0) {
			r = Some((button, self.node_path(id)));
			self.selected = id;
		}
		r
	}
	
	fn draw_node(&self, nodeid: usize) -> Option<(imgui::MouseButton, usize)> {
		let mut r = None;
		let node = self.nodes.get(nodeid).unwrap();
		if node.is_folder {
			if !node.is_enabled {imgui::push_style_color(imgui::Col::Text, imgui::get_color(imgui::Col::TextDisabled))}
			if !aeth::tree(&node.name, || {
				if !node.is_enabled {imgui::pop_style_color(1)}
				node.children.iter().for_each(|(_, n)| if self.nodes[*n].is_folder {r = r.or(self.draw_node(*n))});
				node.children.iter().for_each(|(_, n)| if !self.nodes[*n].is_folder {r = r.or(self.draw_node(*n))});
			}) && !node.is_enabled {imgui::pop_style_color(1)}
		} else {
			if imgui::selectable(
				&node.name,
				nodeid == self.selected,
				if node.is_enabled {imgui::SelectableFlags::None} else {imgui::SelectableFlags::Disabled},
				[0.0, 0.0]
			) {
				r = Some((imgui::MouseButton::Left, nodeid))
			} else if imgui::is_item_clicked(imgui::MouseButton::Right) {
				r = Some((imgui::MouseButton::Right, nodeid))
			}
		}
		
		// if imgui::is_item_hovered() {
		// 	aeth::tooltip(|| {
		// 		imgui::text(&self.node_path(nodeid));
		// 	});
		// }
		
		r
	}
	
	fn node_path(&self, node: usize) -> String {
		let mut node = self.nodes.get(node).unwrap();
		let mut path = node.name.to_owned();
		while let Some(p) = node.parent && p != 0 {
			node = self.nodes.get(p).unwrap();
			path.insert(0, '/');
			path.insert_str(0, &node.name);
		}
		path
	}
}

pub struct Node {
	name: String,
	is_folder: bool,
	is_enabled: bool,
	parent: Option<usize>,
	children: BTreeMap<Rc<str>, usize>,
}