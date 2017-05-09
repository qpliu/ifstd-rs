use std::collections::HashMap;

use super::search;
use super::state::{read_u16,read_u32};
use super::execute::Execute;

const WORDSIZE: u32 = 4;

const FUNC_1_Z__REGION: u32 = 1;
const FUNC_2_CP__TAB: u32 = 2;
const FUNC_3_RA__PR: u32 = 3;
const FUNC_4_RL__PR: u32 = 4;
const FUNC_5_OC__CL: u32 = 5;
const FUNC_6_RV__PR: u32 = 6;
const FUNC_7_OP__PR: u32 = 7;
const FUNC_8_CP__TAB: u32 = 8;
const FUNC_9_RA__PR: u32 = 9;
const FUNC_10_RL__PR: u32 = 10;
const FUNC_11_OC__CL: u32 = 11;
const FUNC_12_RV__PR: u32 = 12;
const FUNC_13_OP__PR: u32 = 13;

pub struct Accel {
    classes_table: u32,
    indiv_prop_start: u32,
    class_metaclass: u32,
    object_metaclass: u32,
    routine_metaclass: u32,
    string_metaclass: u32,
    param_self: u32,
    num_attr_bytes: u32,
    cpv_start: u32,

    funcs: HashMap<usize,u32>,
}

impl Accel {
    pub fn new() -> Self {
        Accel{
            classes_table: 0,
            indiv_prop_start: 0,
            class_metaclass: 0,
            object_metaclass: 0,
            routine_metaclass: 0,
            string_metaclass: 0,
            param_self: 0,
            num_attr_bytes: 0,
            cpv_start: 0,
            funcs: HashMap::new(),
        }
    }

    pub fn func(&mut self, func: u32, addr: usize) {
        if func == 0 {
            self.funcs.remove(&addr);
        } else if supported(func) {
            self.funcs.insert(addr, func);
        }
    }

    pub fn param(&mut self, param: u32, val: u32) {
        match param {
            0 => self.classes_table = val,
            1 => self.indiv_prop_start = val,
            2 => self.class_metaclass = val,
            3 => self.object_metaclass = val,
            4 => self.routine_metaclass = val,
            5 => self.string_metaclass = val,
            6 => self.param_self = val,
            7 => self.num_attr_bytes = val,
            8 => self.cpv_start = val,
            _ => (),
        }
    }

    fn call(&self) -> u32 {
        self.indiv_prop_start + 5
    }

    fn print(&self) -> u32 {
        self.indiv_prop_start + 6
    }

    fn print_to_array(&self) -> u32 {
        self.indiv_prop_start + 7
    }
}

pub fn supported(func: u32) -> bool {
    match func {
        FUNC_1_Z__REGION
            | FUNC_2_CP__TAB
            | FUNC_3_RA__PR
            | FUNC_4_RL__PR
            | FUNC_5_OC__CL
            | FUNC_6_RV__PR
            | FUNC_7_OP__PR
            | FUNC_8_CP__TAB
            | FUNC_9_RA__PR
            | FUNC_10_RL__PR
            | FUNC_11_OC__CL
            | FUNC_12_RV__PR
            | FUNC_13_OP__PR
            => false,
        _ => false,
    }
}

pub fn call(exec: &Execute, addr: usize) -> Option<u32> {
    let arg0 = exec.call_args.get(0).unwrap_or(&0).clone();
    let arg1 = exec.call_args.get(1).unwrap_or(&0).clone();
    match exec.accel.funcs.get(&addr) {
        Some(&FUNC_1_Z__REGION) => Some(FUNC_1_Z__Region(exec, arg0)),
        Some(&FUNC_2_CP__TAB) => Some(FUNC_2_CP__Tab(exec, arg0, arg1)),
        Some(&FUNC_3_RA__PR) => Some(FUNC_3_RA__Pr(exec, arg0, arg1)),
        Some(&FUNC_4_RL__PR) => Some(FUNC_4_RL__Pr(exec, arg0, arg1)),
        Some(&FUNC_5_OC__CL) => Some(FUNC_5_OC__Cl(exec, arg0, arg1)),
        Some(&FUNC_6_RV__PR) => Some(FUNC_6_RV__Pr(exec, arg0, arg1)),
        Some(&FUNC_7_OP__PR) => Some(FUNC_7_OP__Pr(exec, arg0, arg1)),
        Some(&FUNC_8_CP__TAB) => Some(FUNC_8_CP__Tab(exec, arg0, arg1)),
        Some(&FUNC_9_RA__PR) => Some(FUNC_9_RA__Pr(exec, arg0, arg1)),
        Some(&FUNC_10_RL__PR) => Some(FUNC_10_RL__Pr(exec, arg0, arg1)),
        Some(&FUNC_11_OC__CL) => Some(FUNC_11_OC__Cl(exec, arg0, arg1)),
        Some(&FUNC_12_RV__PR) => Some(FUNC_12_RV__Pr(exec, arg0, arg1)),
        Some(&FUNC_13_OP__PR) => Some(FUNC_13_OP__Pr(exec, arg0, arg1)),
        _ => None,
    }
}

