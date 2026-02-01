// visualisation_module/src/capture/ethernet.rs

const MODULE_NAME: &str = "ethernet";
const MODULE_ID: u8 = 4;
const MODULE_VERSION: &str = "1.0";

use std::net::{UdpSocket, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam::queue::SegQueue;

pub struct EthernetClient {
    inner: Arc<EthernetInner>,
}

struct EthernetInner {
    socket: UdpSocket,
    pool_addr: SocketAddr,
    running: Mutex<bool>,
    pool_active: Mutex<bool>,
    send_queue: SegQueue<Vec<u8>>,
    ping_interval_idle: Duration,
    ping_interval_active: Duration,
    last_ping: Mutex<Instant>,
    stats: Mutex<EthernetStats>,
}

pub struct EthernetStats {
    pub last_latency_ms: u128,
    pub frames_sent: usize,
    pub errors: usize,
}

impl Clone for EthernetStats {
    fn clone(&self) -> Self {
        EthernetStats {
            last_latency_ms: self.last_latency_ms,
            frames_sent: self.frames_sent,
            errors: self.errors,
        }
    }
}

impl EthernetClient {
    pub fn new() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap_or_else(|_| {
            UdpSocket::bind("127.0.0.1:0").expect("Impossible de bind le socket")
        });
        socket.set_nonblocking(true).unwrap();
        
        let pool_addr: SocketAddr = format!("{}:{}", crate::config::Config::get_pool_ip(), crate::config::Config::get_pool_port()).parse().unwrap_or_else(|_| {
            "127.0.0.1:5000".parse().unwrap()
        });

        let inner = EthernetInner {
            socket,
            pool_addr,
            running: Mutex::new(false),
            pool_active: Mutex::new(false),
            send_queue: SegQueue::new(),
            ping_interval_idle: Duration::from_secs(1),
            ping_interval_active: Duration::from_millis(100),
            last_ping: Mutex::new(Instant::now()),
            stats: Mutex::new(EthernetStats {
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
        eprintln!("[{}] v{} starting (id: {})", MODULE_NAME, MODULE_VERSION, MODULE_ID);

        thread::spawn(move || {
            while *inner.running.lock().unwrap() {
                let now = Instant::now();

                // Gestion ping adaptatif
                let active = *inner.pool_active.lock().unwrap();
                let interval = if active {
                    inner.ping_interval_active
                } else {
                    inner.ping_interval_idle
                };

                if now.duration_since(*inner.last_ping.lock().unwrap()) >= interval {
                    if inner.socket.send_to(b"PING", inner.pool_addr).is_ok() {
                        *inner.last_ping.lock().unwrap() = now;
                    } else {
                        let mut stats = inner.stats.lock().unwrap();
                        stats.errors += 1;
                    }
                }

                // Réception pong pour détecter pool active
                let mut buf = [0u8; 64];
                if let Ok((size, _)) = inner.socket.recv_from(&mut buf) {
                    if &buf[..size] == b"PONG" {
                        *inner.pool_active.lock().unwrap() = true;
                        let mut stats = inner.stats.lock().unwrap();
                        stats.last_latency_ms = now.elapsed().as_millis();
                    }
                }

                // Envoi des données
                while let Some(data) = inner.send_queue.pop() {
                    if inner.socket.send_to(&data, inner.pool_addr).is_ok() {
                        let mut stats = inner.stats.lock().unwrap();
                        stats.frames_sent += 1;
                    } else {
                        let mut stats = inner.stats.lock().unwrap();
                        stats.errors += 1;
                        // Si envoi échoue, réinsérer dans la queue
                        inner.send_queue.push(data);
                    }
                }

                thread::sleep(Duration::from_millis(5));
            }
        });
    }

    pub fn stop(&self) {
        *self.inner.running.lock().unwrap() = false;
    }

    pub fn send_data(&self, data: Vec<u8>) {
        self.inner.send_queue.push(data);
    }

    pub fn is_pool_active(&self) -> bool {
        *self.inner.pool_active.lock().unwrap()
    }

    pub fn get_stats(&self) -> EthernetStats {
        self.inner.stats.lock().unwrap().clone()
    }
}

