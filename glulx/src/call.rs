use glk::Glk;

use super::{accel,iosys};
use super::state::{read_u32,write_u32,State};
use super::execute::Execute;

pub const DISCARD: u32 = 0;
pub const MEM: u32 = 1;
pub const LOCAL: u32 = 2;
pub const STACK: u32 = 3;

pub const RESUME_E1: u32 = 10;
pub const RESUME_CODE: u32 = 11;
pub const RESUME_NUM: u32 = 12;
pub const RESUME_E0: u32 = 13;
pub const RESUME_E2: u32 = 14;
    
pub const FUNC_C0: u8 = 0xc0;
pub const FUNC_C1: u8 = 0xc1;
pub const STRING_E0: u8 = 0xe0;
pub const STRING_E1: u8 = 0xe1;
pub const STRING_E2: u8 = 0xe2;

pub const LOCAL_NONE: u8 = 0;
pub const LOCAL_8: u8 = 1;
pub const LOCAL_16: u8 = 2;
pub const LOCAL_32: u8 = 4;

pub fn call<G: Glk>(exec: &mut Execute<G>, addr: usize, dest_type: u32, dest_addr: u32) {
    match accel::call(exec, addr) {
        Some(val) => {
            ret_result(exec, val, dest_type, dest_addr as usize);
        },
        None => {
            push_stub(&mut exec.state, dest_type, dest_addr);
            call_func(exec, addr);
        },
    }
}

pub fn tailcall<G: Glk>(exec: &mut Execute<G>, addr: usize) -> bool {
    match accel::call(exec, addr) {
        Some(val) => {
            ret(exec, val)
        },
        None => {
            call_func(exec, addr);
            true
        },
    }
}

pub fn push_stub(state: &mut State, dest_type: u32, dest_addr: u32) {
    state.stack.push(dest_type);
    state.stack.push(dest_addr);
    state.stack.push(state.pc as u32);
    state.stack.push(state.frame_ptr as u32);
}

pub fn call_func<G: Glk>(exec: &mut Execute<G>, addr: usize) {
    let func_type = exec.state.mem[addr];
    exec.state.pc = addr + 1;
    let locals_format = exec.state.pc;
    let frame_ptr = exec.state.stack.len();
    let mut locals_pos = 8u32;
    let mut frame_len = 8u32;
    loop {
        let local_type = exec.state.mem[exec.state.pc];
        let local_count = exec.state.mem[exec.state.pc+1];
        exec.state.pc += 2;
        locals_pos += 2;
        match local_type {
            0 => break,
            1 => panic!("non 32-bit stack locals deprecated"),
            2 => panic!("non 32-bit stack locals deprecated"),
            4 => {
                frame_len += 2 + 4*local_count as u32;
            },
            _ => panic!("unknown stack local type {:x}", local_type),
        }
    }
    if locals_pos % 4 == 2 {
        locals_pos += 2;
        frame_len += 2;
    }
    if frame_len % 4 != 0 {
        frame_len += 4 - frame_len % 4;
    }

    exec.frame_locals = frame_ptr + locals_pos as usize / 4;
    exec.frame_end = frame_ptr + frame_len as usize / 4;
    exec.state.frame_ptr = frame_ptr;

    exec.state.stack.push(locals_pos);
    exec.state.stack.push(frame_len);

    {
        let mut i = locals_format;
        loop {
            let val = read_u32(&exec.state.mem, i);
            i += 4;
            if val & 0xff000000 == 0 {
                exec.state.stack.push(0);
                break;
            } else if val & 0x0000ff00 == 0 {
                exec.state.stack.push(val & 0xffff0000);
                break;
            } else {
                exec.state.stack.push(val);
            }
        }
    }
    assert_eq!(exec.state.stack.len(), exec.frame_locals);

    match func_type {
        FUNC_C0 => {
            let mut i = locals_format;
            loop {
                let local_type = exec.state.mem[i];
                let local_count = exec.state.mem[i+1];
                i += 2;
                match local_type {
                    LOCAL_NONE => break,
                    LOCAL_8 => panic!("non 32-bit stack locals deprecated"),
                    LOCAL_16 => panic!("non 32-bit stack locals deprecated"),
                    LOCAL_32 => {
                        let new_len = exec.state.stack.len() + local_count as usize;
                        exec.state.stack.resize(new_len, 0);
                    },
                    _ => panic!("unknown stack local type {:x}", local_type),
                }
            }
            let argc = exec.call_args.len();
            exec.state.stack.extend_from_slice(&exec.call_args);
            exec.state.stack.push(argc as u32);
        },
        FUNC_C1 => {
            let mut i = locals_format;
            loop {
                let local_type = exec.state.mem[i];
                let local_count = exec.state.mem[i+1] as usize;
                i += 2;
                match local_type {
                    0 => break,
                    1 => panic!("non 32-bit stack locals deprecated"),
                    2 => panic!("non 32-bit stack locals deprecated"),
                    4 => {
                        if exec.call_args.len() < local_count {
                            exec.call_args.resize(local_count, 0);
                        }
                        exec.state.stack.extend(exec.call_args.drain(0 .. local_count));
                    },
                    _ => panic!("unknown stack local type {:x}", local_type),
                }
            }
        },
        _ => panic!("unknown function type {:x}", func_type),
    }
    assert_eq!(exec.state.stack.len(), exec.frame_end);
}

pub fn ret<G: Glk>(exec: &mut Execute<G>, val: u32) -> bool {
    {
        let frame_ptr = exec.state.frame_ptr;
        exec.state.stack.truncate(frame_ptr);
    }
    match exec.state.stack.pop() {
        None => return false,
        Some(frame_ptr) => exec.state.frame_ptr = frame_ptr as usize,
    }
    exec.state.pc = exec.state.stack.pop().unwrap() as usize;
    let dest_addr = exec.state.stack.pop().unwrap() as usize;
    let dest_type = exec.state.stack.pop().unwrap();

    exec.frame_locals = exec.state.frame_ptr + exec.state.stack[exec.state.frame_ptr+1] as usize / 4;
    exec.frame_end = exec.state.frame_ptr + exec.state.stack[exec.state.frame_ptr] as usize / 4;
    ret_result(exec, val, dest_type, dest_addr);
    true
}

fn ret_result<G: Glk>(exec: &mut Execute<G>, val: u32, dest_type: u32, dest_addr: usize) {
    match dest_type {
        DISCARD => (),
        MEM => write_u32(&mut exec.state.mem, dest_addr, val),
        LOCAL => exec.state.stack[exec.frame_locals + dest_addr/4] = val,
        STACK => exec.state.stack.push(val),
        RESUME_E1 => iosys::resume_e1(exec, dest_addr as u8),
        RESUME_CODE => (),
        RESUME_NUM => iosys::resume_num(exec, dest_addr),
        RESUME_E0 => iosys::resume_e0(exec),
        RESUME_E2 => iosys::resume_e2(exec),
        _ => panic!("unknown DestType {:x}", dest_type),
    }
}
