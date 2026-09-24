use std::{sync::LazyLock, fs, path::PathBuf};
use log::{info, warn};
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window_name: String,
    pub allowed_chars: String,

    pub start_pixel_x: u32,
    pub start_pixel_y: u32,

    pub roi_x: u32,
    pub roy_y: u32,
    pub roi_width: u32,
    pub roi_height: u32,

    pub char_width: u32,
    pub char_height: u32,

    pub scan_stride: u32,

    pub required_pixels_for_start: u32,
    pub required_pixels_for_borders: u32,

    pub start_pixel_red: u8,
    pub start_pixel_green: u8,
    pub start_pixel_blue: u8,

    pub start_char_r_max: u8,
    pub start_char_r_min: u8,
    pub start_char_g_max: u8,
    pub start_char_g_min: u8,
    pub start_char_b_max: u8,
    pub start_char_b_min: u8,

    pub bin_start_char_r_max: u8,
    pub bin_start_char_r_min: u8,
    pub bin_start_char_g_max: u8,
    pub bin_start_char_g_min: u8,
    pub bin_start_char_b_max: u8,
    pub bin_start_char_b_min: u8,

    pub char_white: u8,
    pub bin_char_white: u8,

    pub work_timer: u64,

    pub delay_before_word_appears: u64,
    pub delay_type_min: u64,
    pub delay_type_max: u64,
    pub delay_before_new_capture: u64,
    pub delay_defore_new_word_appears: u64,
}


pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    match load_config_from_file() {
        Ok(cfg) => {
            info!("Config loaded from file");
            cfg
        }
        Err(e) => {
            warn!("Could not loading config.toml: {}", e);
            println!("Could not loading config.toml, see logs {}", e);
            println!("There is no default value, bye");
            let _ = std::io::stdin().read_line(&mut String::new());
            panic!("Config loading failed");
        }
    }
});


fn load_config_from_file() -> Result<Config> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap();
    let config_path = exe_dir.join("Config.toml");

    let config_path = if config_path.exists() {
        config_path
    } else {
        PathBuf::from("Config.toml")
    };

    let config_str = fs::read_to_string(&config_path)
        .map_err(|e| anyhow::anyhow!("Coild not read {:?}: {}", config_path, e))?;

    let config: Config = toml::from_str(&config_str)
        .map_err(|e| anyhow::anyhow!("Parsing error config.toml: {}", e))?;

    Ok(config)
}