use crate::cfg::BotConfig;
use once_cell::sync::OnceCell;
use std::fs;

static GLOBAL_CONFIG: OnceCell<BotConfig> = OnceCell::new();

pub fn load_config(cfg_path: &str) {
    let cfg = fs::read(cfg_path).expect("failed to read config");

    let parsed_cfg: BotConfig = toml::from_slice(&cfg[..]).expect("config invalid");

    if parsed_cfg.secret_key == "LcOnTm2274zt7Hh5YboqihqFxUWPksV9" {
        println!("generate a new secret key with `openssl rand -base64 24`");
        panic!("secret key was unchanged!");
    } else if parsed_cfg.secret_key.len() != 32 {
        println!("generate a new secret key that's exactly 32 characters long with `openssl rand -base64 24`");
        panic!("secret key was too short!");
    }

    GLOBAL_CONFIG
        .set(parsed_cfg)
        .unwrap_or_else(|_| panic!("don't call `load_config()` more than once"));
}

pub fn get_config() -> &'static BotConfig {
    GLOBAL_CONFIG
        .get()
        .expect("called `get_config()` before config was initialized")
}
