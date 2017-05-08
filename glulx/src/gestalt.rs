use super::{accel,iosys};
use super::execute::Execute;

const GLULX_VERSION: u32 = 0;
const TERP_VERSION: u32 = 1;
const RESIZE_MEM: u32 = 2;
const UNDO: u32 = 3;
const IOSYS: u32 = 4;
const UNICODE: u32 = 5;
const MEMCOPY: u32 = 6;
const MALLOC: u32 = 7;
const MALLOC_HEAP: u32 = 8;
const ACCELERATION: u32 = 9;
const ACCEL_FUNC: u32 = 10;
const FLOAT: u32 = 11;

pub fn get(exec: &Execute, selector: u32, arg: u32) -> u32 {
    match selector {
        GLULX_VERSION => 0x00030102,
        TERP_VERSION => 0x00000100,
        RESIZE_MEM => 1,
        UNDO => 1,
        IOSYS => if iosys::supported(arg) { 1 } else { 0 },
        UNICODE => 1,
        MEMCOPY => 1,
        MALLOC => 1,
        MALLOC_HEAP => exec.state.heap_ptr as u32,
        ACCELERATION => 1,
        ACCEL_FUNC => if accel::supported(arg) { 1 } else { 0 },
        FLOAT => 1,
        _ => 0,
    }
}
