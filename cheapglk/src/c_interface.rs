use std;
use std::ffi::CString;
use std::os::raw::{c_char,c_int,c_uchar};

static mut MAIN_FUNC: *const u8 = 0 as *const u8;

pub fn init(main_func: fn(super::CheapGlk,Vec<String>)) {
    unsafe {
        MAIN_FUNC = std::mem::transmute(main_func);
    }

    let mut args = Vec::new();
    for arg in std::env::args() {
        if let Ok(carg) = CString::new(arg) {
            args.push(carg);
        }
    }
    let argc = args.len() as c_int;
    let mut argv = Vec::new();
    for arg in &args {
        argv.push(arg.as_ptr());
    }
    argv.push(std::ptr::null());
    unsafe {
        cheapglk_main(argc, argv.as_ptr());
    }
}

#[no_mangle]
pub extern fn glk_main() {
    let main_func: fn(super::CheapGlk,Vec<String>) = unsafe { std::mem::transmute(MAIN_FUNC) };
    super::main_func(main_func, startup_args());
}

#[link(name="cheapglk")]
extern {
    fn cheapglk_main(argc: c_int, argv: *const *const c_char);
}

#[repr(C)]
pub struct glkunix_argumentlist_t {
    name: *const c_char,
    argtype: c_int,
    desc: *const c_char,
}

#[allow(non_upper_case_globals)]
const glkunix_arg_End: c_int = 0;
#[allow(non_upper_case_globals)]
const glkunix_arg_ValueFollows: c_int = 1;
#[allow(non_upper_case_globals)]
const glkunix_arg_NoValue: c_int = 2;
#[allow(non_upper_case_globals)]
const glkunix_arg_ValueCanFollow: c_int = 3;
#[allow(non_upper_case_globals)]
const glkunix_arg_NumberValue: c_int = 4;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut glkunix_arguments: *const glkunix_argumentlist_t = &glkunix_argumentlist_t {
    name: 0 as *const c_char,
    argtype: glkunix_arg_End,
    desc: 0 as *const c_char,
};
static mut ARGUMENTS_SET: usize = 0;

pub fn set_arguments(args: Vec<super::Argument>) {
    if unsafe { ARGUMENTS_SET } > 0 {
        // TODO: drop previously set glkunix_arguments
    }

    let mut list = Vec::new();
    for arg in args {
        let (name,argtype,desc) = match arg {
            super::Argument::ValueFollows(name,desc) =>
                (name,glkunix_arg_ValueFollows,desc),
            super::Argument::NoValue(name,desc) =>
                (name,glkunix_arg_NoValue,desc),
            super::Argument::ValueCanFollow(name,desc) =>
                (name,glkunix_arg_ValueCanFollow,desc),
            super::Argument::NumberValue(name,desc) =>
                (name,glkunix_arg_NumberValue,desc),
        };
        list.push(glkunix_argumentlist_t{
                name: CString::new(name).unwrap().as_ptr(),
                argtype: argtype,
                desc: CString::new(desc).unwrap().as_ptr(),
            });
    }
    list.push(glkunix_argumentlist_t{ name: 0 as *const c_char, argtype: glkunix_arg_End, desc: 0 as *const c_char });

    unsafe {
        ARGUMENTS_SET = list.len();
        glkunix_arguments = list.as_ptr();
    }
}

#[repr(C)]
pub struct glkunix_startup_t {
    argc: c_int,
    argv: *const *const c_char,
}

static mut STARTUP_ARGS: *const u8 = 0 as *const u8;

#[no_mangle]
pub extern fn glkunix_startup_code(data: *const glkunix_startup_t) -> c_int {
    let _ = (data,unsafe { STARTUP_ARGS });
    // TODO: copy into STARTUP_ARGS
    1
}

fn startup_args() -> Vec<String> {
    // TODO: copy out of STARTUP_ARGS
    vec!()
}

#[allow(non_camel_case_types)]
pub enum glk_window_struct {}
#[allow(non_camel_case_types)]
pub type winid_t = *const glk_window_struct;
#[allow(non_camel_case_types)]
pub enum glk_stream_struct {}
#[allow(non_camel_case_types)]
pub type strid_t = *const glk_stream_struct;
#[allow(non_camel_case_types)]
pub enum glk_fileref_struct {}
#[allow(non_camel_case_types)]
pub type frefid_t = *const glk_fileref_struct;
#[allow(non_camel_case_types)]
pub enum glk_schannel_struct {}
#[allow(non_camel_case_types)]
pub type schanid_t = *const glk_schannel_struct;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct event_t {
    pub evtype: u32,
    pub win: winid_t,
    pub val1: u32,
    pub val2: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct stream_result_t {
    pub readcount: u32,
    pub writecount: u32,
}

#[link(name="cheapglk")]
extern {
    pub fn glk_exit() -> !;
    pub fn glk_set_interrupt_handler(func: extern fn());
    pub fn glk_tick();

    pub fn glk_gestalt(sel: u32, val: u32) -> u32;
    pub fn glk_gestalt_ext(sel: u32, val: u32, arr: *const u32, arrlen: u32) -> u32;

    pub fn glk_char_to_lower(ch: c_uchar) -> c_uchar;
    pub fn glk_char_to_upper(ch: c_uchar) -> c_uchar;

    pub fn glk_window_get_root() -> winid_t;
    pub fn glk_window_open(split: winid_t, method: u32, size: u32, wintype: u32, rock: u32) -> winid_t;
    pub fn glk_window_close(win: winid_t, result: *mut stream_result_t);

    pub fn glk_set_window(win: winid_t);

    pub fn glk_put_char(c: c_uchar);
    pub fn glk_put_string(s: *const c_char);
    pub fn glk_put_buffer(buf: *const c_char, len: u32);
    pub fn glk_set_style(styl: u32);
    pub fn glk_request_line_event(win: winid_t, buf: *const c_char, maxlen: u32, initlen: u32);
    pub fn glk_select(event: *mut event_t);
}
