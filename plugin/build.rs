#![allow(dead_code)]

fn main() {
	println!("cargo:rustc-link-search=./plugin/lib");
	println!("cargo:rustc-link-lib=cimgui");
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=./lib/cimgui.lib");
	
	// generate_bindings();
}

fn generate_bindings() {
	bindgen::Builder::default()
		.header("./lib/cimgui.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.clang_arg("--language=c++")
		.clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS")
		.layout_tests(false)
		// .default_enum_style(bindgen::EnumVariation::Rust{non_exhaustive: false})
		.prepend_enum_name(false)
		.generate()
		.unwrap()
		// .to_string()
		.write_to_file(std::path::Path::new("./src/imgui.rs"))
		.unwrap();
}