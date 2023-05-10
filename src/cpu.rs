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
        }
    }

    pub fn step(&mut self, opcode: u16) -> Result<()> {
        let op = Op::from(opcode);

        // match most significant nibble
        match op.get_u4(3) {
            
            // JP   addr
            0x1 => {
                
            },

            // CALL addr
            0x2 => {

            },

            // SE   Vx, byte
            0x3 => {

            },            

            // SNE  Vx, byte
            0x4 => {

            },

            // SE   Vx, Vy
            0x5 => {

            },

            // LD   Vx, byte
            0x6 => {

            },

            // ADD  Vx, byte
            0x7 => {

            },

            // match least significant nibble
            0x8 => match op.get_u4(0) {
                // LD   Vx, Vy
                0x0 => {

                },

                // OR   Vx, Vy
                0x1 => {

                },

                // AND  Vx, Vy
                0x2 => {

                },

                // XOR  Vx, Vy
                0x3 => {
                    
                },

                // ADD  Vx, Vy
                0x4 => {
                    
                },

                // SUB  Vx, Vy
                0x5 => {
                    
                },

                // SHR  Vx, {, Vy}
                0x6 => {
                    
                },

                // SUBN Vx, Vy
                0x7 => {
                    
                },

                // SHL  Vx {, Vy}
                0xE => {
                    
                },

                _ => (),
            },

            // SNE  Vx, Vy,
            0x9 => {

            },

            // LD   I, addr
            0xA => {

            },

            // JP   V0, addr
            0xB => {

            },

            // RND  Vx, byte
            0xC => {

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

                },

                // LD   Vx, K
                0x0A => {

                },

                // LD   DT, Vx
                0x15 => {

                }

                // LD   ST, Vx
                0x18 => {

                },

                // ADD  I,  Vx
                0x1E => {

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
