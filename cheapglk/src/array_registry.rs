use std::os::raw::{c_char,c_void};

#[link(name="cheapglk")]
extern {
    fn gidispatch_set_retained_registry(regi: extern fn(*const c_void,u32,*const c_char) -> gidispatch_rock_t, unregi: extern fn(*const c_void,u32,*const c_char,gidispatch_rock_t));
}

pub fn init() {
    let _ = gidispatch_set_retained_registry;
    unimplemented!()
}

#[allow(non_camel_case_types)]
type gidispatch_rock_t = *const c_void;

pub fn register_line_event(buf: (u32,Box<[u8]>)) -> *const c_char {
    let _ = buf;
    unimplemented!()
}

pub fn register_line_event_uni(buf: (u32,Box<[u32]>)) -> *const u32 {
    let _ = buf;
    unimplemented!()
}

pub fn register_stream_memory(buf: (u32,Box<[u8]>)) -> (usize,*const c_char) {
    let _ = buf;
    unimplemented!()
}

pub fn retrieve_line_event() -> Option<(u32,Box<[u8]>)> {
    unimplemented!()
}

pub fn retrieve_line_event_uni() -> Option<(u32,Box<[u32]>)> {
    unimplemented!()
}

pub fn retrieve_stream_memory(id: usize) -> Option<(u32,Box<[u8]>)> {
    let _ = id;
    unimplemented!()
}
