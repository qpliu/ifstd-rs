use std::collections::HashMap;

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
        match func {
            0 => {
                self.funcs.remove(&addr);
            },
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
                => {
                self.funcs.insert(addr, func);
            },
            _ => (),
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
            => true,
        _ => false,
    }
}

pub fn call(exec: &mut Execute, addr: usize) -> Option<u32> {
    match exec.accel.funcs.get(&addr) {
        Some(&FUNC_1_Z__REGION) => unimplemented!(),
        Some(&FUNC_2_CP__TAB) => unimplemented!(),
        Some(&FUNC_3_RA__PR) => unimplemented!(),
        Some(&FUNC_4_RL__PR) => unimplemented!(),
        Some(&FUNC_5_OC__CL) => unimplemented!(),
        Some(&FUNC_6_RV__PR) => unimplemented!(),
        Some(&FUNC_7_OP__PR) => unimplemented!(),
        Some(&FUNC_8_CP__TAB) => unimplemented!(),
        Some(&FUNC_9_RA__PR) => unimplemented!(),
        Some(&FUNC_10_RL__PR) => unimplemented!(),
        Some(&FUNC_11_OC__CL) => unimplemented!(),
        Some(&FUNC_12_RV__PR) => unimplemented!(),
        Some(&FUNC_13_OP__PR) => unimplemented!(),
        _ => None,
    }
}
