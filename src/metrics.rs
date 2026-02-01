// visualisation_module/src/utils/metrics.rs

use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use std::time::{Instant, Duration};
use sysinfo::System;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum ModuleType {
    Screen,
    Audio,
    Input,
    Preprocessor,
    Transmitter,
}

pub struct Metrics {
    system: Mutex<System>,

    // FPS par module avec historique
    fps_history: Mutex<HashMap<ModuleType, VecDeque<(Instant, u32)>>>,
    fps_max_history: usize, // nombre de valeurs historiques conservées

    // Paquets envoyés
    packets_sent: Mutex<HashMap<ModuleType, u64>>,

    // Latence ping
    ping_latency_history: Mutex<VecDeque<(Instant, Duration)>>,
    ping_max_history: usize,

    // CPU / RAM historique
    cpu_history: Mutex<VecDeque<(Instant, f32)>>,
    ram_history: Mutex<VecDeque<(Instant, u64)>>,
    sys_history_max: usize,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            system: Mutex::new(System::new_all()),
            fps_history: Mutex::new(HashMap::new()),
            fps_max_history: 100,
            packets_sent: Mutex::new(HashMap::new()),
            ping_latency_history: Mutex::new(VecDeque::with_capacity(100)),
            ping_max_history: 100,
            cpu_history: Mutex::new(VecDeque::with_capacity(200)),
            ram_history: Mutex::new(VecDeque::with_capacity(200)),
            sys_history_max: 200,
        })
    }

    /// Met à jour CPU et RAM et conserve un historique
    pub fn update_system_metrics(&self) {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_all();
        let now = Instant::now();
        let cpu = sys.global_cpu_info().cpu_usage();
        let ram = sys.used_memory() / 1024; // en MB

        let mut cpu_hist = self.cpu_history.lock().unwrap();
        cpu_hist.push_back((now, cpu));
        if cpu_hist.len() > self.sys_history_max {
            cpu_hist.pop_front();
        }

        let mut ram_hist = self.ram_history.lock().unwrap();
        ram_hist.push_back((now, ram));
        if ram_hist.len() > self.sys_history_max {
            ram_hist.pop_front();
        }
    }

    /// Retourne CPU / RAM moyens sur l’historique
    pub fn avg_cpu(&self) -> f32 {
        let hist = self.cpu_history.lock().unwrap();
        if hist.is_empty() { return 0.0; }
        hist.iter().map(|(_, v)| *v).sum::<f32>() / hist.len() as f32
    }

    pub fn avg_ram(&self) -> u64 {
        let hist = self.ram_history.lock().unwrap();
        if hist.is_empty() { return 0; }
        hist.iter().map(|(_, v)| *v).sum::<u64>() / hist.len() as u64
    }

    /// Met à jour le FPS d’un module
    pub fn update_fps(&self, module: ModuleType, fps: u32) {
        let now = Instant::now();
        let mut fps_hist = self.fps_history.lock().unwrap();
        let entry = fps_hist.entry(module).or_insert_with(|| VecDeque::with_capacity(self.fps_max_history));
        entry.push_back((now, fps));
        if entry.len() > self.fps_max_history {
            entry.pop_front();
        }
    }

    /// Moyenne FPS sur l’historique
    pub fn avg_fps(&self, module: ModuleType) -> u32 {
        let fps_hist = self.fps_history.lock().unwrap();
        if let Some(hist) = fps_hist.get(&module) {
            if hist.is_empty() { return 0; }
            hist.iter().map(|(_, v)| *v).sum::<u32>() / hist.len() as u32
        } else {
            0
        }
    }

    /// Ajoute des paquets envoyés
    pub fn add_packets(&self, module: ModuleType, count: u64) {
        let mut packets = self.packets_sent.lock().unwrap();
        let entry = packets.entry(module).or_insert(0);
        *entry += count;
    }

    pub fn get_packets(&self, module: ModuleType) -> u64 {
        let packets = self.packets_sent.lock().unwrap();
        *packets.get(&module).unwrap_or(&0)
    }

    /// Met à jour la latence du ping
    pub fn add_ping_latency(&self, latency: Duration) {
        let now = Instant::now();
        let mut hist = self.ping_latency_history.lock().unwrap();
        hist.push_back((now, latency));
        if hist.len() > self.ping_max_history {
            hist.pop_front();
        }
    }

    pub fn avg_ping_latency(&self) -> Option<Duration> {
        let hist = self.ping_latency_history.lock().unwrap();
        if hist.is_empty() { return None; }
        let sum: Duration = hist.iter().map(|(_, d)| *d).sum();
        Some(sum / (hist.len() as u32))
    }

    pub fn get_summary(&self) -> MetricsSummary {
        MetricsSummary {
            avg_cpu: self.avg_cpu(),
            avg_ram_mb: self.avg_ram(),
            avg_fps_screen: self.avg_fps(ModuleType::Screen),
            avg_fps_audio: self.avg_fps(ModuleType::Audio),
            avg_fps_input: self.avg_fps(ModuleType::Input),
            packets_screen: self.get_packets(ModuleType::Screen),
            packets_audio: self.get_packets(ModuleType::Audio),
            packets_input: self.get_packets(ModuleType::Input),
            avg_ping_ms: self.avg_ping_latency().map(|d| d.as_millis() as u64),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub avg_cpu: f32,
    pub avg_ram_mb: u64,
    pub avg_fps_screen: u32,
    pub avg_fps_audio: u32,
    pub avg_fps_input: u32,
    pub packets_screen: u64,
    pub packets_audio: u64,
    pub packets_input: u64,
    pub avg_ping_ms: Option<u64>,
}
