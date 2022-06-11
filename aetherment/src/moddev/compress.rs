use std::{path::Path, collections::HashSet, fs::{self, File}, io::{Read, Write}};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

ffi!(fn compress_mod(mod_path: &str) {
	compress(Path::new(mod_path));
});

pub fn compress(mod_path: &Path) {
	let mut curfiles = fs::read_dir(mod_path.join("files")).unwrap().into_iter()
		.map(|f| f.unwrap().file_name().to_str().unwrap().to_string())
		.collect::<HashSet<String>>();
	
	fs::create_dir_all(mod_path.join("files_compressed")).unwrap();
	fs::read_dir(mod_path.join("files_compressed")).unwrap().into_iter().for_each(|f| {
		let f = f.unwrap();
		let filename = f.file_name();
		let filename = filename.to_str().unwrap();
		if !curfiles.remove(filename) {
			fs::remove_file(f.path()).unwrap();
		}
	});
	
	// brotli 11 quality, 22 lg_window_size
	// zlib default, no clue what that is
	// brotli single threaded: Material UI: 168  seconds, 178 MB > 22.4 MB
	// brotli multi  threaded: Material UI: 26   seconds, 178 MB > 22.4 MB
	// brotli single threaded: TBSE:        1955 seconds, 1.44 GB > 311 MB
	// brotli multi  threaded: TBSE:        270  seconds, 1.44 GB > 311 MB
	// brotli multi  threaded: Bibo+:       364  seconds, 1.33 GB > 233 MB
	// zlib   single threaded: Material UI: 5    seconds, 178 MB > 27.8 MB
	// zlib   multi  threaded: Material UI: 3    seconds, 178 MB > 27.8 MB
	// zlib   single threaded: TBSE:        73   seconds, 1.44 GB > 421 MB
	// zlib   multi  threaded: TBSE:        10   seconds, 1.44 GB > 421 MB
	// zlib   multi  threaded: Bibo+:       6    seconds, 1.33 GB > 325 MB
	// while brotli has better compression ratio and is supposed to be faster
	// decompression (haven't tested myself), it's really fucking slow
	// TODO: mby allow the author to select which method they'd like to use?
	// probably not worth it tho
	
	curfiles.into_par_iter().for_each(|filename| {
		log!(log, "{} Compressing {}", mod_path.file_name().unwrap().to_str().unwrap(), filename);
		let mut file_uncompressed = File::open(mod_path.join("files").join(&filename)).unwrap();
		let file = File::create(mod_path.join("files_compressed").join(&filename)).unwrap();
		let mut buf = [0u8; 4096];
		
		// let mut writer = brotli::CompressorWriter::new(file, 4096, 11, 22);
		// while file_uncompressed.read(&mut buf).unwrap() != 0 {
		// 	writer.write_all(&buf).unwrap();
		// }
		// writer.flush().unwrap();
		
		let mut writer = flate2::write::ZlibEncoder::new(file, flate2::Compression::default());
		while file_uncompressed.read(&mut buf).unwrap() != 0 {
			writer.write_all(&buf).unwrap();
		}
		writer.finish().unwrap();
	});
}