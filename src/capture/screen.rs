// visualisation_module/src/capture/screen.rs

const MODULE_NAME: &str = "screen";
const MODULE_ID: u8 = 1;
const MODULE_VERSION: &str = "1.0";

use scrap::{Capturer, Display};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crossbeam::queue::SegQueue;

use crate::config;
use crate::metrics::{Metrics, ModuleType};

pub struct ScreenCapture {
    fps: u32,
    inner: Arc<ScreenInner>,
    metrics: Option<Arc<Metrics>>,
}

struct ScreenInner {
    running: Mutex<bool>,
    frame_buffer: SegQueue<Vec<u8>>,
    frames_captured: Mutex<u32>,
}

impl ScreenCapture {
    pub fn new() -> Self {
        let resources = config::Config::get_available_resources();
        let fps = if resources.cpu >= 4 { 60 } else { 30 };

        let mut capturers = Vec::new();
        for display in Display::all().unwrap_or_else(|_| vec![Display::primary().unwrap()]) {
            if let Ok(c) = Capturer::new(display) {
                capturers.push(c);
            }
        }

        let inner = ScreenInner {
            running: Mutex::new(false),
            frame_buffer: SegQueue::new(),
            frames_captured: Mutex::new(0),
        };

        Self {
            fps,
            inner: Arc::new(inner),
            metrics: None,
        }
    }

    pub fn attach_metrics(&mut self, metrics: Arc<Metrics>) {
        self.metrics = Some(metrics);
    }

    pub async fn start(&self) {
        let inner = Arc::clone(&self.inner);
        let _fps = self.fps;
        let fps_duration = Duration::from_millis(1000 / self.fps as u64);
        let metrics = self.metrics.clone();

        *inner.running.lock().unwrap() = true;

        // WARN: scrap::Capturer n'est pas Send et ne peut pas être partagé entre threads
        // Solution: créer les capturers DANS le thread, pas avant
        let _handle = std::thread::spawn(move || {
            // Créer les capturers ICI, dans le thread, pour éviter les problèmes Send/Sync
            let mut capturers = Vec::new();
            for display in Display::all().unwrap_or_else(|_| vec![Display::primary().unwrap()]) {
                if let Ok(c) = Capturer::new(display) {
                    capturers.push(c);
                }
            }

            eprintln!("[{}] Screen capture thread started (v{})", MODULE_NAME, MODULE_VERSION);

            let mut frame_count = 0;
            let mut last_fps_update = Instant::now();

            while *inner.running.lock().unwrap() {
                let loop_start = Instant::now();

                for capturer in capturers.iter_mut() {
                    match capturer.frame() {
                        Ok(frame) => {
                            inner.frame_buffer.push(frame.to_vec());
                            let mut count = inner.frames_captured.lock().unwrap();
                            *count = count.saturating_add(1);
                            frame_count += 1;
                            eprintln!("[{}] Captured frame #{} ({} bytes)", MODULE_ID, count, frame.len());
                        }
                        Err(e) => {
                            eprintln!("[{}] Capture error: {}", MODULE_NAME, e);
                        }
                    }
                }

                // Mettre à jour les FPS toutes les 1 secondes
                if last_fps_update.elapsed() >= Duration::from_secs(1) {
                    if let Some(m) = &metrics {
                        m.update_fps(ModuleType::Screen, frame_count);
                    }
                    frame_count = 0;
                    last_fps_update = Instant::now();
                }

                let elapsed = loop_start.elapsed();
                if elapsed < fps_duration {
                    std::thread::sleep(fps_duration - elapsed);
                }
            }
            eprintln!("[{}] Screen capture thread stopped", MODULE_ID);
        });
    }

    pub async fn stop(&self) {
        *self.inner.running.lock().unwrap() = false;
        // Attendre un peu pour que le thread se termine
        std::thread::sleep(Duration::from_millis(100));
    }

    pub fn get_frame(&self) -> Option<Vec<u8>> {
        self.inner.frame_buffer.pop()
    }

    pub fn get_current_fps(&self) -> u32 {
        self.fps
    }

    pub fn clear_buffer(&self) {
        while self.inner.frame_buffer.pop().is_some() {}
    }
}
