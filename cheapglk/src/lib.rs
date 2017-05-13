extern crate glk;

use std::io::{Read,Result,Write};

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

    fn exit(&self) -> ! {
        unsafe {
            c_interface::glk_exit();
        }
    }

    fn set_interrupt_handler(&self, handler: extern fn()) {
        let _ = (handler, c_interface::glk_set_interrupt_handler);
        unimplemented!()
    }

    fn tick(&self) {
        unsafe {
            c_interface::glk_tick();
        }
    }


    fn gestalt(&self, sel: u32, val: u32) -> u32 {
        unsafe {
            c_interface::glk_gestalt(sel, val)
        }
    }

    fn gestalt_ext(&self, sel: u32, val: u32, arr: &mut [u32]) -> u32 {
        let _ = (sel,val,arr,c_interface::glk_gestalt_ext);
        unimplemented!()
    }


    fn char_to_lower(&self, ch: u8) -> u8 {
        unsafe {
            c_interface::glk_char_to_lower(ch as std::os::raw::c_uchar) as u8
        }
    }

    fn char_to_upper(&self, ch: u8) -> u8 {
        unsafe {
            c_interface::glk_char_to_upper(ch as std::os::raw::c_uchar) as u8
        }
    }


    fn window_get_root(&self) -> Self::WinId {
        let win = unsafe {
            c_interface::glk_window_get_root()
        };
        WinId{ ptr: win }
    }

    fn window_open(&self, split: &Self::WinId, method: u32, size: u32, wintype: u32, rock: u32) -> Self::WinId {
        let win = unsafe {
            c_interface::glk_window_open(split.ptr, method, size, wintype, rock)
        };
        WinId{ ptr: win }
    }

    fn window_close(&self, win: &mut Self::WinId) -> (u32,u32) {
        let mut result = c_interface::stream_result_t { readcount: 0, writecount: 0 };
        unsafe {
            c_interface::glk_window_close(win.ptr, &mut result);
        }
        win.ptr = std::ptr::null();
        (result.readcount,result.writecount)
    }

    fn window_get_size(&self, win: &Self::WinId) -> (u32,u32) {
        let _ = win;
        unimplemented!()
    }

    fn window_set_arrangement(&self, win: &Self::WinId, method: u32, size: u32, keywin: &Self::WinId) {
        let _ = (win,method,size,keywin);
        unimplemented!()
    }

    fn window_get_arrangement(&self, win: &Self::WinId) -> (u32,u32,Self::WinId) {
        let _ = win;
        unimplemented!()
    }

    fn window_iterate(&self, win: &Self::WinId) -> Self::WinId {
        let _ = win;
        unimplemented!()
    }

    fn window_get_rock(&self, win: &Self::WinId) -> u32 {
        let _ = win;
        unimplemented!()
    }

    fn window_get_type(&self, win: &Self::WinId) -> u32 {
        let _ = win;
        unimplemented!()
    }

    fn window_get_parent(&self, win: &Self::WinId) -> Self::WinId {
        let _ = win;
        unimplemented!()
    }

    fn window_get_sibling(&self, win: &Self::WinId) -> Self::WinId {
        let _ = win;
        unimplemented!()
    }

    fn window_clear(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }

    fn window_move_cursor(&self, win: &Self::WinId, xpos: u32, ypos: u32) {
        let _ = (win,xpos,ypos);
        unimplemented!()
    }


    fn window_get_stream(&self, win: &Self::WinId) -> Self::StrId {
        let _ = win;
        unimplemented!()
    }

    fn window_set_echo_stream(&self, win: &Self::WinId, str: &Self::StrId) {
        let _ = (win,str);
        unimplemented!()
    }

    fn window_get_echo_stream(&self, win: &Self::WinId) -> Self::StrId {
        let _ = win;
        unimplemented!()
    }

    fn set_window(&self, win: &Self::WinId) {
        unsafe {
            c_interface::glk_set_window(win.ptr);
        }
    }


    fn stream_open_file(&self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId {
        let _ = (fileref,fmode,rock);
        unimplemented!()
    }

    fn stream_open_memory(&self, buf: (u32,Box<[u8]>), fmode: u32, rock: u32) -> Self::StrId {
        let _ = (buf,fmode,rock,array_registry::register_stream_memory);
        unimplemented!()
    }

    fn stream_close(&self, str: &mut Self::StrId) -> (u32,u32,Option<(u32,Box<[u8]>)>) {
        let _ = (str,array_registry::retrieve_stream_memory);
        unimplemented!()
    }

    fn stream_iterate(&self, str: &Self::StrId) -> (Self::StrId,u32) {
        let _ = str;
        unimplemented!()
    }

    fn stream_get_rock(&self, str: &Self::StrId) -> u32 {
        let _ = str;
        unimplemented!()
    }

    fn stream_set_position(&self, str: &Self::StrId, pos: i32, seekmode: u32) {
        let _ = (str,pos,seekmode);
        unimplemented!()
    }

    fn stream_get_position(&self, str: &Self::StrId) -> u32 {
        let _ = str;
        unimplemented!()
    }

    fn stream_set_current(&self, str: &Self::StrId) {
        let _ = str;
        unimplemented!()
    }

    fn stream_get_current(&self) -> Self::StrId {
        unimplemented!()
    }


    fn put_char(&self, ch: u8) {
        unsafe {
            c_interface::glk_put_char(ch as std::os::raw::c_uchar);
        }
    }

    fn put_char_stream(&self, str: &Self::StrId, ch: u8) {
        let _ = (str,ch);
        unimplemented!()
    }

    fn put_string<S: AsRef<[u8]>>(&self, s: S) {
        let _ = c_interface::glk_put_string;
        self.put_buffer(s.as_ref());
    }

    fn put_string_stream<S: AsRef<[u8]>>(&self, str: &Self::StrId, s: S) {
        let _ = (str,s);
        unimplemented!()
    }

    fn put_buffer(&self, buf: &[u8]) {
        unsafe {
            c_interface::glk_put_buffer(buf.as_ptr() as *const std::os::raw::c_char, buf.len() as u32);
        }
    }

    fn put_buffer_stream(&self, str: &Self::StrId, buf: &[u8]) {
        let _ = (str,buf);
        unimplemented!()
    }

    fn set_style(&self, styl: u32) {
        unsafe {
            c_interface::glk_set_style(styl);
        }
    }

    fn set_style_stream(&self, str: &Self::StrId, styl: u32) {
        let _ = (str,styl);
        unimplemented!()
    }


    fn get_char_stream(&self, str: &Self::StrId) -> i32 {
        let _ = str;
        unimplemented!()
    }

    fn get_line_stream(&self, str: &Self::StrId, buf: &mut [u8]) -> u32 {
        let _ = (str,buf);
        unimplemented!()
    }

    fn get_buffer_stream(&self, str: &Self::StrId, buf: &mut [u8]) -> u32 {
        let _ = (str,buf);
        unimplemented!()
    }


    fn stylehint_set(&self, wintype: u32, styl: u32, hint: u32, val: i32) {
        let _ = (wintype,styl,hint,val);
        unimplemented!()
    }

    fn stylehint_clear(&self, wintype: u32, styl: u32, hint: u32) {
        let _ = (wintype,styl,hint);
        unimplemented!()
    }

    fn style_distinguish(&self, win: &Self::WinId, styl1: u32, styl2: u32) -> u32 {
        let _ = (win,styl1,styl2);
        unimplemented!()
    }

    fn style_measure(&self, win: &Self::WinId, styl: u32, hint: u32) -> (u32,u32) {
        let _ = (win,styl,hint);
        unimplemented!()
    }


    fn fileref_create_temp(&self, usage: u32, rock: u32) -> Self::FRefId {
        let _ = (usage,rock);
        unimplemented!()
    }

    fn fileref_create_by_name<S: AsRef<[u8]>>(&self, usage: u32, name: S, rock: u32) -> Self::FRefId {
        let _ = (usage,name,rock);
        unimplemented!()
    }

    fn fileref_create_by_prompt(&self, usage: u32, fmode: u32, rock: u32) -> Self::FRefId {
        let _ = (usage,fmode,rock);
        unimplemented!()
    }

    fn fileref_create_from_fileref(&self, usage: u32, fref: &Self::FRefId, rock: u32) -> Self::FRefId {
        let _ = (usage,fref,rock);
        unimplemented!()
    }

    fn fileref_destroy(&self, fref: &mut Self::FRefId) {
        let _ = fref;
        unimplemented!()
    }

    fn fileref_iterate(&self, fref: &Self::FRefId) -> (Self::FRefId,u32) {
        let _ = fref;
        unimplemented!()
    }

    fn fileref_get_rock(&self, fref: &Self::FRefId) -> u32 {
        let _ = fref;
        unimplemented!()
    }

    fn fileref_delete_file(&self, fref: &Self::FRefId) {
        let _ = fref;
        unimplemented!()
    }

    fn fileref_does_file_exist(&self, fref: &Self::FRefId) -> u32 {
        let _ = fref;
        unimplemented!()
    }


    fn select(&self) -> Self::Event {
        let mut event = c_interface::event_t {
            evtype: 0, win: std::ptr::null(), val1: 0, val2: 0
        };
        unsafe {
            c_interface::glk_select(&mut event);
        }
        Event(event)
    }

    fn select_poll(&self) -> Self::Event {
        unimplemented!()
    }


    fn request_timer_events(&self, millisecs: u32) {
        let _ = millisecs;
        unimplemented!()
    }


    fn request_line_event(&self, win: &Self::WinId, buf: (u32,Box<[u8]>), initlen: u32) {
        let maxlen = buf.1.len() as u32;
        let cbuf = array_registry::register_line_event(buf);
        unsafe {
            c_interface::glk_request_line_event(win.ptr, cbuf, maxlen, initlen);
        }
    }

    fn request_char_event(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }

    fn request_mouse_event(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }


    fn cancel_line_event(&self, win: &Self::WinId) -> Self::Event {
        let _ = win;
        unimplemented!()
    }

    fn cancel_char_event(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }

    fn cancel_mouse_event(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }


    fn set_echo_line_event(&self, win: &Self::WinId, val: u32) {
        let _ = (win,val);
        unimplemented!()
    }


    fn set_terminators_line_event(&self, win: &Self::WinId, keycodes: &[u32]) {
        let _ = (win,keycodes);
        unimplemented!()
    }


    fn buffer_to_lower_case_uni(&self, buf: &mut [u32], numchars: u32) -> u32 {
        let _ = (buf,numchars);
        unimplemented!()
    }

    fn buffer_to_upper_case_uni(&self, buf: &mut [u32], numchars: u32) -> u32 {
        let _ = (buf,numchars);
        unimplemented!()
    }

    fn buffer_to_title_case_uni(&self, buf: &mut [u32], numchars: u32, lowerrest: u32) -> u32 {
        let _ = (buf,numchars,lowerrest);
        unimplemented!()
    }


    fn put_char_uni(&self, ch: u32) {
        let _ = ch;
        unimplemented!()
    }

    fn put_string_uni<SU: AsRef<[u32]>>(&mut self, s: SU) {
        let _ = s;
        unimplemented!()
    }

    fn put_buffer_uni(&self, buf: &[u32]) {
        let _ = buf;
        unimplemented!()
    }

    fn put_char_stream_uni(&self, str: &Self::StrId, ch: u32) {
        let _ = (str,ch);
        unimplemented!()
    }

    fn put_string_stream_uni<SU: AsRef<[u32]>>(&mut self, str: &Self::StrId, s: SU) {
        let _ = (str,s);
        unimplemented!()
    }

    fn put_buffer_stream_uni(&self, str: &Self::StrId, buf: &[u32]) {
        let _ = (str,buf);
        unimplemented!()
    }


    fn get_char_stream_uni(&self, str: &Self::StrId) -> i32 {
        let _ = str;
        unimplemented!()
    }

    fn get_buffer_stream_uni(&self, str: &Self::StrId, buf: &mut [u32]) -> u32 {
        let _ = (str,buf);
        unimplemented!()
    }

    fn get_line_stream_uni(&self, str: &Self::StrId, buf: &mut [u32]) -> u32 {
        let _ = (str,buf);
        unimplemented!()
    }


    fn stream_open_file_uni(&self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId {
        let _ = (fileref,fmode,rock);
        unimplemented!()
    }

    fn stream_open_memory_uni(&self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId {
        let _ = (fileref,fmode,rock);
        unimplemented!()
    }


    fn request_char_event_uni(&self, win: &Self::WinId) {
        let _ = win;;
        unimplemented!()
    }

    fn request_line_event_uni(&self, win: &Self::WinId, buf: (u32,Box<[u32]>), initlen: u32) {
        let _ = (win,buf,initlen,array_registry::register_line_event_uni);
        unimplemented!()
    }


    fn buffer_canon_decompose_uni(&self, buf: &mut [u32], numchars: u32) -> u32 {
        let _ = (buf,numchars);
        unimplemented!()
    }

    fn buffer_canon_normalize_uni(&self, buf: &mut [u32], numchars: u32) -> u32 {
        let _ = (buf,numchars);
        unimplemented!()
    }


    fn image_draw(&self, win: &Self::WinId, image: u32, val1: i32, val2: i32) -> u32 {
        let _ = (win,image,val1,val2);
        unimplemented!()
    }

    fn image_draw_scaled(&self, win: &Self::WinId, image: u32, val1: i32, val2: i32, width: u32, height: u32) -> u32 {
        let _ = (win,image,val1,val2,width,height);
        unimplemented!()
    }

    fn image_get_info(&self, image: u32) -> (u32,u32) {
        let _ = image;
        unimplemented!()
    }


    fn window_flow_break(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }


    fn window_erase_rect(&self, win: &Self::WinId, left: i32, top: i32, width: u32, height: u32) {
        let _ = (win,left,top,width,height);
        unimplemented!()
    }

    fn window_fill_rect(&self, win: &Self::WinId, color: u32, left: i32, top: i32, width: u32, height: u32) {
        let _ = (win,color,left,top,width,height);
        unimplemented!()
    }

    fn window_set_background_color(&self, win: &Self::WinId, color: u32) {
        let _ = (win,color);
        unimplemented!()
    }


    fn schannel_create(&self, rock: u32) -> Self::SChanId {
        let _ = rock;
        unimplemented!()
    }

    fn schannel_destroy(&self, chan: &mut Self::SChanId) {
        let _ = chan;
        unimplemented!()
    }

    fn schannel_iterate(&self, chan: &Self::SChanId) -> (Self::SChanId,u32) {
        let _ = chan;
        unimplemented!()
    }

    fn schannel_get_rock(&self, chan: &Self::SChanId) -> u32 {
        let _ = chan;
        unimplemented!()
    }


    fn schannel_play(&self, chan: &Self::SChanId, snd: u32) -> u32 {
        let _ = (chan,snd);
        unimplemented!()
    }

    fn schannel_play_ext(&self, chan: &Self::SChanId, snd: u32, repeat: u32, notify: u32) -> u32 {
        let _ = (chan,snd,repeat,notify);
        unimplemented!()
    }

    fn schannel_stop(&self, chan: &Self::SChanId) {
        let _ = chan;
        unimplemented!()
    }

    fn schannel_set_volume(&self, chan: &Self::SChanId, vol: u32) {
        let _ = (chan,vol);
        unimplemented!()
    }


    fn sound_load_hint(&self, snd: u32, flag: u32) {
        let _ = (snd,flag);
        unimplemented!()
    }


    fn schannel_create_ext(&self, rock: u32, volume: u32) -> Self::SChanId {
        let _ = (rock,volume);
        unimplemented!()
    }

    fn schannel_play_multi(&self, chanarray: &[Self::SChanId], sndarray: &[u32], notify: u32) -> u32 {
        let _ = (chanarray,sndarray,notify);
        unimplemented!()
    }

    fn schannel_pause(&self, chan: &Self::SChanId) {
        let _ = chan;
        unimplemented!()
    }

    fn schannel_unpause(&self, chan: &Self::SChanId) {
        let _ = chan;
        unimplemented!()
    }

    fn schannel_set_volume_ext(&self, chan: &Self::SChanId, vol: u32, duration: u32, notify: u32) {
        let _ = (chan,vol,duration,notify);
        unimplemented!()
    }


    fn set_hyperlink(&self, linkval: u32) {
        let _ = linkval;
        unimplemented!()
    }

    fn set_hyperlink_stream(&self, str: &Self::StrId, linkval: u32) {
        let _ = (str,linkval);
        unimplemented!()
    }

    fn request_hyperlink_event(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }

    fn cancel_hyperlink_event(&self, win: &Self::WinId) {
        let _ = win;
        unimplemented!()
    }


    fn current_time(&self) -> Self::TimeVal {
        unimplemented!()
    }

    fn current_simple_time(&self, factor: u32) -> i32 {
        let _ = factor;
        unimplemented!()
    }

    fn time_to_date_utc(&self, time: &Self::TimeVal) -> Self::Date {
        let _ = time;
        unimplemented!()
    }

    fn time_to_date_local(&self, time: &Self::TimeVal) -> Self::Date {
        let _ = time;
        unimplemented!()
    }

    fn simple_time_to_date_utc(&self, time: i32, factor: u32) -> Self::Date {
        let _ = (time,factor);
        unimplemented!()
    }

    fn simple_time_to_date_local(&self, time: i32, factor: u32) -> Self::Date {
        let _ = (time,factor);
        unimplemented!()
    }

    fn date_to_time_utc(&self, date: &Self::Date) -> Self::TimeVal {
        let _ = date;
        unimplemented!()
    }

    fn date_to_time_local(&self, date: &Self::Date) -> Self::TimeVal {
        let _ = date;
        unimplemented!()
    }

    fn date_to_simple_time_utc(&self, date: &Self::Date, factor: u32) -> i32 {
        let _ = (date,factor);
        unimplemented!()
    }

    fn date_to_simple_time_local(&self, date: &Self::Date, factor: u32) -> i32 {
        let _ = (date,factor);
        unimplemented!()
    }


    fn stream_open_resource(&self, filenum: u32, rock: u32) -> Self::StrId {
        let _ = (filenum,rock);
        unimplemented!()
    }

    fn stream_open_resource_uni(&self, filenum: u32, rock: u32) -> Self::StrId {
        let _ = (filenum,rock);
        unimplemented!()
    }
}

#[derive(Eq,PartialEq)]
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

    fn id(&self) -> u32 {
        unimplemented!()
    }
}

#[derive(Eq,PartialEq)]
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

    fn id(&self) -> u32 {
        unimplemented!()
    }
}

