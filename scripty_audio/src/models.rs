use coqui_stt::{Model, Stream};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::path::Path;
use std::sync::Arc;

pub static MODELS: OnceCell<DashMap<String, Arc<Model>>> = OnceCell::new();

pub fn load_models(model_dir: &Path) {
    info!("initializing global model map");
    let models = MODELS.get_or_init(DashMap::new);
    info!("finding models in model dir");
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
            if ext == "tflite" {
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
            info!("loaded model, inserting into map");
            models.insert(name.to_string(), Arc::new(model));
        }
    }
    if models.is_empty() {
        panic!(
            "no models found:\
             they must be in a subdirectory with their language name like `en/model.tflite`"
        )
    } else {
        info!("loaded {} models", models.len());
    }
}

/// Removes all models and deallocates them.
pub fn deallocate_models() {
    let models = MODELS.get().expect("no models allocated");
    debug!("found {} models to deallocate", models.len());
    let model_name_list = models
        .iter()
        .map(|k| k.key().to_string())
        .collect::<Vec<_>>();
    for model_name in model_name_list {
        // removes and deallocates the model
        debug!("removing model {}", &model_name);
        let m = models.remove(&model_name);
        debug!("removed model {} from map, dropping", &model_name);
        drop(m);
        debug!("deallocated model {}", &model_name);
    }
}

/// Get all the currently registered model languages.
pub fn get_model_languages() -> Vec<String> {
    MODELS
        .get()
        .expect("load models before fetching names")
        .iter()
        .map(|x| x.key().clone())
        .collect()
}

/// Check if a model language is supported.
///
/// Cheaper than `get_model_languages()`.
pub fn check_model_language(lang: &str) -> bool {
    MODELS
        .get()
        .expect("load models before fetching names")
        .contains_key(lang)
}

/// Get a stream for the selected language.
pub fn get_stream(lang: &str) -> Option<Stream> {
    MODELS
        .get()
        .expect("models should've been initialized before attempting to get a stream")
        .get(lang)
        .and_then(|x| Stream::from_model(x.value().clone()).ok())
}
