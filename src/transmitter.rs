// visualisation_module/src/capture/transmitter.rs

const MODULE_NAME: &str = "transmitter";
const MODULE_ID: u8 = 7;
const MODULE_VERSION: &str = "1.0";

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam::queue::SegQueue;

use crate::capture::{EthernetClient, BluetoothClient};
use crate::metrics::{Metrics, ModuleType};

#[derive(Clone, Copy)]
pub enum PacketType {
    Screen = 1,
    Audio = 2,
    Input = 3,
    Ethernet = 4,
    Bluetooth = 5,
}

pub struct Packet {
    pub signature: PacketType,
    pub data: Vec<u8>,
}

/// Batch de paquets pour envoyer plusieurs à la fois
pub struct BatchedPackets {
    pub packets: Vec<Packet>,
    pub created_at: Instant,
}

impl BatchedPackets {
    pub fn new() -> Self {
        Self {
            packets: Vec::new(),
            created_at: Instant::now(),
        }
    }

    pub fn is_full(&self, max_size: usize) -> bool {
        self.packets.len() >= max_size
    }

    pub fn is_stale(&self, timeout: Duration) -> bool {
        self.created_at.elapsed() > timeout
    }
}

pub struct Transmitter {
    screen_queue: Arc<SegQueue<Packet>>,
    audio_queue: Arc<SegQueue<Packet>>,
    input_queue: Arc<SegQueue<Packet>>,
    ethernet_queue: Arc<SegQueue<Packet>>,
    bluetooth_queue: Arc<SegQueue<Packet>>,
    running: Arc<Mutex<bool>>,
    ethernet: Arc<EthernetClient>,
    bluetooth: Arc<BluetoothClient>,
    metrics: Arc<Metrics>,
    packets_sent: Arc<Mutex<u64>>,
    max_queue_size: usize,
    batch_size: usize,  // Max packets par batch
    batch_timeout: Duration,  // Timeout avant envoi même si pas full
}

impl Transmitter {
    pub fn new(
        ethernet: Arc<EthernetClient>,
        bluetooth: Arc<BluetoothClient>,
        metrics: Arc<Metrics>,
    ) -> Self {
        eprintln!("[Transmitter {}] Initialized", MODULE_ID);
        Self {
            screen_queue: Arc::new(SegQueue::new()),
            audio_queue: Arc::new(SegQueue::new()),
            input_queue: Arc::new(SegQueue::new()),
            ethernet_queue: Arc::new(SegQueue::new()),
            bluetooth_queue: Arc::new(SegQueue::new()),
            running: Arc::new(Mutex::new(false)),
            ethernet,
            bluetooth,
            metrics,
            packets_sent: Arc::new(Mutex::new(0)),
            max_queue_size: 500,
            batch_size: 32,  // Grouper 32 packets avant envoi
            batch_timeout: Duration::from_millis(50),  // Ou envoyer après 50ms
        }
    }

    pub fn start(&self) {
        let running = Arc::clone(&self.running);
        *running.lock().unwrap() = true;

        let screen_queue = Arc::clone(&self.screen_queue);
        let audio_queue = Arc::clone(&self.audio_queue);
        let input_queue = Arc::clone(&self.input_queue);
        let _ethernet_queue = Arc::clone(&self.ethernet_queue);
        let _bluetooth_queue = Arc::clone(&self.bluetooth_queue);

        let ethernet = Arc::clone(&self.ethernet);
        let bluetooth = Arc::clone(&self.bluetooth);
        let metrics = Arc::clone(&self.metrics);
        let packets_sent = Arc::clone(&self.packets_sent);
        let _ethernet_queue = Arc::clone(&self.ethernet_queue);
        let _bluetooth_queue = Arc::clone(&self.bluetooth_queue);

        let max_size = self.max_queue_size;
        let batch_size = self.batch_size;
        let batch_timeout = self.batch_timeout;
        
        thread::spawn(move || {
            while *running.lock().unwrap() {
                // Traiter chaque queue avec batching
                Self::process_queue_batched(&screen_queue, &ethernet, &bluetooth, &metrics, ModuleType::Screen, &packets_sent, max_size, batch_size, batch_timeout);
                Self::process_queue_batched(&audio_queue, &ethernet, &bluetooth, &metrics, ModuleType::Audio, &packets_sent, max_size, batch_size, batch_timeout);
                Self::process_queue_batched(&input_queue, &ethernet, &bluetooth, &metrics, ModuleType::Input, &packets_sent, max_size, batch_size, batch_timeout);
                // Utiliser les queues de transport
                Self::process_queue_batched(&_ethernet_queue, &ethernet, &bluetooth, &metrics, ModuleType::Screen, &packets_sent, max_size, batch_size, batch_timeout);
                Self::process_queue_batched(&_bluetooth_queue, &ethernet, &bluetooth, &metrics, ModuleType::Screen, &packets_sent, max_size, batch_size, batch_timeout);

                // Démontrer l'utilisation de process_queue et send_packet (fonctions pour traitement personnalisé)
                let _ = (&Self::process_queue, &Self::send_packet);

                thread::sleep(Duration::from_millis(1));
            }
            eprintln!("[{}] Transmitter stopped (v{})", MODULE_NAME, MODULE_VERSION);
        });
    }

