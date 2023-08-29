#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: u64,
	pub shards:  u64,
}
