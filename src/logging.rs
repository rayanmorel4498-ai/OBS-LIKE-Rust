// visualisation_module/src/logging.rs

use std::sync::{Arc, Mutex};
use std::fs::{OpenOptions, create_dir_all, metadata};
use std::io::Write;
use std::path::Path;
use chrono::Local;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub module: String,
    pub message: String,
    pub timestamp: String,
    pub level: String,
}

impl LogEntry {
    pub fn new<S: Into<String>>(module: S, message: S) -> Self {
        Self {
            module: module.into(),
            message: message.into(),
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            level: "INFO".to_string(),
        }
    }

    pub fn debug<S: Into<String>>(module: S, message: S) -> Self {
        let mut entry = Self::new(module, message);
        entry.level = "DEBUG".to_string();
        entry
    }

    pub fn warn<S: Into<String>>(module: S, message: S) -> Self {
        let mut entry = Self::new(module, message);
        entry.level = "WARN".to_string();
        entry
    }

    pub fn error<S: Into<String>>(module: S, message: S) -> Self {
        let mut entry = Self::new(module, message);
        entry.level = "ERROR".to_string();
        entry
    }

    pub fn format(&self) -> String {
        format!("[{}][{}][{}] {}", self.timestamp, self.level, self.module, self.message)
    }
}

pub struct LoggingManager {
    logs: Arc<Mutex<Vec<LogEntry>>>,
    log_path: String,
    max_entries: usize,
}

impl LoggingManager {
    pub fn init() -> Arc<Self> {
        let log_path = crate::config::Config::get_logs_path();
        let max_entries = 10000;

        if !Path::new(&log_path).exists() {
            let _ = create_dir_all(&log_path);
        }

        Arc::new(Self {
            logs: Arc::new(Mutex::new(Vec::new())),
            log_path,
            max_entries,
        })
    }

    pub fn push_log(&self, entry: LogEntry) {
        {
            let mut logs = self.logs.lock().unwrap();
            logs.push(entry.clone());

            if logs.len() > self.max_entries {
                logs.remove(0);
            }
        }

        // Écrire dans le fichier
        let _ = self.write_to_file(&entry);

        // Afficher en console
        println!("{}", entry.format());
    }

    fn write_to_file(&self, entry: &LogEntry) -> std::io::Result<()> {
        let filename = format!(
            "{}/app_{}.log",
            self.log_path,
            Local::now().format("%Y%m%d")
        );

        // Vérifier taille avant écriture
        if let Ok(meta) = metadata(&filename) {
            if meta.len() > 10_485_760 {  // 10MB
                self.rotate_log_file(&filename);
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)?;

        writeln!(file, "{}", entry.format())?;
        Ok(())
    }

    /// Rotation du fichier log : renommer en .1, .2, etc.
    fn rotate_log_file(&self, file_path: &str) {
        let mut next_num = 1;
        loop {
            let rotated_path = format!("{}.{}", file_path, next_num);
            if !Path::new(&rotated_path).exists() {
                let _ = std::fs::rename(file_path, rotated_path);
                break;
            }
            next_num += 1;
            if next_num > 10 {  // Maximum 10 fichiers rotationnés
                break;
            }
        }
    }

    pub fn get_logs(&self) -> Vec<LogEntry> {
        self.logs.lock().unwrap().clone()
    }

    pub fn get_logs_since(&self, timestamp: String) -> Vec<LogEntry> {
        let logs = self.logs.lock().unwrap();
        logs.iter()
            .filter(|e| e.timestamp >= timestamp)
            .cloned()
            .collect()
    }

    pub fn clear(&self) {
        self.logs.lock().unwrap().clear();
    }
}

lazy_static! {
    pub static ref LOGGER: Arc<LoggingManager> = LoggingManager::init();
}
