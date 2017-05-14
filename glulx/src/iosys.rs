use std::io::{Error,ErrorKind,Result};
use glk::{Glk,IdType};

use super::{call,save};
use super::state::{cstr,read_u32};
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
    tmp: Vec<u32>,
}

impl IOSys {
    pub fn new() -> Self {
        IOSys{
            mode: Mode::Null,
            rock: 0,
            tmp: Vec::new(),
        }
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

pub fn streamchar<G: Glk>(exec: &mut Execute<G>, val: u8, within_string: bool) {
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                call::ret(exec, 0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(&mut exec.state, call::DISCARD, 0);
            }
            exec.call_args.clear();
            exec.call_args.push(val as u32);
            let addr = exec.iosys.rock as usize;
            call::tailcall(exec, addr);
        },
        Mode::Glk => {
            exec.glk.put_char(val);
            if within_string {
                call::ret(exec, 0);
            }
        },
    }
}

pub fn streamunichar<G: Glk>(exec: &mut Execute<G>, val: u32, within_string: bool) {
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                call::ret(exec, 0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(&mut exec.state, call::DISCARD, 0);
            }
            exec.call_args.clear();
            exec.call_args.push(val);
            let addr = exec.iosys.rock as usize;
            call::tailcall(exec, addr);
        },
        Mode::Glk => {
            exec.glk.put_char_uni(val);
            if within_string {
                call::ret(exec, 0);
            }
        },
    }
}

pub fn streamnum<G: Glk>(exec: &mut Execute<G>, val: i32, within_string: bool) {
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                call::ret(exec, 0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(&mut exec.state, call::RESUME_CODE, 0);
            }
            exec.state.pc = val as usize;
            call::push_stub(&mut exec.state, call::RESUME_NUM, 0);
            call::ret(exec, 0);
        },
        Mode::Glk => {
            let n = if val < 0 {
                exec.glk.put_char(b'-');
                -val
            } else {
                val
            };
            let mut pow10 = 1;
            loop {
                let p = pow10 * 10;
                if p > n || p < pow10 {
                    break;
                }
                pow10 = p;
            }
            while pow10 > 0 {
                exec.glk.put_char(b'0' + (n/pow10%10) as u8);
                pow10 /= 10;
            }
            if within_string {
                call::ret(exec, 0);
            }
        },
    }
}

pub fn streamstr<G: Glk>(exec: &mut Execute<G>, addr: usize, within_string: bool) {
    match exec.state.mem[addr] {
        call::STRING_E0 => stream_e0(exec, addr+1, within_string),
        call::STRING_E1 => stream_e1(exec, addr+1, within_string),
        call::STRING_E2 => {
            assert_eq!(read_u32(&exec.state.mem, addr), 0xe2000000, "{:x}: invalid padding for E2 string object", addr);
            stream_e2(exec, addr+4, within_string);
        },
        b => panic!("{:x}: unknown string type {:x}", addr, b),
    };
}

fn stream_e0<G: Glk>(exec: &mut Execute<G>, addr: usize, within_string: bool) {
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                call::ret(exec, 0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(&mut exec.state, call::RESUME_CODE, 0);
            }
            exec.state.pc = addr;
            call::push_stub(&mut exec.state, call::RESUME_E0, 0);
            call::ret(exec, 0);
        },
        Mode::Glk => {
            exec.glk.put_string(cstr(&exec.state.mem, addr));
            if within_string {
                call::ret(exec, 0);
            }
        },
    }
}

fn stream_e1<G: Glk>(exec: &mut Execute<G>, addr: usize, within_string: bool) {
    if !within_string {
        call::push_stub(&mut exec.state, call::RESUME_CODE, 0);
    }
    exec.state.pc = addr;
    call::push_stub(&mut exec.state, call::RESUME_E1, 0);
    call::ret(exec, 0);
}

fn stream_e2<G: Glk>(exec: &mut Execute<G>, addr: usize, within_string: bool) {
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                call::ret(exec, 0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(&mut exec.state, call::RESUME_CODE, 0);
            }
            exec.state.pc = addr;
            call::push_stub(&mut exec.state, call::RESUME_E2, 0);
            call::ret(exec, 0);
        },
        Mode::Glk => {
            const MAX_BUFFER_SIZE: usize = 255;
            exec.iosys.tmp.clear();
            let mut i = addr;
            loop {
                let val = read_u32(&exec.state.mem, i);
                exec.iosys.tmp.push(val);
                if val == 0 {
                    exec.glk.put_string_uni(&exec.iosys.tmp);
                    break;
                }
                if exec.iosys.tmp.len() >= MAX_BUFFER_SIZE {
                    exec.iosys.tmp.push(val);
                    exec.glk.put_string_uni(&exec.iosys.tmp);
                    exec.iosys.tmp.clear();
                }
                i += 4;
            }
            if within_string {
                call::ret(exec, 0);
            }
        },
    }
}

pub fn resume_e0<G: Glk>(exec: &mut Execute<G>) {
    let val = exec.state.mem[exec.state.pc] as u32;
    if val == 0 {
        call::ret(exec, 0);
        return;
    }
    exec.state.pc += 1;
    exec.call_args.clear();
    exec.call_args.push(val);
    let addr = exec.iosys.rock as usize;
    call::call(exec, addr, call::RESUME_E0, 0);
}

