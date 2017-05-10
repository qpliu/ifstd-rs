#![allow(non_upper_case_globals)]

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

pub trait GiDispatch {
    fn call(funcnum: u32, args: &mut [GlUniversal]);
    fn prototype(funcnum: u32) -> &'static str;
    fn count_classes() -> u32;
    fn get_class(index: u32) -> (&'static str, u32);
    fn count_intconst() -> u32;
    fn get_intconst(index: u32) -> (&'static str, u32);
    fn count_functions() -> u32;
    fn get_function(index: u32) -> (&'static str, fn(&mut [GlUniversal]), u32);
    fn get_function_by_id(id: u32) -> (&'static str, fn(&mut [GlUniversal]), u32);
}
