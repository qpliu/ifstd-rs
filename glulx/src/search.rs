use std::cmp::Ordering;

use super::state::{read_u16,read_u32,State};

const KEY_INDIRECT: u32 = 1;
const ZERO_KEY_TERMINATES: u32 = 2;
const RETURN_INDEX: u32 = 4;

fn zero_key(mem: &[u8], key_size: usize, ikey: usize) -> bool {
    for i in 0 .. key_size {
        if mem[ikey + i] != 0 {
            return false;
        }
    }
    true
}

fn cmp_key(state: &State, indirect: bool, key_size: usize, ikey: usize, key: u32) -> Ordering {
    if !indirect {
        match key_size {
            1 => (state.mem[ikey] as u32).cmp(&(key & 0xff)),
            2 => read_u16(&state.mem, ikey).cmp(&(key & 0xffff)),
            4 => read_u32(&state.mem, ikey).cmp(&key),
            _ => panic!("{:x}: invalid search key size {}", state.pc, key_size),
        }
    } else {
        for i in 0 .. key_size {
            match state.mem[ikey+i].cmp(&state.mem[key as usize + i]) {
                Ordering::Equal => (),
                ordering => return ordering,
            }
        }
        Ordering::Equal
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
    let indirect = options & KEY_INDIRECT != 0;
    for i in 0 .. num_structs {
        let ikey = start + i*struct_size + key_offset;
        if let Ordering::Equal = cmp_key(&state, indirect, key_size, ikey, key) {
            return ret(i, start, struct_size, options);
        }
        if options & ZERO_KEY_TERMINATES != 0 && zero_key(&state.mem, key_size, ikey) {
            break;
        }
    }
    ret(0xffffffff, start, struct_size, options)
}

pub fn binary(state: &State, key: u32, key_size: usize, start: usize, struct_size: usize, num_structs: usize, key_offset: usize, options: u32) -> u32 {
    let indirect = options & KEY_INDIRECT != 0;
    let mut i = 0;
    let mut j = num_structs;
    loop {
        if i >= j {
            return ret(0xffffffff, start, struct_size, options);
        }
        let p = (i + j)/2;
        let ikey = start + p*struct_size + key_offset;
        match cmp_key(&state, indirect, key_size, ikey, key) {
            Ordering::Equal => return ret(p, start, struct_size, options),
            Ordering::Less => i = p + 1,
            Ordering::Greater => j = p,
        }
    }
}

pub fn linked(state: &State, key: u32, key_size: usize, start: usize, key_offset: usize, next_offset: usize, options: u32) -> u32 {
    let indirect = options & KEY_INDIRECT != 0;
    let mut addr = start;
    loop {
        if addr == 0 {
            return 0;
        }
        let ikey = addr+key_offset;
        if let Ordering::Equal = cmp_key(&state, indirect, key_size, ikey, key) {
            return addr as u32;
        }
        if options & ZERO_KEY_TERMINATES != 0 && zero_key(&state.mem, key_size, ikey) {
            return 0;
        }
        addr = read_u32(&state.mem, addr+next_offset) as usize;
    }
}
