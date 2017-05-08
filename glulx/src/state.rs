use std::io;
use std::io::{Error,ErrorKind,Read};

pub struct State {
    pub rom: Box<[u8]>,

    pub mem: Vec<u8>,
    pub pc: usize,
    pub stack: Vec<u32>,
    pub frame_ptr: usize,
    pub heap_ptr: usize,
    pub heap: Vec<MemoryBlock>,
}

pub struct UndoState<T> {
    addr_mode: Option<T>,
    mem: Vec<u8>,
    pc: usize,
    stack: Vec<u32>,
    frame_ptr: usize,
    heap_ptr: usize,
    heap: Vec<MemoryBlock>,
}

#[derive(Clone,Copy,Eq,Ord,PartialEq,PartialOrd)]
pub struct MemoryBlock {
    pub addr: usize,
    pub size: usize,
}

impl State {
    pub fn new<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut rom = Vec::new();
        r.read_to_end(&mut rom)?;
        if rom.len() < 36 {
            return Err(invalid_data("invalid header"));
        }
        if rom[0..4] != b"Glul"[..] {
            return Err(invalid_data("bad magic"));
        }
        let glulx_version = read_u32(&rom, 4);
        if glulx_version < 0x00020000 || glulx_version >= 0x00030200 {
            return Err(invalid_data("unsupported glulx version"));
        }
        let ram_start = read_u32(&rom, 8) as usize;
        if ram_start % 256 != 0 || ram_start < 256 || ram_start >= rom.len() {
            return Err(invalid_data("invalid RAMSTART"));
        }
        let ext_start = read_u32(&rom, 12) as usize;
        if ext_start % 256 != 0 || ext_start != rom.len() {
            return Err(invalid_data("invalid EXTSTART"));
        }
        let end_mem = read_u32(&rom, 16) as usize;
        if end_mem % 256 != 0 || end_mem < ext_start {
            return Err(invalid_data("invalid ENDMEM"));
        }
        let stack_size = read_u32(&rom, 20) as usize;
        let start_func = read_u32(&rom, 24) as usize;
        if start_func >= rom.len() {
            return Err(invalid_data("invalid Start Func"));
        }
        if rom[start_func] != 0xc0 && rom[start_func] != 0xc1 {
            return Err(invalid_data("invalid Start Func"));
        }
        let checksum = read_u32(&rom, 32);
        {
            let mut sum = 0;
            let mut shift = 0;
            for b in &rom {
                shift = (shift + 24) % 32;
                sum += (*b as u32) << shift;
            }
            if sum != checksum + checksum {
                return Err(invalid_data("invalid checksum"));
            }
        }

        let mut state = State{
            rom: rom.into_boxed_slice(),
            mem: Vec::with_capacity(end_mem),
            pc: 0,
            stack: Vec::with_capacity(stack_size/4),
            frame_ptr: 0,
            heap_ptr: 0,
            heap: Vec::new(),
        };
        state.reset_mem();
        Ok(state)
    }

    pub fn reset_mem(&mut self) {
        self.mem.clear();
        self.mem.extend_from_slice(&self.rom);
        let end_mem = read_u32(&self.rom, 16) as usize;
        self.mem.resize(end_mem, 0);
    }
}

impl<T: Copy> UndoState<T> {
    pub fn new() -> Self {
        UndoState{
            addr_mode: None,
            mem: Vec::new(),
            pc: 0,
            stack: Vec::new(),
            frame_ptr: 0,
            heap_ptr: 0,
            heap: Vec::new(),
        }
    }

    pub fn save(&mut self, state: &State, addr_mode: T) {
        self.addr_mode = Some(addr_mode);
        self.mem.clear();
        self.mem.extend_from_slice(&state.mem);
        self.pc = state.pc;
        self.stack.clear();
        self.stack.extend_from_slice(&state.stack);
        self.frame_ptr = state.frame_ptr;
        self.heap_ptr = state.heap_ptr;
        self.heap.clear();
        self.heap.extend_from_slice(&state.heap);
    }

    pub fn restore(&self, state: &mut State) -> Option<T> {
        match self.addr_mode {
            None => None,
            addr_mode => {
                state.mem.clear();
                state.mem.extend_from_slice(&self.mem);
                state.pc = self.pc;
                state.stack.clear();
                state.stack.extend_from_slice(&self.stack);
                state.frame_ptr = self.frame_ptr;
                state.heap_ptr = self.heap_ptr;
                state.heap.clear();
                state.heap.extend_from_slice(&self.heap);
                addr_mode
            },
        }
    }
}

fn invalid_data(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidData, msg)
}

pub fn read_u32(bytes: &[u8], index: usize) -> u32 {
    (bytes[index] as u32) << 24 | (bytes[index+1] as u32) << 16
        | (bytes[index+2] as u32) << 8 | bytes[index+3] as u32
}

pub fn write_u32(bytes: &mut Vec<u8>, index: usize, val: u32) {
    bytes[index] = (val >> 24) as u8;
    bytes[index+1] = (val >> 16) as u8;
    bytes[index+2] = (val >> 8) as u8;
    bytes[index+3] = val as u8;
}

pub fn read_u16(bytes: &[u8], index: usize) -> u32 {
    (bytes[index] as u32) << 8 | bytes[index+1] as u32
}

pub fn write_u16(bytes: &mut Vec<u8>, index: usize, val: u32) {
    bytes[index] = (val >> 8) as u8;
    bytes[index+1] = val as u8;
}
