use std::{collections::HashMap, path::PathBuf};

pub(in crate) static mut REDRAW: fn() = || {};
pub fn redraw() {
	unsafe{REDRAW()}
}

pub(in crate) static mut REDRAWSELF: fn() = || {};
pub fn redraw_self() {
	unsafe{REDRAWSELF()}
}

pub(in crate) static mut ADDMOD: fn(String, String, String, i32) -> u8 = |_, _, _, _| {0};
pub fn add_mod<S>(id: S, paths: HashMap<String, String>, manip: S, priority: i32) -> u8 where
S: Into<String> {
	unsafe{ADDMOD(id.into(), paths.into_iter().map(|v| format!("{}\0{}", v.0, v.1)).collect::<Vec<String>>().join("\0\0"), manip.into(), priority)}
}

pub(in crate) static mut REMOVEMOD: fn(String, i32) -> u8 = |_, _| {0};
pub fn remove_mod<S>(id: S, priority: i32) -> u8 where
S: Into<String> {
	unsafe{REMOVEMOD(id.into(), priority)}
}

pub(in crate) static mut ADDMODENTRY: fn(String) -> u8 = |_| {0};
pub fn add_mod_entry<S>(id: S) -> u8 where
S: Into<String> {
	unsafe{ADDMODENTRY(id.into())}
}

pub(in crate) static mut ROOTPATH: fn() -> *const u128 = || {0 as *const _}; // actually returns a &str, but 'static doesn't work and idk how to handle the lifetime. transmute this
pub fn root_path() -> PathBuf {
	// TODO: probably should do some checks here to check if its valid
	unsafe{std::mem::transmute::<_, &str>(*ROOTPATH())}.into()
}

pub(in crate) static mut ACTIVECOLLECTION: fn() -> *const u128 = || {0 as *const _};
pub fn active_collection() -> &'static str {
	unsafe{std::mem::transmute::<_, &str>(*ACTIVECOLLECTION())}
}