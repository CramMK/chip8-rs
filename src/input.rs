use sdl2;
use sdl2::{event::Event, keyboard::Keycode};

pub struct Input {
    events: sdl2::EventPump,
}

impl Input {
    pub fn new(sdl_ctx: &sdl2::Sdl) -> Self {
        Input {
            events: sdl_ctx.event_pump().unwrap(),
        }
    }

    pub fn fetch(&mut self) -> Result<[bool; 16], ()> {
        // error if we don't get input
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            }
        }

        // get all inputs
        let input: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // set all no not-pressed as default
        let mut keys = [false; 16];

        // map to internal values
        for key in input {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(pos) = index {
                keys[pos] = true;
            }
        }
        Ok(keys)
    }
}
