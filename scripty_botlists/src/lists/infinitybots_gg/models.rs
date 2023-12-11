#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: usize,
	pub shards:  u16,
}
