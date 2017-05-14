use std;
use std::ffi::{CStr,CString};
use std::os::raw::{c_char,c_int,c_uchar};

static mut MAIN_FUNC: *const u8 = 0 as *const u8;

pub fn init(main_func: fn(super::GlkTerm,Vec<String>)) {
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
        glkterm_main(argc, argv.as_ptr());
    }
}

#[no_mangle]
pub extern fn glk_main() {
    let main_func: fn(super::GlkTerm,Vec<String>) = unsafe { std::mem::transmute(MAIN_FUNC) };
    super::main_func(main_func, startup_args());
}

#[link(name="glkterm")]
extern {
    fn glkterm_main(argc: c_int, argv: *const *const c_char);
}

#[derive(Clone,Copy)]
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

const GLKUNIX_ARG_COUNT: usize = 10;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut glkunix_arguments: [glkunix_argumentlist_t; GLKUNIX_ARG_COUNT] = [
    glkunix_argumentlist_t {
        name: 0 as *const c_char,
        argtype: glkunix_arg_End,
        desc: 0 as *const c_char,
    }; GLKUNIX_ARG_COUNT ];

pub fn set_arguments(mut args: Vec<super::Argument>) {
    unsafe {
        for i in 0 .. GLKUNIX_ARG_COUNT {
            let mut arg = &mut glkunix_arguments[i];
            if arg.argtype != glkunix_arg_End {
                if !arg.name.is_null() {
                    CString::from_raw(arg.name as *mut c_char);
                }
                if !arg.desc.is_null() {
                    CString::from_raw(arg.desc as *mut c_char);
                }
            }
            arg.name = std::ptr::null();
            arg.argtype = glkunix_arg_End;
            arg.desc = std::ptr::null();
        }
    }

    args.truncate(GLKUNIX_ARG_COUNT-1);
    for (arg,i) in args.drain(0 ..).zip(0 .. GLKUNIX_ARG_COUNT-1) {
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
        unsafe {
            glkunix_arguments[i].name = CString::new(name).unwrap().into_raw();
            glkunix_arguments[i].argtype = argtype;
            glkunix_arguments[i].desc = CString::new(desc).unwrap().into_raw();
        }
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
    let mut args: Vec<String> = Vec::new();
    unsafe {
        for i in 0 .. (*data).argc as isize {
            args.push(CStr::from_ptr(*(*data).argv.offset(i)).to_string_lossy().into_owned());
        }
        STARTUP_ARGS = Box::into_raw(Box::new(args)) as *const u8;
    }
    1
}

fn startup_args() -> Vec<String> {
    if unsafe { STARTUP_ARGS } == 0 as *const u8 {
        return Vec::new();
    }
    unsafe {
        let args: Vec<String> = *Box::from_raw(STARTUP_ARGS as *mut Vec<String>);
        STARTUP_ARGS = 0 as *const u8;
        args
    }
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
#[derive(Default)]
pub struct stream_result_t {
    pub readcount: u32,
    pub writecount: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default)]
pub struct glktimeval_t {
    pub high_sec: i32,
    pub low_sec: u32,
    pub microsec: i32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default)]
pub struct glkdate_t {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub weekday: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub microsec: i32,
}

