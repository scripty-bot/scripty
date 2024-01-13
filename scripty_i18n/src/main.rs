fn main() {
	// we can assume we're running at the root of the workspace

	// Load the translations
	let num_langs = scripty_i18n::init_i18n_with_path(
		std::env::current_dir()
			.expect("failed to get current dir")
			.join("scripty_i18n")
			.join("locales"),
	);
	println!("Loaded {} languages", num_langs);

	// Iterate over every language and check some specific strings for validity
	let mut errors: Vec<String> = vec![];
	for lang in scripty_i18n::get_all_bundle_languages()
		.into_iter()
		.map(|x| scripty_i18n::get_bundle_for_language(&x))
	{
		lang.value().set_transform()
	}
}
