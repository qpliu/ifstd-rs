use glk::Glk;

use super::call;
use super::state::{read_u16,read_u32,write_u16,write_u32,State};
use super::execute::Execute;

#[derive(Clone,Copy,Debug)]
pub struct Mode(u8);

const CONST0: u8 = 0;
const CONST8: u8 = 1;
const CONST16: u8 = 2;
const CONST32: u8 = 3;
const MEM8: u8 = 5;
const MEM16: u8 = 6;
const MEM32: u8 = 7;
const STACK: u8 = 8;
const LOCAL8: u8 = 9;
const LOCAL16: u8 = 10;
const LOCAL32: u8 = 11;
const RAM8: u8 = 13;
const RAM16: u8 = 14;
const RAM32: u8 = 15;

pub fn next_mode(state: &mut State) -> (Mode,Mode) {
    let b = state.mem[state.pc];
    state.pc += 1;
    (Mode(b & 0xf),Mode(b >> 4))
}

impl Mode {
    pub fn load<'a,G: Glk<'a>>(self, exec: &mut Execute<'a,G>) -> u32 {
        match self.0 {
            CONST0 => 0,
            CONST8 => {
                let val = exec.state.mem[exec.state.pc];
                exec.state.pc += 1;
                val as i8 as i32 as u32
            },
            CONST16 => {
                let val = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                val as i16 as i32 as u32
            },
            CONST32 => {
                let val = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                val
            },
            MEM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                read_u32(&exec.state.mem, addr)
            },
            MEM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                read_u32(&exec.state.mem, addr)
            },
            MEM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                read_u32(&exec.state.mem, addr)
            },
            STACK => exec.state.stack.pop().unwrap(),
            LOCAL8 => {
                let offset = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4]
            },
            LOCAL16 => {
                let offset = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4]
            },
            LOCAL32 => {
                let offset = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4]
            },
            RAM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                read_u32(&exec.state.mem, addr & 0xffffffff)
            },
            RAM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                read_u32(&exec.state.mem, addr & 0xffffffff)
            },
            RAM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                read_u32(&exec.state.mem, addr & 0xffffffff)
            },
            _ => panic!("Unrecognized operand address mode {:x}", self.0),
        }
    }

    pub fn store<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>, val: u32) {
        match self.0 {
            CONST0 => (),
            MEM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                write_u32(&mut exec.state.mem, addr, val);
            },
            MEM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                write_u32(&mut exec.state.mem, addr, val);
            },
            MEM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                write_u32(&mut exec.state.mem, addr, val);
            },
            STACK => exec.state.stack.push(val),
            LOCAL8 => {
                let offset = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4] = val;
            },
            LOCAL16 => {
                let offset = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4] = val;
            },
            LOCAL32 => {
                let offset = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                exec.state.stack[exec.frame_locals + offset/4] = val;
            },
            RAM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                write_u32(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            RAM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                write_u32(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            RAM32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                write_u32(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            _ => panic!("Invalid store operand addr mode {:x} before pc {:x}", self.0, exec.state.pc),
        }
    }

    pub fn load8<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>) -> u32 {
        match self.0 {
            CONST0 => 0,
            CONST8 => {
                let val = exec.state.mem[exec.state.pc];
                exec.state.pc += 1;
                val as u32
            },
            CONST16 => {
                let val = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                val as u32
            },
            CONST32 => {
                let val = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                val
            },
            MEM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                exec.state.mem[addr] as u32
            },
            MEM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                exec.state.mem[addr] as u32
            },
            MEM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                exec.state.mem[addr] as u32
            },
            STACK => exec.state.stack.pop().unwrap(),
            LOCAL8 | LOCAL16 | LOCAL32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            RAM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                exec.state.mem[addr & 0xffffffff] as u32
            },
            RAM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                exec.state.mem[addr & 0xffffffff] as u32
            },
            RAM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                exec.state.mem[addr & 0xffffffff] as u32
            },
            _ => panic!("Invalid store operand addr mode {:x} before pc {:x}", self.0, exec.state.pc),
        }
    }

    pub fn store8<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>, val: u32) {
        match self.0 {
            CONST0 => (),
            MEM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                exec.state.mem[addr] = val as u8;
            },
            MEM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                exec.state.mem[addr] = val as u8;
            },
            MEM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                exec.state.mem[addr] = val as u8;
            },
            STACK => exec.state.stack.push(val),
            LOCAL8 | LOCAL16 | LOCAL32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            RAM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                exec.state.mem[addr & 0xffffffff] = val as u8;
            },
            RAM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                exec.state.mem[addr & 0xffffffff] = val as u8;
            },
            RAM32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                exec.state.mem[addr & 0xffffffff] = val as u8;
            },
            _ => panic!("Invalid store operand addr mode {:x} before pc {:x}", self.0, exec.state.pc),
        }
    }

    pub fn load16<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>) -> u32 {
        match self.0 {
            CONST0 => 0,
            CONST8 => {
                let val = exec.state.mem[exec.state.pc];
                exec.state.pc += 1;
                val as i8 as i16 as u32
            },
            CONST16 => {
                let val = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                val as u32
            },
            CONST32 => {
                let val = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                val
            },
            MEM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                read_u16(&exec.state.mem, addr)
            },
            MEM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                read_u16(&exec.state.mem, addr)
            },
            MEM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                read_u16(&exec.state.mem, addr)
            },
            STACK => exec.state.stack.pop().unwrap(),
            LOCAL8 | LOCAL16 | LOCAL32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            RAM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                read_u16(&exec.state.mem, addr & 0xffffffff)
            },
            RAM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                read_u16(&exec.state.mem, addr & 0xffffffff)
            },
            RAM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                read_u16(&exec.state.mem, addr & 0xffffffff)
            },
            _ => panic!("Invalid store operand addr mode {:x} before pc {:x}", self.0, exec.state.pc),
        }
    }

    pub fn store16<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>, val: u32) {
        match self.0 {
            CONST0 => (),
            MEM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize;
                exec.state.pc += 1;
                write_u16(&mut exec.state.mem, addr, val);
            },
            MEM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 2;
                write_u16(&mut exec.state.mem, addr, val);
            },
            MEM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc) as usize;
                exec.state.pc += 4;
                write_u16(&mut exec.state.mem, addr, val);
            },
            STACK => exec.state.stack.push(val),
            LOCAL8 | LOCAL16 | LOCAL32 =>
                panic!("{:x}:non 32-bit stack locals deprecated", exec.state.pc),
            RAM8 => {
                let addr = exec.state.mem[exec.state.pc] as usize + exec.ram_start;
                exec.state.pc += 1;
                write_u16(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            RAM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 2;
                write_u16(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            RAM32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) as usize + exec.ram_start;
                exec.state.pc += 4;
                write_u16(&mut exec.state.mem, addr & 0xffffffff, val);
            },
            _ => panic!("Invalid store operand addr mode {:x} before pc {:x}", self.0, exec.state.pc),
        }
    }

    pub fn result_dest<'a,G: Glk<'a>>(&self, exec: &mut Execute<'a,G>) -> (u32,u32) {
        match self.0 {
            CONST0 => (call::DISCARD, 0),
            MEM8 => {
                let addr = exec.state.mem[exec.state.pc] as u32;
                exec.state.pc += 1;
                (call::MEM,addr)
            },
            MEM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                (call::MEM,addr)
            },
            MEM32 => {
                let addr = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                (call::MEM,addr)
            },
            STACK => (call::STACK,0),
            LOCAL8 => {
                let offset = exec.state.mem[exec.state.pc] as u32;
                exec.state.pc += 1;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                (call::LOCAL,offset)
            },
            LOCAL16 => {
                let offset = read_u16(&exec.state.mem, exec.state.pc);
                exec.state.pc += 2;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                (call::LOCAL,offset)
            },
            LOCAL32 => {
                let offset = read_u32(&exec.state.mem, exec.state.pc);
                exec.state.pc += 4;
                assert!(offset % 4 == 0, "{:x}:non 32-bit stack locals deprecated", exec.state.pc);
                (call::LOCAL,offset)
            },
            RAM8 => {
                let addr = exec.state.mem[exec.state.pc] as u32 + exec.ram_start as u32;
                exec.state.pc += 1;
                (call::MEM,addr)
            },
            RAM16 => {
                let addr = read_u16(&exec.state.mem, exec.state.pc) + exec.ram_start as u32;
                exec.state.pc += 2;
                (call::MEM,addr)
            },
            RAM32 => {
                let addr = read_u32(&mut exec.state.mem, exec.state.pc) + exec.ram_start as u32;
                exec.state.pc += 4;
                (call::MEM,addr)
            },
            _ => panic!("{:x}: invalid store operand addr mode {:x}", exec.state.pc, self.0),
        }
    }
}
