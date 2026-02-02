// visualisation_module/src/capture/input.rs

const MODULE_NAME: &str = "input";
const MODULE_ID: u8 = 3;
const MODULE_VERSION: &str = "1.0";

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use std::time::{Duration, SystemTime};
use crossbeam::queue::SegQueue;
use rdev::{listen, EventType};
use enigo::{Enigo, MouseButton, Key};
use enigo::{KeyboardControllable, MouseControllable};
use crate::error::ModuleError;

#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputEventType,
    pub timestamp: u128,
}

#[derive(Debug, Clone)]
pub enum InputEventType {
    KeyPress { key: String },
    KeyRelease { key: String },
    MouseMove { x: i32, y: i32 },
    MouseClick { button: String, x: i32, y: i32 },
    MouseRelease { button: String, x: i32, y: i32 },
    Scroll { dx: i32, dy: i32 },
}

pub struct InputCapture {
    inner: Arc<InputInner>,
}

struct InputInner {
    running: Mutex<bool>,
    event_buffer: SegQueue<InputEvent>,
    fps: u32,
    last_mouse_x: Mutex<i32>,
    last_mouse_y: Mutex<i32>,
    thread_handle: Mutex<Option<JoinHandle<()>>>,
}

impl InputCapture {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(InputInner {
                running: Mutex::new(false),
                event_buffer: SegQueue::new(),
                fps: 60,
                last_mouse_x: Mutex::new(0),
                last_mouse_y: Mutex::new(0),
                thread_handle: Mutex::new(None),
            }),
        }
    }

    pub async fn start(&self) {
        let inner = Arc::clone(&self.inner);
        *inner.running.lock().await = true;

        let handle = tokio::spawn(async move {
            let inner_clone = Arc::clone(&inner);
            let _ = tokio::task::spawn_blocking(move || {
                let inner_blocking = Arc::clone(&inner_clone);
                let _ = listen(move |event| {
                    if !*inner_blocking.running.blocking_lock() {
                        return;
                    }

                let input_event = match event.event_type {
                    EventType::KeyPress(key) => {
                        Some(InputEvent {
                            event_type: InputEventType::KeyPress {
                                key: format!("{:?}", key),
                            },
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis(),
                        })
                    }
                    EventType::KeyRelease(key) => {
                        Some(InputEvent {
                            event_type: InputEventType::KeyRelease {
                                key: format!("{:?}", key),
                            },
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis(),
                        })
                    }
                    EventType::MouseMove { x, y } => {
                        let mut last_x = inner_blocking.last_mouse_x.blocking_lock();
                        let mut last_y = inner_blocking.last_mouse_y.blocking_lock();
                        *last_x = x as i32;
                        *last_y = y as i32;
                        
                        Some(InputEvent {
                            event_type: InputEventType::MouseMove {
                                x: x as i32,
                                y: y as i32,
                            },
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis(),
                        })
                    }
                    EventType::ButtonPress(button) => {
                        let x = *inner_blocking.last_mouse_x.blocking_lock();
                        let y = *inner_blocking.last_mouse_y.blocking_lock();
                        Some(InputEvent {
                            event_type: InputEventType::MouseClick {
                                button: format!("{:?}", button),
                                x,
                                y,
                            },
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis(),
                        })
                    }
                    EventType::ButtonRelease(button) => {
                        let x = *inner_blocking.last_mouse_x.blocking_lock();
                        let y = *inner_blocking.last_mouse_y.blocking_lock();
                        Some(InputEvent {
                            event_type: InputEventType::MouseRelease {
                                button: format!("{:?}", button),
                                x,
                                y,
                            },
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis(),
                        })
                    }
                    EventType::Wheel { delta_x, delta_y } => {
                        Some(InputEvent {
                            event_type: InputEventType::Scroll {
                                dx: delta_x as i32,
                                dy: delta_y as i32,
                            },
                            timestamp: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis(),
                        })
                    }
                };

                if let Some(evt) = input_event {
                    inner_blocking.event_buffer.push(evt);
                }
                });
            });
        });

        *self.inner.thread_handle.lock().await = Some(handle);
    }

    pub async fn stop(&self) {
        *self.inner.running.lock().await = false;
        
        // Attendre que le thread de listening se termine
        if let Some(h) = self.inner.thread_handle.lock().await.take() {
            let _ = h.await;
        }
    }

    pub fn get_event(&self) -> Option<InputEvent> {
        self.inner.event_buffer.pop()
    }

    pub fn get_current_fps(&self) -> u32 {
        self.inner.fps
    }

    pub fn get_event_delay(&self) -> Duration {
        Duration::from_millis(1000 / self.inner.fps as u64)
    }

    // === Contrôle d'entrée ===
    pub fn send_key_press(&self, key: &str) -> Result<(), ModuleError> {
        // Validation: max 50 caractères et alphanumeric
        if key.is_empty() || key.len() > 50 {
            return Err(ModuleError::ValidationError("Key length must be 1-50 characters".to_string()));
        }
        
        if !key.chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '-')) {
            return Err(ModuleError::ValidationError("Key contains invalid characters".to_string()));
        }

        let mut enigo = Enigo::new();
        
        // enigo 0.1: utiliser key_click avec Key enum
        match key.to_lowercase().as_str() {
            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" |
            "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" |
            "u" | "v" | "w" | "x" | "y" | "z" => {
                let ch = key.chars().next().unwrap();
                enigo.key_click(Key::Layout(ch));
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                let ch = key.chars().next().unwrap();
                enigo.key_click(Key::Layout(ch));
            }
            "return" | "enter" => {
                enigo.key_click(Key::Return);
            }
            "space" => {
                enigo.key_click(Key::Space);
            }
            _ => {}
        }
        
        eprintln!("[{}:{}] Key pressed: {} (v{})", MODULE_NAME, MODULE_ID, key, MODULE_VERSION);
        Ok(())
    }

    pub fn send_text(&self, text: &str) -> Result<(), ModuleError> {
        // Validation: max 10000 caractères
        if text.is_empty() || text.len() > 10000 {
            return Err(ModuleError::ValidationError("Text length must be 1-10000 characters".to_string()));
        }

        let mut enigo = Enigo::new();
        for ch in text.chars() {
            enigo.key_click(Key::Layout(ch));
        }
        
        eprintln!("[{}] Text sent: {} chars (v{})", MODULE_NAME, text.len(), MODULE_VERSION);
        Ok(())
    }

    pub fn move_mouse(&self, x: i32, y: i32) -> Result<(), ModuleError> {
        // Validation: coordonnées raisonnables (dans limites écran typiques)
        if x < -10000 || x > 20000 || y < -10000 || y > 20000 {
            return Err(ModuleError::ValidationError("Mouse coordinates out of reasonable bounds".to_string()));
        }

        let mut enigo = Enigo::new();
        enigo.mouse_move_to(x, y);
        
        eprintln!("[{}] Mouse moved to ({}, {}) (v{})", MODULE_NAME, x, y, MODULE_VERSION);
        Ok(())
    }

    pub fn click_mouse(&self, button: &str) -> Result<(), ModuleError> {
        // Validation: seulement les boutons valides
        if !["left", "right", "middle"].contains(&button.to_lowercase().as_str()) {
            return Err(ModuleError::ValidationError("Button must be: left, right, or middle".to_string()));
        }

        let mut enigo = Enigo::new();
        match button.to_lowercase().as_str() {
            "left" => enigo.mouse_click(MouseButton::Left),
            "right" => enigo.mouse_click(MouseButton::Right),
            "middle" => enigo.mouse_click(MouseButton::Middle),
            _ => {}
        };
        
        eprintln!("[{}] Mouse clicked: {} (v{})", MODULE_NAME, button, MODULE_VERSION);
        Ok(())
    }

    pub fn scroll_mouse(&self, dx: i32, dy: i32) -> Result<(), ModuleError> {
        // Validation: déltas raisonnables
        if dx.abs() > 1000 || dy.abs() > 1000 {
            return Err(ModuleError::ValidationError("Scroll delta must be -1000 to 1000".to_string()));
        }

        let mut enigo = Enigo::new();
        // enigo 0.1: utiliser scroll si disponible
        if dy != 0 {
            enigo.mouse_scroll_y(dy);
        }
        if dx != 0 {
            enigo.mouse_scroll_x(dx);
        }
        
        eprintln!("[{}] Mouse scrolled: ({}, {}) (v{})", MODULE_NAME, dx, dy, MODULE_VERSION);
        Ok(())
    }
}
