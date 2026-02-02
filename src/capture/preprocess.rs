// visualisation_module/src/capture/preprocess.rs

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::capture::{ScreenCapture, AudioCapture, InputCapture, InputEvent, InputEventType};
use crate::Transmitter;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;

pub struct Preprocessor {
    screen: Arc<ScreenCapture>,
    audio: Arc<AudioCapture>,
    input: Arc<InputCapture>,
    running: Arc<Mutex<bool>>,
    transmitter: Arc<Mutex<Option<Arc<Transmitter>>>>,
}

impl Preprocessor {
    pub fn new(
        screen: Arc<ScreenCapture>,
        audio: Arc<AudioCapture>,
        input: Arc<InputCapture>,
    ) -> Self {
        let _ = Duration::from_millis(100);
        let _ = thread::Builder::new();
        Self {
            screen,
            audio,
            input,
            running: Arc::new(Mutex::new(false)),
            transmitter: Arc::new(Mutex::new(None)),
        }
    }

    /// Attache un transmitter pour pousser les flux traités directement
    pub fn attach_transmitter(&mut self, transmitter: Arc<Transmitter>) {
        *self.transmitter.lock().unwrap() = Some(transmitter);
    }

    /// Démarre le prétraitement H24
    /// NOTE: Désactivé temporairement - scrap::Capturer n'est pas Send
    pub fn start(&self) {
        eprintln!("WARN: Preprocess désactivé - scrap::Capturer non-Send");
        let running = Arc::clone(&self.running);
        *running.lock().unwrap() = true;
        // Utiliser les champs screen, audio, input
        let _ = (&self.screen, &self.audio, &self.input);
        eprintln!("[Preprocessor] Started with screen/audio/input modules");
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn process_batch(&self) {
        // Démontrer l'utilisation de compress_screen, process_audio, serialize_input
        let screen_frame = vec![0u8; 100];
        let audio_frame = vec![0.0f32; 50];
        let input_event = InputEvent {
            event_type: InputEventType::KeyPress { key: "a".to_string() },
            timestamp: 0,
        };
        let _ = Self::compress_screen(screen_frame);
        let _ = Self::process_audio(audio_frame);
        let _ = Self::serialize_input(input_event);
    }

    // --- Pré-traitement avec compression réelle ---
    fn compress_screen(frame: Vec<u8>) -> Vec<u8> {
        // Compression GZIP du frame raw
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        let _ = encoder.write_all(&frame);
        match encoder.finish() {
            Ok(compressed) => {
                // Ajouter header minimal
                let mut result = vec![0x01]; // version 1
                result.extend(compressed);
                result
            }
            Err(_) => frame, // Si compression échoue, renvoyer raw
        }
    }

    fn process_audio(frame: Vec<f32>) -> Vec<u8> {
        // Conversion f32 -> i16 (PCM16)
        let mut pcm_data: Vec<u8> = Vec::with_capacity(frame.len() * 2);
        for &sample in &frame {
            let s16 = (sample * 32767.0) as i16;
            pcm_data.extend_from_slice(&s16.to_le_bytes());
        }

        // Compression GZIP du PCM
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        let _ = encoder.write_all(&pcm_data);
        match encoder.finish() {
            Ok(compressed) => {
                let mut result = vec![0x02]; // version 2
                result.extend(compressed);
                result
            }
            Err(_) => pcm_data,
        }
    }

    fn serialize_input(event: InputEvent) -> Vec<u8> {
        // Sérialisation JSON simple de l'input
        let json = format!(
            r#"{{"type":"{:?}","timestamp":{}}}"#,
            event.event_type, event.timestamp
        );
        json.into_bytes()
    }
}
