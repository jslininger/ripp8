pub struct Renderer {
    grid: [u8; 2048],
    pub display: Vec<u32>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let renderer = Renderer {
            grid: [0; 2048],
            display: vec![0; 2048],
        };
        renderer
    }

    pub fn set_pixel(&mut self, x: u8, y: u8) -> bool {
        let mut xi = x as usize;
        let mut yi = y as usize;
        if x >= 64 {
            xi = xi % 64;
        }
        if y >= 32 {
            yi = yi % 32;
        }

        let pixeli = xi + (yi * 64);
        self.grid[pixeli] ^= 1;
        if self.grid[pixeli] == 1 {
            self.display[pixeli] = (255 << 16) | (255 << 8) | 255;
        }
        else {
            self.display[pixeli] = 0;
        }
        !(self.grid[pixeli] == 1)
    }

    pub fn clear(&mut self) {
        self.grid = [0; 2048];
        self.display = vec![0; 2048];
    }
}