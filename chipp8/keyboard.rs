use minifb::*;
use std::collections::HashSet;

pub struct Keyboard {
    keys: [Key; 16],
    keystate: HashSet<Key>,
    isnewkey: bool,
    newkey: Key,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [Key::Key0,Key::Key1,Key::Key2,Key::Key3,Key::Key4,Key::Key5,Key::Key6,Key::Key7,Key::Key8,Key::Key9,Key::A,Key::B,Key::C,Key::D,Key::E,Key::F],
            keystate: HashSet::new(),
            isnewkey: false,
            newkey: Key::Key0,
        }
    }

    pub fn update(&mut self, window: &Window) {
        self.isnewkey = false;

        self.process_key(window.is_key_down(Key::Key0), Key::Key0);
        self.process_key(window.is_key_down(Key::Key1), Key::Key1);
        self.process_key(window.is_key_down(Key::Key2), Key::Key2);
        self.process_key(window.is_key_down(Key::Key3), Key::Key3);
        self.process_key(window.is_key_down(Key::Key4), Key::Key4);
        self.process_key(window.is_key_down(Key::Key5), Key::Key5);
        self.process_key(window.is_key_down(Key::Key6), Key::Key6);
        self.process_key(window.is_key_down(Key::Key7), Key::Key7);
        self.process_key(window.is_key_down(Key::Key8), Key::Key8);
        self.process_key(window.is_key_down(Key::Key9), Key::Key9);

        self.process_key(window.is_key_down(Key::A), Key::A);
        self.process_key(window.is_key_down(Key::B), Key::B);
        self.process_key(window.is_key_down(Key::C), Key::C);
        self.process_key(window.is_key_down(Key::D), Key::D);
        self.process_key(window.is_key_down(Key::E), Key::E);
        self.process_key(window.is_key_down(Key::F), Key::F);
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keystate.contains(&self.keys[key as usize])
    }

    pub fn is_new_keypress(&self) -> bool {
        self.isnewkey
    }

    pub fn new_keypress(&self) -> u8 {
        match self.newkey {
            Key::Key0 => 0x0,
            Key::Key1 => 0x1,
            Key::Key2 => 0x2,
            Key::Key3 => 0x3,
            Key::Key4 => 0x4,
            Key::Key5 => 0x5,
            Key::Key6 => 0x6,
            Key::Key7 => 0x7,
            Key::Key8 => 0x8,
            Key::Key9 => 0x9,
            Key::A    => 0xA,
            Key::B    => 0xB,
            Key::C    => 0xC,
            Key::D    => 0xD,
            Key::E    => 0xE,
            Key::F    => 0xF,
            _         => panic!("invalid key input"),
        }
    }

    fn process_key(&mut self, pressed: bool, key: Key) {
        if pressed {
            if !self.keystate.contains(&key) {
                self.isnewkey = true;
                self.newkey = key;
            }
            self.keystate.insert(key);
        }
        else if self.keystate.contains(&key) {
            self.keystate.remove(&key);
        }
    }
}