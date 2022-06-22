use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::path::Path;
use std::sync::Arc;
use vosk::{Model, Recognizer};

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
        info!("trying to load model in {}...", &name);

        match Model::new(&dir_path) {
            Ok(model) => {
                info!("loaded model in {}", &name);
                models.insert(name.to_string(), Arc::new(model));
            }
            Err(e) => {
                error!("failed to load model in {}: {}", &name, e);
            }
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
    for model_name in models.iter().map(|k| k.key().to_string()) {
        // removes and deallocates the model
        assert!(models.remove(&model_name).is_some());
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
pub fn get_stream(lang: &str) -> Option<Recognizer> {
    MODELS
        .get()
        .expect("models should've been initialized before attempting to get a stream")
        .get(lang)
        .map(|x| Recognizer::new(x.value(), 16_000.0))
}
