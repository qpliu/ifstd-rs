use std::io::{Error,ErrorKind,Result};
use glk::{Glk,IdType};

use super::{call,save,trace};
use super::state::{cstr,read_u32};
use super::execute::{Execute,Next,NEXT_EXEC};

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

pub fn streamchar<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, val: u8, within_string: bool) -> Next {
    trace::iosys(exec, "streamchar");
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                return Next(0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(exec, call::DISCARD, 0);
                exec.state.frame_ptr = exec.state.stack.len();
            }
            exec.call_args.clear();
            exec.call_args.push(val as u32);
            let addr = exec.iosys.rock as usize;
            call::tailcall(exec, addr);
        },
        Mode::Glk => {
            exec.glk.put_char(val);
            if within_string {
                return Next(0);
            }
        },
    }
    NEXT_EXEC
}

pub fn streamunichar<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, val: u32, within_string: bool) -> Next {
    trace::iosys(exec, "streamunichar");
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                return Next(0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(exec, call::DISCARD, 0);
                exec.state.frame_ptr = exec.state.stack.len();
            }
            exec.call_args.clear();
            exec.call_args.push(val);
            let addr = exec.iosys.rock as usize;
            call::tailcall(exec, addr);
        },
        Mode::Glk => {
            exec.glk.put_char_uni(val);
            if within_string {
                return Next(0);
            }
        },
    }
    NEXT_EXEC
}

pub fn streamnum<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, val: i32, within_string: bool) -> Next {
    trace::iosys(exec, "streamnum");
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                return Next(0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(exec, call::RESUME_CODE, 0);
                exec.state.frame_ptr = exec.state.stack.len();
            }
            exec.state.pc = val as usize;
            call::push_stub(exec, call::RESUME_NUM, 0);
            exec.state.frame_ptr = exec.state.stack.len();
            return Next(0);
        },
        Mode::Glk => {
            let n = if val < 0 {
                exec.glk.put_char(b'-');
                -(val as i64)
            } else {
                val as i64
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
                return Next(0);
            }
        },
    }
    NEXT_EXEC
}

pub fn streamstr<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, within_string: bool) -> Next {
    trace::iosys(exec, "streamstr");
    match exec.state.mem[addr] {
        call::STRING_E0 => stream_e0(exec, addr+1, within_string),
        call::STRING_E1 => stream_e1(exec, addr+1, within_string),
        call::STRING_E2 => {
            assert_eq!(read_u32(&exec.state.mem, addr), 0xe2000000, "{:x}: invalid padding for E2 string object", addr);
            stream_e2(exec, addr+4, within_string)
        },
        b => panic!("{:x}: unknown string type {:x}", addr, b),
    }
}

fn stream_e0<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, within_string: bool) -> Next {
    trace::iosys(exec, "stream_e0");
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                return Next(0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(exec, call::RESUME_CODE, 0);
                exec.state.frame_ptr = exec.state.stack.len();
            }
            exec.state.pc = addr;
            call::push_stub(exec, call::RESUME_E0, 0);
            exec.state.frame_ptr = exec.state.stack.len();
            return Next(0);
        },
        Mode::Glk => {
            exec.glk.put_string(cstr(&exec.state.mem, addr));
            if within_string {
                return Next(0);
            }
        },
    }
    NEXT_EXEC
}

fn stream_e1<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, within_string: bool) -> Next {
    trace::iosys(exec, "stream_e1");
    if !within_string {
        call::push_stub(exec, call::RESUME_CODE, 0);
        exec.state.frame_ptr = exec.state.stack.len();
    }
    exec.state.pc = addr;
    call::push_stub(exec, call::RESUME_E1, 0);
    exec.state.frame_ptr = exec.state.stack.len();
    Next(0)
}

fn stream_e2<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, within_string: bool) -> Next {
    trace::iosys(exec, "stream_e2");
    match exec.iosys.mode {
        Mode::Null => {
            if within_string {
                return Next(0);
            }
        },
        Mode::Filter => {
            if !within_string {
                call::push_stub(exec, call::RESUME_CODE, 0);
                exec.state.frame_ptr = exec.state.stack.len();
            }
            exec.state.pc = addr;
            call::push_stub(exec, call::RESUME_E2, 0);
            exec.state.frame_ptr = exec.state.stack.len();
            return Next(0);
        },
        Mode::Glk => {
            const MAX_BUFFER_SIZE: usize = 255;
            exec.iosys.tmp.clear();
            let mut i = addr;
            loop {
                let val = read_u32(&exec.state.mem, i);
                if val == 0 {
                    exec.glk.put_string_uni(&exec.iosys.tmp);
                    break;
                }
                exec.iosys.tmp.push(val);
                if exec.iosys.tmp.len() >= MAX_BUFFER_SIZE {
                    exec.glk.put_string_uni(&exec.iosys.tmp);
                    exec.iosys.tmp.clear();
                }
                i += 4;
            }
            if within_string {
                return Next(0);
            }
        },
    }
    NEXT_EXEC
}