#[allow(non_snake_case)]
fn ERROR(exec: &Execute, msg: &'static str) {
    let _ = (exec,msg);
    unimplemented!();
}

#[allow(non_snake_case)]
fn OBJ_IN_CLASS(exec: &Execute, addr: usize) -> bool {
    exec.accel.class_metaclass == read_u32(&exec.state.mem, addr + 13 + exec.accel.num_attr_bytes as usize)
}

#[allow(non_snake_case)]
fn FUNC_1_Z__Region(exec: &Execute, arg0: u32) -> u32 {
    let addr = arg0 as usize;
    if addr < 36 {
        return 0;
    }
    if addr >= exec.state.mem.len() {
        return 0;
    }
    let tb = exec.state.mem[addr];
    if tb >= 0xe0 {
        return 3;
    }
    if tb >= 0xc0 {
        return 2;
    }
    if tb >= 0x70 && tb <= 0x7f && addr >= read_u32(&exec.state.mem, 8) as usize {
        return 1;
    }
    0
}

#[allow(non_snake_case)]
fn FUNC_2_CP__Tab(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let id = arg1;
    if FUNC_1_Z__Region(exec, obj as u32) != 1 {
        ERROR(exec, "[** Programming error: tried to find the ~.~ of (something) **]");
        return 0;
    }
    let otab = read_u32(&exec.state.mem, obj + 16) as usize;
    if otab == 0 {
        return 0;
    }
    let max = read_u32(&exec.state.mem, otab) as usize;
    search::binary(&exec.state, id, 2, otab+4, 10, max, 0, 0)
}

#[allow(non_snake_case)]
fn FUNC_3_RA__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let mut obj = arg0 as usize;
    let mut id = arg1;
    let mut cla = 0;
    if id & 0xffff0000 != 0 {
        cla = read_u32(&exec.state.mem, exec.accel.classes_table as usize + 4*(id as usize & 0xffff)) as usize;
        if FUNC_5_OC__Cl(exec, obj as u32, cla as u32) == 0 {
            return 0;
        }
        id >>= 16;
        obj = cla;
    }
    let prop = FUNC_2_CP__Tab(exec, obj as u32, cla as u32) as usize;
    if prop == 0 {
        return 0;
    }
    if OBJ_IN_CLASS(exec, obj) && cla == 0 {
        if id < exec.accel.indiv_prop_start || id >= exec.accel.indiv_prop_start+8 {
            return 0;
        }
    }
    if read_u32(&exec.state.mem, exec.accel.param_self as usize) as usize != obj {
        let ix = exec.state.mem[prop+9] & 1;
        if ix != 0 {
            return 0;
        }
    }
    read_u32(&exec.state.mem, prop+4)
}

#[allow(non_snake_case)]
fn FUNC_4_RL__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let mut obj = arg0 as usize;
    let mut id = arg1;
    let mut cla = 0;
    if id & 0xffff0000 != 0 {
        cla = read_u32(&exec.state.mem, exec.accel.classes_table as usize + 4*(id as usize & 0xffff)) as usize;
        if FUNC_5_OC__Cl(exec, obj as u32, cla as u32) == 0 {
            return 0;
        }
        id >>= 16;
        obj = cla;
    }
    let prop = FUNC_2_CP__Tab(exec, obj as u32, id as u32) as usize;
    if prop == 0 {
        return 0;
    }
    if OBJ_IN_CLASS(exec, obj) && cla == 0 {
        if id < exec.accel.indiv_prop_start || id >= exec.accel.indiv_prop_start+8 {
            return 0;
        }
    }
    if read_u32(&exec.state.mem, exec.accel.param_self as usize) as usize != obj {
        let ix = exec.state.mem[prop+9] & 1;
        if ix != 0 {
            return 0;
        }
    }
    let ix = read_u16(&exec.state.mem, prop + 2);
    WORDSIZE * ix
}

