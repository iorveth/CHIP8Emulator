pub struct Ram {
    pub bytes: [u8; 4096]
}

impl Ram {

    fn write_sprites(bytes: &mut [u8;4096]) -> &mut [u8;4096] {
        let sprites: [[u8;5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];
        let mut i = 0;
        for sprite in sprites.iter(){
            for value in sprite {
                bytes[i] = *value;
                i+=1;
            }
        }
        bytes
    }
    pub fn new() -> Ram {
        let mut bytes: [u8;4096] = [0;4096];
        Ram::write_sprites(& mut bytes);
        Ram {bytes}
    }

    pub fn write_byte(&mut self, position: usize, value: u8) {
        self.bytes[position] = value;
    }

    pub fn read_byte(&self, position: usize) -> u8 {
        self.bytes[position]
    }
}