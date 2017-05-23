use glk::Glk;

use super::call;
use super::state::{read_u16,read_u32,write_u16,write_u32,State};
use super::execute::Execute;

#[derive(Clone,Copy,Debug)]
pub enum Mode {
    Const0,
    Const8,
    Const16,
    Const32,
    Mem8,
    Mem16,
    Mem32,
    Stack,
    Local8,
    Local16,
    Local32,
    Ram8,
    Ram16,
    Ram32,
}

fn to_mode(b: u8) -> Mode {
    match b {
        0 => Mode::Const0,
        1 => Mode::Const8,
        2 => Mode::Const16,
        3 => Mode::Const32,
        5 => Mode::Mem8,
        6 => Mode::Mem16,
        7 => Mode::Mem32,
        8 => Mode::Stack,
        9 => Mode::Local8,
        10 => Mode::Local16,
        11 => Mode::Local32,
        13 => Mode::Ram8,
        14 => Mode::Ram16,
        15 => Mode::Ram32,
        _ => panic!("Unrecognized operand address mode {:x}", b),
    }
}

pub fn next_mode(state: &mut State) -> (Mode,Mode) {
    let b = state.mem[state.pc];
    state.pc += 1;
    (to_mode(b & 0xf),to_mode(b >> 4))
}

impl Mode {
    pub fn load<'a,G: Glk<'a>>(self, exec: &mut Execute<'a,G>) -> u32 {
        match self {
            Mode::Const0 => 0,
            Mode::Const8 => {
                let val = exec.state.mem[exec.state.pc];
                exec.state.pc += 1;
                val as i8 as i32 as u32
            },
            Mode::Const16 => {
                let val = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                val as i16 as i32 as u32
            },
            Mode::Const32 => {
                let val = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                val
            },
            Mode::Mem8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                read_u32(&exec.state.mem, addr)
            },
            Mode::Mem16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                read_u32(&exec.state.mem, addr)
            },
            Mode::Mem32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                read_u32(&exec.state.mem, addr)
            },
            Mode::Stack => exec.state.stack.pop().unwrap(),
            Mode::Local8 => {
                let offset = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4]
            },
            Mode::Local16 => {
                let offset = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4]
            },
            Mode::Local32 => {
                let offset = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4]
            },
            Mode::Ram8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                read_u32(&exec.state.mem, addr & 0xffffffff)
            },
            Mode::Ram16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                read_u32(&exec.state.mem, addr & 0xffffffff)
            },
            Mode::Ram32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                read_u32(&exec.state.mem, addr & 0xffffffff)
            },
        }
    }

    pub fn store<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>, val: u32) {
        match self {
            &Mode::Const0 => (),
            &Mode::Mem8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                write_u32(&mut exec.state.mem, addr, val);
            },
            &Mode::Mem16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                write_u32(&mut exec.state.mem, addr, val);
            },
            &Mode::Mem32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                write_u32(&mut exec.state.mem, addr, val);
            },
            &Mode::Stack => exec.state.stack.push(val),
            &Mode::Local8 => {
                let offset = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4] = val;
            },
            &Mode::Local16 => {
                let offset = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4] = val;
            },
            &Mode::Local32 => {
                let offset = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4] = val;
            },
            &Mode::Ram8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                write_u32(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            &Mode::Ram16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                write_u32(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            &Mode::Ram32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                write_u32(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            _ => panic!("Invalid store operand addr mode {:?} before pc {:x}", self, exec.state.pc),
        }
    }

    pub fn load8<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>) -> u32 {
        match self {
            &Mode::Const0 => 0,
            &Mode::Const8 => {
                let val = exec.state.mem[exec.state.pc];
                exec.state.pc += 1;
                val as u32
            },
            &Mode::Const16 => {
                let val = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                val as u32
            },
            &Mode::Const32 => {
                let val = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                val
            },
            &Mode::Mem8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                exec.state.mem[addr] as u32
            },
            &Mode::Mem16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                exec.state.mem[addr] as u32
            },
            &Mode::Mem32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                exec.state.mem[addr] as u32
            },
            &Mode::Stack => exec.state.stack.pop().unwrap(),
            &Mode::Local8 | &Mode::Local16 | &Mode::Local32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            &Mode::Ram8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                exec.state.mem[addr & 0xffffffff] as u32
            },
            &Mode::Ram16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                exec.state.mem[addr & 0xffffffff] as u32
            },
            &Mode::Ram32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                exec.state.mem[addr & 0xffffffff] as u32
            },
        }
    }

    pub fn store8<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>, val: u32) {
        match self {
            &Mode::Const0 => (),
            &Mode::Mem8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                exec.state.mem[addr] = val as u8;
            },
            &Mode::Mem16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                exec.state.mem[addr] = val as u8;
            },
            &Mode::Mem32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                exec.state.mem[addr] = val as u8;
            },
            &Mode::Stack => exec.state.stack.push(val),
            &Mode::Local8 | &Mode::Local16 | &Mode::Local32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            &Mode::Ram8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                exec.state.mem[addr & 0xffffffff] = val as u8;
            },
            &Mode::Ram16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                exec.state.mem[addr & 0xffffffff] = val as u8;
            },
            &Mode::Ram32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                exec.state.mem[addr & 0xffffffff] = val as u8;
            },
            _ => panic!("Invalid store operand addr mode {:?} before pc {:x}", self, exec.state.pc),
        }
    }

    pub fn load16<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>) -> u32 {
        match self {
            &Mode::Const0 => 0,
            &Mode::Const8 => {
                let val = exec.state.mem[exec.state.pc];
                exec.state.pc += 1;
                val as i8 as i16 as u32
            },
            &Mode::Const16 => {
                let val = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                val as u32
            },
            &Mode::Const32 => {
                let val = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                val
            },
            &Mode::Mem8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                read_u16(&exec.state.mem, addr)
            },
            &Mode::Mem16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                read_u16(&exec.state.mem, addr)
            },
            &Mode::Mem32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                read_u16(&exec.state.mem, addr)
            },
            &Mode::Stack => exec.state.stack.pop().unwrap(),
            &Mode::Local8 | &Mode::Local16 | &Mode::Local32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            &Mode::Ram8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                read_u16(&exec.state.mem, addr & 0xffffffff)
            },
            &Mode::Ram16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                read_u16(&exec.state.mem, addr & 0xffffffff)
            },
            &Mode::Ram32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                read_u16(&exec.state.mem, addr & 0xffffffff)
            },
        }
    }

    pub fn store16<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>, val: u32) {
        match self {
            &Mode::Const0 => (),
            &Mode::Mem8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                write_u16(&mut exec.state.mem, addr, val);
            },
            &Mode::Mem16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                write_u16(&mut exec.state.mem, addr, val);
            },
            &Mode::Mem32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                write_u16(&mut exec.state.mem, addr, val);
            },
            &Mode::Stack => exec.state.stack.push(val),
            &Mode::Local8 | &Mode::Local16 | &Mode::Local32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            &Mode::Ram8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                write_u16(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            &Mode::Ram16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                write_u16(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            &Mode::Ram32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                write_u16(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            _ => panic!("Invalid store operand addr mode {:?} before pc {:x}", self, exec.state.pc),
        }
    }

    pub fn result_dest<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>) -> (u32,u32) {
        match self {
            &Mode::Const0 => (call::DISCARD, 0),
            &Mode::Mem8 => {
                let addr = exec.state.mem[exec.state.pc] as u32;
                exec.state.pc += 1;
                (call::MEM,addr)
            },
            &Mode::Mem16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                (call::MEM,addr)
            },
            &Mode::Mem32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                (call::MEM,addr)
            },
            &Mode::Stack => (call::STACK,0),
            &Mode::Local8 => {
                let offset = exec.state.mem[exec.state.pc] as u32;
                exec.state.pc += 1;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                (call::LOCAL,offset)
            },
            &Mode::Local16 => {
                let offset = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                (call::LOCAL,offset)
            },
            &Mode::Local32 => {
                let offset = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                (call::LOCAL,offset)
            },
            &Mode::Ram8 => {
                let addr = exec.state.mem[exec.state.pc] as u32 + exec.ram_start as u32;
                exec.state.pc += 1;
                (call::MEM,addr)
            },
            &Mode::Ram16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) + exec.ram_start as u32;
                exec.state.pc += 2;
                (call::MEM,addr)
            },
            &Mode::Ram32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) + exec.ram_start as u32;
                exec.state.pc += 4;
                (call::MEM,addr)
            },
            _ => panic!("{:x}: invalid store operand addr mode {:?}", exec.state.pc, self),
        }
    }
}
