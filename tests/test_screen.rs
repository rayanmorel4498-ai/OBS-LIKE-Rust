// visualisation_module/tests/test_screen.rs

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use visualisation_module::capture::{ScreenCapture, Ping, Transmitter};

/// Test écran réel OS-level pendant 5 minutes
/// - Idle strict (aucun pong)
/// - Capture complète de tous les moniteurs
/// - Mesure FPS et stabilité
/// - Pousse les frames vers le transmitter pour simuler l'envoi réel
pub fn test_screen_idle_5min(
    ping: Arc<Ping>,
    screen: Arc<ScreenCapture>,
    transmitter: Arc<Transmitter>,
) {
    // Attente idle strict
    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(50));
    }

    // Démarrage du module écran
    screen.start();

    let start = Instant::now();
    let mut frame_count: u64 = 0;
    let mut last_frame_time = Instant::now();

    while start.elapsed() < Duration::from_secs(300) {
        if let Some(frame) = screen.get_frame() {
            frame_count += 1;

            // Calcul FPS approximatif
            let now = Instant::now();
            let delta = now.duration_since(last_frame_time);
            last_frame_time = now;
            let fps = 1.0 / delta.as_secs_f32();

            // Pousser vers le transmitter réel
            transmitter.push_screen(frame);

            // Optionnel : ajustement dynamique pour pas saturer CPU/GPU
            if fps > 60.0 {
                thread::sleep(Duration::from_millis(1));
            }
        } else {
            // Aucun frame : léger wait pour réduire la charge
            thread::sleep(Duration::from_millis(5));
        }
    }

    screen.stop();

    // Vérifications strictes
    assert!(frame_count > 1000, "Frames écran insuffisantes pendant 5 minutes");
}
