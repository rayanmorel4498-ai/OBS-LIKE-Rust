#![no_std]
#![allow(non_camel_case_types)]

extern crate alloc;

pub mod config;

use crate::config::get_config;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

pub use config::hardware_pool::{HardwareCommandPool, HardwareDriver, HardwareRequest, HardwareResponse, CommandType};
pub use config::hardware_driver_service::{HardwareDriverService, SecureMmioMapping};

pub mod cpu {
    pub mod cpu_cores;
    pub mod cpu_control;
    pub mod cpu_frequency;
    pub mod cpu_power;
    pub mod cpu_registers;
    pub mod cpu_security;
    pub mod cpu_cache;
}

pub mod gpu {
    pub mod gpu_control;
    pub mod gpu_frequency;
    pub mod gpu_memory;
    pub mod gpu_power;
    pub mod gpu_registers;
    pub mod gpu_security;
    pub mod gpu_command;
    pub mod thermal_scaling;
    pub mod mali;
    pub mod screen;
}

pub mod ram {
    pub mod ram_control;
    pub mod ram_refresh;
    pub mod ram_registrers;
    pub mod ram_security;
    pub mod ram_timing;
    pub mod ram_monitor;
}

pub mod display {
    pub mod framebuffer;
    pub mod dynamic;
    pub mod screen;
    pub mod brightness;
    pub mod adaptive_refresh;
    pub mod touch;
    pub mod stylus;
    pub mod display_control;
}

pub mod modem {
    pub mod fiveg;
    pub mod lte;
    pub mod gsm;
    pub mod wifi;
    pub mod bluetooth;
    pub mod esim;
    pub mod sim;
    pub mod satellite;
    pub mod zigbee;
    pub mod thread;
}

pub mod audio {
    pub mod microphone;
    pub mod speaker;
    pub mod audio_codec;
    pub mod audio_input_control;
    pub mod headphone_jack;
    pub mod noise_cancellation;
}

pub mod nfc {
    pub mod reader;
    pub mod writer;
    pub mod payment;
}

pub mod biometric {
    pub mod faceid;
    pub mod fingerprint;
    pub mod iris;
    pub mod voice_biometrics;
}

pub mod camera {
    pub mod front_camera;
    pub mod rear_camera;
    pub mod camera_control;
    pub mod flash;
    pub mod zoom;
    pub mod stabilization;
    pub mod depth_sensor;
}

pub mod gps {
    pub mod gps;
    pub mod location;
    pub mod geofencing;
}

pub mod sensors {
    pub mod accelerometer;
    pub mod gyroscope;
    pub mod magnetometer;
    pub mod light_sensor;
    pub mod proximity_sensor;
    pub mod barometer;
    pub mod thermometer;
    pub mod humidity_sensor;
    pub mod air_quality_sensor;
    pub mod heart_rate_sensor;
    pub mod pulse_oximetry;
    pub mod step_counter;
    pub mod uv_sensor;
}

pub mod power {
    pub mod battery;
    pub mod charging;
    pub mod fast_charging;
    pub mod wireless_charging;
    pub mod power_management;
    pub mod solar_panel;
}

pub mod storage {
    pub mod flash;
    pub mod ufs;
    pub mod sd_card;
    pub mod usb_storage;
    pub mod encryption;
}

pub mod thermal {
    pub mod thermal_sensor;
    pub mod thermal_management;
    pub mod fan_control;
    pub mod passive_cooling;
    pub mod thermal_throttling;
    pub mod thermal_control;
    pub mod manager;
    pub mod battery_sensor;
    pub mod cpu_sensor;
    pub mod gpu_sensor;
}

pub mod security {
    pub mod secure_element;
    pub mod trusted_execution;
    pub mod encryption_module;
    pub mod anti_tamper;
    pub mod intrusion_detection;
    pub mod secure_boot;
    pub mod iommu;
    pub mod key_storage;
}

pub mod haptics {
    pub mod vibrator;
    pub mod linear_actuator;
    pub mod haptics_control;
}

pub mod misc {
    pub mod led;
    pub mod gps_compass;
    pub mod ir_sensor;
    pub mod ambient_microphone;
    pub mod environmental_sensor;
    pub mod fan;
    pub mod vibration_motor;
}

pub mod device_interfaces {
    pub mod gpio;
    pub mod i2c;
    pub mod i2c_master;
    pub mod spi;
    pub mod uart;
    pub mod usb;
    pub mod pci;
}

pub mod hardware_manager;

#[cfg(feature = "embedded-secure-yaml")]
const SECURE_YAML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/secure.yaml"));
#[cfg(not(feature = "embedded-secure-yaml"))]
const SECURE_YAML: &str = "";

static mut SECURE_YAML_OVERRIDE: Option<&'static str> = None;

pub fn set_secure_yaml(yaml: &'static str) {
    unsafe {
        SECURE_YAML_OVERRIDE = Some(yaml);
    }
}

fn secure_yaml() -> &'static str {
    unsafe { SECURE_YAML_OVERRIDE.unwrap_or(SECURE_YAML) }
}

pub fn yaml_get_u64(path: &[&str]) -> Option<u64> {
    yaml_find_value(secure_yaml(), path).and_then(parse_u64)
}

pub fn yaml_get_u32(path: &[&str]) -> Option<u32> {
    yaml_find_value(secure_yaml(), path)
        .and_then(parse_u64)
        .map(|v| v as u32)
}

fn yaml_find_value<'a>(yaml: &'a str, path: &[&str]) -> Option<&'a str> {
    let mut stack: Vec<(usize, &'a str)> = Vec::new();

    for raw_line in yaml.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim_end();
        if line.trim().is_empty() {
            continue;
        }

        let indent = raw_line.chars().take_while(|c| *c == ' ').count();
        let trimmed = line.trim_start();

        let mut parts = trimmed.splitn(2, ':');
        let key = match parts.next() {
            Some(k) if !k.is_empty() => k.trim(),
            _ => continue,
        };
        let value = parts.next().map(|v| v.trim()).unwrap_or("");

        while let Some((prev_indent, _)) = stack.last() {
            if indent <= *prev_indent {
                stack.pop();
            } else {
                break;
            }
        }

        stack.push((indent, key));

        if !value.is_empty() && stack.len() == path.len() {
            let mut matches = true;
            for (idx, expected) in path.iter().enumerate() {
                if stack.get(idx).map(|(_, k)| *k) != Some(*expected) {
                    matches = false;
                    break;
                }
            }

            if matches {
                return Some(strip_quotes(value));
            }
        }
    }

    None
}

fn strip_quotes(value: &str) -> &str {
    let value = value.trim();
    if let Some(stripped) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
        return stripped;
    }
    if let Some(stripped) = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')) {
        return stripped;
    }
    value
}

fn parse_u64(value: &str) -> Option<u64> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        value.parse().ok()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComponentState {
    Uninitialized,
    Initializing,
    Ready,
    Active,
    Sleeping,
    Throttled,       // Réduction de performance (ex: CPU 50%, GPU off)
    ReducedFeature,  // Fonctionnalité réduite (ex: GPU sans shaders)
    OfflineOptional, // Désactivé mais non-critique (GPS off)
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SystemHealth {
    Ready,
    ThrottledOptimal,      // Throttling volontaire (batterie faible)
    DegradedPartial,       // Quelques fonctionnalités perdues (non-critique)
    DegradedLimited,       // Nombreuses restrictions mais bootable
    CriticalReduced,       // Subsystèmes critiques throttled
    Error,
}

// DAG de dépendances: qui dépend de qui?
// Power(tier0) → Bus(tier1) → CPU/GPU/RAM/Display(tier2) → Périphériques(tier3)
pub struct DependencyNode {
    pub name: &'static str,
    pub depends_on: &'static [&'static str],      // Dépendances REQUISES
    pub optional_deps: &'static [&'static str],   // Dépendances NON-bloquantes
    pub critical: bool,                           // Si erreur → SystemHealth::Error
}

