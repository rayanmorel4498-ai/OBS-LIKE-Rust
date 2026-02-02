// visualisation_module/src/ping.rs

const MODULE_NAME: &str = "ping";
const MODULE_ID: u8 = 6;
const MODULE_VERSION: &str = "1.0";

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::net::UdpSocket;

use crate::config::Config;
use crate::metrics::Metrics;

pub struct Ping {
    pool_ip: String,
    pool_port: u16,
    interval_idle: Duration,
    interval_active: Duration,
    pub running: Arc<Mutex<bool>>,
    last_pong: Arc<Mutex<Option<Instant>>>,
    metrics: Arc<Metrics>,
    timeout: Duration,
    failed_attempts: Arc<Mutex<u32>>,  // Track failures for exponential backoff
}

impl Ping {
    /// Crée le module Ping et récupère l'IP depuis config
    pub fn new(metrics: Arc<Metrics>) -> Self {
        let pool_ip = Config::get_pool_ip();
        let pool_port = Config::get_pool_port();
        let timeout = Duration::from_millis(5000);

        Self {
            pool_ip,
            pool_port,
            interval_idle: Duration::from_millis(1000),
            interval_active: Duration::from_millis(100),
            running: Arc::new(Mutex::new(false)),
            last_pong: Arc::new(Mutex::new(None)),
            metrics,
            timeout,
            failed_attempts: Arc::new(Mutex::new(0)),
        }
    }

    /// Démarre le ping H24 et met à jour métriques
    pub fn start(&self) {
        let running = Arc::clone(&self.running);
        *running.lock().unwrap() = true;
        eprintln!("[{}] v{} starting (id: {})", MODULE_NAME, MODULE_VERSION, MODULE_ID);

        let pool_ip = self.pool_ip.clone();
        let pool_port = self.pool_port;
        let interval_idle = self.interval_idle;
        let interval_active = self.interval_active;
        let last_pong = Arc::clone(&self.last_pong);
        let metrics = Arc::clone(&self.metrics);
        let timeout = self.timeout;
        let failed_attempts = Arc::clone(&self.failed_attempts);

        thread::spawn(move || {
            let mut last_ping = Instant::now();

            while *running.lock().unwrap() {
                let now = Instant::now();
                let pool_active = {
                    let pong = last_pong.lock().unwrap();
                    if let Some(pong_time) = *pong {
                        now.duration_since(pong_time) < timeout
                    } else {
                        false
                    }
                };

                // Exponential backoff: augmente délai à chaque échec
                let attempts = *failed_attempts.lock().unwrap();
                let backoff = if attempts > 0 {
                    // 100ms * 1.5^attempts, max 60s
                    let base = 100u64 * (3u64.pow(attempts.min(8)) / 2u64.pow(attempts.min(8)));
                    Duration::from_millis(base.min(60000))
                } else {
                    interval_active
                };

                let interval = if pool_active {
                    backoff
                } else {
                    interval_idle
                };

                // --- Ping UDP ---
                if now.duration_since(last_ping) >= interval {
                    let mut latency: Option<Duration> = None;
                    
                    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
                        let send_time = Instant::now();
                        socket.set_read_timeout(Some(Duration::from_millis(500))).ok();
                        
                        let addr = format!("{}:{}", pool_ip, pool_port);
                        if socket.send_to(b"PING", &addr).is_ok() {
                            let mut buf = [0u8; 4];
                            if let Ok((size, _)) = socket.recv_from(&mut buf) {
                                if size == 4 && &buf == b"PONG" {
                                    let mut pong_lock = last_pong.lock().unwrap();
                                    *pong_lock = Some(Instant::now());
                                    latency = Some(Instant::now().duration_since(send_time));
                                    
                                    // Reset failures on success
                                    *failed_attempts.lock().unwrap() = 0;
                                } else {
                                    // Increment failures
                                    let mut att = failed_attempts.lock().unwrap();
                                    *att = att.saturating_add(1);
                                }
                            } else {
                                // Increment failures on recv timeout
                                let mut att = failed_attempts.lock().unwrap();
                                *att = att.saturating_add(1);
                            }
                        } else {
                            // Increment failures on send error
                            let mut att = failed_attempts.lock().unwrap();
                            *att = att.saturating_add(1);
                        }
                    } else {
                        // Increment failures on socket error
                        let mut att = failed_attempts.lock().unwrap();
                        *att = att.saturating_add(1);
                    }

                    // --- Mise à jour métriques ---
                    if let Some(lat) = latency {
                        metrics.add_ping_latency(lat);
                    }

                    last_ping = now;
                }

                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn is_pool_active(&self) -> bool {
        let pong = self.last_pong.lock().unwrap();
        if let Some(pong_time) = *pong {
            let now = Instant::now();
            now.duration_since(pong_time) < self.timeout
        } else {
            false
        }
    }
}
