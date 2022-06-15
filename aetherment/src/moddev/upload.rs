use std::{path::Path, fs::{File, self}};
use reqwest::blocking as req;
use crate::{CLIENT, SERVER};

ffi!(fn upload_mod(mod_path: &str) {
	let mod_path = Path::new(mod_path);
	
	crate::moddev::index::index(mod_path);
	
	// TODO: login, proper token, figure out a proper way to store the token
	let mut form = req::multipart::Form::new()
		.text("token", "\0".repeat(64))
		.part("meta", req::multipart::Part::reader(File::open(mod_path.join("meta.json")).unwrap()))
		.part("datas", req::multipart::Part::reader(File::open(mod_path.join("datas.json")).unwrap()))
		.part("index", req::multipart::Part::reader(File::open(mod_path.join("index.json")).unwrap()));
	
	let previews_path = mod_path.join("previews");
	if previews_path.exists() {
		for f in fs::read_dir(previews_path).unwrap().into_iter() {
			form = form.part("preview", req::multipart::Part::reader(File::open(f.unwrap().path()).unwrap()));
		}
	}
	
	for f in fs::read_dir(mod_path.join("files_compressed")).unwrap().into_iter() {
		form = form.part("file", req::multipart::Part::reader(File::open(f.unwrap().path()).unwrap()));
	}
	
	let resp = CLIENT.post(format!("{}/mod", SERVER))
		.multipart(form)
		.send()
		.unwrap();
	
	log!(log, "{}\n{}", resp.status(), resp.text().unwrap())
});