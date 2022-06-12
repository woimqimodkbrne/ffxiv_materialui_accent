use std::{io::Cursor, fs::{File, self}, path::Path};
use reqwest::blocking as req;

const SERVER: &'static str = "http://localhost:8080";

lazy_static! {
	static ref CLIENT: req::Client = req::Client::new();
}

ffi!(fn server_search(query: &str, tags: &[i16], page: i32) -> String {
	let tags = tags.into_iter().map(|e| e.to_string()).collect::<Vec<String>>().join(",");
	CLIENT.get(format!("{}/search.json?query={}&tags={}&page={}", SERVER, query, tags, page))
		.send()
		.unwrap()
		.text()
		.unwrap()
});

ffi!(fn server_mod(id: &str) -> String {
	CLIENT.get(format!("{}/mod/{}.json", SERVER, id))
		.send()
		.unwrap()
		.text()
		.unwrap()
});

#[repr(C)] struct Img(u32, u32, Vec<u8>);
ffi!(fn server_download_preview(modid: &str, file: &str) -> Img {
	let img = image::io::Reader::new(Cursor::new(CLIENT.get(format!("{}/mod/{}/{}", SERVER, modid, file))
		.send()
		.unwrap()
		.bytes()
		.unwrap()
		.to_vec()))
		.with_guessed_format()
		.unwrap()
		.decode()
		.unwrap()
		.into_rgba8();
	
	Img(img.width(), img.height(), img.into_raw())
});

// This shouldnt be here
// TODO: organize this mess
ffi!(fn read_image(file: &str) -> Img {
	let img = image::io::Reader::open(file)
		.unwrap()
		.with_guessed_format()
		.unwrap()
		.decode()
		.unwrap()
		.into_rgba8();
	
	Img(img.width(), img.height(), img.into_raw())
});

// Move to moddev mod?
// TODO: organize this mess 2.0
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