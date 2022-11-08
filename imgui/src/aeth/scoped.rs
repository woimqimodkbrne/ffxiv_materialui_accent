#![allow(dead_code)]

use crate as imgui;

macro_rules! scoped {
	($name:ident, ($($param_name:ident: $param_type:ty),*), $begin:path, $end:path) => {
		pub fn $name<F, R>($($param_name: $param_type,)* scope: F) -> Option<R> where F: FnOnce() -> R {
			if $begin($($param_name,)*) {
				let r = scope();
				$end();
				Some(r)
			} else {None}
		}
		
		// macro_rules! $name {
		// 	($($param_name: $param_type,)* $scope:block) => {
		// 		if $begin($($param_name,)*) {
		// 			scope();
		// 			$end();
		// 		}
		// 	}
		// }
		// pub use $name;
	};
	
	($name:ident, ($($param_name:ident: $param_type:ty),*), $begin:path>, $end:path) => {
		pub fn $name<F, R>($($param_name: $param_type,)* scope: F) -> R where F: FnOnce() -> R {
			$begin($($param_name,)*);
			let r = scope();
			$end();
			r
		}
		
		// macro_rules! $name {
		// 	($($param_name: $param_type,)* $scope:block) => {
		// 		$begin($($param_name,)*)
		// 		scope();
		// 		$end();
		// 	}
		// }
		// pub use $name;
	}
}

scoped!(tree, (label: &str), imgui::tree_node, imgui::tree_pop);
scoped!(frame, (), imgui::new_frame>, imgui::end_frame);
scoped!(child, (id: &str, size: [f32; 2], border: bool, flags: imgui::WindowFlags), imgui::begin_child>, imgui::end_child);
scoped!(child2, (id: u32, size: [f32; 2], border: bool, flags: imgui::WindowFlags), imgui::begin_child2>, imgui::end_child);
scoped!(group, (), imgui::begin_group>, imgui::end_group);
scoped!(combo, (label: &str, preview_value: &str, flags: imgui::ComboFlags), imgui::begin_combo, imgui::end_combo);
scoped!(list_box, (label: &str, size: [f32; 2]), imgui::begin_list_box, imgui::end_list_box);
scoped!(menu_bar, (), imgui::begin_menu_bar, imgui::end_menu_bar);
scoped!(main_menu_bar, (), imgui::begin_main_menu_bar, imgui::end_main_menu_bar);
scoped!(menu, (label: &str, enabled: bool), imgui::begin_menu, imgui::end_menu);
scoped!(tooltip, (), imgui::begin_tooltip>, imgui::end_tooltip);
scoped!(popup, (str_id: &str, flags: imgui::WindowFlags), imgui::begin_popup, imgui::end_popup);
scoped!(popup_modal, (name: &str, p_open: &mut bool, flags: imgui::WindowFlags), imgui::begin_popup_modal, imgui::end_popup);
scoped!(table, (str_id: &str, column: i32, flags: imgui::TableFlags, outer_size: [f32; 2], inner_width: f32), imgui::begin_table, imgui::end_table);
// skip tabbar and tabitem cuz we make our own
scoped!(dragdrop_source, (flags: imgui::DragDropFlags), imgui::begin_drag_drop_source, imgui::end_drag_drop_source);
scoped!(dragdrop_target, (), imgui::begin_drag_drop_target, imgui::end_drag_drop_target);
scoped!(disabled, (disabled: bool), imgui::begin_disabled>, imgui::end_disabled);
scoped!(child_frame, (id: u32, size: [f32; 2], flags: imgui::WindowFlags), imgui::begin_child_frame>, imgui::end_child_frame);
scoped!(combo_popup, (popup_id: u32, bb: imgui::sys::ImRect, flags: imgui::ComboFlags), imgui::begin_combo_popup, imgui::end_combo_preview);
scoped!(combo_preview, (), imgui::begin_combo_preview, imgui::end_combo_preview);
scoped!(columns, (str_id: &str, count: i32, flags: imgui::OldColumnFlags), imgui::begin_columns>, imgui::end_columns);
// this isnt all, cba doing the rest