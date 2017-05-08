use super::call;
use super::state::read_u32;
use super::execute::Execute;

const NULL: u32 = 0;
const FILTER: u32 = 1;
const GLK: u32 = 2;

#[derive(Debug)]
enum Mode {
    Null,
    Filter,
    Glk,
}

pub struct IOSys {
    mode: Mode,
    rock: u32,
}

impl IOSys {
    pub fn new() -> Self {
        IOSys{ mode: Mode::Null, rock: 0 }
    }

    pub fn get(&self) -> (u32,u32) {
        let mode = match self.mode {
            Mode::Null => NULL,
            Mode::Filter => FILTER,
            Mode::Glk => GLK,
        };
        (mode, self.rock)
    }

    pub fn set(&mut self, mode: u32, rock: u32) {
        self.mode = match mode {
            NULL => Mode::Null,
            FILTER => Mode::Filter,
            GLK => Mode::Glk,
            _ => Mode::Null,
        };
        self.rock = rock;
    }
}

pub fn supported(mode: u32) -> bool {
    match mode {
        NULL | FILTER | GLK => true,
        _ => false,
    }
}

pub fn streamchar(exec: &mut Execute, val: u8) {
    match exec.iosys.mode {
        Mode::Null => (),
        Mode::Filter => {
            call::push_stub(&mut exec.state, call::DISCARD, 0);
            exec.call_tmp.clear();
            exec.call_tmp.push(val as u32);
            let addr = exec.iosys.rock as usize;
            call::call(exec, addr);
        },
        Mode::Glk => unimplemented!(),
    }
}

pub fn streamunichar(exec: &mut Execute, val: u32) {
    match exec.iosys.mode {
        Mode::Null => (),
        Mode::Filter => {
            call::push_stub(&mut exec.state, call::DISCARD, 0);
            exec.call_tmp.clear();
            exec.call_tmp.push(val);
            let addr = exec.iosys.rock as usize;
            call::call(exec, addr);
        },
        Mode::Glk => unimplemented!(),
    }
}

pub fn streamnum(exec: &mut Execute, val: i32) {
    match exec.iosys.mode {
        Mode::Null => (),
        Mode::Filter => {
            call::push_stub(&mut exec.state, call::RESUME_CODE, 0);
            exec.state.pc = val as usize;
            call::push_stub(&mut exec.state, call::RESUME_NUM, 0);
            call::ret(exec, 0);
        },
        Mode::Glk => unimplemented!(),
    }
}

pub fn streamstr(exec: &mut Execute, addr: usize) {
    let (dest_type,pc) = match exec.state.mem[addr] {
        call::STRING_E0 => (call::RESUME_E0,addr+1),
        call::STRING_E1 => {
            assert_eq!(read_u32(&exec.state.mem, addr), 0xe1000000, "{:x}: invalid padding for E1 string object", addr);
            (call::RESUME_E1,addr+4)
        },
        call::STRING_E2 => (call::RESUME_E2,addr+1),
        b => panic!("{:x}: unknown string type {:x}", addr, b),
    };
    match exec.iosys.mode {
        Mode::Null => unimplemented!(),
        Mode::Filter => {
            call::push_stub(&mut exec.state, call::RESUME_CODE, 0);
            exec.state.pc = pc;
            call::push_stub(&mut exec.state, dest_type, 0);
            call::ret(exec, 0);
        },
        Mode::Glk => unimplemented!(),
    }
}

pub fn save(exec: &Execute, outstream: u32) -> bool {
    match exec.iosys.mode {
        Mode::Null | Mode::Filter => panic!("{:x}: invalid IO System {:?} for save/restore", exec.state.pc, exec.iosys.mode),
        Mode::Glk => {
            let _ = outstream;
            unimplemented!()
        },
    }
}

pub fn restore(exec: &mut Execute, instream: u32) -> bool {
    match exec.iosys.mode {
        Mode::Null | Mode::Filter => panic!("{:x}: invalid IO System {:?} for save/restore", exec.state.pc, exec.iosys.mode),
        Mode::Glk => {
            let _ = instream;
            unimplemented!()
        },
    }
}
