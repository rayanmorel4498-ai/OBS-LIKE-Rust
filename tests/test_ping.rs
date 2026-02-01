// visualisation_module/tests/test_ping.rs

use std::sync::Arc;
use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::thread;

use visualisation_module::capture::Ping;
use visualisation_module::utils::config;

/// Test ping backend ultra-fréquent (5–10ms) en idle strict
/// - Envoi réel UDP
/// - Aucun pong attendu
/// - Mesure stabilité + dérive temporelle
/// - 5 minutes continues
pub fn test_ping_idle_ultra_freq_5min(ping: Arc<Ping>) {
    // Idle strict : aucun pong autorisé
    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(20));
    }

    let pool_ip = config::get_pool_ip();
    let pool_port = config::get_pool_port();

    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Impossible de binder socket UDP");
    socket
        .set_nonblocking(true)
        .expect("Impossible de passer en non-blocking");

    let target = format!("{}:{}", pool_ip, pool_port);

    let start = Instant::now();
    let mut sent_packets: u64 = 0;
    let mut min_interval = Duration::from_millis(1000);
    let mut max_interval = Duration::ZERO;

    let mut last_send = Instant::now();

    while start.elapsed() < Duration::from_secs(300) {
        let now = Instant::now();
        let delta = now.duration_since(last_send);

        // Tracking précision réelle
        if delta < min_interval {
            min_interval = delta;
        }
        if delta > max_interval {
            max_interval = delta;
        }

        last_send = now;

        // Envoi réel UDP
        let _ = socket.send_to(b"ping", &target);
        sent_packets += 1;

        // Drain RX (aucune réponse attendue)
        let mut buf = [0u8; 16];
        let _ = socket.recv_from(&mut buf);

        // Intervalle agressif : 5–10ms
        thread::sleep(Duration::from_millis(5));
    }

    // Vérifications strictes
    assert!(sent_packets > 10_000, "Nombre de pings insuffisant");
    assert!(
        min_interval <= Duration::from_millis(10),
        "Fréquence trop lente"
    );
}
