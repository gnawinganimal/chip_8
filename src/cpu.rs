
pub struct Cpu {

}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {

}

impl Cpu {
    pub fn new() -> Self {
        Cpu {

        }
    }

    pub fn step(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn exec(&mut self) -> Result<()> {        
        Ok(())
    }
}
