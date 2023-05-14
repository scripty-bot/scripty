use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};

static DATABASE_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub fn get_db() -> &'static Pool<Postgres> {
	DATABASE_POOL
		.get()
		.expect("called `get_db()` before initializing db")
}

pub fn set_db(db: Pool<Postgres>) {
	DATABASE_POOL
		.set(db)
		.unwrap_or_else(|_| panic!("called `set_db()` more than once"))
}
