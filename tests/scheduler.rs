use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use visualisation_module::capture::Ping;

// ---- Import des tests unitaires ----
use crate::{
    test_audio::test_audio_5min,
    test_bluetooth::test_bluetooth_5min,
    test_ethernet::test_ethernet_5min,
    test_input::test_input_5min,
    test_ping::test_ping_5min,
    test_screen::test_screen_5min,
    test_state::test_state_for_test_all,
    test_transmitter::test_transmitter_5min,
};

/// Durée standard d’un test unitaire
const TEST_DURATION: Duration = Duration::from_secs(300);

/// Décalage entre lancements async
const ASYNC_LAUNCH_INTERVAL: Duration = Duration::from_secs(60);

pub fn run_all_tests(ping: Arc<Ping>) {
    // ---------- PHASE 0 : IDLE STRICT ----------
    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(50));
    }

    // ---------- PHASE 1 : SYNCHRONE ----------
    test_ping_5min(Arc::clone(&ping));
    test_audio_5min(Arc::clone(&ping));
    test_screen_5min(Arc::clone(&ping));
    test_input_5min(Arc::clone(&ping));
    test_ethernet_5min(Arc::clone(&ping));
    test_bluetooth_5min(Arc::clone(&ping));
    test_transmitter_5min(Arc::clone(&ping));

    // test_state uniquement orchestré par test_all
    test_state_for_test_all(Arc::clone(&ping), TEST_DURATION.as_secs());

    // ---------- PHASE 2 : ASYNC DÉCALÉ ----------
    let async_tests: Vec<fn(Arc<Ping>)> = vec![
        test_ping_5min,
        test_audio_5min,
        test_screen_5min,
        test_input_5min,
        test_ethernet_5min,
        test_bluetooth_5min,
        test_transmitter_5min,
    ];

    let mut handles = Vec::new();

    for test in async_tests {
        let ping_clone = Arc::clone(&ping);
        let handle = thread::spawn(move || {
            test(ping_clone);
        });
        handles.push(handle);
        thread::sleep(ASYNC_LAUNCH_INTERVAL);
    }

    // test_state async avec durée totale
    {
        let ping_clone = Arc::clone(&ping);
        handles.push(thread::spawn(move || {
            test_state_for_test_all(ping_clone, TEST_DURATION.as_secs());
        }));
    }

    for h in handles {
        let _ = h.join();
    }

    // ---------- PHASE 3 : FULL PARALLÈLE ----------
    let parallel_tests: Vec<fn(Arc<Ping>)> = vec![
        test_ping_5min,
        test_audio_5min,
        test_screen_5min,
        test_input_5min,
        test_ethernet_5min,
        test_bluetooth_5min,
        test_transmitter_5min,
    ];

    let mut parallel_handles = Vec::new();

    for test in parallel_tests {
        let ping_clone = Arc::clone(&ping);
        parallel_handles.push(thread::spawn(move || {
            test(ping_clone);
        }));
    }

    // test_state full parallèle
    {
        let ping_clone = Arc::clone(&ping);
        parallel_handles.push(thread::spawn(move || {
            test_state_for_test_all(ping_clone, TEST_DURATION.as_secs());
        }));
    }

    for h in parallel_handles {
        let _ = h.join();
    }
}