pub fn resume_e1<G: Glk>(exec: &mut Execute<G>, bit_index: u8) {
    let mut ptr = exec.stringtbl;
    ptr = read_u32(&exec.state.mem, ptr+8) as usize;

    let mut bi = bit_index;
    let mut b = exec.state.mem[exec.state.pc];
    loop {
        let (argc,addr) = match exec.state.mem[ptr] {
            0 => {
                ptr = read_u32(&exec.state.mem, ptr + if (b >> bi) & 1 == 0 { 1 } else { 5 }) as usize;
                bi += 1;
                if bi >= 8 {
                    bi = 0;
                    exec.state.pc += 1;
                    b = exec.state.mem[exec.state.pc];
                }
                continue;
            },
            1 => {
                call::ret(exec, 0);
                break;
            },
            2 => {
                let val = exec.state.mem[ptr+1];
                call::push_stub(&mut exec.state, call::RESUME_E1, bi as u32);
                streamchar(exec, val, true);
                break;
            },
            3 => {
                call::push_stub(&mut exec.state, call::RESUME_E1, bi as u32);
                stream_e0(exec, ptr+1, true);
                break;
            },
            4 => {
                let val = read_u32(&exec.state.mem, ptr+1);
                call::push_stub(&mut exec.state, call::RESUME_E1, bi as u32);
                streamunichar(exec, val, true);
                break;
            },
            5 => {
                call::push_stub(&mut exec.state, call::RESUME_E1, bi as u32);
                stream_e2(exec, ptr+1, true);
                break;
            },
            8 => (0,read_u32(&exec.state.mem, ptr+1) as usize),
            9 => {
                let indirect = read_u32(&exec.state.mem, ptr+1) as usize;
                (0,read_u32(&exec.state.mem, indirect) as usize)
            },
            10 => {
                let argc = read_u32(&exec.state.mem, ptr+5) as usize;
                (argc,read_u32(&exec.state.mem, ptr+1) as usize)
            },
            11 => {
                let argc = read_u32(&exec.state.mem, ptr+5) as usize;
                let indirect = read_u32(&exec.state.mem, ptr+1) as usize;
                (argc,read_u32(&exec.state.mem, indirect) as usize)
            },
            t => panic!("{:x}: invalid stringtbl node type {:x}", ptr, t),
        };
        call::push_stub(&mut exec.state, call::RESUME_E1, bi as u32);
        match exec.state.mem[addr] {
            call::FUNC_C0 | call::FUNC_C1 => {
                exec.call_args.clear();
                for i in 0 .. argc {
                    let val = read_u32(&exec.state.mem, ptr+5+4*i);
                    exec.call_args.push(val);
                }
                call::tailcall(exec, addr);
            },
            call::STRING_E0 | call::STRING_E1 | call::STRING_E2 => {
                streamstr(exec, addr, true);
            },
            b => panic!("{:x}: invalid object type {:x}", addr, b),
        }
    }
}

pub fn resume_e2<G: Glk>(exec: &mut Execute<G>) {
    let val = read_u32(&exec.state.mem, exec.state.pc);
    if val == 0 {
        call::ret(exec, 0);
        return;
    }
    exec.state.pc += 4;
    exec.call_args.clear();
    exec.call_args.push(val);
    let addr = exec.iosys.rock as usize;
    call::call(exec, addr, call::RESUME_E2, 0);
}

pub fn resume_num<G: Glk>(exec: &mut Execute<G>, pos: usize) {
    let num = exec.state.pc as i32;
    let size = {
        let mut size = 1;
        let mut n = num;
        if n < 0 {
            n = -n;
            size += 1;
        }
        loop {
            n /= 10;
            if n == 0 {
                break;
            }
            size += 1;
        }
        size
    };
    if pos >= size {
        call::ret(exec, 0);
        return;
    }
    let val = if pos == 0 && num < 0 {
        '-' as u32
    } else {
        let mut i = size;
        let mut n = num;
        if n < 0 {
            n = -n;
            i -= 1;
        }
        while i > 0 {
            i -= 1;
            n /= 10;
        }
        '0' as u32 + n as u32
    };
    exec.call_args.clear();
    exec.call_args.push(val);
    let addr = exec.iosys.rock as usize;
    call::call(exec, addr, call::RESUME_NUM, pos as u32 + 1);
}

pub fn save<G: Glk>(exec: &mut Execute<G>, outstream: u32) -> Result<()> {
    match exec.iosys.mode {
        Mode::Null | Mode::Filter => panic!("{:x}: invalid IO System {:?} for save/restore", exec.state.pc, exec.iosys.mode),
        Mode::Glk => {
            let mut strid = exec.dispatch.get_strid(outstream);
            if strid.is_null() {
                Err(Error::new(ErrorKind::NotFound, "invalid stream id"))
            } else {
                save::write(&exec.state, &mut strid)
            }
        },
    }
}

pub fn restore<G: Glk>(exec: &mut Execute<G>, instream: u32) -> Result<()> {
    match exec.iosys.mode {
        Mode::Null | Mode::Filter => panic!("{:x}: invalid IO System {:?} for save/restore", exec.state.pc, exec.iosys.mode),
        Mode::Glk => {
            let mut strid = exec.dispatch.get_strid(instream);
            if strid.is_null() {
                Err(Error::new(ErrorKind::NotFound, "invalid stream id"))
            } else {
                save::read(&mut exec.state, &mut strid)
            }
        },
    }
}
