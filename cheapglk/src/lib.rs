extern crate glk;

use std::ffi::CString;
use std::io::{Error,ErrorKind,Read,Result,Write};
use std::os::raw::{c_char,c_uchar};

use glk::{Glk,DateType,EventType,IdType,TimeValType};

mod array_registry;
mod c_interface;

pub use c_interface::{glk_main,glkunix_arguments,glkunix_startup_code};

pub struct CheapGlk {
}

impl CheapGlk {
    pub fn set_arguments(args: Vec<Argument>) {
        c_interface::set_arguments(args);
    }

    pub fn init(main_func: fn(CheapGlk,Vec<String>)) {
        array_registry::init();
        c_interface::init(main_func);
    }
}

pub enum Argument {
    ValueFollows(String,String),
    NoValue(String,String),
    ValueCanFollow(String,String),
    NumberValue(String,String),
}

fn main_func(main_func: fn(CheapGlk,Vec<String>), args: Vec<String>) {
    main_func(CheapGlk{}, args);
}

impl Glk for CheapGlk {
    type WinId = WinId;
    type StrId = StrId;
    type FRefId = FRefId;
    type SChanId = SChanId;
    type Event = Event;
    type TimeVal = TimeVal;
    type Date = Date;

    fn exit(&mut self) -> ! {
        unsafe {
            c_interface::glk_exit();
        }
    }

    fn set_interrupt_handler(&mut self, handler: extern fn()) {
        unsafe {
            c_interface::glk_set_interrupt_handler(handler);
        }
    }

    fn tick(&mut self) {
        unsafe {
            c_interface::glk_tick();
        }
    }


    fn gestalt(&mut self, sel: u32, val: u32) -> u32 {
        unsafe {
            c_interface::glk_gestalt(sel, val)
        }
    }

    fn gestalt_ext(&mut self, sel: u32, val: u32, arr: &mut [u32]) -> u32 {
        let len = arr.len() as u32;
        unsafe {
            c_interface::glk_gestalt_ext(sel, val, &mut arr[0], len)
        }
    }


    fn char_to_lower(&mut self, ch: u8) -> u8 {
        unsafe {
            c_interface::glk_char_to_lower(ch as std::os::raw::c_uchar) as u8
        }
    }

    fn char_to_upper(&mut self, ch: u8) -> u8 {
        unsafe {
            c_interface::glk_char_to_upper(ch as std::os::raw::c_uchar) as u8
        }
    }


    fn window_get_root(&mut self) -> Self::WinId {
        let ptr = unsafe {
            c_interface::glk_window_get_root()
        };
        WinId{ ptr }
    }

    fn window_open(&mut self, split: &Self::WinId, method: u32, size: u32, wintype: u32, rock: u32) -> Self::WinId {
        let ptr = unsafe {
            c_interface::glk_window_open(split.ptr, method, size, wintype, rock)
        };
        WinId{ ptr }
    }

    fn window_close(&mut self, win: &mut Self::WinId) -> (u32,u32) {
        let mut result = Default::default();
        unsafe {
            c_interface::glk_window_close(win.ptr, &mut result);
        }
        win.ptr = std::ptr::null();
        (result.readcount,result.writecount)
    }

