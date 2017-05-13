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
            // TODO: drop pending RegistryEntry ptrs: probably by implementing Drop for RegistryEntry
        } else {
            REGISTRY_INITIALIZED = true;
        }
        REGISTRY = Box::into_raw(Box::new(Registry {
                    pending_line_event: HashMap::new(),
                    returned_line_event: None,
                    pending_line_event_uni: HashMap::new(),
                    returned_line_event_uni: None,
                    pending_stream_memory: HashMap::new(),
                    returned_stream_memory: None,
                    pending_stream_memory_uni: HashMap::new(),
                    returned_stream_memory_uni: None,
                }));
        gidispatch_set_retained_registry(register, unregister);
    }
}

struct Registry {
    pending_line_event: HashMap<*mut c_void,RegistryEntry>,
    returned_line_event: Option<(u32,Box<[u8]>)>,
    pending_line_event_uni: HashMap<*mut c_void,RegistryEntry>,
    returned_line_event_uni: Option<(u32,Box<[u32]>)>,
    pending_stream_memory: HashMap<*mut c_void,RegistryEntry>,
    returned_stream_memory: Option<(u32,Box<[u8]>)>,
    pending_stream_memory_uni: HashMap<*mut c_void,RegistryEntry>,
    returned_stream_memory_uni: Option<(u32,Box<[u32]>)>,
}

struct RegistryEntry {
    rock: u32,
    ptr: *mut c_void,
    len: usize,
}

impl RegistryEntry {
    fn new<T,P>(buf: (u32,Box<[T]>)) -> (*mut P,Self) {
        let rock = buf.0;
        let len = buf.1.len();
        let ptr = Box::into_raw(buf.1) as *mut c_void;
        (ptr as *mut P,RegistryEntry{ rock, ptr, len })
    }

    fn retrieve<T>(mut self) -> (u32,Box<[T]>) {
        let ptr = self.ptr as *mut T;
        self.ptr = std::ptr::null_mut();
        unsafe {
            (self.rock,Vec::from_raw_parts(ptr, self.len, self.len).into_boxed_slice())
        }
    }
}

enum DispatchRock {
    LineEvent(RegistryEntry),
    LineEventUni(RegistryEntry),
    StreamMemory(RegistryEntry),
    StreamMemoryUni(RegistryEntry),
    Unknown,
}

impl DispatchRock {
    fn rock_t(self) -> gidispatch_rock_t {
        Box::into_raw(Box::new(self)) as gidispatch_rock_t
    }

    fn from_rock_t(rock: gidispatch_rock_t) -> Box<Self> {
        unsafe { Box::from_raw(rock as *mut DispatchRock) }
    }
}

extern fn register(buf: *mut c_void, _: u32, _: *const c_char) -> gidispatch_rock_t {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    if let Some(entry) = registry.pending_line_event.remove(&buf) {
        DispatchRock::LineEvent(entry).rock_t()
    } else if let Some(entry) = registry.pending_line_event_uni.remove(&buf) {
        DispatchRock::LineEventUni(entry).rock_t()
    } else if let Some(entry) = registry.pending_stream_memory.remove(&buf) {
        DispatchRock::StreamMemory(entry).rock_t()
    } else if let Some(entry) = registry.pending_stream_memory_uni.remove(&buf) {
        DispatchRock::StreamMemoryUni(entry).rock_t()
    } else {
        DispatchRock::Unknown.rock_t()
    }
}

extern fn unregister(buf: *mut c_void, _: u32, _: *const c_char, rock: gidispatch_rock_t) {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    match *DispatchRock::from_rock_t(rock) {
        DispatchRock::LineEvent(entry) => {
            registry.returned_line_event = Some(entry.retrieve());
        },
        DispatchRock::LineEventUni(entry) => {
            registry.returned_line_event_uni = Some(entry.retrieve());
        },
        DispatchRock::StreamMemory(entry) => {
            registry.returned_stream_memory = Some(entry.retrieve());
        },
        DispatchRock::StreamMemoryUni(entry) => {
            registry.returned_stream_memory_uni = Some(entry.retrieve());
        },
        DispatchRock::Unknown => {
            unsafe {
                Box::from_raw(buf);
            }
        },
    }
}

#[allow(non_camel_case_types)]
type gidispatch_rock_t = *mut c_void;

pub fn register_line_event(buf: (u32,Box<[u8]>)) -> *mut c_char {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    let (p,entry) = RegistryEntry::new(buf);
    registry.pending_line_event.insert(entry.ptr, entry);
    p
}

pub fn register_line_event_uni(buf: (u32,Box<[u32]>)) -> *mut u32 {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    let (p,entry) = RegistryEntry::new(buf);
    registry.pending_line_event_uni.insert(entry.ptr, entry);
    p
}

pub fn register_stream_memory(buf: (u32,Box<[u8]>)) -> *mut c_char {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    let (p,entry) = RegistryEntry::new(buf);
    registry.pending_stream_memory.insert(entry.ptr, entry);
    p
}

pub fn register_stream_memory_uni(buf: (u32,Box<[u32]>)) -> *mut u32 {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    let (p,entry) = RegistryEntry::new(buf);
    registry.pending_stream_memory_uni.insert(entry.ptr, entry);
    p
}

pub fn retrieve_line_event() -> Option<(u32,Box<[u8]>)> {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.returned_line_event.take()
}

pub fn retrieve_line_event_uni() -> Option<(u32,Box<[u32]>)> {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.returned_line_event_uni.take()
}

pub fn retrieve_stream_memory() -> Option<(u32,Box<[u8]>)> {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.returned_stream_memory.take()
}

pub fn retrieve_stream_memory_uni() -> Option<(u32,Box<[u32]>)> {
    let mut registry = unsafe { REGISTRY.as_mut().unwrap() };
    registry.returned_stream_memory_uni.take()
}