impl Read for StrId {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let _ = buf;
        unimplemented!()
    }
}

impl Write for StrId {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let _ = buf;
        unimplemented!()
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Eq,PartialEq)]
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

    fn id(&self) -> u32 {
        unimplemented!()
    }
}

#[derive(Eq,PartialEq)]
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

    fn id(&self) -> u32 {
        unimplemented!()
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

    fn buf(&self) -> Option<(u32,Box<[u8]>)> {
        array_registry::retrieve_line_event()
    }

    fn buf_uni(&self) -> Option<(u32,Box<[u32]>)> {
        array_registry::retrieve_line_event_uni()
    }
}

pub struct TimeVal {
}

impl TimeValType for TimeVal {
    fn high_sec(&self) -> i32 {
        unimplemented!()
    }

    fn low_sec(&self) -> u32 {
        unimplemented!()
    }

    fn microsec(&self) -> i32 {
        unimplemented!()
    }
}

pub struct Date {
}

impl DateType for Date {
    fn year(&self) -> i32 {
        unimplemented!()
    }

    fn month(&self) -> i32 {
        unimplemented!()
    }

    fn day(&self) -> i32 {
        unimplemented!()
    }

    fn weekday(&self) -> i32 {
        unimplemented!()
    }

    fn hour(&self) -> i32 {
        unimplemented!()
    }

    fn minute(&self) -> i32 {
        unimplemented!()
    }

    fn second(&self) -> i32 {
        unimplemented!()
    }

    fn microsec(&self) -> i32 {
        unimplemented!()
    }
}