// Graphe de dépendances pour ordonnancement d'init + vérification de conditions
pub const HARDWARE_DEPENDENCY_GRAPH: &[DependencyNode] = &[
    // Tier 0: Power (aucune dépendance)
    DependencyNode {
        name: "power",
        depends_on: &[],
        optional_deps: &[],
        critical: true,
    },
    // Tier 1: Bus (dépend de power)
    DependencyNode {
        name: "bus",
        depends_on: &["power"],
        optional_deps: &[],
        critical: true,
    },
    // Tier 2: Compute (dépend de power + bus)
    DependencyNode {
        name: "cpu",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: true,
    },
    DependencyNode {
        name: "gpu",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: true,
    },
    DependencyNode {
        name: "ram",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: true,
    },
    DependencyNode {
        name: "display",
        depends_on: &["power", "bus", "cpu"],
        optional_deps: &[],
        critical: true,
    },
    // Tier 3: Périphériques (dépend de bus)
    DependencyNode {
        name: "modem",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "audio",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "nfc",
        depends_on: &["bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "camera",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "gps",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "sensors",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "biometric",
        depends_on: &["bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "thermal",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: false,
    },
    DependencyNode {
        name: "storage",
        depends_on: &["power", "bus"],
        optional_deps: &[],
        critical: false,
    },
];

// Séquence d'arrêt ordonnée (inverse des dépendances)
pub const RECOVERY_SHUTDOWN_SEQUENCE: &[&str] = &[
    // Tier 3: Périphériques en premier
    "modem", "audio", "nfc", "camera", "gps", "sensors", "biometric", "thermal", "storage",
    // Tier 2: Compute
    "display", "gpu", "ram", "cpu",
    // Tier 1: Bus
    "bus",
    // Tier 0: Power en dernier
    "power",
];

#[derive(Clone, Debug, PartialEq)]
pub struct InitError {
    pub component: &'static str,
    pub message: String,
}

impl InitError {
    pub fn new(component: &'static str, message: String) -> Self {
        Self { component, message }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorTelemetry {
    pub total_count: u32,
    pub last_error: Option<InitError>,
    pub per_component: BTreeMap<&'static str, (u32, String)>,
    // Nouvelle v2: Métriques de récupération
    pub recovery_attempts: u32,
    pub recovery_successes: u32,
    pub last_recovery_ms: u64,  // timestamp du dernier recovery
}

impl ErrorTelemetry {
    pub fn new() -> Self {
        Self {
            total_count: 0,
            last_error: None,
            per_component: BTreeMap::new(),
            recovery_attempts: 0,
            recovery_successes: 0,
            last_recovery_ms: 0,
        }
    }
    
    /// Ajouter une tentative de récupération (appelée par RecoverComponent handler)
    pub fn record_recovery_attempt(&mut self) {
        self.recovery_attempts += 1;
    }
    
    /// Enregistrer un succès de récupération
    pub fn record_recovery_success(&mut self, timestamp_ms: u64) {
        self.recovery_successes += 1;
        self.last_recovery_ms = timestamp_ms;
    }
    
    /// Calculer le taux de succès des récupérations (0-100%)
    pub fn recovery_success_rate(&self) -> f32 {
        if self.recovery_attempts == 0 {
            0.0
        } else {
            (self.recovery_successes as f32 / self.recovery_attempts as f32) * 100.0
        }
    }
}

pub struct HardwareManager {
    pub cpu_state: ComponentState,
    pub gpu_state: ComponentState,
    pub ram_state: ComponentState,
    pub display_state: ComponentState,
    pub power_state: ComponentState,
    pub bus_state: ComponentState,
    pub modem_state: ComponentState,
    pub audio_state: ComponentState,
    pub nfc_state: ComponentState,
    pub camera_state: ComponentState,
    pub gps_state: ComponentState,
    pub sensors_state: ComponentState,
    pub biometric_state: ComponentState,
    pub thermal_state: ComponentState,
    pub storage_state: ComponentState,
    pub system_health: SystemHealth,
    pub errors: Vec<InitError>,
    pub telemetry: ErrorTelemetry,
    // Health monitoring
    pub last_health_poll_ms: u64,
    pub component_last_error_ms: BTreeMap<&'static str, u64>,
}

impl HardwareManager {
    pub fn new() -> Self {
        config::init_config();
        HardwareManager {
            cpu_state: ComponentState::Uninitialized,
            gpu_state: ComponentState::Uninitialized,
            ram_state: ComponentState::Uninitialized,
            display_state: ComponentState::Uninitialized,
            power_state: ComponentState::Uninitialized,
            bus_state: ComponentState::Uninitialized,
            modem_state: ComponentState::Uninitialized,
            audio_state: ComponentState::Uninitialized,
            nfc_state: ComponentState::Uninitialized,
            camera_state: ComponentState::Uninitialized,
            gps_state: ComponentState::Uninitialized,
            sensors_state: ComponentState::Uninitialized,
            biometric_state: ComponentState::Uninitialized,
            thermal_state: ComponentState::Uninitialized,
            storage_state: ComponentState::Uninitialized,
            system_health: SystemHealth::Ready,
            errors: Vec::new(),
            telemetry: ErrorTelemetry::new(),
            last_health_poll_ms: 0,
            component_last_error_ms: BTreeMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        let _config = config::HardwareConfig::default();
        HardwareManager {
            cpu_state: ComponentState::Uninitialized,
            gpu_state: ComponentState::Uninitialized,
            ram_state: ComponentState::Uninitialized,
            display_state: ComponentState::Uninitialized,
            power_state: ComponentState::Uninitialized,
            bus_state: ComponentState::Uninitialized,
            modem_state: ComponentState::Uninitialized,
            audio_state: ComponentState::Uninitialized,
            nfc_state: ComponentState::Uninitialized,
            camera_state: ComponentState::Uninitialized,
            gps_state: ComponentState::Uninitialized,
            sensors_state: ComponentState::Uninitialized,
            biometric_state: ComponentState::Uninitialized,
            thermal_state: ComponentState::Uninitialized,
            storage_state: ComponentState::Uninitialized,
            system_health: SystemHealth::Ready,
            errors: Vec::new(),
            telemetry: ErrorTelemetry::new(),
            last_health_poll_ms: 0,
            component_last_error_ms: BTreeMap::new(),
        }
    }

    fn record_error(&mut self, component: &'static str, message: String) {
        let err = InitError::new(component, message.clone());
        self.errors.push(err.clone());
        self.telemetry.total_count = self.telemetry.total_count.saturating_add(1);
        self.telemetry.last_error = Some(err);
        let entry = self.telemetry.per_component.entry(component).or_insert((0, String::new()));
        entry.0 = entry.0.saturating_add(1);
        entry.1 = message;
    }

    pub fn init_all(&mut self) -> Result<(), String> {
        self.errors.clear();
        self.system_health = SystemHealth::Ready;

        self.power_state = ComponentState::Initializing;
        let mut power_error = false;
        let cfg = get_config();
        if cfg.registers.power_base == 0 {
            power_error = true;
            self.record_error("power.base", String::from("power_base_unset"));
        }
        if cfg.registers.vdd_core == 0 || cfg.registers.vdd_gpu == 0 || cfg.registers.vdd_modem == 0 || cfg.registers.vdd_io == 0 {
            power_error = true;
            self.record_error("power.rails", String::from("vdd_rail_unset"));
        }
        self.power_state = if power_error { ComponentState::Error } else { ComponentState::Ready };

        self.bus_state = ComponentState::Initializing;
        let mut bus_error = false;
        if cfg.registers.i2c_base == 0 {
            bus_error = true;
            self.record_error("bus.i2c", String::from("i2c_base_unset"));
        }
        if cfg.registers.uart_base == 0 {
            bus_error = true;
            self.record_error("bus.uart", String::from("uart_base_unset"));
        }
        if cfg.registers.spi_base == 0 {
            bus_error = true;
            self.record_error("bus.spi", String::from("spi_base_unset"));
        }
        if cfg.registers.pci_base == 0 {
            bus_error = true;
            self.record_error("bus.pci", String::from("pci_base_unset"));
        }
        if cfg.registers.usb_base == 0 {
            bus_error = true;
            self.record_error("bus.usb", String::from("usb_base_unset"));
        }
        self.bus_state = if bus_error { ComponentState::Error } else { ComponentState::Ready };

        self.cpu_state = ComponentState::Initializing;
        let mut cpu_error = false;
        let freq = cpu::cpu_frequency::CpuFreqLevel::Medium.to_freq();
        if let Err(err) = cpu::cpu_frequency::set_raw(freq.big_mhz, freq.little_mhz) {
            cpu_error = true;
            self.record_error("cpu.frequency", err.to_string());
        }
        for core_id in 1..cpu::cpu_cores::MAX_CPU_CORES {
            if let Err(err) = cpu::cpu_cores::power_on(core_id) {
                cpu_error = true;
                self.record_error("cpu.core", err.to_string());
            }
        }
        self.cpu_state = if cpu_error { ComponentState::Error } else { ComponentState::Ready };

        self.gpu_state = ComponentState::Initializing;
        if let Err(err) = gpu::gpu_control::init() {
            self.gpu_state = ComponentState::Error;
            self.record_error("gpu", err.to_string());
        } else {
            self.gpu_state = ComponentState::Ready;
        }

        self.ram_state = ComponentState::Initializing;
        match ram::ram_control::init() {
            Ok(()) => {
                self.ram_state = ComponentState::Ready;
            }
            Err(err) => {
                self.ram_state = ComponentState::Error;
                self.record_error("ram", err.to_string());
            }
        }

        self.display_state = ComponentState::Initializing;
        let display_init = display::screen::init_display();
        let display_brightness = display::screen::set_brightness(200);
        let display_refresh = display::screen::set_refresh_rate(60);
        if let Err(err) = display_init {
            self.display_state = ComponentState::Error;
            self.record_error("display", err.to_string());
        }
        if let Err(err) = display_brightness {
            self.display_state = ComponentState::Error;
            self.record_error("display.brightness", err.to_string());
        }
        if let Err(err) = display_refresh {
            self.display_state = ComponentState::Error;
            self.record_error("display.refresh", err.to_string());
        }
        if self.display_state != ComponentState::Error {
            self.display_state = ComponentState::Ready;
        }

        self.modem_state = ComponentState::Initializing;
        let mut modem_error = false;
        if let Err(err) = modem::lte::init() {
            modem_error = true;
            self.record_error("modem.lte", err.to_string());
        }
        if let Err(err) = modem::fiveg::init() {
            modem_error = true;
            self.record_error("modem.fiveg", err.to_string());
        }
        if let Err(err) = modem::gsm::init() {
            modem_error = true;
            self.record_error("modem.gsm", err.to_string());
        }
        if let Err(err) = modem::wifi::init() {
            modem_error = true;
            self.record_error("modem.wifi", err.to_string());
        }
        if let Err(err) = modem::bluetooth::init() {
            modem_error = true;
            self.record_error("modem.bluetooth", err.to_string());
        }
        if let Err(err) = modem::esim::init() {
            modem_error = true;
            self.record_error("modem.esim", err.to_string());
        }
        if let Err(err) = modem::sim::init() {
            modem_error = true;
            self.record_error("modem.sim", err.to_string());
        }
        if let Err(err) = modem::satellite::init() {
            modem_error = true;
            self.record_error("modem.satellite", err.to_string());
        }
        if let Err(err) = modem::zigbee::init() {
            modem_error = true;
            self.record_error("modem.zigbee", err.to_string());
        }
        if let Err(err) = modem::thread::init() {
            modem_error = true;
            self.record_error("modem.thread", err.to_string());
        }
        self.modem_state = if modem_error { ComponentState::Error } else { ComponentState::Ready };

        self.audio_state = ComponentState::Initializing;
        let mut audio_error = false;
        if let Err(err) = audio::audio_codec::init() {
            audio_error = true;
            self.record_error("audio.codec", err.to_string());
        }
        if let Err(err) = audio::speaker::init() {
            audio_error = true;
            self.record_error("audio.speaker", err.to_string());
        }
        if let Err(err) = audio::microphone::init() {
            audio_error = true;
            self.record_error("audio.microphone", err.to_string());
        }
        if let Err(err) = audio::headphone_jack::init() {
            audio_error = true;
            self.record_error("audio.headphone_jack", err.to_string());
        }
        if let Err(err) = audio::noise_cancellation::init() {
            audio_error = true;
            self.record_error("audio.noise_cancellation", err.to_string());
        }
        if let Err(err) = audio::audio_input_control::init() {
            audio_error = true;
            self.record_error("audio.input_control", err.to_string());
        }
        self.audio_state = if audio_error { ComponentState::Error } else { ComponentState::Ready };

        self.nfc_state = ComponentState::Initializing;
        let mut nfc_error = false;
        if let Err(err) = nfc::reader::NFCReader::init() {
            nfc_error = true;
            self.record_error("nfc.reader", err.to_string());
        }
        if let Err(err) = nfc::writer::NFCWriter::init() {
            nfc_error = true;
            self.record_error("nfc.writer", err.to_string());
        }
        if let Err(err) = nfc::payment::NFCPayment::init() {
            nfc_error = true;
            self.record_error("nfc.payment", err.to_string());
        }
        self.nfc_state = if nfc_error { ComponentState::Error } else { ComponentState::Ready };
        self.camera_state = ComponentState::Initializing;
        let mut camera_error = false;
        if let Err(err) = camera::front_camera::init() {
            camera_error = true;
            self.record_error("camera.front", err.to_string());
        }
        if let Err(err) = camera::rear_camera::init() {
            camera_error = true;
            self.record_error("camera.rear", err.to_string());
        }
        if let Err(err) = camera::depth_sensor::init() {
            camera_error = true;
            self.record_error("camera.depth", err.to_string());
        }
        if let Err(err) = camera::camera_control::enable() {
            camera_error = true;
            self.record_error("camera.control", err.to_string());
        }
        if let Err(err) = camera::flash::enable() {
            camera_error = true;
            self.record_error("camera.flash", err.to_string());
        }
        if let Err(err) = camera::stabilization::enable() {
            camera_error = true;
            self.record_error("camera.stabilization", err.to_string());
        }
        self.camera_state = if camera_error { ComponentState::Error } else { ComponentState::Ready };

        self.gps_state = ComponentState::Initializing;
        if let Err(err) = gps::gps::enable() {
            self.gps_state = ComponentState::Error;
            self.record_error("gps", err.to_string());
        } else {
            self.gps_state = ComponentState::Ready;
        }

        self.sensors_state = ComponentState::Initializing;
        let mut sensors_error = false;
        if let Err(err) = sensors::accelerometer::AccelerometerDriver::init() {
            sensors_error = true;
            self.record_error("sensors.accelerometer", err.to_string());
        }
        if let Err(err) = sensors::gyroscope::GyroscopeDriver::init() {
            sensors_error = true;
            self.record_error("sensors.gyroscope", err.to_string());
        }
        if let Err(err) = sensors::magnetometer::MagnetometerDriver::init() {
            sensors_error = true;
            self.record_error("sensors.magnetometer", err.to_string());
        }
        if let Err(err) = sensors::barometer::BarometerDriver::init() {
            sensors_error = true;
            self.record_error("sensors.barometer", err.to_string());
        }
        self.sensors_state = if sensors_error { ComponentState::Error } else { ComponentState::Ready };

        self.biometric_state = ComponentState::Initializing;
        let mut biometric_error = false;
        if let Err(err) = biometric::fingerprint::init() {
            biometric_error = true;
            self.record_error("biometric.fingerprint", err.to_string());
        }
        if let Err(err) = biometric::faceid::init() {
            biometric_error = true;
            self.record_error("biometric.faceid", err.to_string());
        }
        if let Err(err) = biometric::iris::init() {
            biometric_error = true;
            self.record_error("biometric.iris", err.to_string());
        }
        if let Err(err) = biometric::voice_biometrics::init() {
            biometric_error = true;
            self.record_error("biometric.voice", err.to_string());
        }
        self.biometric_state = if biometric_error { ComponentState::Error } else { ComponentState::Ready };

        self.thermal_state = ComponentState::Initializing;
        let mut thermal_error = false;
        if let Err(err) = thermal::thermal_management::init() {
            thermal_error = true;
            self.record_error("thermal.management", err.to_string());
        }
        if let Err(err) = thermal::thermal_sensor::init() {
            thermal_error = true;
            self.record_error("thermal.sensor", err.to_string());
        }
        self.thermal_state = if thermal_error { ComponentState::Error } else { ComponentState::Ready };

        self.storage_state = ComponentState::Initializing;
        if let Err(err) = storage::ufs::init() {
            self.storage_state = ComponentState::Error;
            self.record_error("storage.ufs", err.to_string());
        } else {
            self.storage_state = ComponentState::Ready;
        }

        let critical_error = self.power_state == ComponentState::Error
            || self.bus_state == ComponentState::Error
            || self.cpu_state == ComponentState::Error
            || self.gpu_state == ComponentState::Error
            || self.ram_state == ComponentState::Error
            || self.display_state == ComponentState::Error;

        if critical_error {
            self.system_health = SystemHealth::Error;
            return Err(format!("{} erreur(s) critique(s)", self.errors.len()));
        }

        if self.errors.is_empty() {
            self.system_health = SystemHealth::Ready;
            Ok(())
        } else {
            self.system_health = SystemHealth::DegradedPartial;
            Ok(())
        }
    }

    pub fn errors(&self) -> &[InitError] {
        &self.errors
    }

    pub fn take_errors(&mut self) -> Vec<InitError> {
        core::mem::take(&mut self.errors)
    }

    pub fn system_health(&self) -> SystemHealth {
        self.system_health
    }

    pub fn telemetry(&self) -> &ErrorTelemetry {
        &self.telemetry
    }

    pub fn low_power_mode(&mut self) {
        self.gpu_state = ComponentState::Sleeping;
        gpu::gpu_control::disable();
        cpu::cpu_frequency::force_low_power();
        let _ = display::screen::set_refresh_rate(30);
        let _ = display::screen::set_brightness(80);
    }

    pub fn exit_low_power_mode(&mut self) {
        gpu::gpu_control::enable();
        self.gpu_state = ComponentState::Ready;
        let _ = cpu::cpu_frequency::set(cpu::cpu_frequency::CpuFreqLevel::Medium);
        let _ = display::screen::set_refresh_rate(60);
        let _ = display::screen::set_brightness(200);
    }

    pub fn hard_reset(&mut self) {
        // Séquence de récupération ordonnée (inverse des dépendances)
        // Tier 3: Arrêt des périphériques en premier
        let _ = modem::lte::disable();
        let _ = modem::wifi::disable();
        let _ = modem::bluetooth::disable();
        let _ = audio::speaker::disable();
        let _ = camera::camera_control::disable();
        let _ = gps::gps::disable();
        
        // Tier 2: Compute
        let _ = display::screen::disable_display();
        gpu::gpu_control::hard_reset();
        ram::ram_control::clear_all();
        cpu::cpu_cores::disable_all_secondary();
        
        // Reset tous les états à Uninitialized
        self.cpu_state = ComponentState::Uninitialized;
        self.gpu_state = ComponentState::Uninitialized;
        self.ram_state = ComponentState::Uninitialized;
        self.display_state = ComponentState::Uninitialized;
        self.power_state = ComponentState::Uninitialized;
        self.bus_state = ComponentState::Uninitialized;
        self.modem_state = ComponentState::Uninitialized;
        self.audio_state = ComponentState::Uninitialized;
        self.nfc_state = ComponentState::Uninitialized;
        self.camera_state = ComponentState::Uninitialized;
        self.gps_state = ComponentState::Uninitialized;
        self.sensors_state = ComponentState::Uninitialized;
        self.biometric_state = ComponentState::Uninitialized;
        self.thermal_state = ComponentState::Uninitialized;
        self.storage_state = ComponentState::Uninitialized;
        
        // Reset santé globale et telémétrie
        self.system_health = SystemHealth::Ready;
        self.errors.clear();
        self.telemetry = ErrorTelemetry::new();
    }
    
    // Vérifier si tous les composants critiques d'une couche (tier) sont Ready
    pub fn tier_ready(&self, tier: u8) -> bool {
        match tier {
            0 => self.power_state == ComponentState::Ready,
            1 => self.bus_state == ComponentState::Ready,
            2 => self.cpu_state == ComponentState::Ready
                && self.gpu_state == ComponentState::Ready
                && self.ram_state == ComponentState::Ready
                && self.display_state == ComponentState::Ready,
            _ => false,
        }
    }
    
    // Obtenir le nœud de dépendances pour un composant
    pub fn get_dependencies(component: &str) -> Option<&'static DependencyNode> {
        HARDWARE_DEPENDENCY_GRAPH.iter().find(|n| n.name == component)
    }
    
    // Vérifier si un composant peut être initialisé (toutes ses dépendances satisfaites)
    pub fn can_initialize(&self, component: &str) -> bool {
        match Self::get_dependencies(component) {
            Some(node) => {
                // Toutes les dépendances requises doivent être Ready ou Active
                node.depends_on.iter().all(|dep| {
                    let state = self.get_state(dep);
                    state == ComponentState::Ready || state == ComponentState::Active
                })
            },
            None => false,
        }
    }
    
    // Calculer SystemHealth basé sur état des composants
    pub fn compute_system_health(&self) -> SystemHealth {
        // Vérifier les critiques en premier (Power → Bus → CPU/GPU/RAM/Display)
        if self.power_state == ComponentState::Error
            || self.bus_state == ComponentState::Error
            || self.cpu_state == ComponentState::Error
            || self.gpu_state == ComponentState::Error
            || self.ram_state == ComponentState::Error
            || self.display_state == ComponentState::Error
        {
            return SystemHealth::Error;
        }
        
        // Compter les états dégradés
        let states = [
            self.cpu_state, self.gpu_state, self.ram_state, self.display_state,
            self.modem_state, self.audio_state, self.nfc_state, self.camera_state,
            self.gps_state, self.sensors_state, self.biometric_state, self.thermal_state,
            self.storage_state,
        ];
        
        let throttled_count = states.iter().filter(|s| **s == ComponentState::Throttled).count();
        let reduced_count = states.iter().filter(|s| **s == ComponentState::ReducedFeature).count();
        let offline_count = states.iter().filter(|s| **s == ComponentState::OfflineOptional).count();
        
        // Décider du niveau de santé globale
        if throttled_count > 5 || reduced_count > 3 {
            SystemHealth::CriticalReduced
        } else if reduced_count > 0 || throttled_count > 2 {
            SystemHealth::DegradedLimited
        } else if offline_count > 2 || throttled_count > 0 {
            SystemHealth::DegradedPartial
        } else if self.errors.is_empty() {
            SystemHealth::Ready
        } else {
            SystemHealth::ThrottledOptimal
        }
    }

    // 1. AMÉLIORATION CRITIQUE: Gestion d'états dégradés lors d'erreurs
    // Transition granulaire: Error → OfflineOptional si non-critique
    pub fn apply_degradation_strategy(&mut self, component: &str, is_critical: bool) {
        if is_critical {
            // Critiques restent en Error (bloqueront le boot)
            self.set_component_state(component, ComponentState::Error);
        } else {
            // Non-critiques dégradent gracieusement
            match component {
                "modem" => self.modem_state = ComponentState::OfflineOptional,
                "audio" => self.audio_state = ComponentState::OfflineOptional,
                "nfc" => self.nfc_state = ComponentState::OfflineOptional,
                "camera" => self.camera_state = ComponentState::OfflineOptional,
                "gps" => self.gps_state = ComponentState::OfflineOptional,
                "sensors" => self.sensors_state = ComponentState::OfflineOptional,
                "biometric" => self.biometric_state = ComponentState::OfflineOptional,
                "thermal" => self.thermal_state = ComponentState::OfflineOptional,
                "storage" => self.storage_state = ComponentState::OfflineOptional,
                _ => {},
            }
            self.system_health = SystemHealth::DegradedPartial;
        }
    }
    
    // 2. AMÉLIORATION HIGH: Stratégie de recovery avec respect des dépendances
    pub fn recover(&mut self) -> Result<(), String> {
        let mut recovered_count = 0;
        let mut skipped_count = 0;
        
        // Parcourir le graphe de dépendances dans l'ordre correct
        for node in HARDWARE_DEPENDENCY_GRAPH {
            let state = self.get_state(node.name);
            
            // Ne tenter recovery que pour les composants en Error ou OfflineOptional
            if state != ComponentState::Error && state != ComponentState::OfflineOptional {
                continue;
            }
            
            // Vérifier que toutes les dépendances sont satisfaites
            let deps_satisfied = node.depends_on.iter().all(|dep| {
                let dep_state = self.get_state(dep);
                dep_state == ComponentState::Ready || dep_state == ComponentState::Active
            });
            
            if !deps_satisfied {
                skipped_count += 1;
                continue;
            }
            
            // Essayer de réinitialiser le composant
            match self.reinit_component(node.name) {
                Ok(()) => {
                    self.set_component_state(node.name, ComponentState::Ready);
                    recovered_count += 1;
                },
                Err(e) => {
                    // Échec: rester en Error mais continuer les autres
                    self.record_error(node.name, format!("recovery_failed: {}", e));
                },
            }
        }
        
        // Recalculer la santé globale
        self.system_health = self.compute_system_health();
        
        if recovered_count > 0 {
            Ok(())
        } else if skipped_count > 0 {
            Err(format!("recovery_skipped: {} components waiting for dependencies", skipped_count))
        } else {
            Err("recovery_no_components_to_recover".to_string())
        }
    }
    
    // Helper: Réinitialiser un composant individuel
    fn reinit_component(&mut self, component: &str) -> Result<(), String> {
        let result: Result<(), String> = match component {
            "power" => {
                let cfg = get_config();
                if cfg.registers.power_base != 0 { Ok(()) } else { Err("no_power_base".to_string()) }
            },
            "bus" => Ok(()),
            "cpu" => { cpu::cpu_frequency::set(cpu::cpu_frequency::CpuFreqLevel::Medium); Ok(()) },
            "gpu" => gpu::gpu_control::init().map_err(|e| e.to_string()),
            "ram" => ram::ram_control::init().map_err(|e| e.to_string()),
            "display" => display::screen::init_display().map_err(|e| e.to_string()),
            "modem" => { let _ = modem::wifi::init(); Ok(()) },
            "audio" => { let _ = audio::audio_codec::init(); Ok(()) },
            "nfc" => { let _ = nfc::reader::NFCReader::init(); Ok(()) },
            "camera" => { let _ = camera::front_camera::init(); Ok(()) },
            "gps" => gps::gps::enable().map_err(|e| e.to_string()),
            "sensors" => { let _ = sensors::accelerometer::AccelerometerDriver::init(); Ok(()) },
            "biometric" => { let _ = biometric::fingerprint::init(); Ok(()) },
            "thermal" => thermal::thermal_management::init().map_err(|e| e.to_string()),
            "storage" => storage::ufs::init().map_err(|e| e.to_string()),
            _ => Err("unknown_component".to_string()),
        };
        result
    }
    
    // 4. AMÉLIORATION MONITORING: Health polling périodique
    pub fn health_poll(&mut self, timestamp_ms: u64) -> bool {
        const POLL_INTERVAL_MS: u64 = 10000; // 10 secondes
        const RECOVERY_RETRY_MS: u64 = 60000; // 60 secondes avant nouvelle tentative
        
        // Vérifier si intervalle écoulé
        if timestamp_ms.saturating_sub(self.last_health_poll_ms) < POLL_INTERVAL_MS {
            return false;
        }
        
        self.last_health_poll_ms = timestamp_ms;
        let mut state_changed = false;
        
        // Parcourir les composants en OfflineOptional
        let components_to_check = [
            "modem", "audio", "nfc", "camera", "gps", "sensors", "biometric", "thermal", "storage"
        ];
        
        for comp in &components_to_check {
            if self.get_state(comp) == ComponentState::OfflineOptional {
                // Vérifier si assez de temps s'est écoulé pour une nouvelle tentative
                if let Some(&last_error_ms) = self.component_last_error_ms.get(comp) {
                    if timestamp_ms.saturating_sub(last_error_ms) >= RECOVERY_RETRY_MS {
                        // Tenter recovery pour ce composant
                        if let Ok(()) = self.reinit_component(comp) {
                            self.set_component_state(comp, ComponentState::Ready);
                            self.component_last_error_ms.remove(*comp);
                            state_changed = true;
                        }
                    }
                }
            }
        }

        if state_changed {
            self.system_health = self.compute_system_health();
        }
        
        state_changed
    }

    fn set_component_state(&mut self, component: &str, state: ComponentState) {
        match component {
            "cpu" => self.cpu_state = state,
            "gpu" => self.gpu_state = state,
            "ram" => self.ram_state = state,
            "display" => self.display_state = state,
            "power" => self.power_state = state,
            "bus" => self.bus_state = state,
            "modem" => self.modem_state = state,
            "audio" => self.audio_state = state,
            "nfc" => self.nfc_state = state,
            "camera" => self.camera_state = state,
            "gps" => self.gps_state = state,
            "sensors" => self.sensors_state = state,
            "biometric" => self.biometric_state = state,
            "thermal" => self.thermal_state = state,
            "storage" => self.storage_state = state,
            _ => {},
        }
    }

    #[allow(dead_code)]
    pub fn notify_degradation(&self) {
        if self.system_health == SystemHealth::DegradedPartial 
            || self.system_health == SystemHealth::DegradedLimited
            || self.system_health == SystemHealth::CriticalReduced {
            
            // Collecter les composants échoués
            let _failed_comps: Vec<&str> = HARDWARE_DEPENDENCY_GRAPH
                .iter()
                .filter(|node| {
                    let state = self.get_state(node.name);
                    state == ComponentState::Error || state == ComponentState::OfflineOptional
                })
                .map(|node| node.name)
                .collect();
            
            // Placeholder pour intégration AICore/SafeAI
            // with_anomaly_detector(|det| {
            //     det.record_alert(AnomalyType::HardwareDegraded, {
            //         components: failed_comps,
            //         recoverable: !failed_comps.iter().any(|c| {
            //             if let Some(n) = Self::get_dependencies(c) { n.critical } else { false }
            //         }),
            //         severity: match self.system_health {
            //             SystemHealth::DegradedPartial => AlertSeverity::Low,
            //             SystemHealth::DegradedLimited => AlertSeverity::Medium,
            //             SystemHealth::CriticalReduced => AlertSeverity::High,
            //             _ => AlertSeverity::None,
            //         },
            //     });
            // });
        }
    }

    pub fn get_state(&self, component: &str) -> ComponentState {
        match component {
            "cpu" => self.cpu_state,
            "gpu" => self.gpu_state,
            "ram" => self.ram_state,
            "display" => self.display_state,
            "power" => self.power_state,
            "bus" => self.bus_state,
            "modem" => self.modem_state,
            "audio" => self.audio_state,
            "nfc" => self.nfc_state,
            "camera" => self.camera_state,
            "gps" => self.gps_state,
            "sensors" => self.sensors_state,
            "biometric" => self.biometric_state,
            "thermal" => self.thermal_state,
            "storage" => self.storage_state,
            _ => ComponentState::Error,
        }
    }

    pub fn all_critical_ready(&self) -> bool {
        self.cpu_state == ComponentState::Ready
            && self.gpu_state == ComponentState::Ready
            && self.ram_state == ComponentState::Ready
            && self.display_state == ComponentState::Ready
            && self.power_state == ComponentState::Ready
            && self.bus_state == ComponentState::Ready
    }
}

    pub fn cpu_apcs_base() -> u64 { get_config().registers.cpu_apcs_base }
    pub fn cpu_core_ctrl_base() -> u64 { get_config().registers.cpu_core_ctrl_base }
    pub fn cpu_big_freq_reg() -> u64 { get_config().registers.cpu_big_freq_reg }
    pub fn cpu_little_freq_reg() -> u64 { get_config().registers.cpu_little_freq_reg }
    pub fn cpu_big_volt_reg() -> u64 { get_config().registers.cpu_big_volt_reg }
    pub fn cpu_little_volt_reg() -> u64 { get_config().registers.cpu_little_volt_reg }

    pub fn gpio_base() -> u64 { get_config().registers.gpio_base }
    pub fn gpio_dir() -> u64 { get_config().registers.gpio_dir }
    pub fn gpio_out() -> u64 { get_config().registers.gpio_out }
    pub fn gpio_in() -> u64 { get_config().registers.gpio_in }
    pub fn gpio_drive() -> u64 { get_config().registers.gpio_drive }
    pub fn gpio_mode() -> u64 { get_config().registers.gpio_mode }

    pub fn i2c_base() -> u64 { get_config().registers.i2c_base }
    pub fn uart_base() -> u64 { get_config().registers.uart_base }

    pub fn pci_base() -> u64 { get_config().registers.pci_base }
    pub fn pci_cfg_addr() -> u64 { get_config().registers.pci_cfg_addr }
    pub fn pci_cfg_data() -> u64 { get_config().registers.pci_cfg_data }
    pub fn pci_status() -> u64 { get_config().registers.pci_status }
    pub fn pci_ctrl() -> u64 { get_config().registers.pci_ctrl }

    pub fn usb_base() -> u64 { get_config().registers.usb_base }

pub fn usb_ctrl() -> u64 { get_config().registers.usb_ctrl }
pub fn usb_status() -> u64 { get_config().registers.usb_status }
pub fn usb_speed() -> u64 { get_config().registers.usb_speed }
pub fn usb_power() -> u64 { get_config().registers.usb_power }

pub fn spi_base() -> u64 { get_config().registers.spi_base }
pub fn spi_ctrl() -> u64 { get_config().registers.spi_ctrl }
pub fn spi_status() -> u64 { get_config().registers.spi_status }
pub fn spi_tx() -> u64 { get_config().registers.spi_tx }
pub fn spi_rx() -> u64 { get_config().registers.spi_rx }
pub fn spi_clk() -> u64 { get_config().registers.spi_clk }

pub fn gpu_freq_ctrl() -> u64 { get_config().registers.gpu_freq_ctrl }
pub fn gpu_freq_status() -> u64 { get_config().registers.gpu_freq_status }
pub fn gpu_vram_base() -> u64 { get_config().registers.gpu_vram_base }
pub fn gpu_mem_ctrl() -> u64 { get_config().registers.gpu_mem_ctrl }
pub fn gpu_mem_status() -> u64 { get_config().registers.gpu_mem_status }
pub fn gpu_base() -> u64 { get_config().registers.gpu_base }
pub fn gpu_power_control() -> u64 { get_config().registers.gpu_power_control }
pub fn gpu_reset_control() -> u64 { get_config().registers.gpu_reset_control }
pub fn gpu_clock_control() -> u64 { get_config().registers.gpu_clock_control }
pub fn gpu_status_reg() -> u64 { get_config().registers.gpu_status_reg }
pub fn gpu_command_reg() -> u64 { get_config().registers.gpu_command_reg }
pub fn gpu_frequency_reg() -> u64 { get_config().registers.gpu_frequency_reg }
pub fn gpu_interrupt_status() -> u64 { get_config().registers.gpu_interrupt_status }
pub fn gpu_interrupt_mask() -> u64 { get_config().registers.gpu_interrupt_mask }
pub fn gpu_shader_cores_enable() -> u64 { get_config().registers.gpu_shader_cores_enable }
pub fn gpu_cores_status() -> u64 { get_config().registers.gpu_cores_status }
pub fn gpu_power_domain_0() -> u64 { get_config().registers.gpu_power_domain_0 }
pub fn gpu_power_domain_1() -> u64 { get_config().registers.gpu_power_domain_1 }
pub fn gpu_power_domain_2() -> u64 { get_config().registers.gpu_power_domain_2 }
pub fn gpu_power_domain_3() -> u64 { get_config().registers.gpu_power_domain_3 }
pub fn gpu_power_ctrl() -> u64 { get_config().registers.gpu_power_ctrl }
pub fn gpu_power_status() -> u64 { get_config().registers.gpu_power_status }
pub fn gpu_security_base() -> u32 { get_config().registers.gpu_security_base }
pub fn gpu_cmd_base() -> u64 { get_config().registers.gpu_cmd_base }
pub fn gpu_cmd_status() -> u64 { get_config().registers.gpu_cmd_status }
pub fn gpu_cmd_fence() -> u64 { get_config().registers.gpu_cmd_fence }

pub fn ddr_phy_base() -> u64 { get_config().registers.ddr_phy_base }
pub fn memc_base() -> u64 { get_config().registers.memc_base }
pub fn ddr_axi_base() -> u64 { get_config().registers.ddr_axi_base }
pub fn phy_freq_reg() -> u64 { get_config().registers.phy_freq_reg }
pub fn phy_status_reg() -> u64 { get_config().registers.phy_status_reg }
pub fn phy_mode_reg() -> u64 { get_config().registers.phy_mode_reg }
pub fn phy_timing_reg() -> u64 { get_config().registers.phy_timing_reg }
pub fn phy_voltage_reg() -> u64 { get_config().registers.phy_voltage_reg }
pub fn phy_power_reg() -> u64 { get_config().registers.phy_power_reg }
pub fn phy_security_ctrl() -> u64 { get_config().registers.phy_security_ctrl }
pub fn phy_security_status() -> u64 { get_config().registers.phy_security_status }
pub fn memc_ctrl_reg() -> u64 { get_config().registers.memc_ctrl_reg }
pub fn memc_status_reg() -> u64 { get_config().registers.memc_status_reg }
pub fn memc_freq_reg() -> u64 { get_config().registers.memc_freq_reg }
pub fn memc_refresh_reg() -> u64 { get_config().registers.memc_refresh_reg }
pub fn memc_timing_reg() -> u64 { get_config().registers.memc_timing_reg }
pub fn memc_lock_ctrl() -> u64 { get_config().registers.memc_lock_ctrl }
pub fn memc_erase_ctrl() -> u64 { get_config().registers.memc_erase_ctrl }
pub fn memc_debug_ctrl() -> u64 { get_config().registers.memc_debug_ctrl }
pub fn axi_config_reg() -> u64 { get_config().registers.axi_config_reg }
pub fn axi_status_reg() -> u64 { get_config().registers.axi_status_reg }
pub fn ram_base() -> u64 { get_config().registers.ram_base }

pub fn power_base() -> u64 { get_config().registers.power_base }
pub fn vdd_core_reg() -> u64 { get_config().registers.vdd_core }
pub fn vdd_gpu_reg() -> u64 { get_config().registers.vdd_gpu }
pub fn vdd_modem_reg() -> u64 { get_config().registers.vdd_modem }
pub fn vdd_io_reg() -> u64 { get_config().registers.vdd_io }
pub fn refresh_status() -> u64 { get_config().registers.refresh_status }
pub fn refresh_timer() -> u64 { get_config().registers.refresh_timer }
pub fn refresh_interval() -> u64 { get_config().registers.refresh_interval }
pub fn refresh_ctrl() -> u64 { get_config().registers.refresh_ctrl }
pub fn ram_timing_ctrl() -> u64 { get_config().registers.ram_timing_ctrl }

pub fn display_ctrl_base() -> u64 { get_config().registers.display_ctrl_base }
pub fn display_ctrl_reg() -> u64 { get_config().registers.display_ctrl }
pub fn display_status_reg() -> u64 { get_config().registers.display_status }
pub fn display_width_reg() -> u64 { get_config().registers.display_width }
pub fn display_height_reg() -> u64 { get_config().registers.display_height }
pub fn display_mode_reg() -> u64 { get_config().registers.display_mode }
pub fn display_refresh_reg() -> u64 { get_config().registers.display_refresh }
pub fn display_config_reg() -> u64 { get_config().registers.display_config }
pub fn display_data_reg() -> u64 { get_config().registers.display_data }

pub fn screen_base() -> u64 { get_config().registers.screen_base }
pub fn screen_ctrl_reg() -> u64 { get_config().registers.screen_ctrl }
pub fn screen_status_reg() -> u64 { get_config().registers.screen_status }
pub fn screen_width_reg() -> u64 { get_config().registers.screen_width }
pub fn screen_height_reg() -> u64 { get_config().registers.screen_height }
pub fn screen_refresh_reg() -> u64 { get_config().registers.screen_refresh }
pub fn screen_brightness_reg() -> u64 { get_config().registers.screen_brightness }
pub fn screen_config_reg() -> u64 { get_config().registers.screen_config }
pub fn screen_data_reg() -> u64 { get_config().registers.screen_data }

pub fn brightness_base() -> u64 { get_config().registers.brightness_base }
pub fn brightness_ctrl() -> u64 { get_config().registers.brightness_ctrl }
pub fn brightness_status() -> u64 { get_config().registers.brightness_status }
pub fn brightness_level() -> u64 { get_config().registers.brightness_level }
pub fn brightness_min() -> u64 { get_config().registers.brightness_min }
pub fn brightness_max() -> u64 { get_config().registers.brightness_max }
pub fn brightness_config() -> u64 { get_config().registers.brightness_config }
pub fn brightness_mode() -> u64 { get_config().registers.brightness_mode }
pub fn brightness_data() -> u64 { get_config().registers.brightness_data }

pub fn fingerprint_base() -> u64 { get_config().registers.fingerprint_base }
pub fn fingerprint_ctrl() -> u64 { get_config().registers.fingerprint_ctrl }
pub fn fingerprint_status() -> u64 { get_config().registers.fingerprint_status }
pub fn fingerprint_verify() -> u64 { get_config().registers.fingerprint_verify }
pub fn fingerprint_data() -> u64 { get_config().registers.fingerprint_data }
pub fn fingerprint_enroll() -> u64 { get_config().registers.fingerprint_enroll }
pub fn fingerprint_template() -> u64 { get_config().registers.fingerprint_template }
pub fn fingerprint_attempts() -> u64 { get_config().registers.fingerprint_attempts }
pub fn fingerprint_lock() -> u64 { get_config().registers.fingerprint_lock }

pub fn iris_base() -> u64 { get_config().registers.iris_base }
pub fn iris_ctrl() -> u64 { get_config().registers.iris_ctrl }
pub fn iris_status() -> u64 { get_config().registers.iris_status }

pub fn voice_base() -> u64 { get_config().registers.voice_base }
pub fn voice_ctrl() -> u64 { get_config().registers.voice_ctrl }
pub fn voice_status() -> u64 { get_config().registers.voice_status }

pub fn faceid_base() -> u64 { get_config().registers.faceid_base }
pub fn faceid_ctrl() -> u64 { get_config().registers.faceid_ctrl }
pub fn faceid_status() -> u64 { get_config().registers.faceid_status }
pub fn faceid_enroll() -> u64 { get_config().registers.faceid_enroll }
pub fn faceid_verify() -> u64 { get_config().registers.faceid_verify }
pub fn faceid_conf() -> u64 { get_config().registers.faceid_conf }
pub fn faceid_attempts() -> u64 { get_config().registers.faceid_attempts }
pub fn faceid_lock() -> u64 { get_config().registers.faceid_lock }
pub fn faceid_data() -> u64 { get_config().registers.faceid_data }

pub fn camera_ctrl_base() -> u64 { get_config().registers.camera_ctrl_base }
pub fn camera_ctrl() -> u64 { get_config().registers.camera_ctrl }
pub fn camera_status() -> u64 { get_config().registers.camera_status }
pub fn camera_select() -> u64 { get_config().registers.camera_select }
pub fn camera_power() -> u64 { get_config().registers.camera_power }
pub fn camera_reset() -> u64 { get_config().registers.camera_reset }
pub fn camera_config() -> u64 { get_config().registers.camera_config }
pub fn camera_mode() -> u64 { get_config().registers.camera_mode }
pub fn camera_data() -> u64 { get_config().registers.camera_data }

pub fn front_isp_base() -> u64 { get_config().registers.front_isp_base }
pub fn front_isp_ctrl() -> u64 { get_config().registers.front_isp_ctrl }
pub fn front_isp_status() -> u64 { get_config().registers.front_isp_status }
pub fn front_isp_config() -> u64 { get_config().registers.front_isp_config }
pub fn front_isp_resolution() -> u64 { get_config().registers.front_isp_resolution }
pub fn front_isp_frame_rate() -> u64 { get_config().registers.front_isp_frame_rate }
pub fn front_isp_mode() -> u64 { get_config().registers.front_isp_mode }
pub fn front_isp_format() -> u64 { get_config().registers.front_isp_format }
pub fn front_isp_data() -> u64 { get_config().registers.front_isp_data }

pub fn rear_isp_base() -> u64 { get_config().registers.rear_isp_base }
pub fn rear_isp_ctrl() -> u64 { get_config().registers.rear_isp_ctrl }
pub fn rear_isp_status() -> u64 { get_config().registers.rear_isp_status }
pub fn rear_isp_config() -> u64 { get_config().registers.rear_isp_config }
pub fn rear_isp_resolution() -> u64 { get_config().registers.rear_isp_resolution }
pub fn rear_isp_frame_rate() -> u64 { get_config().registers.rear_isp_frame_rate }
pub fn rear_isp_mode() -> u64 { get_config().registers.rear_isp_mode }
pub fn rear_isp_format() -> u64 { get_config().registers.rear_isp_format }
pub fn rear_isp_data() -> u64 { get_config().registers.rear_isp_data }

pub fn flash_ctrl_base() -> u64 { get_config().registers.flash_ctrl_base }
pub fn flash_ctrl() -> u64 { get_config().registers.flash_ctrl }
pub fn flash_status() -> u64 { get_config().registers.flash_status }
pub fn flash_pwm() -> u64 { get_config().registers.flash_pwm }
pub fn flash_brightness() -> u64 { get_config().registers.flash_brightness }
pub fn flash_timing() -> u64 { get_config().registers.flash_timing }
pub fn flash_mode() -> u64 { get_config().registers.flash_mode }
pub fn flash_config() -> u64 { get_config().registers.flash_config }
pub fn flash_data() -> u64 { get_config().registers.flash_data }

pub fn zoom_ctrl_base() -> u64 { get_config().registers.zoom_ctrl_base }
pub fn zoom_ctrl() -> u64 { get_config().registers.zoom_ctrl }
pub fn zoom_status() -> u64 { get_config().registers.zoom_status }
pub fn zoom_level() -> u64 { get_config().registers.zoom_level }
pub fn zoom_max() -> u64 { get_config().registers.zoom_max }
pub fn zoom_min() -> u64 { get_config().registers.zoom_min }
pub fn zoom_config() -> u64 { get_config().registers.zoom_config }
pub fn zoom_mode() -> u64 { get_config().registers.zoom_mode }
pub fn zoom_data() -> u64 { get_config().registers.zoom_data }

pub fn stabilization_ctrl_base() -> u64 { get_config().registers.stabilization_ctrl_base }
pub fn stabilization_ctrl() -> u64 { get_config().registers.stabilization_ctrl }
pub fn stabilization_status() -> u64 { get_config().registers.stabilization_status }
pub fn stabilization_x_offset() -> u64 { get_config().registers.stabilization_x_offset }
pub fn stabilization_y_offset() -> u64 { get_config().registers.stabilization_y_offset }
pub fn stabilization_gain() -> u64 { get_config().registers.stabilization_gain }
pub fn stabilization_config() -> u64 { get_config().registers.stabilization_config }
pub fn stabilization_mode() -> u64 { get_config().registers.stabilization_mode }
pub fn stabilization_data() -> u64 { get_config().registers.stabilization_data }

pub fn depth_ctrl_base() -> u64 { get_config().registers.depth_ctrl_base }
pub fn depth_ctrl() -> u64 { get_config().registers.depth_ctrl }
pub fn depth_status() -> u64 { get_config().registers.depth_status }
pub fn depth_data() -> u64 { get_config().registers.depth_data }
pub fn depth_range() -> u64 { get_config().registers.depth_range }
pub fn depth_accuracy() -> u64 { get_config().registers.depth_accuracy }
pub fn depth_config() -> u64 { get_config().registers.depth_config }
pub fn depth_mode() -> u64 { get_config().registers.depth_mode }
pub fn depth_result() -> u64 { get_config().registers.depth_result }

pub fn nfc_base() -> u64 { get_config().registers.nfc_base }
pub fn nfc_ctrl_reg() -> u64 { get_config().registers.nfc_ctrl_reg }
pub fn nfc_status_reg() -> u64 { get_config().registers.nfc_status_reg }
pub fn nfc_interrupt_reg() -> u64 { get_config().registers.nfc_interrupt_reg }
pub fn nfc_error_reg() -> u64 { get_config().registers.nfc_error_reg }
pub fn nfc_command_reg() -> u64 { get_config().registers.nfc_command_reg }
pub fn nfc_response_reg() -> u64 { get_config().registers.nfc_response_reg }
pub fn nfc_fifo_reg() -> u64 { get_config().registers.nfc_fifo_reg }
pub fn nfc_timeout_reg() -> u64 { get_config().registers.nfc_timeout_reg }
pub fn nfc_config_reg() -> u64 { get_config().registers.nfc_config_reg }
pub fn nfc_mode_reg() -> u64 { get_config().registers.nfc_mode_reg }

pub fn payment_ctrl_reg() -> u64 { get_config().registers.payment_ctrl_reg }
pub fn payment_status_reg() -> u64 { get_config().registers.payment_status_reg }
pub fn payment_amount_reg() -> u64 { get_config().registers.payment_amount_reg }
pub fn payment_currency_reg() -> u64 { get_config().registers.payment_currency_reg }
pub fn payment_security_reg() -> u64 { get_config().registers.payment_security_reg }
pub fn payment_log_reg() -> u64 { get_config().registers.payment_log_reg }
pub fn payment_config_reg() -> u64 { get_config().registers.payment_config_reg }

pub fn reader_config_reg() -> u64 { get_config().registers.reader_config_reg }
pub fn reader_detect_reg() -> u64 { get_config().registers.reader_detect_reg }
pub fn uid_reg() -> u64 { get_config().registers.uid_reg }
pub fn whitelist_reg() -> u64 { get_config().registers.whitelist_reg }

pub fn writer_config_reg() -> u64 { get_config().registers.writer_config_reg }
pub fn writer_erase_reg() -> u64 { get_config().registers.writer_erase_reg }
pub fn write_data_reg() -> u64 { get_config().registers.write_data_reg }
pub fn write_addr_reg() -> u64 { get_config().registers.write_addr_reg }

pub fn audio_base() -> u64 { get_config().registers.audio_base }
pub fn audio_codec_base() -> u64 { get_config().registers.audio_codec_base }
pub fn speaker_base() -> u64 { get_config().registers.speaker_base }
pub fn microphone_base() -> u64 { get_config().registers.microphone_base }
pub fn headphone_jack_base() -> u64 { get_config().registers.headphone_jack_base }
pub fn noise_cancellation_base() -> u64 { get_config().registers.noise_cancellation_base }
pub fn audio_input_base() -> u64 { get_config().registers.audio_input_base }

pub fn gnss_ctrl_base() -> u64 { get_config().registers.gnss_ctrl_base }
pub fn gnss_ctrl() -> u64 { get_config().registers.gnss_ctrl }
pub fn gnss_status() -> u64 { get_config().registers.gnss_status }
pub fn gnss_lat() -> u64 { get_config().registers.gnss_lat }
pub fn gnss_lon() -> u64 { get_config().registers.gnss_lon }
pub fn gnss_alt() -> u64 { get_config().registers.gnss_alt }
pub fn gnss_config() -> u64 { get_config().registers.gnss_config }
pub fn gnss_mode() -> u64 { get_config().registers.gnss_mode }
pub fn gnss_data() -> u64 { get_config().registers.gnss_data }

pub fn geo_ctrl_base() -> u64 { get_config().registers.geo_ctrl_base }
pub fn geo_ctrl() -> u64 { get_config().registers.geo_ctrl }
pub fn geo_status() -> u64 { get_config().registers.geo_status }
pub fn geo_lat() -> u64 { get_config().registers.geo_lat }
pub fn geo_lon() -> u64 { get_config().registers.geo_lon }
pub fn geo_radius() -> u64 { get_config().registers.geo_radius }
pub fn geo_config() -> u64 { get_config().registers.geo_config }
pub fn geo_mode() -> u64 { get_config().registers.geo_mode }
pub fn geo_data() -> u64 { get_config().registers.geo_data }

pub fn loc_ctrl_base() -> u64 { get_config().registers.loc_ctrl_base }
pub fn loc_ctrl() -> u64 { get_config().registers.loc_ctrl }
pub fn loc_status() -> u64 { get_config().registers.loc_status }
pub fn loc_lat() -> u64 { get_config().registers.loc_lat }
pub fn loc_lon() -> u64 { get_config().registers.loc_lon }
pub fn loc_alt() -> u64 { get_config().registers.loc_alt }
pub fn loc_config() -> u64 { get_config().registers.loc_config }
pub fn loc_mode() -> u64 { get_config().registers.loc_mode }
pub fn loc_data() -> u64 { get_config().registers.loc_data }

pub fn bt_base() -> u64 { get_config().registers.bt_base }
pub fn bt_ctrl() -> u64 { get_config().registers.bt_ctrl }
pub fn bt_status() -> u64 { get_config().registers.bt_status }
pub fn bt_freq() -> u64 { get_config().registers.bt_freq }
pub fn bt_band() -> u64 { get_config().registers.bt_band }

pub fn wifi_base() -> u64 { get_config().registers.wifi_base }
pub fn wifi_ctrl() -> u64 { get_config().registers.wifi_ctrl }
pub fn wifi_status() -> u64 { get_config().registers.wifi_status }
pub fn wifi_freq() -> u64 { get_config().registers.wifi_freq }
pub fn wifi_channel() -> u64 { get_config().registers.wifi_channel }

pub fn lte_base() -> u64 { get_config().registers.lte_base }
pub fn fiveg_base() -> u64 { get_config().registers.fiveg_base }
pub fn gsm_base() -> u64 { get_config().registers.gsm_base }
pub fn esim_base() -> u64 { get_config().registers.esim_base }
pub fn sim_base() -> u64 { get_config().registers.sim_base }
pub fn satellite_base() -> u64 { get_config().registers.satellite_base }
pub fn zigbee_base() -> u64 { get_config().registers.zigbee_base }
pub fn thread_base() -> u64 { get_config().registers.thread_base }

pub fn key_storage_base() -> u64 { get_config().registers.key_storage_base }
pub fn key_ctrl() -> u64 { get_config().registers.key_ctrl }
pub fn key_status() -> u64 { get_config().registers.key_status }
pub fn key_addr() -> u64 { get_config().registers.key_addr }
pub fn key_size() -> u64 { get_config().registers.key_size }
pub fn key_config() -> u64 { get_config().registers.key_config }
pub fn key_mode() -> u64 { get_config().registers.key_mode }
pub fn key_lock() -> u64 { get_config().registers.key_lock }
pub fn key_data() -> u64 { get_config().registers.key_data }

pub fn battery_i2c_addr() -> u8 { get_config().registers.battery_i2c_addr }
pub fn battery_reg_voltage() -> u8 { get_config().registers.battery_reg_voltage }
pub fn battery_reg_current() -> u8 { get_config().registers.battery_reg_current }
pub fn battery_reg_soc() -> u8 { get_config().registers.battery_reg_soc }
pub fn battery_reg_status() -> u8 { get_config().registers.battery_reg_status }

pub fn pmic_chg_ctrl() -> u8 { get_config().registers.pmic_chg_ctrl }
pub fn pmic_chg_status() -> u8 { get_config().registers.pmic_chg_status }
pub fn pmic_chg_current() -> u8 { get_config().registers.pmic_chg_current }
pub fn pmic_chg_voltage() -> u8 { get_config().registers.pmic_chg_voltage }
/// Helper pour l'intégration HardwareDriver: réinitialiser un composant par son nom
/// Utilisé par RecoverComponent command dans hardware_pool
pub fn recover_component_by_name(component_name: &str) -> Result<(), String> {
    let _manager = HardwareManager::new();
    
    // Essayer le reinit (via private method, donc on recréé les opérations)
    match component_name {
        "power" => {
            let cfg = get_config();
            if cfg.registers.power_base != 0 { Ok(()) } else { Err("no_power_base".to_string()) }
        },
        "bus" => Ok(()),
        "cpu" => { cpu::cpu_frequency::set(cpu::cpu_frequency::CpuFreqLevel::Medium); Ok(()) },
        "gpu" => gpu::gpu_control::init().map_err(|e| e.to_string()),
        "ram" => ram::ram_control::init().map_err(|e| e.to_string()),
        "display" => display::screen::init_display().map_err(|e| e.to_string()),
        "modem" => { let _ = modem::wifi::init(); Ok(()) },
        "audio" => { let _ = audio::audio_codec::init(); Ok(()) },
        "nfc" => { let _ = nfc::reader::NFCReader::init(); Ok(()) },
        "camera" => { let _ = camera::front_camera::init(); Ok(()) },
        "gps" => gps::gps::enable().map_err(|e| e.to_string()),
        "sensors" => { let _ = sensors::accelerometer::AccelerometerDriver::init(); Ok(()) },
        "biometric" => { let _ = biometric::fingerprint::init(); Ok(()) },
        "thermal" => thermal::thermal_management::init().map_err(|e| e.to_string()),
        "storage" => storage::ufs::init().map_err(|e| e.to_string()),
        _ => Err(format!("unknown_component: {}", component_name)),
    }
}