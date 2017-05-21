extern crate glk;

use std::io::{Error,ErrorKind,Read,Result,Write};

use glk::{Glk,DateType,EventType,IdType,TimeValType};

pub enum TestOutput<'a> {
    Match(&'a str),
    Check(&'a Fn(&str) -> bool),
}

struct MemoryStream {
    rock: u32,
    readcount: u32,
    writecount: u32,
    out: (u32,Box<[u8]>),
}

pub struct GlkTest<'a> {
    root: usize,
    rock: u32,
    current: usize,
    streams: Vec<Option<MemoryStream>>,
    out: String,
    line_input: Option<(u32,Box<[u8]>)>,

    test: Vec<(TestOutput<'a>,&'a str)>,
}

impl<'a> GlkTest<'a> {
    pub fn new(test: Vec<(TestOutput<'a>,&'a str)>) -> Self {
        GlkTest{
            root: 0,
            rock: 0,
            current: 0,
            streams: vec![None],
            out: String::new(),
            line_input: None,
            test: test,
        }
    }
}

impl<'a> Glk for GlkTest<'a> {
    type WinId = WinId;
    type StrId = StrId;
    type FRefId = FRefId;
    type SChanId = SChanId;
    type Event = Event;
    type TimeVal = TimeVal;
    type Date = Date;

    fn exit(&mut self) -> ! {
        std::process::exit(0);
    }

    fn set_interrupt_handler(&mut self, _handler: extern fn()) {
    }

    fn tick(&mut self) {
    }


    fn gestalt(&mut self, sel: u32, val: u32) -> u32 {
        match sel {
            glk::gestalt_Version => 0x00000705,
            glk::gestalt_LineInput => if val >= 32 && val < 127 { 1 } else { 0 },
            glk::gestalt_CharOutput => {
                if val >= 32 && val < 127 {
                    glk::gestalt_CharOutput_ExactPrint
                } else {
                    glk::gestalt_CharOutput_CannotPrint
                }
            },
            _ => 0,
        }
    }

    fn gestalt_ext(&mut self, sel: u32, val: u32, arr: &mut [u32]) -> u32 {
        match sel {
            glk::gestalt_Version => 0x00000705,
            glk::gestalt_LineInput => if val >= 32 && val < 127 { 1 } else { 0 },
            glk::gestalt_CharOutput => {
                if val >= 32 && val < 127 {
                    if arr.len() > 0 {
                        arr[0] = 1;
                    }
                    glk::gestalt_CharOutput_ExactPrint
                } else {
                    glk::gestalt_CharOutput_CannotPrint
                }
            },
            _ => 0,
        }
    }


    fn char_to_lower(&mut self, ch: u8) -> u8 {
        (ch as char).to_lowercase().next().unwrap() as u8
    }

    fn char_to_upper(&mut self, ch: u8) -> u8 {
        (ch as char).to_uppercase().next().unwrap() as u8
    }


    fn window_get_root(&mut self) -> Self::WinId {
        WinId(self.root)
    }

    fn window_open(&mut self, _split: &Self::WinId, _method: u32, _size: u32, wintype: u32, rock: u32) -> Self::WinId {
        if self.root != 0 || wintype != glk::wintype_TextBuffer {
            WinId(0)
        } else {
            let mut index = 0;
            for i in 1 .. self.streams.len() {
                if self.streams[i].is_none() {
                    index = i;
                    break;
                }
            }
            if index == 0 {
                index = self.streams.len();
                self.streams.push(None);
            }
            self.streams[index] = Some(MemoryStream{
                    rock: 0,
                    readcount: 0,
                    writecount: 0,
                    out: (0,vec![].into_boxed_slice()),
                });
            self.root = index;
            self.rock = rock;
            WinId(index)
        }
    }

    fn window_close(&mut self, win: &mut Self::WinId) -> (u32,u32,Option<(u32,Box<[u8]>)>,Option<(u32,Box<[u32]>)>) {
        if self.root == 0 || win.0 != self.root {
            (0,0,None,None)
        } else {
            let mut str = StrId(self.root);
            self.stream_close(&mut str)
        }
    }

    fn window_get_size(&mut self, _win: &Self::WinId) -> (u32,u32) {
        (80,24)
    }

