use serde::{Deserialize, Serialize};

pub mod penumbra;

#[derive(Deserialize, Serialize)]
pub struct Datas {
	pub penumbra: penumbra::Config,
}