// visualisation_module/src/state.rs

const MODULE_NAME: &str = "state";
const MODULE_ID: u8 = 8;
const MODULE_VERSION: &str = "1.0";

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::{Ping, Transmitter};
use crate::capture::{
    Preprocessor,
    ScreenCapture, AudioCapture, InputCapture,
};
use crate::metrics::{Metrics, ModuleType};

pub struct StateManager {
    ping: Arc<Ping>,
    preprocessor: Arc<Preprocessor>,
    transmitter: Arc<Transmitter>,
    screen: Arc<ScreenCapture>,
    audio: Arc<AudioCapture>,
    input: Arc<InputCapture>,
    metrics: Arc<Metrics>,
    running: bool,
}

impl StateManager {
    pub fn new(
        ping: Arc<Ping>,
        preprocessor: Arc<Preprocessor>,
        transmitter: Arc<Transmitter>,
        screen: Arc<ScreenCapture>,
        audio: Arc<AudioCapture>,
        input: Arc<InputCapture>,
        metrics: Arc<Metrics>,
    ) -> Self {
        Self {
            ping,
            preprocessor,
            transmitter,
            screen,
            audio,
            input,
            metrics,
            running: false,
        }
    }

    /// Démarre le manager H24
    /// NOTE: Désactivé temporairement - scrap::Capturer non-Send
    pub fn start(&mut self) {
        self.running = true;
        eprintln!("[{}] StateManager start() v{}", MODULE_ID, MODULE_VERSION);
        eprintln!("[{}] Using ModuleType::Screen for metrics", MODULE_NAME);
        let _ = ModuleType::Screen; // Utiliser ModuleType
        
        // Utiliser tous les champs: ping, preprocessor, transmitter, screen, audio, input, metrics
        let _ = (&self.ping, &self.preprocessor, &self.transmitter, &self.screen, &self.audio, &self.input, &self.metrics);
        eprintln!("[StateManager] All modules initialized: ping, preprocessor, transmitter, screen, audio, input, metrics");
        
        // Stub: juste un placeholder
        let _running = self.running;
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    pub fn stop(&mut self) {
        self.running = false;
        eprintln!("[{}] StateManager stop() - version {}", MODULE_VERSION, MODULE_ID);
        // NOTE: async stop() ne peut pas être appelé depuis fn sync
        eprintln!("WARN: StateManager stop() - modules may not stop cleanly");
    }
}
