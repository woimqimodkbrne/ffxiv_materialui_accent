#![feature(let_chains)]

use std::io::{Read, Write, Seek};

trait NullReader {
	fn null_terminated(&self) -> Option<String>;
}

impl NullReader for [u8] {
	fn null_terminated(&self) -> Option<String> {
		let p = std::str::from_utf8(&self).ok()?;
		Some(if let Some(l) = p.find('\0') {&p[0..l]} else {p}.to_owned())
	}
}

pub mod formats;

// there probably is a smarter way to do this but thats to figure out later
pub fn convert<R, W>(source: &mut R, source_ext: &str, target: &mut W, target_ext: &str) -> bool where
R: Read + Seek,
W: Write + Seek {
	use formats::{game::tex::Tex, external::{dds::Dds, png::Png}};
	
	if source_ext == target_ext {
		let mut buf = [0u8; 4096];
		while let readcount = source.read(&mut buf).unwrap() && readcount != 0 {
			target.write_all(&buf[0..readcount]).unwrap();
		}
		
		return true;
	}
	
	match source_ext {
		"tex" | "atex" => {
			let v = Tex::read(source);
			match target_ext {
				"dds" => <Tex as Dds>::write(&v, target),
				"png" => <Tex as Png>::write(&v, target),
				_ => return false,
			}
		}
		_ => match target_ext {
			"tex" | "atex" => {
				match source_ext {
					"dds" => <Tex as Dds>::read(source),
					"png" => <Tex as Png>::read(source),
					_ => return false,
				}.write(target);
			}
			_ => return false,
		},
	}
	
	true
}