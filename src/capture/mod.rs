// visualisation_module/src/capture/mod.rs
//! Module `capture`
//! Centralisation de tous les modules de capture et pré-traitement

pub mod screen;
pub mod audio;
pub mod input;
pub mod ethernet;
pub mod bluetooth;
pub mod preprocess;

// Réexport des structures principales pour usage externe
pub use screen::ScreenCapture;
pub use audio::AudioCapture;
pub use input::{InputCapture, InputEvent, InputEventType};
pub use ethernet::EthernetClient;
pub use bluetooth::BluetoothClient;
pub use preprocess::Preprocessor;
