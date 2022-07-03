use serde::Deserialize;

pub mod penumbra;

#[derive(Deserialize)]
pub struct Datas {
	pub penumbra: penumbra::Config,
}