use std::io;
use std::io::{Read,Write};

use iff::{Chunk,TypeID};

use super::State;

#[allow(non_upper_case_globals)]
mod ids {
    use iff::TypeID;
    pub use iff::FORM;

    pub const IFZS: TypeID = TypeID([b'I',b'F',b'Z',b'S']);
    pub const IFhd: TypeID = TypeID([b'I',b'F',b'h',b'd']);
    pub const CMem: TypeID = TypeID([b'C',b'M',b'e',b'm']);
    pub const UMem: TypeID = TypeID([b'U',b'M',b'e',b'm']);
    pub const Stks: TypeID = TypeID([b'S',b't',b'k',b's']);
    pub const MAll: TypeID = TypeID([b'M',b'A',b'l',b'l']);
}

pub fn write<W: Write>(state: &State, w: &mut W) -> io::Result<()> {
    let mut cmem = Vec::new();
    {
        super::push_u32(&mut cmem, state.mem.len() as u32);

        let mut run_len: usize = 0;
        let ram_start = super::read_u32(&state.mem, 8) as usize;
        for i in ram_start .. state.mem.len() {
            let b = if i < state.rom.len() {
                state.mem[i] ^ state.rom[i]
            } else {
                state.mem[i]
            };
            if b == 0 {
                run_len += 1;
            } else {
                while run_len > 0 {
                    cmem.push(0);
                    if run_len < 256 {
                        cmem.push((run_len - 1) as u8);
                        run_len = 0;
                    } else {
                        cmem.push(255);
                        run_len -= 256;
                    }
                }
                cmem.push(b);
            }
        }
    }

    let mut stks = Vec::with_capacity(4*state.stack.len());
    for word in &state.stack[..] {
        super::push_u32(&mut stks, *word);
    }

    let mut mall = Vec::new();
    if !state.heap.is_empty() {
        super::push_u32(&mut mall, state.heap_ptr as u32);
        super::push_u32(&mut mall, state.heap.len() as u32);
        for block in &state.heap[..] {
            super::push_u32(&mut mall, block.addr as u32);
            super::push_u32(&mut mall, block.size as u32);
        }
    }

    let mut chunk = Chunk::create(ids::FORM, ids::IFZS);
    chunk.append_data(ids::IFhd, &state.rom[0..128]);
    chunk.append_data(ids::CMem, &cmem[..]);
    chunk.append_data(ids::Stks, &stks[..]);
    if !mall.is_empty() {
        chunk.append_data(ids::MAll, &mall[..]);
    }
    chunk.write(w)
}

pub fn read<R: Read>(state: &mut State, r: &mut R) -> io::Result<()> {
    let mut vec = Vec::new();
    r.read_to_end(&mut vec)?;
    let chunk = Chunk::new(&vec[..])?;
    if !chunk.has_envelope_type(ids::FORM, ids::IFZS) {
        return Err(super::invalid_data("invalid save data"));
    }
    read_save_chunks(state, &chunk.data_chunks()[..])
}

fn read_save_chunks(state: &mut State, chunks: &[(TypeID,&[u8])]) -> io::Result<()> {
    if chunks.len() < 3 {
        return Err(super::invalid_data("invalid save data"))
    }
    if chunks[0].0 != ids::IFhd {
        return Err(super::invalid_data("invalid save data"))
    }
    if chunks[0].1 != &state.mem[0..128] {
        return Err(super::invalid_data("invalid save data"))
    }
    for &(id,data) in chunks {
        match id {
            ids::CMem => read_cmem(state, data)?,
            ids::UMem => read_umem(state, data)?,
            ids::Stks => read_stks(state, data)?,
            ids::MAll => read_mall(state, data)?,
            _ => (),
        }
    }
    Ok(())
}

fn read_cmem(state: &mut State, data: &[u8]) -> io::Result<()> {
    if data.len() < 4 {
        return Err(super::invalid_data("invalid CMem chunk"))
    }
    let mem_size = super::read_u32(&data, 0) as usize;
    state.mem.clear();
    state.mem.extend_from_slice(&state.rom[..]);
    state.mem.resize(mem_size, 0);

    let mut i = super::read_u32(&state.mem, 8) as usize;
    let mut zero = false;
    for b in &data[4..] {
        if zero {
            i += *b as usize;
            zero = false;
        } else if *b == 0 {
            i += 1;
            zero = true;
        } else {
            state.mem[i] ^= *b;
            i += 1;
        }
    }
    Ok(())
}

fn read_umem(state: &mut State, data: &[u8]) -> io::Result<()> {
    if data.len() < 4 {
        return Err(super::invalid_data("invalid UMem chunk"))
    }
    let mem_size = super::read_u32(&data, 0) as usize;
    state.mem.clear();
    state.mem.extend_from_slice(&state.rom[..]);
    state.mem.resize(mem_size, 0);

    let ram_start = super::read_u32(&state.mem, 8) as usize;
    for i in 4 .. data.len() {
        state.mem[ram_start - 4 + i] = data[i];
    }
    Ok(())
}

fn read_stks(state: &mut State, data: &[u8]) -> io::Result<()> {
    if data.len() % 4 != 0 && data.len() < 16 {
        return Err(super::invalid_data("invalid Stks chunk"))
    }
    state.stack.clear();
    for i in 0 .. data.len()/4 {
        state.stack.push(super::read_u32(data, i*4));
    }
    Ok(())
}

fn read_mall(state: &mut State, data: &[u8]) -> io::Result<()> {
    if data.len() < 8 {
        return Err(super::invalid_data("invalid MAll chunk"))
    }
    state.heap_ptr = super::read_u32(data, 0) as usize;
    state.heap.clear();
    for i in 0 .. super::read_u32(data, 4) as usize {
        state.heap.push(super::MemoryBlock{
                addr: super::read_u32(data, 8+i*8) as usize,
                size: super::read_u32(data, 12+i*8) as usize,
            })
    }
    state.heap.sort();
    Ok(())
}
