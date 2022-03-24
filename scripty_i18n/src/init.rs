use crate::{init_cache, set_i18n_store};
use dashmap::DashMap;
use fluent::{bundle::FluentBundle, FluentResource};
use std::fs::read_dir;
use unic_langid::LanguageIdentifier;

pub fn init_i18n() {
    let cfg = scripty_config::get_config();

    let bundles = DashMap::new();
    for i18n_file in
        read_dir(&cfg.i18n_dir).expect("failed to read i18n dir: does it exist and is readable?")
    {
        let f = i18n_file.expect("failed to read i18n file info");
        let path = f.path();
        let stem = match path.file_stem() {
            Some(s) => s,
            None => {
                warn!("no filename for file {:?}", f);
                continue;
            }
        };
        let lang_id = match stem.to_string_lossy().parse::<LanguageIdentifier>() {
            Ok(id) => id,
            Err(e) => {
                warn!(
                    "filename {:?} is not a valid language identifier: {}",
                    path, e
                );
                continue;
            }
        };
        info!("found language {}", lang_id);
        let resource = match std::fs::read_to_string(&path).map(FluentResource::try_new) {
            Ok(Ok(r)) => r,
            Err(e) => {
                warn!(file=?path, %lang_id, "failed to read file to string: {}", e);
                continue;
            }
            Ok(Err((r, errs))) => {
                warn!(file=?path, %lang_id, "failed to parse Fluent resource: {} errors", errs.len());
                for err in errs {
                    warn!(file=?path, %lang_id, "failed to parse file as Fluent bundle: {:?}", err);
                }
                r
            }
        };
        let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
        if let Err(errs) = bundle.add_resource(resource) {
            for err in errs {
                warn!(%lang_id, "failed to add Fluent resource: {:?}", err);
            }
        };
        info!(%lang_id, "loaded lang successfully");
        bundles.insert(lang_id, bundle);
    }
    info!("found {} language localizations", bundles.len());
    set_i18n_store(bundles);

    init_cache();
}
