use once_cell::sync::OnceCell;

/// Http client for non-Discord requests
static THIRDPARTY_HTTP_CLIENT: OnceCell<reqwest::Client> = OnceCell::new();

pub fn get_thirdparty_http() -> &'static reqwest::Client {
	THIRDPARTY_HTTP_CLIENT
		.get()
		.expect("http should be set before calling get_http")
}

pub fn init_thirdparty_http() {
	THIRDPARTY_HTTP_CLIENT
		.set(reqwest::Client::new())
		.unwrap_or_else(|_| panic!("init_thirdparty_http should be called only once"))
}
