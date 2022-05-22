pub mod renderer;
pub mod keyboard;
pub mod cpu;
use std::borrow::BorrowMut;
use minifb;
use std::time::Duration;

pub struct Chipp8 {
    pub window: minifb::Window,
    cpu: cpu::Cpu,
}

impl Chipp8 {
    pub fn new(s:minifb::Scale) -> Chipp8 {
        let chipp8 = Chipp8 {
            window: minifb::Window::new("ripp8", 64, 32, minifb::WindowOptions { scale: s, ..minifb::WindowOptions::default()}).unwrap(),
            cpu: cpu::Cpu::new(6),
        };
        // chipp8.init(rom);
        chipp8
    }

    pub fn init(&mut self, rom: &String) {
        self.window.limit_update_rate(Some(Duration::from_micros(16666)));
        self.render();
        self.cpu.load_sprites();
        // self.cpu.load_rom("/home/jslin/Projects/ripp8/roms/PONG2");
        self.cpu.load_rom(rom);
    }

    fn render(&mut self) {
        self.window.update_with_buffer(&self.cpu.display.display.borrow_mut(), 64, 32).unwrap();
    }

    pub fn run(&mut self) {
        self.cpu.keyboard.update(&self.window);
        self.cpu.cycle();

        self.render()

    }
}