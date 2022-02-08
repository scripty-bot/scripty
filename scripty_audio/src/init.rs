use std::path::Path;

pub fn init_stt() {
    let cfg = scripty_config::get_config();

    crate::models::load_models(Path::new(&cfg.model_dir));

    crate::threadpool::init_threadpool();
}
