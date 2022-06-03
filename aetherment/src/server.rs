use std::io::Cursor;
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

#[repr(C)] struct Img(u32, u32, Vec<u8>);
ffi!(fn server_download_preview(modid: i32, file: &str) -> Img {
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