#[allow(non_snake_case)]
fn FUNC_5_OC__Cl(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let cla = arg1 as usize;
    let zr = FUNC_1_Z__Region(exec, obj as u32);
    if zr == 3 {
        if cla == exec.accel.string_metaclass as usize {
            return 1;
        }
        return 0;
    }
    if zr == 2 {
        if cla == exec.accel.routine_metaclass as usize {
            return 1;
        }
        return 0;
    }
    if zr != 1 {
        return 0;
    }
    if cla == exec.accel.class_metaclass as usize {
        if OBJ_IN_CLASS(exec, obj)
            || obj == exec.accel.class_metaclass as usize
            || obj == exec.accel.string_metaclass as usize
            || obj == exec.accel.routine_metaclass as usize
            || obj == exec.accel.object_metaclass as usize {
            return 1;
        }
        return 0;
    }
    if cla == exec.accel.object_metaclass as usize {
        if OBJ_IN_CLASS(exec, obj)
            || obj == exec.accel.class_metaclass as usize
            || obj == exec.accel.string_metaclass as usize
            || obj == exec.accel.routine_metaclass as usize
            || obj == exec.accel.object_metaclass as usize {
            return 0;
        }
        return 1;
    }
    if cla == exec.accel.string_metaclass as usize || cla == exec.accel.routine_metaclass as usize {
        return 0;
    }
    if !OBJ_IN_CLASS(exec, cla) {
        ERROR(exec, "[** Programming error: tried to apply 'ofclass' with non-class **]");
        return 0;
    }
    let inlist = FUNC_3_RA__Pr(exec, obj as u32, 2) as usize;
    if inlist == 0 {
        return 0;
    }
    let inlistlen = (FUNC_4_RL__Pr(exec, obj as u32, 2) / WORDSIZE) as usize;
    for jx in 0 .. inlistlen {
        if read_u32(&exec.state.mem, inlist + 4*jx) as usize == cla {
            return 1;
        }
    }
    0
}

#[allow(non_snake_case)]
fn FUNC_6_RV__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let id = arg1;
    let addr = FUNC_3_RA__Pr(exec, obj as u32, id) as usize;
    if addr == 0 {
        if id > 0 && id < exec.accel.indiv_prop_start {
            return read_u32(&exec.state.mem, (exec.accel.cpv_start + id*4) as usize);
        }
        ERROR(exec, "[** Programming error: tried to read (something) **]");
        return 0;
    }
    read_u32(&exec.state.mem, addr)
}

#[allow(non_snake_case)]
fn FUNC_7_OP__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let id = arg1;
    let zr = FUNC_1_Z__Region(exec, obj as u32);
    if zr == 3 {
        if id == exec.accel.print() || id == exec.accel.print_to_array() {
            return 1;
        }
        return 0;
    }
    if zr == 2 {
        if id == exec.accel.call() {
            return 1;
        }
        return 0;
    }
    if zr != 1 {
        return 0;
    }
    if id >= exec.accel.indiv_prop_start && id < exec.accel.indiv_prop_start+8 {
        if OBJ_IN_CLASS(exec, obj) {
            return 1;
        }
    }
    if FUNC_3_RA__Pr(exec, obj as u32, id) != 0 {
        return 1;
    }
    0
}

#[allow(non_snake_case)]
fn FUNC_8_CP__Tab(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let id = arg1;
    if FUNC_1_Z__Region(exec, obj as u32) != 1 {
        ERROR(exec, "[** Programming error: tried to find the ~.~ of (something) **]");
        return 0;
    }
    let otab = read_u32(&exec.state.mem, obj + 4*(3+exec.accel.num_attr_bytes as usize/4)) as usize;
    if otab == 0 {
        return 0;
    }
    let max = read_u32(&exec.state.mem, otab) as usize;
    search::binary(&exec.state, id, 2, otab+4, 10, max, 0, 0)
}

#[allow(non_snake_case)]
fn FUNC_9_RA__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let mut obj = arg0 as usize;
    let mut id = arg1;
    let mut cla = 0;
    if id & 0xffff0000 != 0 {
        cla = read_u32(&exec.state.mem, exec.accel.classes_table as usize + 4*(id as usize & 0xffff)) as usize;
        if FUNC_11_OC__Cl(exec, obj as u32, cla as u32) == 0 {
            return 0;
        }
        id >>= 16;
        obj = cla;
    }
    let prop = FUNC_8_CP__Tab(exec, obj as u32, cla as u32) as usize;
    if prop == 0 {
        return 0;
    }
    if OBJ_IN_CLASS(exec, obj) && cla == 0 {
        if id < exec.accel.indiv_prop_start || id >= exec.accel.indiv_prop_start+8 {
            return 0;
        }
    }
    if read_u32(&exec.state.mem, exec.accel.param_self as usize) as usize != obj {
        let ix = exec.state.mem[prop+9] & 1;
        if ix != 0 {
            return 0;
        }
    }
    read_u32(&exec.state.mem, prop+4)
}

