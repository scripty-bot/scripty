use coqui_stt::{Model, Stream};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::path::Path;

pub static MODELS: OnceCell<DashMap<String, Stream>> = OnceCell::new();

pub fn load_models(model_dir: &Path) {
    let models = MODELS.get_or_init(DashMap::new);
    for dir in model_dir.read_dir().expect("IO error") {
        let dir = dir.expect("IO error");
        let dir_path = dir.path();
        if !dir_path.is_dir() {
            continue;
        }
        let file_name = dir.file_name();
        let name = file_name.to_string_lossy();
        if name.len() != 2 {
            continue;
        }
        info!("searching for models in dir {}...", &name);

        let mut model_path = None;
        let mut scorer_path = None;
        for file in dir_path.read_dir().expect("IO error") {
            let file = file.expect("IO error");
            let path = file.path();
            let ext = match path.extension() {
                Some(ext) => ext,
                None => continue,
            };
            if ext == "pb" || ext == "pbmm" {
                model_path = Some(
                    path.to_str()
                        .expect("non-utf-8 chars found in filename")
                        .to_owned(),
                );
            } else if ext == "scorer" {
                scorer_path = Some(
                    path.to_str()
                        .expect("non-utf-8 chars found in filename")
                        .to_owned(),
                );
            }
        }
        if let Some(model_path) = model_path {
            info!("found model: {:?}", model_path);
            let mut model = Model::new(model_path).expect("failed to load model");
            if let Some(scorer_path) = scorer_path {
                info!("found scorer: {:?}", scorer_path);
                model
                    .enable_external_scorer(scorer_path)
                    .expect("failed to load scorer");
            }
            let stream = model
                .into_streaming()
                .expect("failed to convert Model into Stream");
            models.insert(name.to_string(), stream);
        }
    }
    if models.is_empty() {
        panic!(
            "no models found:\
             they must be in a subdirectory with their language name like `en/model.pbmm`"
        )
    }
}
