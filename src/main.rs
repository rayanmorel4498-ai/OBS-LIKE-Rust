// visualisation_module/src/main.rs

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use visualisation_module::{
    ScreenCapture, AudioCapture, InputCapture,
    Preprocessor, Transmitter,
    EthernetClient, BluetoothClient,
    Ping, StateManager,
    Metrics, LoggingManager, Config,
};

#[tokio::main]
async fn main() {
    // --- Initialisation config et logs ---
    let _ = Config::get_pool_ip();
    let logging = Arc::new(LoggingManager::init());
    logging.push_log(visualisation_module::LogEntry::new(
        "main",
        "Démarrage du module de visualisation..."
    ));

    // --- Métriques ---
    let metrics = Arc::new(Metrics::new());

    // --- Modules hardware ---
    let mut screen = ScreenCapture::new();
    screen.attach_metrics(Arc::clone(&metrics));
    let screen = Arc::new(screen);
    
    let mut audio = AudioCapture::new();
    audio.attach_metrics(Arc::clone(&metrics));
    let audio = Arc::new(audio);
    
    let input = Arc::new(InputCapture::new());

    logging.push_log(visualisation_module::LogEntry::new("main", "Modules capture initialisés"));

    // --- Réseau ---
    let ethernet = Arc::new(EthernetClient::new());
    let bluetooth = Arc::new(BluetoothClient::new());
    let transmitter = Arc::new(
        Transmitter::new(Arc::clone(&ethernet), Arc::clone(&bluetooth), Arc::clone(&metrics))
    );

    logging.push_log(visualisation_module::LogEntry::new("main", "Modules réseau initialisés"));

    // --- Démarrer les captures en async ---
    let screen_clone = Arc::clone(&screen);
    let audio_clone = Arc::clone(&audio);
    let input_clone = Arc::clone(&input);

    tokio::spawn(async move {
        screen_clone.start().await;
    });

    tokio::spawn(async move {
        audio_clone.start().await;
    });

    tokio::spawn(async move {
        input_clone.start().await;
    });

    // --- Préprocess ---
    let mut preprocessor = Preprocessor::new(
        Arc::clone(&screen),
        Arc::clone(&audio),
        Arc::clone(&input),
    );
    preprocessor.attach_transmitter(Arc::clone(&transmitter));
    let preprocessor = Arc::new(preprocessor);

    logging.push_log(visualisation_module::LogEntry::new("main", "Préprocesseur initialisé"));

    // --- Ping H24 ---
    let ping = Arc::new(Ping::new(Arc::clone(&metrics)));
    ping.start();

    logging.push_log(visualisation_module::LogEntry::new("main", "Ping started"));

    // --- State manager ---
    let mut state_manager = StateManager::new(
        Arc::clone(&ping),
        Arc::clone(&preprocessor),
        Arc::clone(&transmitter),
        Arc::clone(&screen),
        Arc::clone(&audio),
        Arc::clone(&input),
        Arc::clone(&metrics),
    );
    state_manager.start();

    logging.push_log(visualisation_module::LogEntry::new("main", "StateManager started"));

    // --- Transmitter start ---
    transmitter.start();

    logging.push_log(visualisation_module::LogEntry::new("main", "Module opérationnel - Mode H24"));

    // --- Processus résident H24 ---
    loop {
        thread::sleep(Duration::from_secs(10));
        
        // Mettre à jour les métriques système
        metrics.update_system_metrics();
        
        // Log des métriques tous les 10s
        let summary = metrics.get_summary();
        let log_msg = format!(
            "CPU: {:.1}% | RAM: {}MB | Screen FPS: {} | Ping: {}ms",
            summary.avg_cpu,
            summary.avg_ram_mb,
            summary.avg_fps_screen,
            summary.avg_ping_ms.unwrap_or(0)
        );
        logging.push_log(visualisation_module::LogEntry::debug("metrics", &log_msg));
    }
}
