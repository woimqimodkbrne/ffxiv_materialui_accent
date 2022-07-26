use serde::{Deserialize, Serialize};

pub mod penumbra;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Datas {
	pub penumbra: penumbra::Config,
}