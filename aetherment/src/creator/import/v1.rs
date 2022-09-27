use std::{path::Path, fs::{self, File}, io::{Write, Cursor}, collections::{HashMap, HashSet, BTreeSet}};
use noumenon::formats::{game::tex::Tex, external::dds::Dds};
use serde::Deserialize;
use serde_json::json;
use crate::apply::penumbra;
use super::super::modpack::Error;

pub fn import<P, P2>(v1_dir: P, target_dir: P2) -> Result<(), Error> where
P: AsRef<Path>,
P2: AsRef<Path> {
	let v1_dir = v1_dir.as_ref();
	let target_dir = target_dir.as_ref();
	let files_dir = target_dir.join("files");
	if !files_dir.exists() {fs::create_dir_all(&files_dir)?}
	
	#[derive(Deserialize)]
	struct Meta {
		name: String,
		description: String,
		color_options: Vec<ColorOption>,
		penumbra: Vec<PenumbraOption>,
	}
	
	#[derive(Deserialize)]
	struct ColorOption {
		id: String,
		name: String,
		default: HashMap<String, u8>,
	}
	
	#[derive(Deserialize)]
	struct PenumbraOption {
		name: String,
		options: HashMap<String, Vec<String>>,
	}
	
	let v1_meta: Meta = serde_json::from_reader(File::open(v1_dir.join("options.json"))?)?;
	File::create(target_dir.join("meta.json"))?.write_all(
		crate::serialize_json(json!({
			"name": v1_meta.name,
			"description": v1_meta.description,
			"nsfw": false,
			"previews": [],
			"contributors": [],
			"dependencies": [],
		})).as_bytes()
	)?;
	
	let mut option_paths = HashSet::new();
	for opt in &v1_meta.penumbra {
		for (_name, sub) in &opt.options {
			for path in sub {
				option_paths.insert(path.to_ascii_lowercase().split("/option").next().unwrap().to_owned());
			}
		}
	}
	
	let mui_colors = HashMap::from([
		("accent", ("Accent", [99.0f32 / 255.0, 60.0 / 255.0, 181.0 / 255.0])),
		("party", ("Party and Alliance List Target", [1.0, 173.0 / 255.0, 56.0 / 255.0])),
		("enemy", ("Enemy List Target", [1.0, 173.0 / 255.0, 56.0 / 255.0])),
		("proc", ("Ability Proc", [1.0, 173.0 / 255.0, 56.0 / 255.0])),
		("shield", ("Party List Shield", [1.0, 211.0 / 255.0, 0.0])),
		("targetcast", ("Target Castbar", [209.0 / 255.0, 134.0 / 255.0, 11.0 / 255.0])),
		("targetcastinter", ("Target Castbar Interrupt", [139.0 / 255.0, 15.0 / 255.0, 15.0 / 255.0])),
		("recast", ("Recast", [1.0, 1.0, 1.0])),
		("recast1", ("Stack-Based oGCD Recharge", [1.0, 1.0, 1.0])),
		("recast2", ("Extended GCD Recharge", [214.0 / 255.0, 109.0 / 255.0, 0.0])),
		("limitbreak1", ("Limit Break Filling", [25.0 / 255.0, 220.0 / 255.0, 222.0 / 255.0])),
		("limitbreak2", ("Limit Break Full", [190.0 / 255.0, 99.0 / 255.0, 221.0 / 255.0])),
	]);
	
	let handle_file = |filepath: &Path| -> Result<String, Error> {
		log!("{:?}", filepath);
		let mut data = Vec::new();
		<Tex as Dds>::read(&mut File::open(filepath)?).write(&mut Cursor::new(&mut data));
		let hash = crate::hash_str(blake3::hash(&data).as_bytes());
		let path = format!("files/{hash}");
		File::create(files_dir.join(hash))?.write_all(&data)?;
		Ok(path)
	};
	
	let mut used_mui_colors = BTreeSet::new();
	let mut handle_folder = |relative_path: String| -> Result<(String, penumbra::PenumbraFile), Error> {
		let path = v1_dir.join("elements_black").join(&relative_path);
		let mut paths = Vec::<penumbra::FileLayer>::new();
		
		if path.join("underlay.dds").exists() {
			paths.push(penumbra::FileLayer {
				id: None,
				paths: vec![handle_file(&path.join("underlay.dds"))?]
			});
		}
		
		if path.join("overlay.dds").exists() {
			used_mui_colors.insert("accent".to_owned());
			paths.push(penumbra::FileLayer {
				id: Some("accent".to_owned()),
				paths: vec![handle_file(&path.join("overlay.dds"))?]
			});
		}
		
		for (id, _) in &mui_colors {
			let path = path.join(format!("overlay_{id}.dds"));
			if path.exists() {
				used_mui_colors.insert(id.to_string());
				paths.push(penumbra::FileLayer {
					id: Some(id.to_string()),
					paths: vec![handle_file(&path)?]
				});
			}
		}
		
		for clr in &v1_meta.color_options {
			let path = path.join(format!("overlay_{}.dds", clr.id));
			if path.exists() {
				paths.push(penumbra::FileLayer {
					id: Some(clr.id.clone()),
					paths: vec![handle_file(&path)?]
				});
			}
		}
		
		lazy_static! {static ref ICON: regex::Regex = regex::Regex::new(r"/icon/(?P<first>\d{3})(?P<second>\d{3})").unwrap();}
		
		Ok((
			ICON.replace(&format!("{}_hr1.tex", relative_path.split("/option").next().unwrap()), "/icon/${first}000/${first}${second}").into_owned(),
			penumbra::PenumbraFile(paths)
		))
	};
	
	let mut penumbra = penumbra::Config::default();
	
	// Non option paths
	for rootpath in ["ui/uld", "ui/icon"] {
		if let Ok(v) = fs::read_dir(v1_dir.join("elements_black").join(rootpath)) {
			for entry in v {
				let path = format!("{rootpath}/{}", entry?.file_name().to_str().unwrap());
				if option_paths.contains(&path) {continue}
				
				let (a, b) = handle_folder(path)?;
				penumbra.files.insert(a, b);
			}
		}
	}
	
	// Option paths
	for opt in v1_meta.penumbra {
		let mut option = penumbra::TypPenumbra {
			name: opt.name,
			description: "".to_owned(),
			options: Vec::new(),
		};
		
		for (name, sub) in opt.options {
			// penumbra.options.insert(index, element)
			let mut sub_option = penumbra::PenumbraOption {
				name: name,
				..Default::default()
			};
			
			for path in sub {
				let (a, b) = handle_folder(path)?;
				sub_option.files.insert(a, b);
			}
			
			option.options.push(sub_option);
		}
		
		penumbra.options.push(penumbra::ConfOption::Single(option));
	}
	
	// Color options, reversed insert to have correct order above non color options
	for clr in v1_meta.color_options.into_iter().rev() {
		penumbra.options.insert(0, penumbra::ConfOption::Rgb(penumbra::TypRgb {
			id: clr.id,
			name: clr.name,
			description: "".to_owned(),
			default: [clr.default["r"] as f32 / 255.0, clr.default["g"] as f32 / 255.0, clr.default["b"] as f32 / 255.0],
		}));
	}
	
	for id in used_mui_colors {
		let (name, default) = mui_colors[id.as_str()];
		penumbra.options.insert(0, penumbra::ConfOption::Rgb(penumbra::TypRgb {
			id,
			name: name.to_owned(),
			description: "".to_owned(),
			default,
		}));
	}
	
	File::create(target_dir.join("datas.json"))?.write_all(
		crate::serialize_json(json!(crate::apply::Datas {
			penumbra: Some(penumbra),
			..Default::default()
		})).as_bytes()
	)?;
	
	Ok(())
}