    fn window_set_arrangement(&mut self, _win: &Self::WinId, _method: u32, _size: u32, _keywin: &Self::WinId) {
    }

    fn window_get_arrangement(&mut self, _win: &Self::WinId) -> (u32,u32,Self::WinId) {
        (0,0,WinId(0))
    }

    fn window_iterate(&mut self, win: &Self::WinId) -> (Self::WinId,u32) {
        if win.0 == 0 {
            (WinId(self.root),self.rock)
        } else {
            (WinId(0),0)
        }
    }

    fn window_get_rock(&mut self, win: &Self::WinId) -> u32 {
        if win.0 == self.root { self.rock } else { 0 }
    }

    fn window_get_type(&mut self, win: &Self::WinId) -> u32 {
        if win.0 == self.root { glk::wintype_TextBuffer } else { 0 }
    }

    fn window_get_parent(&mut self, _win: &Self::WinId) -> Self::WinId {
        WinId(0)
    }

    fn window_get_sibling(&mut self, _win: &Self::WinId) -> Self::WinId {
        WinId(0)
    }

    fn window_clear(&mut self, _win: &Self::WinId) {
    }

    fn window_move_cursor(&mut self, _win: &Self::WinId, _xpos: u32, _ypos: u32) {
    }


    fn window_get_stream(&mut self, win: &Self::WinId) -> Self::StrId {
        StrId(win.0)
    }

    fn window_set_echo_stream(&mut self, _win: &Self::WinId, _str: &Self::StrId) {
    }

    fn window_get_echo_stream(&mut self, _win: &Self::WinId) -> Self::StrId {
        StrId(0)
    }

    fn set_window(&mut self, win: &Self::WinId) {
        self.current = win.0;
    }


    fn stream_open_file(&mut self, _fileref: &Self::FRefId, _fmode: u32, _rock: u32) -> Self::StrId {
        StrId(0)
    }

    fn stream_open_memory(&mut self, buf: (u32,Box<[u8]>), _fmode: u32, rock: u32) -> Self::StrId {
        let mut index = 0;
        for i in 1 .. self.streams.len() {
            if self.streams[i].is_none() {
                index = i;
                break;
            }
        }
        if index == 0 {
            index = self.streams.len();
            self.streams.push(None);
        }
        self.streams[index] = Some(MemoryStream{
                rock: rock,
                readcount: 0,
                writecount: 0,
                out: buf,
            });
        StrId(index)
    }

    fn stream_close(&mut self, str: &mut Self::StrId) -> (u32,u32,Option<(u32,Box<[u8]>)>,Option<(u32,Box<[u32]>)>) {
        if str.0 >= self.streams.len() || self.streams[str.0].is_none() {
            return (0,0,None,None);
        }
        let stream = self.streams[str.0].take().unwrap();
        (stream.readcount,stream.writecount,Some(stream.out),None)
    }

    fn stream_iterate(&mut self, str: &Self::StrId) -> (Self::StrId,u32) {
        if str.0 < self.streams.len() {
            for i in str.0 + 1 .. self.streams.len() {
                if let &Some(MemoryStream{ rock, readcount:_, writecount:_ , out:_ }) = &self.streams[i] {
                    return (StrId(i),rock);
                }
            }
        }
        (StrId(0),0)
    }

    fn stream_get_rock(&mut self, str: &Self::StrId) -> u32 {
        if let Some(&Some(MemoryStream{ rock, readcount:_, writecount:_ , out:_ })) = self.streams.get(str.0) {
            rock
        } else {
            0
        }
    }

    fn stream_set_position(&mut self, _str: &Self::StrId, _pos: i32, _seekmode: u32) {
        panic!();
    }

    fn stream_get_position(&mut self, str: &Self::StrId) -> u32 {
        if let Some(&Some(MemoryStream{ rock:_, readcount:_, writecount, out:_ })) = self.streams.get(str.0) {
            writecount
        } else {
            0
        }
    }

    fn stream_set_current(&mut self, str: &Self::StrId) {
        self.current = str.0;
    }

    fn stream_get_current(&mut self) -> Self::StrId {
        StrId(self.current)
    }


