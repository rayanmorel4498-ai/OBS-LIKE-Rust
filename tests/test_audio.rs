// visualisation_module/tests/test_audio.rs

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use visualisation_module::capture::{AudioCapture, Ping, Transmitter};

/// Test audio réel système pendant 5 minutes
/// - Idle strict (aucun pong)
/// - Capture toutes les sorties audio (entrée/sortie si possible)
/// - Pousse les buffers vers le transmitter
pub fn test_audio_idle_5min(
    ping: Arc<Ping>,
    audio: Arc<AudioCapture>,
    transmitter: Arc<Transmitter>,
) {
    // Attente idle strict
    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(50));
    }

    // Démarrage du module audio
    audio.start();

    let start = Instant::now();
    let mut buffer_count: u64 = 0;

    while start.elapsed() < Duration::from_secs(300) {
        if let Some(buffer) = audio.get_frame() {
            buffer_count += 1;

            // Pousser le buffer réel vers le transmitter
            transmitter.push_audio(buffer);

            // Légère pause pour réguler CPU si nécessaire
            thread::sleep(Duration::from_millis(5));
        } else {
            // Aucun buffer capturé : wait léger
            thread::sleep(Duration::from_millis(5));
        }
    }

    audio.stop();

    // Vérifications strictes
    assert!(buffer_count > 1000, "Buffers audio insuffisants pendant 5 minutes");
}
