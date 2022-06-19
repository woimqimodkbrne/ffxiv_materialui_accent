trait NullReader {
	fn null_terminated(&self) -> Option<String>;
}

impl NullReader for [u8] {
	fn null_terminated(&self) -> Option<String> {
		let mut s = String::with_capacity(128);
		let p = std::str::from_utf8(&self).ok()?;
		s.push_str(if let Some(l) = p.find('\0') {&p[0..l]} else {p});
		Some(s)
	}
}

pub mod formats;