    fn put_char(&mut self, ch: u8) {
        if let Some(&mut Some(MemoryStream{ rock:_, readcount:_, ref mut writecount, out:(_,ref mut mem) })) = self.streams.get_mut(self.current) {
            if self.current == self.root {
                self.out.push(ch as char);
            } else if *writecount < mem.len() as u32 {
                mem[*writecount as usize] = ch;
            }
            *writecount += 1;
        }
    }

    fn put_char_stream(&mut self, str: &Self::StrId, ch: u8) {
        if let Some(&mut Some(MemoryStream{ rock:_, readcount:_, ref mut writecount, out:(_,ref mut mem) })) = self.streams.get_mut(str.0) {
            if str.0 == self.root {
                self.out.push(ch as char);
            } else if *writecount < mem.len() as u32 {
                mem[*writecount as usize] = ch;
            }
            *writecount += 1;
        }
    }

    fn put_string<S: AsRef<[u8]>>(&mut self, s: S) {
        for ch in s.as_ref() {
            self.put_char(*ch);
        }
    }

    fn put_string_stream<S: AsRef<[u8]>>(&mut self, str: &Self::StrId, s: S) {
        for ch in s.as_ref() {
            self.put_char_stream(str, *ch);
        }
    }

    fn put_buffer(&mut self, buf: &[u8]) {
        for ch in buf {
            self.put_char(*ch);
        }
    }

    fn put_buffer_stream(&mut self, str: &Self::StrId, buf: &[u8]) {
        for ch in buf {
            self.put_char_stream(str, *ch);
        }
    }

    fn set_style(&mut self, _styl: u32) {
    }

    fn set_style_stream(&mut self, _str: &Self::StrId, _styl: u32) {
    }


    fn get_char_stream(&mut self, _str: &Self::StrId) -> i32 {
        panic!()
    }

    fn get_line_stream(&mut self, _str: &Self::StrId, _buf: &mut [u8]) -> u32 {
        panic!()
    }

    fn get_buffer_stream(&mut self, _str: &Self::StrId, _buf: &mut [u8]) -> u32 {
        panic!()
    }


    fn stylehint_set(&mut self, _wintype: u32, _styl: u32, _hint: u32, _val: i32) {
    }

    fn stylehint_clear(&mut self, _wintype: u32, _styl: u32, _hint: u32) {
    }

    fn style_distinguish(&mut self, _win: &Self::WinId, styl1: u32, styl2: u32) -> bool {
        styl1 != styl2
    }

    fn style_measure(&mut self, _win: &Self::WinId, _styl: u32, _hint: u32) -> (bool,u32) {
        (false,0)
    }


    fn fileref_create_temp(&mut self, _usage: u32, _rock: u32) -> Self::FRefId {
        FRefId(())
    }

    fn fileref_create_by_name<S: AsRef<[u8]>>(&mut self, _usage: u32, _name: S, _rock: u32) -> Self::FRefId {
        FRefId(())
    }

    fn fileref_create_by_prompt(&mut self, _usage: u32, _fmode: u32, _rock: u32) -> Self::FRefId {
        FRefId(())
    }

    fn fileref_create_from_fileref(&mut self, _usage: u32, _fref: &Self::FRefId, _rock: u32) -> Self::FRefId {
        FRefId(())
    }

    fn fileref_destroy(&mut self, _fref: &mut Self::FRefId) {
    }

    fn fileref_iterate(&mut self, _fref: &Self::FRefId) -> (Self::FRefId,u32) {
        (FRefId(()),0)
    }

    fn fileref_get_rock(&mut self, _fref: &Self::FRefId) -> u32 {
        0
    }

    fn fileref_delete_file(&mut self, _fref: &Self::FRefId) {
    }

    fn fileref_does_file_exist(&mut self, _fref: &Self::FRefId) -> bool {
        false
    }


    fn select(&mut self) -> Self::Event {
        if self.line_input.is_none() {
            Event::None
        } else {
            let (test,line) = self.test.remove(0);
            match test {
                TestOutput::Match(expected) => {
                    assert_eq!(expected, self.out.as_str());
                },
                TestOutput::Check(f) => {
                    assert!(f(&self.out));
                },
            }
            self.out.clear();
            let mut input = self.line_input.take().unwrap();
            {
                let mut buf = &mut input.1;
                for i in 0 .. std::cmp::min(buf.len(), line.as_bytes().len()) {
                    buf[i] = line.as_bytes()[i];
                }
            }
            Event::Line((self.root,line.as_bytes().len() as u32,Some(input)))
        }
    }

