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

ffi!(fn server_download_preview(modid: i32, file: &str) -> Vec<u8> {
	CLIENT.get(format!("{}/mod/{}/{}", SERVER, modid, file))
		.send()
		.unwrap()
		.bytes()
		.unwrap()
		.to_vec()
});