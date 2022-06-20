use std::{fs::File, path::Path, collections::HashSet};
use anyhow::Context;
use crate::downloader::{download::Config, penumbra::{PenumbraFile, ConfOption}};

ffi!(fn explorer_datas_load(path: &str) -> *mut Config {
	Box::into_raw(Box::new(serde_json::from_reader(File::open(Path::new(path).join("datas.json"))?)?))
});

ffi!(fn explorer_datas_gamepaths(datas: *mut Config) -> Vec<String> {
	let datas = unsafe { &*datas };
	
	let mut gamepaths = HashSet::new();
	datas.penumbra.files.keys()
		.for_each(|p| {gamepaths.insert(p.to_string());});
	
	datas.penumbra.options.iter()
		.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
			opt.options.iter()
				.for_each(|s| s.files.keys()
					.for_each(|p| {gamepaths.insert(p.to_string());}))
		});
	
	Vec::from_iter(gamepaths)
});

ffi!(fn explorer_datas_option_gamepaths(datas: *mut Config, option: &str, suboption: &str) -> Vec<String> {
	let datas = unsafe { &*datas };
	
	let mut gamepaths = HashSet::new();
	if option == "" && suboption == "" {
		datas.penumbra.files.keys()
			.for_each(|p| {gamepaths.insert(p.to_string());});
	} else {
		datas.penumbra.options.iter()
			.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == option {
				opt.options.iter()
					.filter(|s| s.name == suboption)
					.for_each(|s| s.files.keys()
						.for_each(|p| {gamepaths.insert(p.to_string());}))
			});
	}
	
	Vec::from_iter(gamepaths)
});

ffi!(fn explorer_datas_paths(datas: *mut Config, gamepath: &str, option: &str, suboption: &str) -> Vec<Vec<String>> {
	let datas = unsafe { &*datas };
	
	let solve_file = |f: &PenumbraFile| -> Vec<Vec<String>> {
		match f {
			PenumbraFile::Simple(path) => vec![vec![path.to_string()]],
			PenumbraFile::Complex(paths) => paths.iter()
				.map(|o|
					o.iter()
					.map(|p| if let Some(path) = p {path.to_string()} else {"".to_string()})
					.collect())
				.collect()
		}
	};
	
	if option == "" && suboption == "" {
		solve_file(datas.penumbra.files.get(gamepath).context("Invalid gamepath")?)
	} else {
		solve_file(datas.penumbra.options.iter()
			.find_map(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == option {Some(opt)} else {None})
			.context("Invalid option name")?.options.iter()
				.find(|s| s.name == suboption)
				.context("Invalid suboption name")?.files.get(gamepath).context("Invalid gamepath")?)
	}
});

ffi!(fn explorer_datas_options(datas: *mut Config) -> Vec<String> {
	let datas = unsafe { &*datas };
	
	let mut options = Vec::new();
	datas.penumbra.options.iter()
		.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
			options.push(opt.name.to_string());
		});
	
	options
});

ffi!(fn explorer_datas_suboptions(datas: *mut Config, option: &str) -> Vec<String> {
	let datas = unsafe { &*datas };
	
	let mut options = Vec::new();
	datas.penumbra.options.iter()
		.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o && opt.name == option {
			opt.options.iter()
				.for_each(|o| options.push(o.name.to_string()))
		});
	
	options
});

// #[allow(dead_code)]
// #[repr(packed)]
// struct Paths {
// 	gamepaths: Vec<String>,
// 	realpaths: Vec<String>,
// }

// ffi!(fn explorer_mod_paths(path: &str) -> Paths {
// 	let data: Config = serde_json::from_reader(File::open(Path::new(path).join("datas.json"))?)?;
// 	let mut gamepaths = Vec::new();
// 	let mut realpaths = Vec::new();
	
// 	let mut do_files = |add: &str, files: &HashMap<String, PenumbraFile>| {
// 		files.iter()
// 			.for_each(|f| {
// 				gamepaths.push(format!("{}\0{}", add, f.0));
// 				match f.1 {
// 					PenumbraFile::Simple(path) => realpaths.push(path.to_string()),
// 					PenumbraFile::Complex(paths) => realpaths.push(
// 						paths.iter()
// 						.map(|o|
// 							o.iter()
// 							.map(|p| match p {
// 								Some(path) => path.to_string(),
// 								None => "".to_string(),
// 							}).collect::<Vec<String>>().join("\0")
// 						).collect::<Vec<String>>().join("\0\0")
// 					),
// 				}
// 			});
// 	};
	
// 	do_files("\0", &data.penumbra.files);
// 	data.penumbra.options.iter()
// 		.for_each(|o| if let ConfOption::Multi(opt) | ConfOption::Single(opt) = o {
// 			opt.options.iter()
// 				.for_each(|o| do_files(&format!("{}\0{}", opt.name, o.name), &o.files))
// 		});
	
// 	Paths {gamepaths, realpaths}
// });