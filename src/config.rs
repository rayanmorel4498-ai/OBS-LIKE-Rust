// visualisation_module/src/config.rs

use std::sync::{Arc, Mutex};
use std::fs;
use std::path::Path;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigFile {
    pub pool_ip: String,
    pub pool_port: u16,
    pub screen_min_fps: u32,
    pub screen_max_fps: u32,
    pub screen_compression: String,
    pub screen_quality: u32,
    pub audio_enabled: bool,
    pub audio_sample_rate: u32,
    pub audio_buffer_size: usize,
    pub audio_compression: String,
    pub audio_bitrate: u32,
    pub input_enabled: bool,
    pub input_sample_rate: u32,
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub logs_path: String,
    pub error_logs_path: String,
    pub log_level: String,
    pub ping_interval_idle_ms: u64,
    pub ping_interval_active_ms: u64,
    pub ping_timeout_ms: u64,
    pub ethernet_enabled: bool,
    pub bluetooth_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub file: ConfigFile,
}

#[derive(Debug, Clone)]
pub struct SystemResources {
    pub cpu: u32,
    pub ram: u32,
}

lazy_static! {
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::load()));
}

impl Config {
    /// Charge le fichier YAML avec valeurs par défaut
    pub fn load() -> Self {
        let path = Path::new("default.yaml");
        
        if !path.exists() {
            panic!("default.yaml manquant! Créez le fichier avec les paramètres requis.");
        }

        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Impossible de lire default.yaml: {}", e));

        match serde_yaml::from_str::<ConfigFile>(&content) {
            Ok(file_config) => Self { file: file_config },
            Err(e) => panic!("default.yaml invalide: {}. Vérifiez la syntaxe YAML.", e),
        }
    }

    pub fn default() -> Self {
        Self {
            file: ConfigFile {
                pool_ip: "127.0.0.1".to_string(),
                pool_port: 5000,
                screen_min_fps: 5,
                screen_max_fps: 60,
                screen_compression: "png".to_string(),
                screen_quality: 85,
                audio_enabled: true,
                audio_sample_rate: 48000,
                audio_buffer_size: 2048,
                audio_compression: "flate2".to_string(),
                audio_bitrate: 128,
                input_enabled: true,
                input_sample_rate: 60,
                cpu_cores: 4,
                ram_gb: 8,
                logs_path: "./logs".to_string(),
                error_logs_path: "./logs/errors".to_string(),
                log_level: "info".to_string(),
                ping_interval_idle_ms: 1000,
                ping_interval_active_ms: 100,
                ping_timeout_ms: 5000,
                ethernet_enabled: true,
                bluetooth_enabled: false,
            },
        }
    }

    pub fn get_pool_ip() -> String {
        CONFIG.lock().unwrap().file.pool_ip.clone()
    }

    pub fn get_pool_port() -> u16 {
        CONFIG.lock().unwrap().file.pool_port
    }

    pub fn get_logs_path() -> String {
        CONFIG.lock().unwrap().file.logs_path.clone()
    }

    pub fn get_error_logs_path() -> String {
        CONFIG.lock().unwrap().file.error_logs_path.clone()
    }

    pub fn get_available_resources() -> SystemResources {
        let conf = CONFIG.lock().unwrap();
        SystemResources {
            cpu: conf.file.cpu_cores,
            ram: conf.file.ram_gb,
        }
    }

    pub fn get_screen_max_fps() -> u32 {
        CONFIG.lock().unwrap().file.screen_max_fps
    }

    pub fn get_audio_sample_rate() -> u32 {
        CONFIG.lock().unwrap().file.audio_sample_rate
    }

    pub fn get_screen_compression() -> String {
        CONFIG.lock().unwrap().file.screen_compression.clone()
    }
}
