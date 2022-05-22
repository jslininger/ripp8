use crate::chipp8::renderer;
use crate::chipp8::keyboard;
use std::fs;
// use minifb;
use rand;

pub struct Cpu {
    pub display: renderer::Renderer,
    pub keyboard: keyboard::Keyboard,
    pc: u16,
    mem: [u8; 4096],
    v: [u8; 16],
    i: u16,
    sound_timer: u8,
    delay_timer: u8,
    stack: Vec<u16>,
    speed: u8,
    paused: bool,
    tostore: usize, 
}

impl Cpu {
    pub fn new(sp: u8) -> Cpu {
        let cpu = Cpu {
            display: renderer::Renderer::new(),
            keyboard: keyboard::Keyboard::new(),
            pc: 0x200,
            mem: [0; 4096],
            v: [0; 16],
            i: 0,
            sound_timer: 0,
            delay_timer: 0,
            stack: Vec::with_capacity(16),
            speed: sp,
            paused: false,
            tostore: 0,
        };
        cpu
    }

    pub fn load_sprites(&mut self) {
        let sprites: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, //0
                                 0x20, 0x60, 0x20, 0x20, 0x70, //1
                                 0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
                                 0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
                                 0x90, 0x90, 0xF0, 0x10, 0x10, //4
                                 0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
                                 0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
                                 0xF0, 0x10, 0x20, 0x40, 0x40, //7
                                 0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
                                 0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
                                 0xF0, 0x90, 0xF0, 0x90, 0x90, //A
                                 0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
                                 0xF0, 0x80, 0x80, 0x80, 0xF0, //C
                                 0xE0, 0x90, 0x90, 0x90, 0xE0, //D
                                 0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
                                 0xF0, 0x80, 0xF0, 0x80, 0x80];//F
        for i in 0..80 {
            self.mem[i] = sprites[i];
        }
    }

    pub fn load_rom(&mut self, rom: &str) {
        let bytes: Vec<u8> = fs::read(rom).expect("file not found");
        for i in 0..bytes.len() {
            if i > 3583 {break;}
            self.mem[0x200 + i] = bytes[i];
        }
    }

    pub fn cycle(&mut self) {
        if self.paused && self.keyboard.is_new_keypress() {
            self.paused = false;
            self.v[self.tostore] = self.keyboard.new_keypress();
        }

        for _i in 0..self.speed {
            if !self.paused {
                let j = self.pc as usize;
                let left: u16 = (self.mem[j] as u16) << 8;
                let right: u16 = self.mem[j + 1] as u16;
                let opcode = left | right;
                self.execute_instruction(opcode);
            }
        }
        if !self.paused{
            self.update_timers();
        }
    }

    fn execute_instruction(&mut self, opcode: u16) {
        // println!("{:#X}", opcode);
        self.pc += 2;
        let most_sig = (opcode & 0xF000) >> 12;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let least_sig = opcode & 0x000F;

        match (most_sig, y, least_sig) {
            (0x0, 0xE, 0x0) => self.display.clear(),
            (0x0, 0xE, 0xE) => self.pc = self.stack.pop().expect("nothing in stack"),
            (0x1, _, _)     => self.pc = opcode & 0x0FFF,
            (0x2, _, _)     => {self.stack.push(self.pc); self.pc = opcode & 0x0FFF}
            (0x3, _, _)     => {if self.v[x as usize] == (opcode & 0x00FF) as u8 {self.pc += 2;}}
            (0x4, _, _)     => {if self.v[x as usize] != (opcode & 0x00FF) as u8 {self.pc += 2;}}
            (0x5, _, 0x0)   => {if self.v[x as usize] == self.v[y as usize] {self.pc += 2;}}
            (0x6, _, _)     => self.v[x as usize] = (opcode & 0x00FF) as u8,
            (0x7, _, _)     => self.v[x as usize] += (opcode & 0x00FF) as u8,
            (0x8, _, 0x0)   => self.v[x as usize] = self.v[y as usize],
            (0x8, _, 0x1)   => self.v[x as usize] = self.v[x as usize] | self.v[y as usize],
            (0x8, _, 0x2)   => self.v[x as usize] = self.v[x as usize] & self.v[y as usize],
            (0x8, _, 0x3)   => self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize],
            (0x8, _, 0x4)   => {
                                self.v[0xF] = 0;
                                let sum: u16 = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                                if sum > 255 {self.v[0xF] = 1;}
                                self.v[x as usize] = (sum & 0xFF) as u8;
                            }
            (0x8, _, 0x5)   => {
                                self.v[0xF] = 0; 
                                if self.v[x as usize] > self.v[y as usize] {self.v[0xF] = 1;} 
                                self.v[x as usize] -= self.v[y as usize];
                            }
            (0x8, _, 0x6)   => {self.v[0xF] = self.v[x as usize] & 0x1; self.v[x as usize] >>= 1;}
            (0x8, _, 0x7)   => {
                                self.v[0xF] = 0; 
                                if self.v[y as usize] > self.v[x as usize] {self.v[0xF] = 1;} 
                                self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                            }
            (0x8, _, 0xE)   => {self.v[0xF] = self.v[x as usize] & 0x80; self.v[x as usize] <<= 1;}
            (0x9, _, 0x0)   => {if self.v[x as usize] != self.v[y as usize] {self.pc += 2;}}
            (0xA, _, _)     => self.i = opcode & 0x0FFF,
            (0xB, _, _)     => self.pc = (self.v[0x0] as u16) + (opcode & 0x0FFF),
            (0xC, _, _)     => self.v[x as usize] = ((opcode | 0x00FF) as u8) & rand::random::<u8>(),
            (0xD, _, _)     => {
                                self.v[0xF] = 0;
                                for byte in 0..least_sig {
                                    let mut sprite = self.mem[(self.i as usize) + byte as usize];
                                    for bit in 0..8 {
                                        if (sprite & 0x80) != 0 {
                                            if self.display.set_pixel(self.v[x as usize] + (bit as u8), self.v[y as usize] + (byte as u8)) 
                                                {self.v[0xF] = 1;}
                                        }
                                        sprite <<= 1;
                                    }
                                }
                            }
            (0xE, 0x9, 0xE) => {if self.keyboard.is_key_pressed(self.v[x as usize]) {self.pc += 2;}}
            (0xE, 0xA, 0x1) => {if !self.keyboard.is_key_pressed(self.v[x as usize]) {self.pc += 2;}}
            (0xF, 0x0, 0x7) => self.v[x as usize] = self.delay_timer,
            (0xF, 0x0, 0xA) => {self.paused = true; self.tostore = x as usize}
            (0xF, 0x1, 0x5) => self.delay_timer = self.v[x as usize],
            (0xF, 0x1, 0x8) => self.sound_timer = self.v[x as usize],
            (0xF, 0x1, 0xE) => self.i += (self.v[x as usize]) as u16,
            (0xF, 0x2, 0x9) => self.i = ((self.v[x as usize]) as u16) * 5,
            (0xF, 0x3, 0x3) => {
                                self.mem[self.i as usize] = self.v[x as usize] / 100; 
                                self.mem[self.i as usize + 1] = (self.v[x as usize] % 100) / 10;
                                self.mem[self.i as usize + 2] = self.v[x as usize] % 10;
                            }
            (0xF, 0x5, 0x5) => {
                                for j in 0..=x as usize {
                                    self.mem[self.i as usize + j] = self.v[j];
                                }
                            }
            (0xF, 0x6, 0x5) => {
                                for j in 0..=x as usize {
                                    self.v[j] = self.mem[self.i as usize + j];
                                }
                            }
            _               => panic!("invalid instruction!"),
        }
    }

    fn update_timers(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}