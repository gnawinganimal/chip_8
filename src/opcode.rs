
#[derive(Clone, Copy, Debug)]
pub struct Op(u16);

impl Op {
    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn get_u4(&self, n: u8) -> u8 {
        (self.0 >> (n * 4)) as u8 & 0x0F
    }

    pub fn get_u8(&self, n: u8) -> u8 {
        (self.0 >> (n * 4)) as u8
    }

    pub fn get_u12(&self, n: u8) -> u16 {
        (self.0 >> (n * 4)) & 0x0FFF
    }
}

impl From<u16> for Op {
    fn from(opcode: u16) -> Self {
        Op(opcode)
    }
}