#[allow(non_snake_case)]
fn FUNC_10_RL__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let mut obj = arg0 as usize;
    let mut id = arg1;
    let mut cla = 0;
    if id & 0xffff0000 != 0 {
        cla = read_u32(&exec.state.mem, exec.accel.classes_table as usize + 4*(id as usize & 0xffff)) as usize;
        if FUNC_11_OC__Cl(exec, obj as u32, cla as u32) == 0 {
            return 0;
        }
        id >>= 16;
        obj = cla;
    }
    let prop = FUNC_8_CP__Tab(exec, obj as u32, id as u32) as usize;
    if prop == 0 {
        return 0;
    }
    if OBJ_IN_CLASS(exec, obj) && cla == 0 {
        if id < exec.accel.indiv_prop_start || id >= exec.accel.indiv_prop_start+8 {
            return 0;
        }
    }
    if read_u32(&exec.state.mem, exec.accel.param_self as usize) as usize != obj {
        let ix = exec.state.mem[prop+9] & 1;
        if ix != 0 {
            return 0;
        }
    }
    let ix = read_u16(&exec.state.mem, prop + 2);
    WORDSIZE * ix
}

#[allow(non_snake_case)]
fn FUNC_11_OC__Cl(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let cla = arg1 as usize;
    let zr = FUNC_1_Z__Region(exec, obj as u32);
    if zr == 3 {
        if cla == exec.accel.string_metaclass as usize {
            return 1;
        }
        return 0;
    }
    if zr == 2 {
        if cla == exec.accel.routine_metaclass as usize {
            return 1;
        }
        return 0;
    }
    if zr != 1 {
        return 0;
    }
    if cla == exec.accel.class_metaclass as usize {
        if OBJ_IN_CLASS(exec, obj)
            || obj == exec.accel.class_metaclass as usize
            || obj == exec.accel.string_metaclass as usize
            || obj == exec.accel.routine_metaclass as usize
            || obj == exec.accel.object_metaclass as usize {
            return 1;
        }
        return 0;
    }
    if cla == exec.accel.object_metaclass as usize {
        if OBJ_IN_CLASS(exec, obj)
            || obj == exec.accel.class_metaclass as usize
            || obj == exec.accel.string_metaclass as usize
            || obj == exec.accel.routine_metaclass as usize
            || obj == exec.accel.object_metaclass as usize {
            return 0;
        }
        return 1;
    }
    if cla == exec.accel.string_metaclass as usize || cla == exec.accel.routine_metaclass as usize {
        return 0;
    }
    if !OBJ_IN_CLASS(exec, cla) {
        ERROR(exec, "[** Programming error: tried to apply 'ofclass' with non-class **]");
        return 0;
    }
    let inlist = FUNC_9_RA__Pr(exec, obj as u32, 2) as usize;
    if inlist == 0 {
        return 0;
    }
    let inlistlen = (FUNC_10_RL__Pr(exec, obj as u32, 2) / WORDSIZE) as usize;
    for jx in 0 .. inlistlen {
        if read_u32(&exec.state.mem, inlist + 4*jx) as usize == cla {
            return 1;
        }
    }
    0
}

#[allow(non_snake_case)]
fn FUNC_12_RV__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let id = arg1;
    let addr = FUNC_9_RA__Pr(exec, obj as u32, id) as usize;
    if addr == 0 {
        if id > 0 && id < exec.accel.indiv_prop_start {
            return read_u32(&exec.state.mem, (exec.accel.cpv_start + id*4) as usize);
        }
        ERROR(exec, "[** Programming error: tried to read (something) **]");
        return 0;
    }
    read_u32(&exec.state.mem, addr)
}

#[allow(non_snake_case)]
fn FUNC_13_OP__Pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let obj = arg0 as usize;
    let id = arg1;
    let zr = FUNC_1_Z__Region(exec, obj as u32);
    if zr == 3 {
        if id == exec.accel.print() || id == exec.accel.print_to_array() {
            return 1;
        }
        return 0;
    }
    if zr == 2 {
        if id == exec.accel.call() {
            return 1;
        }
        return 0;
    }
    if zr != 1 {
        return 0;
    }
    if id >= exec.accel.indiv_prop_start && id < exec.accel.indiv_prop_start+8 {
        if OBJ_IN_CLASS(exec, obj) {
            return 1;
        }
    }
    if FUNC_9_RA__Pr(exec, obj as u32, id) != 0 {
        return 1;
    }
    0
}