    fn select_poll(&mut self) -> Self::Event {
        self.select()
    }


    fn request_timer_events(&mut self, _millisecs: u32) {
    }


    fn request_line_event(&mut self, win: &Self::WinId, buf: (u32,Box<[u8]>), _initlen: u32) {
        if win.0 == self.root && self.root != 0 {
            self.line_input = Some(buf);
        }
    }

    fn request_char_event(&mut self, _win: &Self::WinId) {
    }

    fn request_mouse_event(&mut self, _win: &Self::WinId) {
    }


    fn cancel_line_event(&mut self, _win: &Self::WinId) -> Self::Event {
        Event::None
    }

    fn cancel_char_event(&mut self, _win: &Self::WinId) {
    }

    fn cancel_mouse_event(&mut self, _win: &Self::WinId) {
    }


    fn set_echo_line_event(&mut self, _win: &Self::WinId, _val: u32) {
    }


    fn set_terminators_line_event(&mut self, _win: &Self::WinId, _keycodes: &[u32]) {
    }


    fn buffer_to_lower_case_uni(&mut self, _buf: &mut [u32], _numchars: u32) -> u32 {
        0
    }

    fn buffer_to_upper_case_uni(&mut self, _buf: &mut [u32], _numchars: u32) -> u32 {
        0
    }

    fn buffer_to_title_case_uni(&mut self, _buf: &mut [u32], _numchars: u32, _lowerrest: u32) -> u32 {
        0
    }


    fn put_char_uni(&mut self, ch: u32) {
        if let Some(&mut Some(MemoryStream{ rock:_, readcount:_, ref mut writecount, out:(_,ref mut mem) })) = self.streams.get_mut(self.current) {
            if self.current == self.root {
                if let Some(c) = std::char::from_u32(ch) {
                    self.out.push(c);
                }
            } else if *writecount*4+3 < mem.len() as u32 {
                mem[*writecount as usize * 4] = (ch >> 24) as u8;
                mem[*writecount as usize * 4 + 1] = (ch >> 16) as u8;
                mem[*writecount as usize * 4 + 2] = (ch >> 8) as u8;
                mem[*writecount as usize * 4 + 3] = ch as u8;
            }
            *writecount += 1;
        }
    }

    fn put_string_uni<SU: AsRef<[u32]>>(&mut self, s: SU) {
        for ch in s.as_ref() {
            self.put_char_uni(*ch);
        }
    }

    fn put_buffer_uni(&mut self, buf: &[u32]) {
        for ch in buf {
            self.put_char_uni(*ch);
        }
    }

    fn put_char_stream_uni(&mut self, str: &Self::StrId, ch: u32) {
        if let Some(&mut Some(MemoryStream{ rock:_, readcount:_, ref mut writecount, out:(_,ref mut mem) })) = self.streams.get_mut(str.0) {
            if str.0 == self.root {
                if let Some(c) = std::char::from_u32(ch) {
                    self.out.push(c);
                }
            } else if *writecount < mem.len() as u32 {
                mem[*writecount as usize * 4] = (ch >> 24) as u8;
                mem[*writecount as usize * 4 + 1] = (ch >> 16) as u8;
                mem[*writecount as usize * 4 + 2] = (ch >> 8) as u8;
                mem[*writecount as usize * 4 + 3] = ch as u8;
            }
            *writecount += 1;
        }
    }

    fn put_string_stream_uni<SU: AsRef<[u32]>>(&mut self, str: &Self::StrId, s: SU) {
        for ch in s.as_ref() {
            self.put_char_stream_uni(str, *ch);
        }
    }

    fn put_buffer_stream_uni(&mut self, str: &Self::StrId, buf: &[u32]) {
        for ch in buf {
            self.put_char_stream_uni(str, *ch);
        }
    }


    fn get_char_stream_uni(&mut self, _str: &Self::StrId) -> i32 {
        panic!()
    }

