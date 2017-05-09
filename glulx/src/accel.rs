use std::collections::HashMap;

use super::state::read_u32;
use super::execute::Execute;

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
        Some(&FUNC_1_Z__REGION) => Some(func_1_z__region(exec, arg0)),
        Some(&FUNC_2_CP__TAB) => Some(func_2_cp__tab(exec, arg0, arg1)),
        Some(&FUNC_3_RA__PR) => Some(func_3_ra__pr(exec, arg0, arg1)),
        Some(&FUNC_4_RL__PR) => Some(func_4_rl__pr(exec, arg0, arg1)),
        Some(&FUNC_5_OC__CL) => Some(func_5_oc__cl(exec, arg0, arg1)),
        Some(&FUNC_6_RV__PR) => Some(func_6_rv__pr(exec, arg0, arg1)),
        Some(&FUNC_7_OP__PR) => Some(func_7_op__pr(exec, arg0, arg1)),
        Some(&FUNC_8_CP__TAB) => Some(func_8_cp__tab(exec, arg0, arg1)),
        Some(&FUNC_9_RA__PR) => Some(func_9_ra__pr(exec, arg0, arg1)),
        Some(&FUNC_10_RL__PR) => Some(func_10_rl__pr(exec, arg0, arg1)),
        Some(&FUNC_11_OC__CL) => Some(func_11_oc__cl(exec, arg0, arg1)),
        Some(&FUNC_12_RV__PR) => Some(func_12_rv__pr(exec, arg0, arg1)),
        Some(&FUNC_13_OP__PR) => Some(func_13_op__pr(exec, arg0, arg1)),
        _ => None,
    }
}

fn obj_in_class(exec: &Execute, addr: usize) -> bool {
    exec.accel.class_metaclass == read_u32(&exec.state.mem, addr + 13 + exec.accel.num_attr_bytes as usize)
}

#[allow(non_snake_case)]
fn func_1_z__region(exec: &Execute, arg0: u32) -> u32 {
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
fn func_2_cp__tab(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_3_ra__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_4_rl__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_5_oc__cl(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_6_rv__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_7_op__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_8_cp__tab(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_9_ra__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_10_rl__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_11_oc__cl(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_12_rv__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}

#[allow(non_snake_case)]
fn func_13_op__pr(exec: &Execute, arg0: u32, arg1: u32) -> u32 {
    let _ = (exec,arg0,arg1,obj_in_class);
    unimplemented!()
}
