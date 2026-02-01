// visualisation_module/tests/test_input.rs

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::thread;

use rdev::{listen, Event, EventType};
use visualisation_module::capture::Ping;

/// Test réel clavier / souris (OS-level hook)
/// - Aucun input synthétique
/// - Capture réelle des événements utilisateur
/// - Uniquement en idle (aucun pong)
/// - Durée : 5 minutes
pub fn test_input_5min_idle(ping: Arc<Ping>) {
    // --- Attente idle strict ---
    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(50));
    }

    let running = Arc::new(AtomicBool::new(true));
    let running_listener = Arc::clone(&running);

    let start = Instant::now();
    let mut keyboard_events: u64 = 0;
    let mut mouse_events: u64 = 0;

    // --- Thread listener OS-level ---
    let listener = thread::spawn(move || {
        let callback = move |event: Event| {
            if !running_listener.load(Ordering::Relaxed) {
                return;
            }

            match event.event_type {
                EventType::KeyPress(_) |
                EventType::KeyRelease(_) => {
                    keyboard_events += 1;
                }

                EventType::ButtonPress(_) |
                EventType::ButtonRelease(_) |
                EventType::MouseMove { .. } |
                EventType::Wheel { .. } => {
                    mouse_events += 1;
                }

                _ => {}
            }
        };

        // Hook système réel (bloquant)
        if let Err(_) = listen(callback) {
            // Pas de print, pas de panic
        }
    });

    // --- Durée du test ---
    while start.elapsed() < Duration::from_secs(300) {
        thread::sleep(Duration::from_millis(10));
    }

    // --- Stop ---
    running.store(false, Ordering::Relaxed);

    // On laisse le listener se terminer proprement
    let _ = listener.join();

    // --- Assertions MINIMALES ---
    // On vérifie que le système a bien capturé du réel
    assert!(
        keyboard_events > 0 || mouse_events > 0,
        "Aucun événement input réel capturé pendant 5 minutes"
    );
}
