use crate::{ModelState, StreamingState};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub static MODELS: OnceCell<DashMap<String, ModelState>> = OnceCell::new();

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
                    CString::new(path.as_os_str().as_bytes())
                        .expect("path contains null bytes in it"),
                );
            } else if ext == "scorer" {
                scorer_path = Some(
                    CString::new(path.as_os_str().as_bytes())
                        .expect("path contains null bytes in it"),
                );
            }
        }
        if let Some(model_path) = model_path {
            info!("found model: {:?}", model_path);

            let mut model_state = std::ptr::null_mut::<coqui_stt_sys::ModelState>();
            let retval = unsafe {
                coqui_stt_sys::STT_CreateModel(
                    model_path.as_ptr(),
                    std::ptr::addr_of_mut!(model_state),
                )
            };
            if retval != 0 || model_state.is_null() {
                error!(
                    "failed to create model: {:?} (code {:?})",
                    model_path, retval
                );
                continue;
            }
            let model = crate::ModelState::new(model_state);

            if let Some(scorer_path) = scorer_path {
                info!("found scorer: {:?}", scorer_path);
                let retval = unsafe {
                    coqui_stt_sys::STT_EnableExternalScorer(model.as_ptr(), scorer_path.as_ptr())
                };
                if retval != 0 {
                    error!(
                        "failed to enable external scorer: {:?} (code {:?})",
                        scorer_path, retval
                    );
                    continue;
                } else {
                    info!("enabled external scorer");
                }
            }
            info!("loaded model, inserting into map");
            models.insert(name.to_string(), model);
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
pub fn get_stream(lang: &str) -> Option<StreamingState> {
    let model = MODELS
        .get()
        .expect("models should've been initialized before attempting to get a stream")
        .get(lang)?;

    let mut stream = std::ptr::null_mut::<coqui_stt_sys::StreamingState>();
    let retval =
        unsafe { coqui_stt_sys::STT_CreateStream(model.as_ptr(), std::ptr::addr_of_mut!(stream)) };
    if retval != 0 || stream.is_null() {
        error!("failed to create stream: {:?} (code {:?})", lang, retval);
        return None;
    }
    Some(crate::StreamingState::new(stream))
}