pub fn resume_e0<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>) -> Next {
    trace::iosys(exec, "resume_e0");
    let val = exec.state.mem[exec.state.pc] as u32;
    if val == 0 {
        return Next(0);
    }
    exec.state.pc += 1;
    exec.call_args.clear();
    exec.call_args.push(val);
    let addr = exec.iosys.rock as usize;
    call::call(exec, addr, call::RESUME_E0, 0);
    NEXT_EXEC
}

pub fn resume_e1<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, bit_index: u8) -> Next {
    trace::iosys(exec, "resume_e1");
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
                return Next(0);
            },
            2 => {
                let val = exec.state.mem[ptr+1];
                call::push_stub(exec, call::RESUME_E1, bi as u32);
                exec.state.frame_ptr = exec.state.stack.len();
                return streamchar(exec, val, true);
            },
            3 => {
                call::push_stub(exec, call::RESUME_E1, bi as u32);
                exec.state.frame_ptr = exec.state.stack.len();
                return stream_e0(exec, ptr+1, true);
            },
            4 => {
                let val = read_u32(&exec.state.mem, ptr+1);
                call::push_stub(exec, call::RESUME_E1, bi as u32);
                exec.state.frame_ptr = exec.state.stack.len();
                return streamunichar(exec, val, true);
            },
            5 => {
                call::push_stub(exec, call::RESUME_E1, bi as u32);
                exec.state.frame_ptr = exec.state.stack.len();
                return stream_e2(exec, ptr+1, true);
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
        call::push_stub(exec, call::RESUME_E1, bi as u32);
        exec.state.frame_ptr = exec.state.stack.len();
        match exec.state.mem[addr] {
            call::FUNC_C0 | call::FUNC_C1 => {
                exec.call_args.clear();
                for i in 0 .. argc {
                    let val = read_u32(&exec.state.mem, ptr+9+4*i);
                    exec.call_args.push(val);
                }
                return call::tailcall(exec, addr);
            },
            call::STRING_E0 | call::STRING_E1 | call::STRING_E2 => {
                return streamstr(exec, addr, true);
            },
            b => panic!("{:x}: invalid object type {:x}", addr, b),
        }
    }
}

pub fn resume_e2<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>) -> Next{
    trace::iosys(exec, "resume_e2");
    let val = read_u32(&exec.state.mem, exec.state.pc);
    if val == 0 {
        return Next(0);
    }
    exec.state.pc += 4;
    exec.call_args.clear();
    exec.call_args.push(val);
    let addr = exec.iosys.rock as usize;
    call::call(exec, addr, call::RESUME_E2, 0);
    NEXT_EXEC
}

pub fn resume_num<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, pos: usize) -> Next {
    trace::iosys(exec, "resume_num");
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
        return Next(0);
    }
    let val = if pos == 0 && num < 0 {
        '-' as u8
    } else {
        let mut i = size - 1;
        let mut n = num;
        if n < 0 {
            n = -n;
        }
        while i > pos {
            i -= 1;
            n /= 10;
        }
        '0' as u8 + (n % 10) as u8
    };
    call::push_stub(exec, call::RESUME_NUM, pos as u32 + 1);
    exec.state.frame_ptr = exec.state.stack.len();
    streamchar(exec, val, true)
}

pub fn save<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, outstream: u32) -> Result<()> {
    match exec.iosys.mode {
        Mode::Null | Mode::Filter => panic!("{:x}: invalid IO System {:?} for save/restore", exec.state.pc, exec.iosys.mode),
        Mode::Glk => {
            let mut strid = exec.dispatch.get_strid(outstream);
            if strid.is_null() {
                Err(Error::new(ErrorKind::NotFound, "invalid stream id"))
            } else {
                let mut io_stream = exec.glk.io_stream(&mut strid);
                save::write(&exec.state, &mut io_stream)
            }
        },
    }
}

pub fn restore<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, instream: u32) -> Result<()> {
    match exec.iosys.mode {
        Mode::Null | Mode::Filter => panic!("{:x}: invalid IO System {:?} for save/restore", exec.state.pc, exec.iosys.mode),
        Mode::Glk => {
            let mut strid = exec.dispatch.get_strid(instream);
            if strid.is_null() {
                Err(Error::new(ErrorKind::NotFound, "invalid stream id"))
            } else {
                let mut io_stream = exec.glk.io_stream(&mut strid);
                save::read(&mut exec.state, &mut io_stream)
            }
        },
    }
}
