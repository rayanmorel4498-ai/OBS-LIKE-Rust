// visualisation_module/src/capture/audio.rs

const MODULE_NAME: &str = "audio";
const MODULE_ID: u8 = 2;
const MODULE_VERSION: &str = "1.0";

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use std::time::{Duration, Instant};
use crossbeam::queue::SegQueue;
use cpal::{SampleFormat, Stream};

use crate::config::Config;
use crate::metrics::{Metrics, ModuleType};

pub struct AudioCapture {
    sample_rate: u32,
    buffer_size: usize,
    inner: Arc<AudioInner>,
    metrics: Option<Arc<Metrics>>,
}

struct AudioInner {
    input_stream: Mutex<Option<Stream>>,
    output_stream: Mutex<Option<Stream>>,
    running: Mutex<bool>,
    frame_buffer: SegQueue<Vec<f32>>,
    thread_handle: Mutex<Option<JoinHandle<()>>>,
}

impl AudioCapture {
    pub fn new() -> Self {
        let resources = Config::get_available_resources();

        let sample_rate = if resources.cpu >= 4 { 48000 } else { 44100 };
        let buffer_size = if resources.ram >= 8 { 2048 } else { 1024 };

        let inner = AudioInner {
            input_stream: Mutex::new(None),
            output_stream: Mutex::new(None),
            running: Mutex::new(false),
            frame_buffer: SegQueue::new(),
            thread_handle: Mutex::new(None),
        };

        Self {
            sample_rate,
            buffer_size,
            inner: Arc::new(inner),
            metrics: None,
        }
    }

    pub fn attach_metrics(&mut self, metrics: Arc<Metrics>) {
        self.metrics = Some(metrics);
    }

    /// Démarre la capture audio (entrée + sortie) dans des threads séparés
    /// Appelé uniquement par main.rs
    pub async fn start(&self) {
        use cpal::traits::{DeviceTrait, HostTrait};
        
        let inner = Arc::clone(&self.inner);
        *inner.running.lock().await = true;

        let host = cpal::default_host();
        let device = host.default_input_device();

        eprintln!("[{}:{}] Audio capture starting (sample_rate: {}, buffer: {})", 
                  MODULE_NAME, MODULE_ID, self.sample_rate, self.buffer_size);

        let buffer_size = self.buffer_size;
        let sample_rate = self.sample_rate;
        let metrics = self.metrics.clone();

        let handle = tokio::spawn(async move {
            if let Some(dev) = device {
                if let Ok(desc) = dev.description() {
                    eprintln!("[{}] Using device: {:?}", MODULE_NAME, desc);
                }
            }

            let expected_fps = (sample_rate / buffer_size as u32).max(1);
            eprintln!("[{}] Expected audio FPS: {}", MODULE_NAME, expected_fps);
            let mut last_fps_update = Instant::now();
            let mut frame_count = 0;
            
            while *inner.running.lock().await {
                // Simuler la lecture de frames audio avec le sample_rate et buffer_size
                let frame = vec![0.0f32; buffer_size];
                inner.frame_buffer.push(frame);
                frame_count += 1;
                
                // Mettre à jour les FPS toutes les 1 secondes
                if last_fps_update.elapsed() >= Duration::from_secs(1) {
                    if let Some(m) = &metrics {
                        m.update_fps(ModuleType::Audio, frame_count);
                    }
                    frame_count = 0;
                    last_fps_update = Instant::now();
                }
                
                let sleep_duration = Duration::from_millis(
                    (buffer_size as u64 * 1000) / sample_rate as u64
                );
                tokio::time::sleep(sleep_duration).await;
            }
            eprintln!("[{}] Audio capture stopped", MODULE_NAME);
        });

        *self.inner.thread_handle.lock().await = Some(handle);
    }

    /// Stoppe la capture audio
    pub async fn stop(&self) {
        let mut running = self.inner.running.lock().await;
        *running = false;
        drop(running);

        if let Some(stream) = self.inner.input_stream.lock().await.take() {
            drop(stream);
        }
        if let Some(stream) = self.inner.output_stream.lock().await.take() {
            drop(stream);
        }

        // Attendre que le thread se termine proprement
        if let Some(h) = self.inner.thread_handle.lock().await.take() {
            let _ = h.await;
        }
    }

    /// Récupère la dernière frame audio
    pub fn get_frame(&self) -> Option<Vec<f32>> {
        self.inner.frame_buffer.pop()
    }

    pub fn get_current_fps(&self) -> u32 {
        (self.sample_rate / self.buffer_size as u32).max(1)
    }

    /// Vide le buffer pour libérer de la mémoire
    pub fn clear_buffer(&self) {
        while self.inner.frame_buffer.pop().is_some() {}
    }

    /// Affiche les infos sur les devices disponibles (utilise DeviceTrait, HostTrait)
    pub fn list_available_devices(&self) {
        use cpal::traits::{DeviceTrait, HostTrait};
        
        let host = cpal::default_host();
        
        eprintln!("[{}] Available input devices:", MODULE_NAME);
        if let Ok(devices) = host.input_devices() {
            for device in devices {
                if let Ok(desc) = device.description() {
                    eprintln!("  - {:?}", desc);
                }
            }
        }
        
        eprintln!("[{}] Available output devices:", MODULE_NAME);
        if let Ok(devices) = host.output_devices() {
            for device in devices {
                if let Ok(desc) = device.description() {
                    eprintln!("  - {:?}", desc);
                }
            }
        }
    }

    /// Retourne le format d'échantillon (utilise SampleFormat, StreamTrait)
    pub fn get_sample_format(&self) -> &'static str {
        use cpal::traits::{DeviceTrait, HostTrait};
        
        let host = cpal::default_host();
        let sample_fmt = SampleFormat::F32;
        
        if let Some(device) = host.default_input_device() {
            if let Ok(desc) = device.description() {
                eprintln!("[{}] Device: {:?}", MODULE_NAME, desc);
            }
        }
        
        eprintln!("[{}] Using format: {:?}", MODULE_VERSION, sample_fmt);
        "F32"
    }
}
