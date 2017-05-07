use super::state::{MemoryBlock,State};

pub fn malloc(state: &mut State, size: usize) -> usize {
    if size == 0 {
        0
    } else if state.heap.is_empty() {
        let addr = state.mem.len();
        state.heap_ptr = addr;
        state.mem.resize(addr + size, 0);
        state.heap.push(MemoryBlock{ addr, size });
        addr
    } else {
        let mut addr = state.heap_ptr;
        let mut index = state.heap.len();
        for i in 0 .. state.heap.len() {
            let block = state.heap[i];
            assert!(addr <= block.addr, "{:x}:malloc {:x}: heap block[{}].addr={:x} overlaps {:x} ", state.pc, size, i, block.addr, addr);
            if addr + size <= block.addr {
                index = i;
                break;
            }
            addr = block.addr + block.size;
        }
        state.heap.insert(index, MemoryBlock{ addr, size });
        if addr + size > state.mem.len() {
            state.mem.resize(addr + size, 0);
        }
        addr
    }
}

pub fn free(state: &mut State, addr: usize) {
    use std::cmp::Ordering::{Equal,Greater,Less};
    // binary search
    let mut i = 0;
    let mut j = state.heap.len();
    loop {
        if i >= j {
            break;
        }
        let k = (i + j) / 2;
        match state.heap[k].addr.cmp(&addr) {
            Equal => {
                state.heap.remove(k);
                if state.heap.is_empty() {
                    state.mem.truncate(state.heap_ptr);
                    state.heap_ptr = 0;
                }
                break;
            },
            Less => i = k + 1,
            Greater => j = k,
        }
    }
}
