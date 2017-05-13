use std;
use std::collections::HashMap;
use std::os::raw::{c_char,c_void};

#[link(name="cheapglk")]
extern {
    fn gidispatch_set_retained_registry(regi: extern fn(*mut c_void,u32,*const c_char) -> gidispatch_rock_t, unregi: extern fn(*mut c_void,u32,*const c_char,gidispatch_rock_t));
}

static mut REGISTRY: *mut Registry = 0 as *mut Registry;
static mut REGISTRY_INITIALIZED: bool = false;

pub fn init() {
    unsafe {
        if REGISTRY_INITIALIZED {
            Box::from_raw(REGISTRY);
            // TODO: drop RegistryEntry ptrs
        } else {
            REGISTRY_INITIALIZED = true;
        }
        REGISTRY = Box::into_raw(Box::new(Registry {
                    line_event: RegistryEntry::new(),
                    line_event_uni: RegistryEntry::new(),
                    stream_memory: Vec::new(),
                    stream_memory_index: HashMap::new(),
                }));
        gidispatch_set_retained_registry(register, unregister);
    }
}

struct Registry {
    line_event: RegistryEntry,
    line_event_uni: RegistryEntry,
    stream_memory: Vec<RegistryEntry>,
    stream_memory_index: HashMap<*mut c_void,usize>,
}

struct RegistryEntry {
    registered: bool,
    rock: u32,
    ptr: *mut c_void,
    len: usize,
}

impl RegistryEntry {
    fn new() -> Self {
        RegistryEntry{
            registered: false,
            rock: 0,
            ptr: std::ptr::null_mut(),
            len: 0,
        }
    }

    fn set<T,P>(&mut self,buf: (u32,Box<[T]>)) -> *const P {
        if self.registered {
            return std::ptr::null();
        }
        let _: Option<(u32,Box<[T]>)> = self.retrieve();
        self.rock = buf.0;
        self.len = buf.1.len();
        self.ptr = Box::into_raw(buf.1) as *mut c_void;
        self.ptr as *const P
    }

    fn retrieve<T>(&mut self) -> Option<(u32,Box<[T]>)> {
        if self.registered || self.ptr.is_null() {
            None
        } else {
            let ptr = self.ptr as *mut T;
            self.ptr = std::ptr::null_mut();
            unsafe {
                Some((self.rock,Vec::from_raw_parts(ptr, self.len, self.len).into_boxed_slice()))
            }
        }
    }
}

extern fn register(buf: *mut c_void, _: u32, _: *const c_char) -> gidispatch_rock_t {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    if registry.line_event.ptr == buf {
        registry.line_event.registered = true;
        0 as gidispatch_rock_t
    } else if registry.line_event_uni.ptr == buf {
        registry.line_event_uni.registered = true;
        1 as gidispatch_rock_t
    } else if let Some(i) = registry.stream_memory_index.get(&buf) {
        registry.stream_memory[*i].registered = true;
        2 as gidispatch_rock_t
    } else {
        3 as gidispatch_rock_t
    }
}

extern fn unregister(buf: *mut c_void, len: u32, _: *const c_char, rock: gidispatch_rock_t) {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    if registry.line_event.ptr == buf {
        registry.line_event.registered = false;
    } else if registry.line_event_uni.ptr == buf {
        registry.line_event_uni.registered = false;
    } else if let Some(i) = registry.stream_memory_index.get(&buf) {
        registry.stream_memory[*i].registered = false;
    } else if rock == 0 as gidispatch_rock_t || rock == 2 as gidispatch_rock_t {
        unsafe {
            Vec::from_raw_parts(buf as *mut u8, len as usize, len as usize);
        }
    } else if rock == 1 as gidispatch_rock_t {
        unsafe {
            Vec::from_raw_parts(buf as *mut u32, len as usize, len as usize);
        }
    } else {
        unsafe {
            Box::from_raw(buf);
        }
    }
}

#[allow(non_camel_case_types)]
type gidispatch_rock_t = *const c_void;

pub fn register_line_event(buf: (u32,Box<[u8]>)) -> *const c_char {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.line_event.set(buf)
}

pub fn register_line_event_uni(buf: (u32,Box<[u32]>)) -> *const u32 {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.line_event_uni.set(buf)
}

pub fn register_stream_memory(buf: (u32,Box<[u8]>)) -> (usize,*const c_char) {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    for i in 0 .. registry.stream_memory.len() {
        let mut entry = &mut registry.stream_memory[i];
        if !entry.registered && entry.ptr.is_null() {
            let p = &entry.set(buf);
            return (i,*p)
        }
    }
    let i = registry.stream_memory.len();
    let mut entry = RegistryEntry::new();
    let p = &entry.set(buf);
    registry.stream_memory_index.insert(entry.ptr, i);
    registry.stream_memory.push(entry);
    (i,*p)
}

pub fn retrieve_line_event() -> Option<(u32,Box<[u8]>)> {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.line_event.retrieve()
}

pub fn retrieve_line_event_uni() -> Option<(u32,Box<[u32]>)> {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.line_event_uni.retrieve()
}

pub fn retrieve_stream_memory(id: usize) -> Option<(u32,Box<[u8]>)> {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    if id < registry.stream_memory.len() {
        registry.stream_memory[id].retrieve()
    } else {
        None
    }
}
