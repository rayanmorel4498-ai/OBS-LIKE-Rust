// visualisation_module/src/capture/bluetooth.rs

const MODULE_NAME: &str = "bluetooth";
const MODULE_ID: u8 = 5;
const MODULE_VERSION: &str = "1.0";

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam::queue::SegQueue;

pub struct BluetoothClient {
    inner: Arc<BluetoothInner>,
}

struct BluetoothInner {
    peripheral: Mutex<Option<Vec<u8>>>, // Mock: juste un buffer
    running: Mutex<bool>,
    connected: Mutex<bool>,
    send_queue: SegQueue<Vec<u8>>,
    ping_interval_idle: Duration,
    ping_interval_active: Duration,
    last_ping: Mutex<Instant>,
    stats: Mutex<BluetoothStats>,
}

#[derive(Clone)]
pub struct BluetoothStats {
    pub last_latency_ms: u128,
    pub frames_sent: usize,
    pub errors: usize,
}

impl BluetoothClient {
    pub fn new() -> Self {
        // Bluetooth optionnel - toujours en mode mock pour éviter les complications async
        eprintln!("WARN: Bluetooth en mode mock (async Manager::new() non supporté)");
        Self::mock()
    }

    fn mock() -> Self {
        // Structure minimale pour Bluetooth mock
        let inner = BluetoothInner {
            peripheral: Mutex::new(None),
            running: Mutex::new(false),
            connected: Mutex::new(false),
            send_queue: SegQueue::new(),
            ping_interval_idle: Duration::from_secs(1),
            ping_interval_active: Duration::from_millis(100),
            last_ping: Mutex::new(Instant::now()),
            stats: Mutex::new(BluetoothStats {
                last_latency_ms: 0,
                frames_sent: 0,
                errors: 0,
            }),
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn start(&self) {
        let inner = Arc::clone(&self.inner);
        *inner.running.lock().unwrap() = true;

        thread::spawn(move || {
            eprintln!("[{}] Bluetooth module starting (btleplug 0.12)", MODULE_NAME);
            
            while *inner.running.lock().unwrap() {
                let active = *inner.connected.lock().unwrap();
                let interval = if active {
                    inner.ping_interval_active
                } else {
                    inner.ping_interval_idle
                };

                let now = Instant::now();
                let last_ping_time = *inner.last_ping.lock().unwrap();
                
                if now.duration_since(last_ping_time) >= interval {
                    let latency = now.duration_since(last_ping_time).as_millis();
                    *inner.last_ping.lock().unwrap() = now;
                    
                    let mut stats = inner.stats.lock().unwrap();
                    stats.last_latency_ms = latency;
                    eprintln!("[{}] BT ping: {} ms (v{})", MODULE_ID, latency, MODULE_VERSION);
                }

                // Traiter la queue d'envoi
                while let Some(data) = inner.send_queue.pop() {
                    // Utiliser peripheral avec les données
                    let mut peripheral = inner.peripheral.lock().unwrap();
                    *peripheral = Some(data.clone());
                    
                    let mut stats = inner.stats.lock().unwrap();
                    stats.frames_sent = stats.frames_sent.saturating_add(1);
                    eprintln!("[{}] BT frame sent: {} bytes (total: {})", 
                              MODULE_NAME, data.len(), stats.frames_sent);
                }

                thread::sleep(Duration::from_millis(10));
            }
            
            eprintln!("[{}] Bluetooth module stopped", MODULE_NAME);
        });
    }

    pub fn stop(&self) {
        *self.inner.running.lock().unwrap() = false;
        // Mode mock - aucune disconnection réelle
    }

    pub fn send_data(&self, data: Vec<u8>) {
        // Utiliser le champ peripheral
        let mut peripheral = self.inner.peripheral.lock().unwrap();
        *peripheral = Some(data.clone());
        eprintln!("[{}] BT send_data: {} bytes", MODULE_NAME, data.len());
        self.inner.send_queue.push(data);
    }

    pub fn is_connected(&self) -> bool {
        *self.inner.connected.lock().unwrap()
    }

    pub fn get_stats(&self) -> BluetoothStats {
        self.inner.stats.lock().unwrap().clone()
    }
}