    fn get_buffer_stream_uni(&mut self, _str: &Self::StrId, _buf: &mut [u32]) -> u32 {
        panic!()
    }

    fn get_line_stream_uni(&mut self, _str: &Self::StrId, _buf: &mut [u32]) -> u32 {
        panic!()
    }


    fn stream_open_file_uni(&mut self, _fileref: &Self::FRefId, _fmode: u32, _rock: u32) -> Self::StrId {
        StrId(0)
    }

    fn stream_open_memory_uni(&mut self, _buf: (u32,Box<[u32]>), _fmode: u32, _rock: u32) -> Self::StrId {
        StrId(0)
    }


    fn request_char_event_uni(&mut self, _win: &Self::WinId) {
    }

    fn request_line_event_uni(&mut self, win: &Self::WinId, buf: (u32,Box<[u32]>), _initlen: u32) {
        let _ = (win,buf);
    }


    fn buffer_canon_decompose_uni(&mut self, _buf: &mut [u32], _numchars: u32) -> u32 {
        0
    }

    fn buffer_canon_normalize_uni(&mut self, _buf: &mut [u32], _numchars: u32) -> u32 {
        0
    }


    fn image_draw(&mut self, _win: &Self::WinId, _image: u32, _val1: i32, _val2: i32) -> bool {
        false
    }

    fn image_draw_scaled(&mut self, _win: &Self::WinId, _image: u32, _val1: i32, _val2: i32, _width: u32, _height: u32) -> bool {
        false
    }

    fn image_get_info(&mut self, _image: u32) -> (bool,u32,u32) {
        (false,0,0)
    }


    fn window_flow_break(&mut self, _win: &Self::WinId) {
    }


    fn window_erase_rect(&mut self, _win: &Self::WinId, _left: i32, _top: i32, _width: u32, _height: u32) {
    }

    fn window_fill_rect(&mut self, _win: &Self::WinId, _color: u32, _left: i32, _top: i32, _width: u32, _height: u32) {
    }

    fn window_set_background_color(&mut self, _win: &Self::WinId, _color: u32) {
    }


    fn schannel_create(&mut self, _rock: u32) -> Self::SChanId {
        SChanId(())
    }

    fn schannel_destroy(&mut self, _chan: &mut Self::SChanId) {
    }

    fn schannel_iterate(&mut self, _chan: &Self::SChanId) -> (Self::SChanId,u32) {
        (SChanId(()),0)
    }

    fn schannel_get_rock(&mut self, _chan: &Self::SChanId) -> u32 {
        0
    }


    fn schannel_play(&mut self, _chan: &Self::SChanId, _snd: u32) -> bool {
        false
    }

    fn schannel_play_ext(&mut self, _chan: &Self::SChanId, _snd: u32, _repeat: u32, _notify: u32) -> bool {
        false
    }

    fn schannel_stop(&mut self, _chan: &Self::SChanId) {
    }

    fn schannel_set_volume(&mut self, _chan: &Self::SChanId, _vol: u32) {
    }


    fn sound_load_hint(&mut self, _snd: u32, _flag: u32) {
    }


    fn schannel_create_ext(&mut self, _rock: u32, _volume: u32) -> Self::SChanId {
        SChanId(())
    }

    fn schannel_play_multi(&mut self, _chanarray: &[Self::SChanId], _sndarray: &[u32], _notify: u32) -> bool {
        false
    }

    fn schannel_pause(&mut self, _chan: &Self::SChanId) {
    }

    fn schannel_unpause(&mut self, _chan: &Self::SChanId) {
    }

    fn schannel_set_volume_ext(&mut self, _chan: &Self::SChanId, _vol: u32, _duration: u32, _notify: u32) {
    }


    fn set_hyperlink(&mut self, _linkval: u32) {
    }

    fn set_hyperlink_stream(&mut self, _str: &Self::StrId, _linkval: u32) {
    }

    fn request_hyperlink_event(&mut self, _win: &Self::WinId) {
    }

    fn cancel_hyperlink_event(&mut self, _win: &Self::WinId) {
    }


    fn current_time(&mut self) -> Self::TimeVal {
        TimeVal(())
    }

    fn current_simple_time(&mut self, _factor: u32) -> i32 {
        0
    }

