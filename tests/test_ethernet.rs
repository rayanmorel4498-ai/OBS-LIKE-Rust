use std::sync::Arc;
use std::time::{Duration, Instant};

use pcap::{Capture, Device};
use visualisation_module::capture::Ping;

/// Test réel Ethernet : capture trafic entrant + sortant du PC
/// Fonctionne UNIQUEMENT en idle (aucun pong backend)
pub fn test_ethernet_idle_5min(ping: Arc<Ping>) {
    // Attendre idle total (AUCUN pong)
    while ping.is_pool_active() {
        std::thread::sleep(Duration::from_millis(10));
    }

    // Sélection interface réseau principale
    let device = Device::lookup().expect("Aucune interface réseau trouvée");

    // Capture live (trafic réel du PC)
    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)          // tout le trafic
        .immediate_mode(true)   // latence minimale
        .timeout(10)            // ms
        .open()
        .unwrap();

    let start = Instant::now();

    let mut packets_in = 0u64;
    let mut packets_out = 0u64;
    let mut bytes_total = 0u64;

    while start.elapsed() < Duration::from_secs(300) {
        if let Ok(packet) = cap.next_packet() {
            bytes_total += packet.data.len() as u64;

            // Analyse Ethernet basique
            if packet.data.len() >= 14 {
                let ethertype = u16::from_be_bytes([packet.data[12], packet.data[13]]);

                match ethertype {
                    0x0800 | 0x86DD => {
                        // IPv4 / IPv6
                        packets_in += 1;
                    }
                    _ => {
                        packets_out += 1;
                    }
                }
            }
        }
    }

    // Assertions minimales (activité réseau réelle obligatoire)
    assert!(bytes_total > 0, "Aucun trafic réseau capturé");
    assert!(packets_in + packets_out > 0, "Aucun paquet capturé");
}
