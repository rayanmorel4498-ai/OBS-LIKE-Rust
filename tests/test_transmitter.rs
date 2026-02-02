use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use visualisation_module::capture::{
    Transmitter, Ping,
    EthernetClient, BluetoothClient,
};

/// Test réel et complet du Transmitter en idle strict
/// Durée : 5 minutes
pub fn test_transmitter_5min(ping: Arc<Ping>) {
    // --- Idle strict obligatoire ---
    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(50));
    }

    // --- Clients réels ---
    let ethernet = Arc::new(EthernetClient::new());
    let bluetooth = Arc::new(BluetoothClient::new());

    let transmitter = Arc::new(Transmitter::new(
        Arc::clone(&ethernet),
        Arc::clone(&bluetooth),
    ));

    transmitter.start();

    let start = Instant::now();

    // Compteurs de validation
    let mut ethernet_ok = false;
    let mut bluetooth_ok = false;

    while start.elapsed() < Duration::from_secs(300) {
        // --- Screen ---
        transmitter.push_screen(vec![0xAA; 1024]);

        // --- Audio ---
        transmitter.push_audio(vec![0xBB; 512]);

        // --- Input ---
        transmitter.push_input(vec![0xCC; 64]);

        // --- Ethernet pur ---
        transmitter.push_ethernet(vec![0xDD; 256]);

        // --- Bluetooth pur ---
        transmitter.push_bluetooth(vec![0xEE; 128]);

        // --- Réception réelle ---
        if let Some(data) = ethernet.receive_data() {
            assert!(!data.is_empty());
            ethernet_ok = true;
        }

        if let Some(data) = bluetooth.receive_data() {
            assert!(!data.is_empty());
            bluetooth_ok = true;
        }

        // Idle-friendly
        thread::sleep(Duration::from_millis(10));
    }

    transmitter.stop();

    // --- Validation finale ---
    assert!(ethernet_ok, "Aucune donnée Ethernet reçue");
    assert!(bluetooth_ok, "Aucune donnée Bluetooth reçue");
}