    fn time_to_date_utc(&mut self, _time: &Self::TimeVal) -> Self::Date {
        Date(())
    }

    fn time_to_date_local(&mut self, _time: &Self::TimeVal) -> Self::Date {
        Date(())
    }

    fn simple_time_to_date_utc(&mut self, _time: i32, _factor: u32) -> Self::Date {
        Date(())
    }

    fn simple_time_to_date_local(&mut self, _time: i32, _factor: u32) -> Self::Date {
        Date(())
    }

    fn date_to_time_utc(&mut self, _date: &Self::Date) -> Self::TimeVal {
        TimeVal(())
    }

    fn date_to_time_local(&mut self, _date: &Self::Date) -> Self::TimeVal {
        TimeVal(())
    }

    fn date_to_simple_time_utc(&mut self, _date: &Self::Date, _factor: u32) -> i32 {
        0
    }

    fn date_to_simple_time_local(&mut self, _date: &Self::Date, _factor: u32) -> i32 {
        0
    }


    fn stream_open_resource(&mut self, _filenum: u32, _rock: u32) -> Self::StrId {
        StrId(0)
    }

    fn stream_open_resource_uni(&mut self, _filenum: u32, _rock: u32) -> Self::StrId {
        StrId(0)
    }


    fn set_resource_map(&mut self, _str: Self::StrId) -> u32 {
        0
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct WinId(usize);

impl IdType for WinId {
    fn null() -> Self {
        WinId(0)
    }

    fn is_null(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct StrId(usize);

impl IdType for StrId {
    fn null() -> Self {
        StrId(0)
    }

    fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl Read for StrId {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        return Err(Error::new(ErrorKind::NotConnected, "not implemented"))
    }
}

impl Write for StrId {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        return Err(Error::new(ErrorKind::NotConnected, "not implemented"))
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct FRefId(());

impl IdType for FRefId {
    fn null() -> Self {
        FRefId(())
    }

    fn is_null(&self) -> bool {
        true
    }
}

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct SChanId(());

impl IdType for SChanId {
    fn null() -> Self {
        SChanId(())
    }

    fn is_null(&self) -> bool {
        true
    }
}

pub enum Event {
    Line((usize,u32,Option<(u32,Box<[u8]>)>)),
    None,
}

impl EventType<WinId> for Event {
    fn evtype(&self) -> u32 {
        match self {
            &Event::Line(_) => glk::evtype_LineInput,
            &Event::None => glk::evtype_None,
        }
    }

    fn win(&self) -> WinId {
        match self {
            &Event::Line((win,_,_)) => WinId(win),
            &Event::None => WinId(0),
        }
    }

    fn val1(&self) -> u32 {
        match self {
            &Event::Line((_,val,_)) => val,
            &Event::None => 0,
        }
    }

    fn val2(&self) -> u32 {
        0
    }

    fn buf(&mut self) -> Option<(u32,Box<[u8]>)> {
        match self {
            &mut Event::Line((_,_,ref mut buf)) => buf.take(),
            &mut Event::None => None,
        }
    }

    fn buf_uni(&mut self) -> Option<(u32,Box<[u32]>)> {
        None
    }
}

pub struct TimeVal(());

impl TimeValType for TimeVal {
    fn new(_high_sec: i32, _low_sec: u32, _microsec: i32) -> Self {
        TimeVal(())
    }

    fn high_sec(&self) -> i32 {
        0
    }

    fn low_sec(&self) -> u32 {
        0
    }

    fn microsec(&self) -> i32 {
        0
    }
}

pub struct Date(());

impl DateType for Date {
     fn new(_year: i32, _month: i32, _day: i32, _weekday: i32, _hour: i32, _minute: i32, _second: i32, _microsec: i32) -> Self {
        Date(())
    }

   fn year(&self) -> i32 {
        0
    }

    fn month(&self) -> i32 {
        0
    }

    fn day(&self) -> i32 {
        0
    }

    fn weekday(&self) -> i32 {
        0
    }

    fn hour(&self) -> i32 {
        0
    }

    fn minute(&self) -> i32 {
        0
    }

    fn second(&self) -> i32 {
        0
    }

    fn microsec(&self) -> i32 {
        0
    }
}
