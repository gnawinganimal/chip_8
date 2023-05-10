use rand::prelude::*;
use crate::opcode::Op;


pub struct Cpu {
    i:  u16,
    pc: u16,
    m:  [u8; 0x1000],
    v:  [u8; 0x10],
    
    keypad:  (), // TODO
    display: (), // TODO

    stack: [u16; 0x10],
    sp:    u8,

    dt: u8,
    st: u8,
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {

}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            i:  0,
            pc: 0,
            m:  [0; 0x1000],
            v:  [0; 0x10],

            keypad:  (),
            display: (),

            stack: [0; 0x10],
            sp:    0,

            dt: 0,
            st: 0,
        }
    }

    pub fn vx(&self, op: Op) -> &u8 {
        &self.v[op.get_u4(2) as usize]
    }

    pub fn vy(&self, op: Op) -> &u8 {
        &self.v[op.get_u4(1) as usize]
    }

    pub fn vx_mut(&mut self, op: Op) -> &mut u8 {
        &mut self.v[op.get_u4(2) as usize]
    }

    pub fn vy_mut(&mut self, op: Op) -> &mut u8 {
        &mut self.v[op.get_u4(1) as usize]
    }

    pub fn step(&mut self, op: Op) -> Result<()> {
        // match most significant nibble
        match op.get_u4(3) {
            // match least significant byte
            0x0 => match op.get_u8(0) {
                // CLS
                0xE0 => {

                },

                // RET
                0xEE => {

                },

                _ => (),
            }
            
            // JP   addr
            0x1 => {
                self.pc = op.get_u12(0);
            },

            // CALL addr
            0x2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;

                self.pc = op.get_u12(0);
            },

            // SE   Vx, byte
            0x3 => {
                if *self.vx(op) == op.get_u8(0) {
                    self.pc += 2;
                }
            },            

            // SNE  Vx, byte
            0x4 => {
                if *self.vx(op) != op.get_u8(0) {
                    self.pc += 2;
                }
            },

            // SE   Vx, Vy
            0x5 => {
                if self.vx(op) == self.vy(op) {
                    self.pc += 2;
                }
            },

            // LD   Vx, byte
            0x6 => {
                *self.vx_mut(op) = op.get_u8(0);
            },

            // ADD  Vx, byte
            0x7 => {
                *self.vx_mut(op) += op.get_u8(0);
            },

            // match least significant nibble
            0x8 => match op.get_u4(0) {
                // LD   Vx, Vy
                0x0 => {
                    *self.vx_mut(op) = *self.vy(op);
                },

                // OR   Vx, Vy
                0x1 => {
                    *self.vx_mut(op) = self.vx(op) | self.vy(op);
                },

                // AND  Vx, Vy
                0x2 => {
                    *self.vx_mut(op) = self.vx(op) & self.vy(op);
                },

                // XOR  Vx, Vy
                0x3 => {
                    *self.vx_mut(op) = self.vx(op) ^ self.vy(op);
                },

                // ADD  Vx, Vy
                0x4 => {
                    let result = *self.vx(op) as u16 + *self.vy(op) as u16;

                    if result > 255 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }

                    *self.vx_mut(op) = result as u8;
                },

                // SUB  Vx, Vy
                0x5 => {
                    if self.vx(op) > self.vy(op) {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    
                    *self.vx_mut(op) = self.vx(op) - self.vy(op);
                },

                // SHR  Vx, {, Vy}
                0x6 => {
                    if self.vx(op) & 1 == 1 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }

                    *self.vx_mut(op) /= 2;
                },

                // SUBN Vx, Vy
                0x7 => {
                    if self.vx(op) > self.vy(op) {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    
                    *self.vx_mut(op) = *self.vx(op) - *self.vy(op);
                },

                // SHL  Vx {, Vy}
                0xE => {
                    if self.vx(op) & 0b1000_0000 == 0b1000_0000 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }

                    *self.vx_mut(op) *= 2;
                },

                _ => (),
            },

            // SNE  Vx, Vy,
            0x9 => {
                if self.vx(op) != self.vy(op) {
                    self.pc += 2;
                }
            },

            // LD   I, addr
            0xA => {
                self.i = op.get_u12(0);
            },

            // JP   V0, addr
            0xB => {
                self.pc = op.get_u12(0) + self.v[0] as u16;
            },

            // RND  Vx, byte
            0xC => {
                *self.vx_mut(op) = random::<u8>() & op.get_u8(0);
            },

            // DRW  Vx, Vy, nibble
            0xD => {
                
            },

            // match least significant byte
            0xE => match op.get_u8(0) {
                // SKP  Vx
                0x9E => {
                    
                },
            
                // SKNP Vx
                0xA1 => {

                },
            
                _ => (),
            },

            // match least significant byte
            0xF => match op.get_u8(0) {
                // LD   Vx, DT
                0x07 => {
                    *self.vx_mut(op) = self.dt;
                },

                // LD   Vx, K
                0x0A => {
                    
                },

                // LD   DT, Vx
                0x15 => {
                    self.dt = *self.vx(op);
                }

                // LD   ST, Vx
                0x18 => {
                    self.st = *self.vx(op);
                },

                // ADD  I,  Vx
                0x1E => {
                    self.i = self.i + *self.vx(op) as u16;
                },

                // LD   F,  Vx
                0x29 => {
                    
                },

                // LD   B,  Vx
                0x33 => {

                },

                // LD   [I], Vx
                0x55 => {

                },

                // LD   Vx, [I]
                0x65 => {

                },

                _ => (),
            }

            _ => (),
        }
        
        Ok(())
    }

    pub fn exec(&mut self) -> Result<()> { 
        Ok(())
    }
}
