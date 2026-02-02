// visualisation_module/src/capture/error.rs

use std::sync::{Arc, Mutex};
use std::fmt;
use std::fs::{OpenOptions, create_dir_all, metadata};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::panic;

/// Enum personnalisé pour tous les types d'erreurs du module
#[derive(Debug, Clone)]
pub enum ModuleError {
    ConfigError(String),
    CaptureError(String),
    NetworkError(String),
    ThreadError(String),
    ValidationError(String),
    IoError(String),
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleError::ConfigError(msg) => write!(f, "ConfigError: {}", msg),
            ModuleError::CaptureError(msg) => write!(f, "CaptureError: {}", msg),
            ModuleError::NetworkError(msg) => write!(f, "NetworkError: {}", msg),
            ModuleError::ThreadError(msg) => write!(f, "ThreadError: {}", msg),
            ModuleError::ValidationError(msg) => write!(f, "ValidationError: {}", msg),
            ModuleError::IoError(msg) => write!(f, "IoError: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}

/// Structure d’une erreur capturée
#[derive(Debug, Clone)]
pub struct CaptureError {
    pub module: String,
    pub message: String,
    pub timestamp: u128,
}

impl CaptureError {
    pub fn new<S: Into<String>>(module: S, msg: S) -> Self {
        Self {
            module: module.into(),
            message: msg.into(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
        }
    }
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}][{}] {}", self.timestamp, self.module, self.message)
    }
}

/// Gestionnaire global d’erreurs
pub struct ErrorManager {
    errors: Arc<Mutex<Vec<CaptureError>>>,
    base_path: String,
}

impl ErrorManager {
    pub fn new(base_path: &str) -> Arc<Self> {
        let path = Path::new(base_path);
        if !path.exists() {
            let _ = create_dir_all(path);
        }

        let manager = Arc::new(Self {
            errors: Arc::new(Mutex::new(Vec::new())),
            base_path: base_path.to_string(),
        });

        // Hook global pour capturer tous les panics
        {
            let manager_clone = Arc::clone(&manager);
            panic::set_hook(Box::new(move |info| {
                let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = info.payload().downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };
                let location = info.location()
                    .map_or("unknown".to_string(), |l| format!("{}:{}", l.file(), l.line()));
                
                let msg_str = location + " - " + &msg;
                manager_clone.push_error(CaptureError::new("panic", &msg_str));
            }));
        }

        manager
    }

    /// Ajoute une erreur et l’écrit automatiquement dans le fichier module.log
    /// Implémente la rotation : si le fichier > 10MB, le renommer en .1, .2, etc.
    pub fn push_error(&self, error: CaptureError) {
        {
            let mut errors = self.errors.lock().unwrap();
            errors.push(error.clone());
        }

        let file_path = format!("{}/{}.log", self.base_path, error.module);
        
        // Vérifier taille fichier avant écriture
        if let Ok(meta) = metadata(&file_path) {
            if meta.len() > 10_485_760 {  // 10MB
                self.rotate_log_file(&file_path);
            }
        }
        
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&file_path) {
            let _ = writeln!(f, "{}", error);
        }
    }

    /// Rotation du fichier log : renommer en .1, .2, etc.
    fn rotate_log_file(&self, file_path: &str) {
        // Trouver le prochain numéro disponible
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

    pub fn get_errors(&self) -> Vec<CaptureError> {
        let errors = self.errors.lock().unwrap();
        errors.clone()
    }

    pub fn clear_errors(&self) {
        let mut errors = self.errors.lock().unwrap();
        errors.clear();
    }
}

// Instance globale ErrorManager pour gestion d'erreurs centralisée
lazy_static::lazy_static! {
    pub static ref ERROR_MANAGER: Arc<ErrorManager> = ErrorManager::new("logs/errors");
}

/// --- Macro automatique pour wrapper toutes les fonctions critiques ---
/// Cette macro transforme n'importe quelle fonction pour capturer automatiquement
/// les Result::Err et les transformer en logs, sans toucher au code du module.
#[macro_export]
macro_rules! auto_capture {
    ($func:item) => {
        {
            #[allow(non_snake_case)]
            fn wrapped() {
                let result = std::panic::catch_unwind(|| {
                    $func
                });

                if let Err(e) = result {
                    let msg = match e.downcast_ref::<String>() {
                        Some(s) => s.clone(),
                        None => match e.downcast_ref::<&str>() {
                            Some(s) => s.to_string(),
                            None => "Unknown error".to_string(),
                        },
                    };
                    $crate::capture::error::ERROR_MANAGER.push_error(
                        $crate::capture::error::CaptureError::new("auto", msg)
                    );
                }
            }
            wrapped();
        }
    };
}
