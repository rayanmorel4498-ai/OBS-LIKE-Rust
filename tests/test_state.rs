// visualisation_module/tests/test_state.rs

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

use visualisation_module::utils::config;
use visualisation_module::capture::{
    Ping, StateManager, ScreenCapture, AudioCapture, InputCapture, Preprocessor, Transmitter,
    EthernetClient, BluetoothClient,
};

/// Indique si ce test est lancé depuis test_all
pub static mut RUNNING_FROM_TEST_ALL: bool = false;

/// Ton test interne complet (5 minutes) pour usage unitaire
fn test_state_internal(ping: Arc<Ping>) {
    // ==== GARDÉ TEL QUEL ====
    let screen = Arc::new(ScreenCapture::new());
    let audio = Arc::new(AudioCapture::new());
    let input = Arc::new(InputCapture::new());

    let ethernet = Arc::new(EthernetClient::new());
    let bluetooth = Arc::new(BluetoothClient::new());
    let transmitter = Arc::new(Transmitter::new(Arc::clone(&ethernet), Arc::clone(&bluetooth)));

    let preprocessor = Arc::new(Preprocessor::new(
        Arc::clone(&screen),
        Arc::clone(&audio),
        Arc::clone(&input),
    ));
    preprocessor.attach_transmitter(Arc::clone(&transmitter));

    let mut state_manager = StateManager::new(
        Arc::clone(&ping),
        Arc::clone(&preprocessor),
        Arc::clone(&transmitter),
        Arc::clone(&screen),
        Arc::clone(&audio),
        Arc::clone(&input),
    );

    while ping.is_pool_active() {
        thread::sleep(Duration::from_millis(50));
    }

    state_manager.start();

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(300) { // 5 minutes
        let pool_active = ping.is_pool_active();

        if pool_active {
            assert!(screen.is_running());
            assert!(audio.is_running());
            assert!(input.is_running());
            assert!(preprocessor.is_running());
            assert!(transmitter.is_running());
        } else {
            assert!(!screen.is_running());
            assert!(!audio.is_running());
            assert!(!input.is_running());
            assert!(!preprocessor.is_running());
            assert!(!transmitter.is_running());
        }

        thread::sleep(Duration::from_millis(50));
    }

    state_manager.stop();
}

/// Test pour test_all : pool aléatoire durant toute la durée de test_all
pub fn test_state_for_test_all(ping: Arc<Ping>, test_all_duration_secs: u64) {
    unsafe { RUNNING_FROM_TEST_ALL = true; }

    let mut rng = rand::thread_rng();

    // Choisir un moment aléatoire pour démarrer la pool
    let pool_start_delay = rng.gen_range(0..=test_all_duration_secs.saturating_sub(1));
    // Choisir une durée aléatoire de la pool
    let pool_duration = rng.gen_range(1..=(test_all_duration_secs - pool_start_delay).max(1));

    let ping_clone = Arc::clone(&ping);

    let pool_handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(pool_start_delay));
        start_backend_pool();
        let start_time = Instant::now();
        while start_time.elapsed() < Duration::from_secs(pool_duration) {
            if ping_clone.is_pool_active() {
                thread::sleep(Duration::from_millis(10));
            }
        }
        stop_backend_pool();
    });

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(test_all_duration_secs) {
        thread::sleep(Duration::from_millis(50));
    }

    let _ = pool_handle.join();
    unsafe { RUNNING_FROM_TEST_ALL = false; }
}

/// Démarre la pool réelle via le backend configuré dynamiquement
fn start_backend_pool() {
    let backend_url = config::get_pool_backend_url(); // récupère depuis config.rs
    let client = reqwest::blocking::Client::new();
    let _ = client.post(format!("{}/start_pool", backend_url))
        .send()
        .expect("Impossible de démarrer la pool backend");
}

/// Stoppe la pool réelle via le backend configuré dynamiquement
fn stop_backend_pool() {
    let backend_url = config::get_pool_backend_url(); // récupère depuis config.rs
    let client = reqwest::blocking::Client::new();
    let _ = client.post(format!("{}/stop_pool", backend_url))
        .send()
        .expect("Impossible d'arrêter la pool backend");
}
