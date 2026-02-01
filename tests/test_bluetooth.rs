// visualisation_module/tests/test_bluetooth.rs

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use std::fs::File;
use std::io::Write;

use pcap::{Capture, Device};
use visualisation_module::capture::Ping;

/// Test Bluetooth réel OS-level (HCI) pendant 5 minutes
/// - Idle only (aucun pong)
/// - Capture tout le trafic Bluetooth réel
/// - Écrit dump .pcap pour analyse
pub fn test_bluetooth_idle_5min(ping: Arc<Ping>) {
    // Attendre idle total
    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(50));
    }

    // Choisir l’interface HCI (Linux/macOS)
    let device = Device::list()
        .unwrap()
        .into_iter()
        .find(|d| d.name.contains("hci"))
        .expect("Pas d'interface HCI trouvée");

    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)          // capture tout
        .immediate_mode(true)   // latence minimale
        .timeout(10)            // ms
        .open()
        .unwrap();

    // Dump pcap pour analyse
    let mut pcap_file = cap
        .savefile("logs/bluetooth_capture.pcap")
        .expect("Impossible de créer le fichier pcap");

    let start = Instant::now();
    let mut total_bytes = 0u64;
    let mut total_packets = 0u64;

    while start.elapsed() < Duration::from_secs(300) {
        if let Ok(packet) = cap.next_packet() {
            total_bytes += packet.data.len() as u64;
            total_packets += 1;

            // Écrire directement dans le fichier pcap
            let _ = pcap_file.write(&packet);
        }
    }

    // Assertions minimales
    assert!(total_bytes > 0, "Aucun trafic Bluetooth capturé");
    assert!(total_packets > 0, "Aucun paquet Bluetooth capturé");
}
