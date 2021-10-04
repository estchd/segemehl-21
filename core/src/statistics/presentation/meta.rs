#[derive(Debug, Copy, Clone)]
pub enum BinConfig {
	FixedSize(usize),
	FixedCount(usize),
}

#[derive(Debug, Copy, Clone)]
pub struct Meta {
	pub bin_config: BinConfig
}