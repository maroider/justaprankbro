#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cursor;

use cursor::{Cursor, CursorKind};

fn main() {
    let cursor = Cursor::from_file(r"normal.cur")
        .unwrap()
        .replace_system(CursorKind::Normal);

    let event_loop = winit::event_loop::EventLoop::new();
    let _window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let mut unlock_sequence = KeySequence::default();

    event_loop.run(move |event, _, control_flow| {
        use winit::{
            event::{DeviceEvent, ElementState, Event},
            event_loop::ControlFlow,
        };

        match event {
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::Key(keyboard_input) = event {
                    if let Some(keycode) = keyboard_input.virtual_keycode {
                        if keyboard_input.state == ElementState::Pressed
                            && unlock_sequence.process_input(keycode)
                        {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
            }
            Event::LoopDestroyed => {
                let _ = &cursor;
            }
            _ => {}
        }
    });
}

#[derive(Debug)]
struct KeySequence {
    keys: Vec<winit::event::VirtualKeyCode>,
    idx: usize,
}

impl KeySequence {
    fn process_input(&mut self, keycode: winit::event::VirtualKeyCode) -> bool {
        if self.keys[self.idx] == keycode {
            if self.idx == self.keys.len() - 1 {
                true
            } else {
                self.idx += 1;
                false
            }
        } else {
            self.idx = 0;
            false
        }
    }
}

impl Default for KeySequence {
    fn default() -> Self {
        use winit::event::VirtualKeyCode::*;

        Self {
            keys: vec![J, U, S, T, A, P, R, A, N, K, B, R, O],
            idx: 0,
        }
    }
}
