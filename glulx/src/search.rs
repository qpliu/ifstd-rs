use super::state::{read_u16,read_u32,State};

const KEY_INDIRECT: u32 = 1;
const ZERO_KEY_TERMINATES: u32 = 2;
const RETURN_INDEX: u32 = 4;

fn make_key(state: &State, key: u32, key_size: usize, options: u32) -> u32 {
    if options & KEY_INDIRECT == 0 {
        match key_size {
            1 => key & 0xff,
            2 => key & 0xffff,
            4 => key,
            _ => panic!("{:x}: invalid search key size {}", state.pc, key_size),
        }
    } else {
        match key_size {
            1 => state.mem[key as usize] as u32,
            2 => read_u16(&state.mem, key as usize),
            4 => read_u32(&state.mem, key as usize),
            _ => panic!("{:x}: invalid search key size {}", state.pc, key_size),
        }
    }
}

fn read_key(state: &State, key_size: usize, addr: usize) -> u32 {
    match key_size {
        1 => state.mem[addr] as u32,
        2 => read_u16(&state.mem, addr),
        4 => read_u32(&state.mem, addr),
        _ => panic!("{:x}: invalid search key size {}", state.pc, key_size),
    }
}

fn ret(index: usize, start: usize, struct_size: usize, options: u32) -> u32 {
    if options & RETURN_INDEX != 0 {
        index as u32
    } else if index == 0xffffffff {
        0
    } else {
        (start + index*struct_size) as u32
    }
}

pub fn linear(state: &State, key: u32, key_size: usize, start: usize, struct_size: usize, num_structs: usize, key_offset: usize, options: u32) -> u32 {
    let k = make_key(&state, key, key_size, options);
    for i in 0 .. num_structs {
        let addr = start + i*struct_size + key_offset;
        let ik = read_key(&state, key_size, addr);
        if ik == k {
            return ret(i, start, struct_size, options);
        }
        if options & ZERO_KEY_TERMINATES != 0 && ik == 0 {
            break;
        }
    }
    ret(0xffffffff, start, struct_size, options)
}

pub fn binary(state: &State, key: u32, key_size: usize, start: usize, struct_size: usize, num_structs: usize, key_offset: usize, options: u32) -> u32 {
    use std::cmp::Ordering::{Equal,Greater,Less};
    let k = make_key(&state, key, key_size, options);
    let mut i = 0;
    let mut j = num_structs;
    loop {
        if i >= j {
            return ret(0xffffffff, start, struct_size, options);
        }
        let p = (i + j)/2;
        let addr = start + p*struct_size + key_offset;
        let ik = read_key(&state, key_size, addr);
        match ik.cmp(&k) {
            Equal => return ret(p, start, struct_size, options),
            Less => i = p + 1,
            Greater => j = p,
        }
    }
}

pub fn linked(state: &State, key: u32, key_size: usize, start: usize, key_offset: usize, next_offset: usize, options: u32) -> u32 {
    let k = make_key(&state, key, key_size, options);
    let mut addr = start;
    loop {
        if addr == 0 {
            return 0;
        }
        let ik = read_key(&state, key_size, addr+key_offset);
        if ik == k {
            return addr as u32;
        }
        if options & ZERO_KEY_TERMINATES != 0 && ik == 0 {
            return 0;
        }
        addr = read_u32(&state.mem, addr+next_offset) as usize;
    }
}