    fn window_get_size(&mut self, win: &Self::WinId) -> (u32,u32) {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            c_interface::glk_window_get_size(win.ptr, &mut width, &mut height);
        }
        (width,height)
    }

    fn window_set_arrangement(&mut self, win: &Self::WinId, method: u32, size: u32, keywin: &Self::WinId) {
        unsafe {
            c_interface::glk_window_set_arrangement(win.ptr, method, size, keywin.ptr);
        }
    }

    fn window_get_arrangement(&mut self, win: &Self::WinId) -> (u32,u32,Self::WinId) {
        let mut method = 0;
        let mut size = 0;
        let mut keywin = std::ptr::null();
        unsafe {
            c_interface::glk_window_get_arrangement(win.ptr, &mut method, &mut size, &mut keywin);
        }
        (method,size,WinId{ ptr: keywin })
    }

    fn window_iterate(&mut self, win: &Self::WinId) -> (Self::WinId,u32) {
        let mut rock = 0;
        let ptr = unsafe {
            c_interface::glk_window_iterate(win.ptr, &mut rock)
        };
        (WinId{ ptr },rock)
    }

    fn window_get_rock(&mut self, win: &Self::WinId) -> u32 {
        unsafe {
            c_interface::glk_window_get_rock(win.ptr)
        }
    }

    fn window_get_type(&mut self, win: &Self::WinId) -> u32 {
        unsafe {
            c_interface::glk_window_get_type(win.ptr)
        }
    }

    fn window_get_parent(&mut self, win: &Self::WinId) -> Self::WinId {
        let ptr = unsafe { c_interface::glk_window_get_parent(win.ptr) };
        WinId{ ptr }
    }

    fn window_get_sibling(&mut self, win: &Self::WinId) -> Self::WinId {
        let ptr = unsafe { c_interface::glk_window_get_sibling(win.ptr) };
        WinId{ ptr }
    }

    fn window_clear(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_window_clear(win.ptr);
        }
    }

    fn window_move_cursor(&mut self, win: &Self::WinId, xpos: u32, ypos: u32) {
        unsafe {
            c_interface::glk_window_move_cursor(win.ptr, xpos, ypos);
        }
    }


    fn window_get_stream(&mut self, win: &Self::WinId) -> Self::StrId {
        let ptr = unsafe { c_interface::glk_window_get_stream(win.ptr) };
        StrId{ ptr }
    }

    fn window_set_echo_stream(&mut self, win: &Self::WinId, str: &Self::StrId) {
        unsafe {
            c_interface::glk_window_set_echo_stream(win.ptr, str.ptr);
        }
    }

    fn window_get_echo_stream(&mut self, win: &Self::WinId) -> Self::StrId {
        let ptr = unsafe { c_interface::glk_window_get_echo_stream(win.ptr) };
        StrId{ ptr }
    }

    fn set_window(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_set_window(win.ptr);
        }
    }


    fn stream_open_file(&mut self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId {
        let ptr = unsafe { c_interface::glk_stream_open_file(fileref.ptr, fmode, rock) };
        StrId{ ptr }
    }

    fn stream_open_memory(&mut self, buf: (u32,Box<[u8]>), fmode: u32, rock: u32) -> Self::StrId {
        let buflen = buf.1.len() as u32;
        let ptr = unsafe {
            c_interface::glk_stream_open_memory(array_registry::register_stream_memory(buf), buflen, fmode, rock)
        };
        StrId{ ptr }
    }

    fn stream_close(&mut self, str: &mut Self::StrId) -> (u32,u32,Option<(u32,Box<[u8]>)>,Option<(u32,Box<[u32]>)>) {
        let mut result = Default::default();
        unsafe {
            c_interface::glk_stream_close(str.ptr, &mut result);
        }
        str.ptr = std::ptr::null();
        (result.readcount,result.writecount,
         array_registry::retrieve_stream_memory(),
         array_registry::retrieve_stream_memory_uni())
    }

    fn stream_iterate(&mut self, str: &Self::StrId) -> (Self::StrId,u32) {
        let mut rock = 0;
        let ptr = unsafe {
            c_interface::glk_stream_iterate(str.ptr, &mut rock)
        };
        (StrId{ ptr },rock)
    }

    fn stream_get_rock(&mut self, str: &Self::StrId) -> u32 {
        unsafe {
            c_interface::glk_stream_get_rock(str.ptr)
        }
    }

    fn stream_set_position(&mut self, str: &Self::StrId, pos: i32, seekmode: u32) {
        unsafe {
            c_interface::glk_stream_set_position(str.ptr, pos, seekmode);
        }
    }

    fn stream_get_position(&mut self, str: &Self::StrId) -> u32 {
        unsafe {
            c_interface::glk_stream_get_position(str.ptr)
        }
    }

    fn stream_set_current(&mut self, str: &Self::StrId) {
        unsafe {
            c_interface::glk_stream_set_current(str.ptr);
        }
    }

    fn stream_get_current(&mut self) -> Self::StrId {
        let ptr = unsafe { c_interface::glk_stream_get_current() };
        StrId{ ptr }
    }


    fn put_char(&mut self, ch: u8) {
        unsafe {
            c_interface::glk_put_char(ch as c_uchar);
        }
    }

    fn put_char_stream(&mut self, str: &Self::StrId, ch: u8) {
        unsafe {
            c_interface::glk_put_char_stream(str.ptr, ch as c_uchar);
        }
    }

    fn put_string<S: AsRef<[u8]>>(&mut self, s: S) {
        let _ = c_interface::glk_put_string;
        self.put_buffer(s.as_ref());
    }

    fn put_string_stream<S: AsRef<[u8]>>(&mut self, str: &Self::StrId, s: S) {
        let _ = c_interface::glk_put_string_stream;
        self.put_buffer_stream(str, s.as_ref());
    }

    fn put_buffer(&mut self, buf: &[u8]) {
        unsafe {
            c_interface::glk_put_buffer(buf.as_ptr() as *const c_char, buf.len() as u32);
        }
    }

    fn put_buffer_stream(&mut self, str: &Self::StrId, buf: &[u8]) {
        unsafe {
            c_interface::glk_put_buffer_stream(str.ptr, buf.as_ptr() as *const c_char, buf.len() as u32);
        }
    }

    fn set_style(&mut self, styl: u32) {
        unsafe {
            c_interface::glk_set_style(styl);
        }
    }

    fn set_style_stream(&mut self, str: &Self::StrId, styl: u32) {
        unsafe {
            c_interface::glk_set_style_stream(str.ptr, styl);
        }
    }


    fn get_char_stream(&mut self, str: &Self::StrId) -> i32 {
        unsafe {
            c_interface::glk_get_char_stream(str.ptr)
        }
    }

    fn get_line_stream(&mut self, str: &Self::StrId, buf: &mut [u8]) -> u32 {
        unsafe {
            c_interface::glk_get_line_stream(str.ptr, buf.as_mut_ptr() as *mut c_char, buf.len() as u32)
        }
    }

    fn get_buffer_stream(&mut self, str: &Self::StrId, buf: &mut [u8]) -> u32 {
        unsafe {
            c_interface::glk_get_buffer_stream(str.ptr, buf.as_mut_ptr() as *mut c_char, buf.len() as u32)
        }
    }


    fn stylehint_set(&mut self, wintype: u32, styl: u32, hint: u32, val: i32) {
        unsafe {
            c_interface::glk_stylehint_set(wintype, styl, hint, val);
        }
    }

    fn stylehint_clear(&mut self, wintype: u32, styl: u32, hint: u32) {
        unsafe {
            c_interface::glk_stylehint_clear(wintype, styl, hint);
        }
    }

    fn style_distinguish(&mut self, win: &Self::WinId, styl1: u32, styl2: u32) -> bool {
        let result = unsafe {
            c_interface::glk_style_distinguish(win.ptr, styl1, styl2)
        };
        result != 0
    }

    fn style_measure(&mut self, win: &Self::WinId, styl: u32, hint: u32) -> (bool,u32) {
        let mut result = 0;
        let success = unsafe {
            c_interface::glk_style_measure(win.ptr, styl, hint, &mut result)
        };
        (success != 0,result)
    }


    fn fileref_create_temp(&mut self, usage: u32, rock: u32) -> Self::FRefId {
        let ptr = unsafe {
            c_interface::glk_fileref_create_temp(usage, rock)
        };
        FRefId{ ptr }
    }

    fn fileref_create_by_name<S: AsRef<[u8]>>(&mut self, usage: u32, name: S, rock: u32) -> Self::FRefId {
        let mut vec = Vec::new();
        vec.extend_from_slice(name.as_ref());
        let ptr = if let Ok(cname) = CString::new(vec) {
            unsafe {
                c_interface::glk_fileref_create_by_name(usage, cname.as_ptr(), rock)
            }
        } else {
            std::ptr::null()
        };
        FRefId{ ptr }
    }

    fn fileref_create_by_prompt(&mut self, usage: u32, fmode: u32, rock: u32) -> Self::FRefId {
        let ptr = unsafe {
            c_interface::glk_fileref_create_by_prompt(usage, fmode, rock)
        };
        FRefId{ ptr }
    }

    fn fileref_create_from_fileref(&mut self, usage: u32, fref: &Self::FRefId, rock: u32) -> Self::FRefId {
        let ptr = unsafe {
            c_interface::glk_fileref_create_from_fileref(usage, fref.ptr, rock)
        };
        FRefId{ ptr }
    }

    fn fileref_destroy(&mut self, fref: &mut Self::FRefId) {
        unsafe {
            c_interface::glk_fileref_destroy(fref.ptr);
        }
        fref.ptr = std::ptr::null();
    }

    fn fileref_iterate(&mut self, fref: &Self::FRefId) -> (Self::FRefId,u32) {
        let mut rock = 0;
        let ptr = unsafe {
            c_interface::glk_fileref_iterate(fref.ptr, &mut rock)
        };
        (FRefId{ ptr },rock)
    }

    fn fileref_get_rock(&mut self, fref: &Self::FRefId) -> u32 {
        unsafe {
            c_interface::glk_fileref_get_rock(fref.ptr)
        }
    }

    fn fileref_delete_file(&mut self, fref: &Self::FRefId) {
        unsafe {
            c_interface::glk_fileref_delete_file(fref.ptr);
        }
    }

    fn fileref_does_file_exist(&mut self, fref: &Self::FRefId) -> bool {
        let result = unsafe {
            c_interface::glk_fileref_does_file_exist(fref.ptr)
        };
        result != 0
    }


    fn select(&mut self) -> Self::Event {
        let mut event = c_interface::event_t {
            evtype: 0, win: std::ptr::null(), val1: 0, val2: 0
        };
        unsafe {
            c_interface::glk_select(&mut event);
        }
        Event(event)
    }

    fn select_poll(&mut self) -> Self::Event {
        let mut event = c_interface::event_t {
            evtype: 0, win: std::ptr::null(), val1: 0, val2: 0
        };
        unsafe {
            c_interface::glk_select_poll(&mut event);
        }
        Event(event)
    }


    fn request_timer_events(&mut self, millisecs: u32) {
        unsafe {
            c_interface::glk_request_timer_events(millisecs);
        }
    }


    fn request_line_event(&mut self, win: &Self::WinId, buf: (u32,Box<[u8]>), initlen: u32) {
        let maxlen = buf.1.len() as u32;
        let cbuf = array_registry::register_line_event(buf);
        unsafe {
            c_interface::glk_request_line_event(win.ptr, cbuf, maxlen, initlen);
        }
    }

    fn request_char_event(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_request_char_event(win.ptr);
        }
    }

    fn request_mouse_event(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_request_mouse_event(win.ptr);
        }
    }


    fn cancel_line_event(&mut self, win: &Self::WinId) -> Self::Event {
        let mut event = c_interface::event_t {
            evtype: 0, win: std::ptr::null(), val1: 0, val2: 0
        };
        unsafe {
            c_interface::glk_cancel_line_event(win.ptr, &mut event);
        }
        Event(event)
    }

    fn cancel_char_event(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_cancel_char_event(win.ptr);
        }
    }

    fn cancel_mouse_event(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_cancel_mouse_event(win.ptr);
        }
    }


    fn set_echo_line_event(&mut self, win: &Self::WinId, val: u32) {
        unsafe {
            c_interface::glk_set_echo_line_event(win.ptr, val);
        }
    }


    fn set_terminators_line_event(&mut self, win: &Self::WinId, keycodes: &[u32]) {
        unsafe {
            c_interface::glk_set_terminators_line_event(win.ptr, keycodes.as_ptr(), keycodes.len() as u32);
        }
    }


    fn buffer_to_lower_case_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32 {
        unsafe {
            c_interface::glk_buffer_to_lower_case_uni(buf.as_mut_ptr(), buf.len() as u32, numchars)
        }
    }

    fn buffer_to_upper_case_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32 {
        unsafe {
            c_interface::glk_buffer_to_upper_case_uni(buf.as_mut_ptr(), buf.len() as u32, numchars)
        }
    }

    fn buffer_to_title_case_uni(&mut self, buf: &mut [u32], numchars: u32, lowerrest: u32) -> u32 {
        unsafe {
            c_interface::glk_buffer_to_title_case_uni(buf.as_mut_ptr(), buf.len() as u32, numchars, lowerrest)
        }
    }


    fn put_char_uni(&mut self, ch: u32) {
        unsafe {
            c_interface::glk_put_char_uni(ch);
        }
    }

    fn put_string_uni<SU: AsRef<[u32]>>(&mut self, s: SU) {
        let _ = c_interface::glk_put_string_uni;
        self.put_buffer_uni(s.as_ref());
    }

    fn put_buffer_uni(&mut self, buf: &[u32]) {
        unsafe {
            c_interface::glk_put_buffer_uni(buf.as_ptr(), buf.len() as u32);
        }
    }

    fn put_char_stream_uni(&mut self, str: &Self::StrId, ch: u32) {
        unsafe {
            c_interface::glk_put_char_stream_uni(str.ptr, ch);
        }
    }

    fn put_string_stream_uni<SU: AsRef<[u32]>>(&mut self, str: &Self::StrId, s: SU) {
        let _ = c_interface::glk_put_string_stream_uni;
        self.put_buffer_stream_uni(str, s.as_ref());
    }

    fn put_buffer_stream_uni(&mut self, str: &Self::StrId, buf: &[u32]) {
        unsafe {
            c_interface::glk_put_buffer_stream_uni(str.ptr, buf.as_ptr(), buf.len() as u32);
        }
    }


    fn get_char_stream_uni(&mut self, str: &Self::StrId) -> i32 {
        unsafe {
            c_interface::glk_get_char_stream_uni(str.ptr)
        }
    }

    fn get_buffer_stream_uni(&mut self, str: &Self::StrId, buf: &mut [u32]) -> u32 {
        unsafe {
            c_interface::glk_get_buffer_stream_uni(str.ptr, buf.as_mut_ptr(), buf.len() as u32)
        }
    }

    fn get_line_stream_uni(&mut self, str: &Self::StrId, buf: &mut [u32]) -> u32 {
        unsafe {
            c_interface::glk_get_line_stream_uni(str.ptr, buf.as_mut_ptr(), buf.len() as u32)
        }
    }


    fn stream_open_file_uni(&mut self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId {
        let ptr = unsafe { c_interface::glk_stream_open_file_uni(fileref.ptr, fmode, rock) };
        StrId{ ptr }
    }

    fn stream_open_memory_uni(&mut self, buf: (u32,Box<[u32]>), fmode: u32, rock: u32) -> Self::StrId {
        let buflen = buf.1.len() as u32;
        let ptr = unsafe {
            c_interface::glk_stream_open_memory_uni(array_registry::register_stream_memory_uni(buf), buflen, fmode, rock)
        };
        StrId{ ptr }
    }


    fn request_char_event_uni(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_request_char_event_uni(win.ptr);
        }
    }

    fn request_line_event_uni(&mut self, win: &Self::WinId, buf: (u32,Box<[u32]>), initlen: u32) {
        let maxlen = buf.1.len() as u32;
        let cbuf = array_registry::register_line_event_uni(buf);
        unsafe {
            c_interface::glk_request_line_event_uni(win.ptr, cbuf, maxlen, initlen);
        }
    }


    fn buffer_canon_decompose_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32 {
        unsafe {
            c_interface::glk_buffer_canon_decompose_uni(buf.as_mut_ptr(), buf.len() as u32, numchars)
        }
    }

    fn buffer_canon_normalize_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32 {
        unsafe {
            c_interface::glk_buffer_canon_normalize_uni(buf.as_mut_ptr(), buf.len() as u32, numchars)
        }
    }


    fn image_draw(&mut self, win: &Self::WinId, image: u32, val1: i32, val2: i32) -> bool {
        let result = unsafe {
            c_interface::glk_image_draw(win.ptr, image, val1, val2)
        };
        result != 0
    }

    fn image_draw_scaled(&mut self, win: &Self::WinId, image: u32, val1: i32, val2: i32, width: u32, height: u32) -> bool {
        let result = unsafe {
            c_interface::glk_image_draw_scaled(win.ptr, image, val1, val2, width, height)
        };
        result != 0
    }

    fn image_get_info(&mut self, image: u32) -> (bool,u32,u32) {
        let mut width = 0;
        let mut height = 0;
        let result = unsafe {
            c_interface::glk_image_get_info(image, &mut width, &mut height)
        };
        (result != 0,width,height)
    }


    fn window_flow_break(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_window_flow_break(win.ptr);
        }
    }


    fn window_erase_rect(&mut self, win: &Self::WinId, left: i32, top: i32, width: u32, height: u32) {
        unsafe {
            c_interface::glk_window_erase_rect(win.ptr, left, top, width, height);
        }
    }

    fn window_fill_rect(&mut self, win: &Self::WinId, color: u32, left: i32, top: i32, width: u32, height: u32) {
        unsafe {
            c_interface::glk_window_fill_rect(win.ptr, color, left, top, width, height);
        }
    }

    fn window_set_background_color(&mut self, win: &Self::WinId, color: u32) {
        unsafe {
            c_interface::glk_window_set_background_color(win.ptr, color);
        }
    }


    fn schannel_create(&mut self, rock: u32) -> Self::SChanId {
        let ptr = unsafe { c_interface::glk_schannel_create(rock) };
        SChanId{ ptr }
    }

    fn schannel_destroy(&mut self, chan: &mut Self::SChanId) {
        unsafe {
            c_interface::glk_schannel_destroy(chan.ptr);
        }
        chan.ptr = std::ptr::null();
    }

    fn schannel_iterate(&mut self, chan: &Self::SChanId) -> (Self::SChanId,u32) {
        let mut rock = 0;
        let ptr = unsafe { c_interface::glk_schannel_iterate(chan.ptr, &mut rock) };
        (SChanId{ ptr },rock)
    }

    fn schannel_get_rock(&mut self, chan: &Self::SChanId) -> u32 {
        unsafe {
            c_interface::glk_schannel_get_rock(chan.ptr)
        }
    }


    fn schannel_play(&mut self, chan: &Self::SChanId, snd: u32) -> bool {
        let result = unsafe { c_interface::glk_schannel_play(chan.ptr, snd) };
        result != 0
    }

    fn schannel_play_ext(&mut self, chan: &Self::SChanId, snd: u32, repeat: u32, notify: u32) -> bool {
        let result = unsafe { c_interface::glk_schannel_play_ext(chan.ptr, snd, repeat, notify) };
        result != 0
    }

    fn schannel_stop(&mut self, chan: &Self::SChanId) {
        unsafe {
            c_interface::glk_schannel_stop(chan.ptr);
        }
    }

    fn schannel_set_volume(&mut self, chan: &Self::SChanId, vol: u32) {
        unsafe {
            c_interface::glk_schannel_set_volume(chan.ptr, vol);
        }
    }


    fn sound_load_hint(&mut self, snd: u32, flag: u32) {
        unsafe {
            c_interface::glk_sound_load_hint(snd, flag);
        }
    }


    fn schannel_create_ext(&mut self, rock: u32, volume: u32) -> Self::SChanId {
        let ptr = unsafe { c_interface::glk_schannel_create_ext(rock, volume) };
        SChanId{ ptr }
    }

    fn schannel_play_multi(&mut self, chanarray: &[Self::SChanId], sndarray: &[u32], notify: u32) -> bool {
        let mut chans = Vec::with_capacity(chanarray.len());
        for chan in chanarray {
            chans.push(chan.ptr);
        }
        let result = unsafe {
            c_interface::glk_schannel_play_multi(chans.as_ptr(), chanarray.len() as u32, sndarray.as_ptr(), sndarray.len() as u32, notify)
        };
        result != 0
    }

    fn schannel_pause(&mut self, chan: &Self::SChanId) {
        unsafe {
            c_interface::glk_schannel_pause(chan.ptr);
        }
    }

    fn schannel_unpause(&mut self, chan: &Self::SChanId) {
        unsafe {
            c_interface::glk_schannel_unpause(chan.ptr);
        }
    }

    fn schannel_set_volume_ext(&mut self, chan: &Self::SChanId, vol: u32, duration: u32, notify: u32) {
        unsafe {
            c_interface::glk_schannel_set_volume_ext(chan.ptr, vol, duration, notify);
        }
    }


    fn set_hyperlink(&mut self, linkval: u32) {
        unsafe {
            c_interface::glk_set_hyperlink(linkval);
        }
    }

    fn set_hyperlink_stream(&mut self, str: &Self::StrId, linkval: u32) {
        unsafe {
            c_interface::glk_set_hyperlink_stream(str.ptr, linkval);
        }
    }

    fn request_hyperlink_event(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_request_hyperlink_event(win.ptr);
        }
    }

    fn cancel_hyperlink_event(&mut self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_cancel_hyperlink_event(win.ptr);
        }
    }


    fn current_time(&mut self) -> Self::TimeVal {
        let mut time = Default::default();
        unsafe {
            c_interface::glk_current_time(&mut time);
        }
        TimeVal(time)
    }

    fn current_simple_time(&mut self, factor: u32) -> i32 {
        unsafe {
            c_interface::glk_current_simple_time(factor)
        }
    }

    fn time_to_date_utc(&mut self, time: &Self::TimeVal) -> Self::Date {
        let mut date = Default::default();
        unsafe {
            c_interface::glk_time_to_date_utc(&time.0, &mut date);
        }
        Date(date)
    }

    fn time_to_date_local(&mut self, time: &Self::TimeVal) -> Self::Date {
        let mut date = Default::default();
        unsafe {
            c_interface::glk_time_to_date_local(&time.0, &mut date);
        }
        Date(date)
    }

    fn simple_time_to_date_utc(&mut self, time: i32, factor: u32) -> Self::Date {
        let mut date = Default::default();
        unsafe {
            c_interface::glk_simple_time_to_date_utc(time, factor, &mut date);
        }
        Date(date)
    }

    fn simple_time_to_date_local(&mut self, time: i32, factor: u32) -> Self::Date {
        let mut date = Default::default();
        unsafe {
            c_interface::glk_simple_time_to_date_local(time, factor, &mut date);
        }
        Date(date)
    }

    fn date_to_time_utc(&mut self, date: &Self::Date) -> Self::TimeVal {
        let mut time = Default::default();
        unsafe {
            c_interface::glk_date_to_time_utc(&date.0, &mut time);
        }
        TimeVal(time)
    }

    fn date_to_time_local(&mut self, date: &Self::Date) -> Self::TimeVal {
        let mut time = Default::default();
        unsafe {
            c_interface::glk_date_to_time_local(&date.0, &mut time);
        }
        TimeVal(time)
    }

    fn date_to_simple_time_utc(&mut self, date: &Self::Date, factor: u32) -> i32 {
        unsafe {
            c_interface::glk_date_to_simple_time_utc(&date.0, factor)
        }
    }

    fn date_to_simple_time_local(&mut self, date: &Self::Date, factor: u32) -> i32 {
        unsafe {
            c_interface::glk_date_to_simple_time_local(&date.0, factor)
        }
    }


    fn stream_open_resource(&mut self, filenum: u32, rock: u32) -> Self::StrId {
        let ptr = unsafe { c_interface::glk_stream_open_resource(filenum, rock) };
        StrId{ ptr }
    }

    fn stream_open_resource_uni(&mut self, filenum: u32, rock: u32) -> Self::StrId {
        let ptr = unsafe { c_interface::glk_stream_open_resource_uni(filenum, rock) };
        StrId{ ptr }
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct WinId {
    ptr: c_interface::winid_t,
}

impl IdType for WinId {
    fn null() -> Self {
        WinId{ ptr: std::ptr::null() }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct StrId {
    ptr: c_interface::strid_t,
}

impl IdType for StrId {
    fn null() -> Self {
        StrId{ ptr: std::ptr::null() }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl Read for StrId {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.is_null() {
            return Err(Error::new(ErrorKind::NotConnected, "null stream"))
        }
        let count = unsafe {
            c_interface::glk_get_buffer_stream(self.ptr, buf.as_mut_ptr() as *mut c_char, buf.len() as u32)
        };
        Ok(count as usize)
    }
}

impl Write for StrId {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.is_null() {
            return Err(Error::new(ErrorKind::NotConnected, "null stream"))
        }
        unsafe {
            c_interface::glk_put_buffer_stream(self.ptr, buf.as_ptr() as *const c_char, buf.len() as u32);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct FRefId {
    ptr: c_interface::frefid_t,
}

impl IdType for FRefId {
    fn null() -> Self {
        FRefId{ ptr: std::ptr::null() }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct SChanId {
    ptr: c_interface::schanid_t,
}

impl IdType for SChanId {
    fn null() -> Self {
        SChanId{ ptr: std::ptr::null() }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

pub struct Event(c_interface::event_t);

impl EventType<WinId> for Event {
    fn evtype(&self) -> u32 {
        self.0.evtype
    }

    fn win(&self) -> WinId {
        WinId { ptr: self.0.win }
    }

    fn val1(&self) -> u32 {
        self.0.val1
    }

    fn val2(&self) -> u32 {
        self.0.val2
    }

    fn buf(&mut self) -> Option<(u32,Box<[u8]>)> {
        array_registry::retrieve_line_event()
    }

    fn buf_uni(&mut self) -> Option<(u32,Box<[u32]>)> {
        array_registry::retrieve_line_event_uni()
    }
}

pub struct TimeVal(c_interface::glktimeval_t);

impl TimeValType for TimeVal {
    fn new(high_sec: i32, low_sec: u32, microsec: i32) -> Self {
        TimeVal(c_interface::glktimeval_t{ high_sec, low_sec, microsec })
    }

    fn high_sec(&self) -> i32 {
        self.0.high_sec
    }

    fn low_sec(&self) -> u32 {
        self.0.low_sec
    }

    fn microsec(&self) -> i32 {
        self.0.microsec
    }
}

pub struct Date(c_interface::glkdate_t);

impl DateType for Date {
    fn new(year: i32, month: i32, day: i32, weekday: i32, hour: i32, minute: i32, second: i32, microsec: i32) -> Self {
        Date(c_interface::glkdate_t{ year, month, day, weekday, hour, minute, second, microsec })
    }

    fn year(&self) -> i32 {
        self.0.year
    }

    fn month(&self) -> i32 {
        self.0.month
    }

    fn day(&self) -> i32 {
        self.0.day
    }

    fn weekday(&self) -> i32 {
        self.0.weekday
    }

    fn hour(&self) -> i32 {
        self.0.hour
    }

    fn minute(&self) -> i32 {
        self.0.minute
    }

    fn second(&self) -> i32 {
        self.0.second
    }

    fn microsec(&self) -> i32 {
        self.0.microsec
    }
}
