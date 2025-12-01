use std::collections::HashMap;
use serde::Deserialize;
use std::fs;
use std::error::Error;

#[derive(Deserialize)]
pub struct PathsConfig {
    pub links_file: String,
    pub save_root: String,
    pub sub_dir: String,
    pub start_name: String,
}

#[derive(Deserialize)]
pub struct TimingsConfig {
    pub request_delay: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub paths: PathsConfig,

    pub timings: TimingsConfig,

    #[serde(rename = "cookies")]
    pub site_cookies: HashMap<String, String>,


}

impl Config {
    pub fn get_cookie_for_site(&self, site_name: &str) -> &str {
        self.site_cookies
            .get(site_name)
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}


pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_content = fs::read_to_string("config.toml")
        .map_err(|e| format!("Ошибка чтения config.toml: {}", e))?;

    let config: Config = toml::from_str(&config_content)
        .map_err(|e| format!("Ошибка парсинга config.toml: {}", e))?;

    Ok(config)
}