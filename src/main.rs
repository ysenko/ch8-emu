mod chip8;

use std::collections::HashMap;
use std::iter;
use std::time::Duration;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::Key;
use winit::window::Window;

const CHIP8_OPS_PER_SECOND: u64 = 600;
const WAIT_DURATION: Duration = Duration::from_micros(1_000_000 / CHIP8_OPS_PER_SECOND);
const WINDOW_TITLE: &str = "Chip8 Emulator";

const KEY_MAP: [(&str, &str); 16] = [
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "c"),
    ("q", "4"),
    ("w", "5"),
    ("e", "6"),
    ("r", "d"),
    ("a", "7"),
    ("s", "8"),
    ("d", "9"),
    ("f", "e"),
    ("z", "a"),
    ("x", "0"),
    ("c", "b"),
    ("v", "f"),
];

#[derive(Debug)]
struct Emulator<'a> {
    system: chip8::Chip8,
    window: Option<Window>,
    key_map: HashMap<&'a str, &'a str>,
}

fn main() {
    let mut ch8 = chip8::Chip8::new();
    ch8.boot().unwrap();

    let event_loop = EventLoop::new().unwrap();
    let control_flow = ControlFlow::wait_duration(WAIT_DURATION);
    event_loop.set_control_flow(control_flow);

    let mut emulator = Emulator {
        system: ch8,
        window: None,
        key_map: HashMap::from_iter(KEY_MAP.iter().cloned()),
    };

    event_loop.run_app(&mut emulator).unwrap();
}

impl Emulator<'_> {
    fn get_mapped_key(&self, pressed_key: &str) -> Option<&str> {
        self.key_map.get(pressed_key).copied()
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.logical_key.as_ref() {
            Key::Character(x) => {
                if key_event.state.is_pressed() {
                    let key = self.get_mapped_key(x);
                    if key.is_some() {
                        let some_key = key.unwrap().to_owned();
                        self.system.press_key(some_key.as_str());
                    } else {
                        println!("Key not supported");
                    }
                } else {
                    self.system.release_key();
                }
            }
            _ => {
                println!("Key not supported");
            }
        }
    }
}

impl ApplicationHandler for Emulator<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes().with_title(WINDOW_TITLE);
        self.window = Some(event_loop.create_window(window_attrs).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => self.handle_key_event(event),
            _ => {}
        }
    }
}
