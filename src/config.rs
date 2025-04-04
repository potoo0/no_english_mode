use dotenvy::dotenv;
use std::env;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub ime_input_method: u32,
    pub ime_cmode: isize,
    pub sleep_millis: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            ime_input_method: env::var("IME_SWITCHER_INPUT_METHOD")
                .ok()
                .or(Some("0x0804".into()))
                .and_then(|hex_str| Some(hex_str.trim_start_matches(|c| c == '0' || c == 'x' || c == 'X').to_string()))
                .and_then(|hex_str| u32::from_str_radix(&hex_str, 16).ok())
                .expect("IME_SWITCHER_INPUT_METHOD must be hex-string, see https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/default-input-locales-for-windows-language-packs?view=windows-11#input-locales"),

            ime_cmode: env::var("IME_SWITCHER_CMODE")
                .unwrap_or("1025".into())
                .parse()
                .expect("IME_SWITCHER_CMODE must be integer, see https://learn.microsoft.com/en-us/windows/win32/intl/ime-conversion-mode-values"),

            sleep_millis: env::var("IME_SWITCHER_SLEEP_MILLIS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(50),
        }
    }
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::from_env());
