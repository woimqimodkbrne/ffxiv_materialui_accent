use std::{collections::{HashMap, HashSet}, fs::File, io::Write};
use regex::Regex;

fn main() {
	println!("cargo:rustc-link-search=./aetherment/lib");
	println!("cargo:rustc-link-lib=cimgui");
	// println!("cargo:rerun-if-changed=build.rs");
	// println!("cargo:rerun-if-changed=./lib/cimgui.lib");
	
	// fucking stop holy shit
	// generate_bindings();
}

#[allow(dead_code)]
fn generate_bindings() {
	let bindings = bindgen::Builder::default()
		.header("./lib/cimgui.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.clang_arg("--language=c++")
		.clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS")
		.layout_tests(false)
		// .default_enum_style(bindgen::EnumVariation::Rust{non_exhaustive: false})
		.prepend_enum_name(false)
		.generate()
		.unwrap()
		.to_string();
		// .write_to_file(std::path::Path::new("./src/bindings.rs"))
		// .unwrap();
	let b = &bindings;
	
	// autogen the usefull file, it won't be 100% correct but it's good enough
	let types = HashMap::from([
		("ImVec2", "[f32; 2]"),
		("ImVec4", "[f32; 4]"),
		("ImU32", "u32"),
		("::std::os::raw::c_int", "i32"),
	]);
	
	let fix_type = |t: &str| -> String {
		match types.get(t) {
			Some(v) => v,
			None => t,
		}.to_owned()
	};
	
	let re_rename = Regex::new(r"([^A-Z]*)([A-Z]+)([^A-Z]*)").unwrap();
	let rename = |s: &str| -> String {
		if s.chars().any(char::is_uppercase) {
			re_rename.captures_iter(s).map(|s| format!("{}_{}{}",
				s.get(1).unwrap().as_str(),
				s.get(2).unwrap().as_str().to_lowercase(),
				s.get(3).unwrap().as_str()))
				.collect::<Vec<String>>().join("")
		} else {
			s.to_owned()
		}
	};
	
	let re_enumname = Regex::new(r"Im[A-Z][a-z]+([a-zA-Z0-9]+)").unwrap();
	let enumname = |s: &str| -> String {
		re_enumname.captures(s).unwrap().get(1).unwrap().as_str().to_owned()
	};
	
	let mut f = File::create("./src/gui/imgui/mod.rs").unwrap();
	f.write_all("#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::CString;

#[path = \"bindings.rs\"]
pub mod sys;\n".as_bytes()).unwrap();
	
	// enums
	let mut enums = HashMap::new();
	for cap in Regex::new(r"pub const [a-zA-Z]+_(?P<name>[a-zA-Z]+): (?P<enum>[a-zA-Z]+)[a-zA-Z_]+ =\s+(?P<value>\d+)").unwrap().captures_iter(b) {
		let n = cap["enum"].to_owned();
		let l = if n.contains("Flags") {format!("\tconst {} = {};", &cap["name"], &cap["value"])} else {format!("\t{} = {},", &cap["name"], &cap["value"])};
		enums.entry(n).or_insert(Vec::new()).push(l);
	}
	
	let mut types = HashSet::new();
	let mut done = HashSet::new();
	for cap in Regex::new(r"pub type (?P<name>[a-zA-Z0-9]+)(?P<name2>[a-zA-Z0-9_]*)\s+=\s+(?P<type>[^;]+)").unwrap().captures_iter(b) {
		if !done.insert(cap["name"].to_owned()) {
			continue;
		}
		
		if let Some(e) = enums.get(&cap["name"]) {
			if cap["name"].contains("Flags") {
				f.write_all(format!("\nbitflags::bitflags!{{pub struct {}: {} {{\n{}\n}}}}\n", enumname(&cap["name"]), fix_type(&cap["type"]), e.join("\n")).as_bytes()).unwrap();
			} else {
				f.write_all(format!("\n#[repr({})]\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\npub enum {} {{\n{}\n}}\n", fix_type(&cap["type"]), enumname(&cap["name"]), e.join("\n")).as_bytes()).unwrap();
			}
		} else {
			types.insert(format!("{}{}", &cap["name"], &cap["name2"]));
		}
	}
	
	for cap in Regex::new(r"pub struct (?P<name>[a-zA-Z0-9_]+) \{").unwrap().captures_iter(b) {
		types.insert(cap["name"].to_owned());
	}
	
	let re_prefix = Regex::new(r"(?P<pre>(?:(?:\*mut|\*const) )*)(?P<suf>[^ ]*)").unwrap();
	let re_param = Regex::new(r"(?P<name>[A-Za-z0-9_]+): (?P<type>[^,]+)").unwrap();
	for cap in Regex::new(r"pub fn (?P<name>[0-9a-zA-Z_]+)\((?P<params>[^)]*)\)(?:\s+(?P<r>->)\s+)?(?P<return>[^;]*)").unwrap().captures_iter(b) {
		if cap["params"].contains("extern") {
			continue; // dont bother with callback things
		}
		
		let mut name = &cap["name"];
		if !(name.starts_with("ig") || name.starts_with("ImDrawList")) {
			continue; // we dont care about other stuff (for now)
		}
		
		if name.starts_with("ig") {name = &name[2..]}
		let name = rename(&format!("{}{}", name[0..1].to_lowercase(), &name[1..]));
		
		let mut param_names = Vec::new();
		let mut params = Vec::new();
		let mut block = String::new();
		for param in re_param.captures_iter(&cap["params"]) {
			let mut name = param["name"].to_owned();
			let typ = &param["type"];
			let typ = if typ == "*const ::std::os::raw::c_char" {
				block.push_str(&format!("\tlet {n}_ = CString::new({n}).unwrap();\n", n = name));
				name = format!("{}_.as_ptr()", name);
				"&str".to_owned()
			} else if enums.contains_key(typ) {
				name = if typ.contains("Flags") {format!("{}.bits", name)} else {format!("{} as i32", name)};
				// typ.to_owned()
				enumname(typ)
			} else {
				let s = re_prefix.captures(typ).unwrap();
				let pre = &s["pre"].replace("*mut", "&mut");
				let t = &s["suf"];
				if types.contains(t) {
					if fix_type(t) == t {
						format!("{}sys::{}", pre, t)
					} else {
						format!("{}{}", pre, fix_type(t))
					}
				} else {
					if enums.contains_key(t) {
						name = format!("{} as {}i32", name, pre);
					}
					format!("{}{}", pre, fix_type(t))
				}
			};
			
			param_names.push(name.to_owned());
			params.push(format!("{}: {}", &param["name"], typ));
		}
		
		block.push_str(&format!("\tunsafe{{sys::{}({})}}", &cap["name"], param_names.join(", ")));
		
		let rtn = &cap["return"];
		let rtn = if enums.contains_key(rtn) {
			"i32".to_owned()
		} else {
			let s = re_prefix.captures(rtn).unwrap();
			let pre = &s["pre"];
			let t = &s["suf"];
			if types.contains(t) && fix_type(t) == t {
				format!("{}sys::{}", pre, t)
			} else {
				format!("{}{}", pre, fix_type(t))
			}
		};
		
		f.write_all(format!("\npub fn {}({}){} {{\n{}\n}}\n",
			name,
			params.join(", "),
			if cap.name("r") != None {format!(" -> {}", rtn)} else {"".to_owned()},
			block,
		).as_bytes()).unwrap();
	}
	
	let mut f = File::create("./src/gui/imgui/bindings.rs").unwrap();
	f.write_all(bindings
		.replace(" ImVec2,", " [f32; 2],")
		.replace(" ImVec2)", " [f32; 2])")
		.replace(" ImVec4,", " [f32; 4],")
		.replace(" ImVec4)", " [f32; 4])").as_bytes()).unwrap();
}