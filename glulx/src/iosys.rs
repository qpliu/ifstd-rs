pub const NULL: u32 = 0;
pub const FILTER: u32 = 1;
pub const GLK: u32 = 2;

pub struct IOSys {
    mode: u32,
    rock: u32,
}

impl IOSys {
    pub fn new() -> Self {
        IOSys{ mode: 0, rock: 0 }
    }

    pub fn supported(&self, mode: u32) -> bool {
        match mode {
            NULL | FILTER | GLK => true,
            _ => false,
        }
    }

    pub fn get(&self) -> (u32,u32) {
        (self.mode,self.rock)
    }

    pub fn set(&mut self, mode: u32, rock: u32) {
        match mode {
            NULL | FILTER | GLK => {
                self.mode = mode;
                self.rock = rock;
            },
            _ => {
                self.mode = NULL;
                self.rock = rock;
            },
        }
    }
}
