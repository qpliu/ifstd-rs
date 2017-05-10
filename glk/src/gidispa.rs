#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use super::Glk;

pub const Class_Window: u32 = 0;
pub const Class_Stream: u32 = 1;
pub const Class_Fileref: u32 = 2;
pub const Class_Schannel: u32 = 3;

pub enum GlUniversal {
    Uint(u32),
    Sint(i32),
    OpaqueRef(), // TODO -- use registry mechanism to get u32
    Uch(u8),
    Sch(i8),
    CharStr(), // TODO
    UnicharStr(), // TODO
    Array(), // TODO
    PtrFlag(u32),
}

pub trait GiDispatch<G: Glk> {
    fn new(glk: G) -> Self;
    fn glk(&mut self) -> &mut G;
    fn call(&mut self, funcnum: u32, args: &mut [GlUniversal]);
    fn prototype(&self, funcnum: u32) -> &'static str;
    fn count_classes(&self) -> u32;
    fn get_class(&self, index: u32) -> (&'static str, u32);
    fn count_intconst(&self) -> u32;
    fn get_intconst(&self, index: u32) -> (&'static str, u32);
    fn count_functions(&self) -> u32;
    fn get_function(&self, index: u32) -> (&'static str, fn(&mut [GlUniversal]), u32);
    fn get_function_by_id(&self, id: u32) -> (&'static str, fn(&mut [GlUniversal]), u32);
}

pub struct ObjectRegistry<T> {
    by_id: Vec<Option<*mut T>>,
    by_obj: HashMap<*mut T,usize>,
}

impl<T> ObjectRegistry<T> {
    pub fn new() -> Self {
        ObjectRegistry{ by_id: Vec::new(), by_obj: HashMap::new() }
    }

    pub fn add(&mut self, obj: *mut T) -> usize {
        if let Some(&id) = self.by_obj.get(&obj) {
            return id;
        }
        for i in 0 .. self.by_id.len() {
            if self.by_id[i].is_none() {
                self.by_id[i] = Some(obj);
                self.by_obj.insert(obj, i);
                return i;
            }
        }
        let id = self.by_id.len();
        self.by_id.push(Some(obj));
        self.by_obj.insert(obj, id);
        id
    }

    pub fn remove(&mut self, obj: *mut T) {
        if let Some(&id) = self.by_obj.get(&obj) {
            self.by_id[id] = None;
        }
    }
}