#[link(name="glkterm")]
extern {
    pub fn glk_exit() -> !;
    pub fn glk_set_interrupt_handler(func: extern fn());
    pub fn glk_tick();

    pub fn glk_gestalt(sel: u32, val: u32) -> u32;
    pub fn glk_gestalt_ext(sel: u32, val: u32, arr: *mut u32, arrlen: u32) -> u32;

    pub fn glk_char_to_lower(ch: c_uchar) -> c_uchar;
    pub fn glk_char_to_upper(ch: c_uchar) -> c_uchar;

    pub fn glk_window_get_root() -> winid_t;
    pub fn glk_window_open(split: winid_t, method: u32, size: u32, wintype: u32, rock: u32) -> winid_t;
    pub fn glk_window_close(win: winid_t, result: *mut stream_result_t);
    pub fn glk_window_get_size(win: winid_t, widthptr: *mut u32, heightptr: *mut u32);
    pub fn glk_window_set_arrangement(win: winid_t, method: u32, size: u32, keywin: winid_t);
    pub fn glk_window_get_arrangement(win: winid_t, methodptr: *mut u32, sizeptr: *mut u32, keywinptr: *mut winid_t);
    pub fn glk_window_iterate(win: winid_t, rockptr: *mut u32) -> winid_t;
    pub fn glk_window_get_rock(win: winid_t) -> u32;
    pub fn glk_window_get_type(win: winid_t) -> u32;
    pub fn glk_window_get_parent(win: winid_t) -> winid_t;
    pub fn glk_window_get_sibling(win: winid_t) -> winid_t;
    pub fn glk_window_clear(win: winid_t);
    pub fn glk_window_move_cursor(win: winid_t, xpos: u32, ypos: u32);

    pub fn glk_window_get_stream(win: winid_t) -> strid_t;
    pub fn glk_window_set_echo_stream(win: winid_t, str: strid_t);
    pub fn glk_window_get_echo_stream(win: winid_t) -> strid_t;
    pub fn glk_set_window(win: winid_t);

    pub fn glk_stream_open_file(fileref: frefid_t, fmode: u32, rock: u32) -> strid_t;
    pub fn glk_stream_open_memory(buf: *const c_char, buflen: u32, fmode: u32, rock: u32) -> strid_t;
    pub fn glk_stream_close(str: strid_t, result: *mut stream_result_t);
    pub fn glk_stream_iterate(str: strid_t, rockptr: *mut u32) -> strid_t;
    pub fn glk_stream_get_rock(str: strid_t) -> u32;
    pub fn glk_stream_set_position(str: strid_t, pos: i32, seekmode: u32);
    pub fn glk_stream_get_position(str: strid_t) -> u32;
    pub fn glk_stream_set_current(str: strid_t);
    pub fn glk_stream_get_current() -> strid_t;

    pub fn glk_put_char(c: c_uchar);
    pub fn glk_put_char_stream(str: strid_t, ch: c_uchar);
    pub fn glk_put_string(s: *const c_char);
    pub fn glk_put_string_stream(str: strid_t, s: *const c_char);
    pub fn glk_put_buffer(buf: *const c_char, len: u32);
    pub fn glk_put_buffer_stream(str: strid_t, buf: *const c_char, len: u32);
    pub fn glk_set_style(styl: u32);
    pub fn glk_set_style_stream(str: strid_t, styl: u32);

    pub fn glk_get_char_stream(str: strid_t) -> i32;
    pub fn glk_get_line_stream(str: strid_t, buf: *mut c_char, len: u32) -> u32;
    pub fn glk_get_buffer_stream(str: strid_t, buf: *mut c_char, len: u32) -> u32;

    pub fn glk_stylehint_set(wintype: u32, styl: u32, hint: u32, val: i32);
    pub fn glk_stylehint_clear(wintype: u32, styl: u32, hint: u32);
    pub fn glk_style_distinguish(win: winid_t, styl1: u32, styl2: u32) -> u32;
    pub fn glk_style_measure(win: winid_t, styl: u32, hint: u32, result: *mut u32) -> u32;

    pub fn glk_fileref_create_temp(usage: u32, rock: u32) -> frefid_t;
    pub fn glk_fileref_create_by_name(usage: u32, name: *const c_char, rock: u32) -> frefid_t;
    pub fn glk_fileref_create_by_prompt(usage: u32, fmode: u32, rock: u32) -> frefid_t;
    pub fn glk_fileref_create_from_fileref(usage: u32, fref: frefid_t, rock: u32) -> frefid_t;
    pub fn glk_fileref_destroy(fref: frefid_t);
    pub fn glk_fileref_iterate(fref: frefid_t, rockptr: *mut u32) -> frefid_t;
    pub fn glk_fileref_get_rock(fref: frefid_t) -> u32;
    pub fn glk_fileref_delete_file(fref: frefid_t);
    pub fn glk_fileref_does_file_exist(fref: frefid_t) -> u32;

    pub fn glk_select(event: *mut event_t);
    pub fn glk_select_poll(event: *mut event_t);

    pub fn glk_request_timer_events(millisecs: u32);

    pub fn glk_request_line_event(win: winid_t, buf: *mut c_char, maxlen: u32, initlen: u32);
    pub fn glk_request_char_event(win: winid_t);
    pub fn glk_request_mouse_event(win: winid_t);

    pub fn glk_cancel_line_event(win: winid_t, event: *mut event_t);
    pub fn glk_cancel_char_event(win: winid_t);
    pub fn glk_cancel_mouse_event(win: winid_t);

    pub fn glk_set_echo_line_event(win: winid_t, val: u32);

    pub fn glk_set_terminators_line_event(win: winid_t, keycodes: *const u32, count: u32);

    pub fn glk_buffer_to_lower_case_uni(buf: *mut u32, len: u32, numchars: u32) -> u32;
    pub fn glk_buffer_to_upper_case_uni(buf: *mut u32, len: u32, numchars: u32) -> u32;
    pub fn glk_buffer_to_title_case_uni(buf: *mut u32, len: u32, numchars: u32, lowerrest: u32) -> u32;

    pub fn glk_put_char_uni(ch: u32);
    pub fn glk_put_string_uni(s: *const u32);
    pub fn glk_put_buffer_uni(buf: *const u32, len: u32);
    pub fn glk_put_char_stream_uni(str: strid_t, ch: u32);
    pub fn glk_put_string_stream_uni(str: strid_t, s: *const u32);
    pub fn glk_put_buffer_stream_uni(str: strid_t, buf: *const u32, len: u32);

    pub fn glk_get_char_stream_uni(str: strid_t) -> i32;
    pub fn glk_get_buffer_stream_uni(str: strid_t, buf: *mut u32, len: u32) -> u32;
    pub fn glk_get_line_stream_uni(str: strid_t, buf: *mut u32, len: u32) -> u32;

    pub fn glk_stream_open_file_uni(fileref: frefid_t, fmode: u32, rock: u32) -> strid_t;
    pub fn glk_stream_open_memory_uni(buf: *mut u32, buflen: u32, fmode: u32, rock: u32) -> strid_t;

    pub fn glk_request_char_event_uni(win: winid_t);
    pub fn glk_request_line_event_uni(win: winid_t, buf: *mut u32, maxlen: u32, initlen: u32);

    pub fn glk_buffer_canon_decompose_uni(buf: *mut u32, len: u32, numchars: u32) -> u32;
    pub fn glk_buffer_canon_normalize_uni(buf: *mut u32, len: u32, numchars: u32) -> u32;

    pub fn glk_image_draw(win: winid_t, image: u32, val1: i32, val2: i32) -> u32;
    pub fn glk_image_draw_scaled(win: winid_t, image: u32, val1: i32, val2: i32, width: u32, height: u32) -> u32;
    pub fn glk_image_get_info(image: u32, width: *mut u32, height: *mut u32) -> u32;

    pub fn glk_window_flow_break(win: winid_t);

    pub fn glk_window_erase_rect(win: winid_t, left: i32, top: i32, width: u32, height: u32);
    pub fn glk_window_fill_rect(win: winid_t, color: u32, left: i32, top: i32, width: u32, height: u32);
    pub fn glk_window_set_background_color(win: winid_t, color: u32);

    pub fn glk_schannel_create(rock: u32) ->  schanid_t;
    pub fn glk_schannel_destroy(chan: schanid_t);
    pub fn glk_schannel_iterate(chan: schanid_t, rockptr: *mut u32) -> schanid_t;
    pub fn glk_schannel_get_rock(chan: schanid_t) -> u32;

    pub fn glk_schannel_play(chan: schanid_t, snd: u32) -> u32;
    pub fn glk_schannel_play_ext(chan: schanid_t, snd: u32, repeats: u32, notify: u32) -> u32;
    pub fn glk_schannel_stop(chan: schanid_t);
    pub fn glk_schannel_set_volume(chan: schanid_t, vol: u32);

    pub fn glk_sound_load_hint(snd: u32, flag: u32);

    pub fn glk_schannel_create_ext(rock: u32, volume: u32) -> schanid_t;
    pub fn glk_schannel_play_multi(chanarray: *const schanid_t, chancount: u32, sndarray: *const u32, soundcount: u32, notify: u32) -> u32;
    pub fn glk_schannel_pause(chan: schanid_t);
    pub fn glk_schannel_unpause(chan: schanid_t);
    pub fn glk_schannel_set_volume_ext(chan: schanid_t, vol: u32, duration: u32, notify: u32);

    pub fn glk_set_hyperlink(linkval: u32);
    pub fn glk_set_hyperlink_stream(str: strid_t, linkval: u32);
    pub fn glk_request_hyperlink_event(win: winid_t);
    pub fn glk_cancel_hyperlink_event(win: winid_t);

    pub fn glk_current_time(time: *mut glktimeval_t);
    pub fn glk_current_simple_time(factor: u32) -> i32;
    pub fn glk_time_to_date_utc(time: *const glktimeval_t, date: *mut glkdate_t);
    pub fn glk_time_to_date_local(time: *const glktimeval_t, date: *mut glkdate_t);
    pub fn glk_simple_time_to_date_utc(time: i32, factor: u32, date: *mut glkdate_t);
    pub fn glk_simple_time_to_date_local(time: i32, factor: u32, date: *mut glkdate_t);
    pub fn glk_date_to_time_utc(date: *const glkdate_t, time: *mut glktimeval_t);
    pub fn glk_date_to_time_local(date: *const glkdate_t, time: *mut glktimeval_t);
    pub fn glk_date_to_simple_time_utc(date: *const glkdate_t, factor: u32) -> i32;
    pub fn glk_date_to_simple_time_local(date: *const glkdate_t, factor: u32) -> i32;

    pub fn glk_stream_open_resource(filenum: u32, rock: u32) -> strid_t;
    pub fn glk_stream_open_resource_uni(filenum: u32, rock: u32) -> strid_t;
}