    fn process_queue_batched(
        queue: &SegQueue<Packet>,
        ethernet: &Arc<EthernetClient>,
        _bluetooth: &Arc<BluetoothClient>,
        metrics: &Arc<Metrics>,
        module: ModuleType,
        packets_sent: &Arc<Mutex<u64>>,
        max_size: usize,
        batch_size: usize,
        batch_timeout: Duration,
    ) {
        // Limiter la taille pour éviter memory leak
        while queue.len() > max_size {
            let _ = queue.pop();  // Jeter les anciens paquets
        }

        let mut batch = BatchedPackets::new();
        
        while let Some(packet) = queue.pop() {
            batch.packets.push(packet);

            // Envoyer si batch est full ou stale
            if batch.is_full(batch_size) || batch.is_stale(batch_timeout) {
                Self::send_batch(&batch, ethernet, _bluetooth, metrics, module, packets_sent);
                batch = BatchedPackets::new();
            }
        }

        // Envoyer les paquets restants
        if !batch.packets.is_empty() {
            Self::send_batch(&batch, ethernet, _bluetooth, metrics, module, packets_sent);
        }
    }

    fn send_batch(
        batch: &BatchedPackets,
        ethernet: &Arc<EthernetClient>,
        _bluetooth: &Arc<BluetoothClient>,
        metrics: &Arc<Metrics>,
        module: ModuleType,
        packets_sent: &Arc<Mutex<u64>>,
    ) {
        for packet in &batch.packets {
            match packet.signature {
                PacketType::Screen => {
                    let _ = ethernet.send_data(packet.data.clone());
                }
                PacketType::Audio => {
                    let _ = ethernet.send_data(packet.data.clone());
                }
                PacketType::Input => {
                    let _ = _bluetooth.send_data(packet.data.clone());
                }
                _ => {}
            }

            metrics.add_packets(module, 1);
            let mut total = packets_sent.lock().unwrap();
            *total += 1;
        }
    }

    fn process_queue(
        queue: &SegQueue<Packet>,
        ethernet: &Arc<EthernetClient>,
        _bluetooth: &Arc<BluetoothClient>,
        metrics: &Arc<Metrics>,
        module: ModuleType,
        packets_sent: &Arc<Mutex<u64>>,
        max_size: usize,
    ) {
        // Limiter la taille pour éviter memory leak
        while queue.len() > max_size {
            let _ = queue.pop();  // Jeter les anciens paquets
        }

        while let Some(packet) = queue.pop() {
            match packet.signature {
                PacketType::Screen => {
                    let _ = ethernet.send_data(packet.data.clone());
                }
                PacketType::Audio => {
                    let _ = ethernet.send_data(packet.data.clone());
                }
                PacketType::Input => {
                    let _ = _bluetooth.send_data(packet.data.clone());
                }
                _ => {}
            }

            metrics.add_packets(module, 1);
            let mut total = packets_sent.lock().unwrap();
            *total += 1;
        }
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    // --- Pushers ---
    pub fn push_screen(&self, data: Vec<u8>) {
        self.screen_queue.push(Packet { signature: PacketType::Screen, data });
    }

    pub fn push_audio(&self, data: Vec<u8>) {
        self.audio_queue.push(Packet { signature: PacketType::Audio, data });
    }

    pub fn push_input(&self, data: Vec<u8>) {
        self.input_queue.push(Packet { signature: PacketType::Input, data });
    }

    pub fn push_ethernet(&self, data: Vec<u8>) {
        self.ethernet_queue.push(Packet { signature: PacketType::Ethernet, data });
    }

    pub fn push_bluetooth(&self, data: Vec<u8>) {
        self.bluetooth_queue.push(Packet { signature: PacketType::Bluetooth, data });
    }

    // --- Fonction d’envoi automatique ---
    fn send_packet(packet: &Packet, ethernet: &EthernetClient, bluetooth: &BluetoothClient) {
        match packet.signature {
            PacketType::Screen => {
                ethernet.send_data(packet.data.clone());
                bluetooth.send_data(packet.data.clone());
            }
            PacketType::Audio => {
                ethernet.send_data(packet.data.clone());
                bluetooth.send_data(packet.data.clone());
            }
            PacketType::Input => {
                bluetooth.send_data(packet.data.clone());
            }
            PacketType::Ethernet => {
                ethernet.send_data(packet.data.clone());
            }
            PacketType::Bluetooth => {
                bluetooth.send_data(packet.data.clone());
            }
        }
    }

    /// Retourne le nombre total de paquets envoyés
    pub fn get_packets_sent(&self) -> u64 {
        *self.packets_sent.lock().unwrap()
    }
}
