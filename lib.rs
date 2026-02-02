// visualisation_module/src/lib.rs

//! Crate `visualisation_module`
//! Fournit la capture multi-entrée (écran, audio, input, ethernet, bluetooth)
//! avec ping/pool, prétraitement et transmission H24

pub mod capture;
pub mod config;
pub mod logging;
pub mod metrics;
pub mod error;
pub mod state;
pub mod ping;
pub mod transmitter;
pub mod utils;

// Réexports public pour les consommateurs externes
pub use capture::{
    ScreenCapture, AudioCapture, InputCapture, 
    Preprocessor,
    EthernetClient, BluetoothClient
};

pub use config::Config;
pub use logging::{LOGGER, LoggingManager, LogEntry};
pub use metrics::{Metrics, MetricsSummary, ModuleType};
pub use error::ErrorManager;
pub use ping::Ping;
pub use transmitter::{Transmitter, Packet, PacketType};
pub use state::StateManager;

