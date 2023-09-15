use std::{fs::File, io::Read};

pub struct Generic {
	name: String,
	path: String,
	real_path: Option<String>,
	data: Option<Vec<u8>>,
}

impl Generic {
	pub fn new(path: &str, real_path: Option<&str>) -> Result<Self, super::BacktraceError> {
		let mut v = Self {
			name: path.split("/").last().unwrap().to_owned(),
			path: path.to_owned(),
			real_path: real_path.map(|v| v.to_owned()),
			data: None,
		};
		
		v.load_data()?;
		
		Ok(v)
	}
	
	fn load_data(&mut self) -> Result<(), super::BacktraceError> {
		self.data = None;
		let data = if let Some(real_path) = &self.real_path {
			let mut file = File::open(real_path).map_err(|_| super::ExplorerError::RealPath(real_path.clone()))?;
			let mut buf = Vec::new();
			file.read_to_end(&mut buf)?;
			buf
		} else {
			crate::noumenon().as_ref().ok_or(super::ExplorerError::Path(self.path.clone()))?.file::<Vec<u8>>(&self.path)?
		};
		
		self.data = Some(data);
		
		Ok(())
	}
}

impl super::View for Generic {
	fn name(&self) -> &str {
		&self.name
	}
	
	fn path(&self) -> &str {
		&self.path
	}
	
	fn exts(&self) -> Vec<&str> {
		vec![self.name.split(".").last().unwrap()]
	}
	
	fn render(&mut self, ui: &mut egui::Ui) -> Result<(), super::BacktraceError> {
		ui.label("TODO");
		
		Ok(())
	}
	
	fn export(&self, _ext: &str, mut writer: Box<dyn super::Writer>) -> Result<(), super::BacktraceError> {
		if let Some(data) = &self.data {
			writer.write_all(&data)?;
			Ok(())
		} else {
			Err(super::ExplorerError::Data.into())
		}
	}
}