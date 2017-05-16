#![allow(non_upper_case_globals)]

use std::io::{Read,Write};

pub trait Glk {
    type WinId: IdType;
    type StrId: IdType + Read + Write;
    type FRefId: IdType;
    type SChanId: IdType;
    type Event: EventType<Self::WinId>;
    type TimeVal: TimeValType;
    type Date: DateType;

    fn exit(&mut self) -> !;
    fn set_interrupt_handler(&mut self, handler: extern fn());
    fn tick(&mut self);

    fn gestalt(&mut self, sel: u32, val: u32) -> u32;
    fn gestalt_ext(&mut self, sel: u32, val: u32, arr: &mut [u32]) -> u32;

    fn char_to_lower(&mut self, ch: u8) -> u8;
    fn char_to_upper(&mut self, ch: u8) -> u8;

    fn window_get_root(&mut self) -> Self::WinId;
    fn window_open(&mut self, split: &Self::WinId, method: u32, size: u32, wintype: u32, rock: u32) -> Self::WinId;
    fn window_close(&mut self, win: &mut Self::WinId) -> (u32,u32);
    fn window_get_size(&mut self, win: &Self::WinId) -> (u32,u32);
    fn window_set_arrangement(&mut self, win: &Self::WinId, method: u32, size: u32, keywin: &Self::WinId);
    fn window_get_arrangement(&mut self, win: &Self::WinId) -> (u32,u32,Self::WinId);
    fn window_iterate(&mut self, win: &Self::WinId) -> (Self::WinId,u32);
    fn window_get_rock(&mut self, win: &Self::WinId) -> u32;
    fn window_get_type(&mut self, win: &Self::WinId) -> u32;
    fn window_get_parent(&mut self, win: &Self::WinId) -> Self::WinId;
    fn window_get_sibling(&mut self, win: &Self::WinId) -> Self::WinId;
    fn window_clear(&mut self, win: &Self::WinId);
    fn window_move_cursor(&mut self, win: &Self::WinId, xpos: u32, ypos: u32);

    fn window_get_stream(&mut self, win: &Self::WinId) -> Self::StrId;
    fn window_set_echo_stream(&mut self, win: &Self::WinId, str: &Self::StrId);
    fn window_get_echo_stream(&mut self, win: &Self::WinId) -> Self::StrId;
    fn set_window(&mut self, win: &Self::WinId);

    fn stream_open_file(&mut self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId;
    fn stream_open_memory(&mut self, buf: (u32,Box<[u8]>), fmode: u32, rock: u32) -> Self::StrId;
    fn stream_close(&mut self, str: &mut Self::StrId) -> (u32,u32,Option<(u32,Box<[u8]>)>,Option<(u32,Box<[u32]>)>);
    fn stream_iterate(&mut self, str: &Self::StrId) -> (Self::StrId,u32);
    fn stream_get_rock(&mut self, str: &Self::StrId) -> u32;
    fn stream_set_position(&mut self, str: &Self::StrId, pos: i32, seekmode: u32);
    fn stream_get_position(&mut self, str: &Self::StrId) -> u32;
    fn stream_set_current(&mut self, str: &Self::StrId);
    fn stream_get_current(&mut self) -> Self::StrId;

    fn put_char(&mut self, ch: u8);
    fn put_char_stream(&mut self, str: &Self::StrId, ch: u8);
    fn put_string<S: AsRef<[u8]>>(&mut self, s: S);
    fn put_string_stream<S: AsRef<[u8]>>(&mut self, str: &Self::StrId, s: S);
    fn put_buffer(&mut self, buf: &[u8]);
    fn put_buffer_stream(&mut self, str: &Self::StrId, buf: &[u8]);
    fn set_style(&mut self, styl: u32);
    fn set_style_stream(&mut self, str: &Self::StrId, styl: u32);

    fn get_char_stream(&mut self, str: &Self::StrId) -> i32;
    fn get_line_stream(&mut self, str: &Self::StrId, buf: &mut [u8]) -> u32;
    fn get_buffer_stream(&mut self, str: &Self::StrId, buf: &mut [u8]) -> u32;

    fn stylehint_set(&mut self, wintype: u32, styl: u32, hint: u32, val: i32);
    fn stylehint_clear(&mut self, wintype: u32, styl: u32, hint: u32);
    fn style_distinguish(&mut self, win: &Self::WinId, styl1: u32, styl2: u32) -> bool;
    fn style_measure(&mut self, win: &Self::WinId, styl: u32, hint: u32) -> (bool,u32);

    fn fileref_create_temp(&mut self, usage: u32, rock: u32) -> Self::FRefId;
    fn fileref_create_by_name<S: AsRef<[u8]>>(&mut self, usage: u32, name: S, rock: u32) -> Self::FRefId;
    fn fileref_create_by_prompt(&mut self, usage: u32, fmode: u32, rock: u32) -> Self::FRefId;
    fn fileref_create_from_fileref(&mut self, usage: u32, fref: &Self::FRefId, rock: u32) -> Self::FRefId;
    fn fileref_destroy(&mut self, fref: &mut Self::FRefId);
    fn fileref_iterate(&mut self, fref: &Self::FRefId) -> (Self::FRefId,u32);
    fn fileref_get_rock(&mut self, fref: &Self::FRefId) -> u32;
    fn fileref_delete_file(&mut self, fref: &Self::FRefId);
    fn fileref_does_file_exist(&mut self, fref: &Self::FRefId) -> bool;

    fn select(&mut self) -> Self::Event;
    fn select_poll(&mut self) -> Self::Event;

    fn request_timer_events(&mut self, millisecs: u32);

    fn request_line_event(&mut self, win: &Self::WinId, buf: (u32,Box<[u8]>), initlen: u32);
    fn request_char_event(&mut self, win: &Self::WinId);
    fn request_mouse_event(&mut self, win: &Self::WinId);

    fn cancel_line_event(&mut self, win: &Self::WinId) -> Self::Event;
    fn cancel_char_event(&mut self, win: &Self::WinId);
    fn cancel_mouse_event(&mut self, win: &Self::WinId);

    fn set_echo_line_event(&mut self, win: &Self::WinId, val: u32);

    fn set_terminators_line_event(&mut self, win: &Self::WinId, keycodes: &[u32]);

    fn buffer_to_lower_case_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32;
    fn buffer_to_upper_case_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32;
    fn buffer_to_title_case_uni(&mut self, buf: &mut [u32], numchars: u32, lowerrest: u32) -> u32;

    fn put_char_uni(&mut self, ch: u32);
    fn put_string_uni<SU: AsRef<[u32]>>(&mut self, s: SU);
    fn put_buffer_uni(&mut self, buf: &[u32]);
    fn put_char_stream_uni(&mut self, str: &Self::StrId, ch: u32);
    fn put_string_stream_uni<SU: AsRef<[u32]>>(&mut self, str: &Self::StrId, s: SU);
    fn put_buffer_stream_uni(&mut self, str: &Self::StrId, buf: &[u32]);

    fn get_char_stream_uni(&mut self, str: &Self::StrId) -> i32;
    fn get_buffer_stream_uni(&mut self, str: &Self::StrId, buf: &mut [u32]) -> u32;
    fn get_line_stream_uni(&mut self, str: &Self::StrId, buf: &mut [u32]) -> u32;

    fn stream_open_file_uni(&mut self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId;
    fn stream_open_memory_uni(&mut self, buf: (u32,Box<[u32]>), fmode: u32, rock: u32) -> Self::StrId;

    fn request_char_event_uni(&mut self, win: &Self::WinId);
    fn request_line_event_uni(&mut self, win: &Self::WinId, buf: (u32,Box<[u32]>), initlen: u32);

    fn buffer_canon_decompose_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32;
    fn buffer_canon_normalize_uni(&mut self, buf: &mut [u32], numchars: u32) -> u32;

    fn image_draw(&mut self, win: &Self::WinId, image: u32, val1: i32, val2: i32) -> bool;
    fn image_draw_scaled(&mut self, win: &Self::WinId, image: u32, val1: i32, val2: i32, width: u32, height: u32) -> bool;
    fn image_get_info(&mut self, image: u32) -> (bool,u32,u32);

    fn window_flow_break(&mut self, win: &Self::WinId);

    fn window_erase_rect(&mut self, win: &Self::WinId, left: i32, top: i32, width: u32, height: u32);
    fn window_fill_rect(&mut self, win: &Self::WinId, color: u32, left: i32, top: i32, width: u32, height: u32);
    fn window_set_background_color(&mut self, win: &Self::WinId, color: u32);

    fn schannel_create(&mut self, rock: u32) -> Self::SChanId;
    fn schannel_destroy(&mut self, chan: &mut Self::SChanId);
    fn schannel_iterate(&mut self, chan: &Self::SChanId) -> (Self::SChanId,u32);
    fn schannel_get_rock(&mut self, chan: &Self::SChanId) -> u32;

    fn schannel_play(&mut self, chan: &Self::SChanId, snd: u32) -> bool;
    fn schannel_play_ext(&mut self, chan: &Self::SChanId, snd: u32, repeat: u32, notify: u32) -> bool;
    fn schannel_stop(&mut self, chan: &Self::SChanId);
    fn schannel_set_volume(&mut self, chan: &Self::SChanId, vol: u32);

    fn sound_load_hint(&mut self, snd: u32, flag: u32);

    fn schannel_create_ext(&mut self, rock: u32, volume: u32) -> Self::SChanId;
    fn schannel_play_multi(&mut self, chanarray: &[Self::SChanId], sndarray: &[u32], notify: u32) -> bool;
    fn schannel_pause(&mut self, chan: &Self::SChanId);
    fn schannel_unpause(&mut self, chan: &Self::SChanId);
    fn schannel_set_volume_ext(&mut self, chan: &Self::SChanId, vol: u32, duration: u32, notify: u32);

    fn set_hyperlink(&mut self, linkval: u32);
    fn set_hyperlink_stream(&mut self, str: &Self::StrId, linkval: u32);
    fn request_hyperlink_event(&mut self, win: &Self::WinId);
    fn cancel_hyperlink_event(&mut self, win: &Self::WinId);

    fn current_time(&mut self) -> Self::TimeVal;
    fn current_simple_time(&mut self, factor: u32) -> i32;
    fn time_to_date_utc(&mut self, time: &Self::TimeVal) -> Self::Date;
    fn time_to_date_local(&mut self, time: &Self::TimeVal) -> Self::Date;
    fn simple_time_to_date_utc(&mut self, time: i32, factor: u32) -> Self::Date;
    fn simple_time_to_date_local(&mut self, time: i32, factor: u32) -> Self::Date;
    fn date_to_time_utc(&mut self, date: &Self::Date) -> Self::TimeVal;
    fn date_to_time_local(&mut self, date: &Self::Date) -> Self::TimeVal;
    fn date_to_simple_time_utc(&mut self, date: &Self::Date, factor: u32) -> i32;
    fn date_to_simple_time_local(&mut self, date: &Self::Date, factor: u32) -> i32;

    fn stream_open_resource(&mut self, filenum: u32, rock: u32) -> Self::StrId;
    fn stream_open_resource_uni(&mut self, filenum: u32, rock: u32) -> Self::StrId;

    fn set_resource_map(&mut self, str: Self::StrId) -> u32;
}

pub trait IdType: Clone + Eq + std::hash::Hash {
    fn null() -> Self;
    fn is_null(&self) -> bool;
}

pub trait EventType<WinId> {
    fn evtype(&self) -> u32;
    fn win(&self) -> WinId;
    fn val1(&self) -> u32;
    fn val2(&self) -> u32;
    fn buf(&mut self) -> Option<(u32,Box<[u8]>)>;
    fn buf_uni(&mut self) -> Option<(u32,Box<[u32]>)>;
}

pub trait TimeValType {
    fn new(high_sec: i32, low_sec: u32, microsec: i32) -> Self;
    fn high_sec(&self) -> i32;
    fn low_sec(&self) -> u32;
    fn microsec(&self) -> i32;
}

pub trait DateType {
    fn new(year: i32, month: i32, day: i32, weekday: i32, hour: i32, minute: i32, second: i32, microsec: i32) -> Self;
    fn year(&self) -> i32;
    fn month(&self) -> i32;
    fn day(&self) -> i32;
    fn weekday(&self) -> i32;
    fn hour(&self) -> i32;
    fn minute(&self) -> i32;
    fn second(&self) -> i32;
    fn microsec(&self) -> i32;
}

pub const gestalt_Version: u32 = 0;
pub const gestalt_CharInput: u32 = 1;
pub const gestalt_LineInput: u32 = 2;
pub const gestalt_CharOutput: u32 = 3;
pub const gestalt_CharOutput_CannotPrint: u32 = 0;
pub const gestalt_CharOutput_ApproxPrint: u32 = 1;
pub const gestalt_CharOutput_ExactPrint: u32 = 2;
pub const gestalt_MouseInput: u32 = 4;
pub const gestalt_Timer: u32 = 5;
pub const gestalt_Graphics: u32 = 6;
pub const gestalt_DrawImage: u32 = 7;
pub const gestalt_Sound: u32 = 8;
pub const gestalt_SoundVolume: u32 = 9;
pub const gestalt_SoundNotify: u32 = 10;
pub const gestalt_Hyperlinks: u32 = 11;
pub const gestalt_HyperlinkInput: u32 = 12;
pub const gestalt_SoundMusic: u32 = 13;
pub const gestalt_GraphicsTransparency: u32 = 14;
pub const gestalt_Unicode: u32 = 15;
pub const gestalt_UnicodeNorm: u32 = 16;
pub const gestalt_LineInputEcho: u32 = 17;
pub const gestalt_LineTerminators: u32 = 18;
pub const gestalt_LineTerminatorKey: u32 = 19;
pub const gestalt_DateTime: u32 = 20;
pub const gestalt_Sound2: u32 = 21;
pub const gestalt_ResourceStream: u32 = 22;
pub const gestalt_GraphicsCharInput: u32 = 23;

pub const evtype_None: u32 = 0;
pub const evtype_Timer: u32 = 1;
pub const evtype_CharInput: u32 = 2;
pub const evtype_LineInput: u32 = 3;
pub const evtype_MouseInput: u32 = 4;
pub const evtype_Arrange: u32 = 5;
pub const evtype_Redraw: u32 = 6;
pub const evtype_SoundNotify: u32 = 7;
pub const evtype_Hyperlink: u32 = 8;
pub const evtype_VolumeNotify: u32 = 9;

pub const keycode_Unknown: u32 = 0xffffffff;
pub const keycode_Left: u32 = 0xfffffffe;
pub const keycode_Right: u32 = 0xfffffffd;
pub const keycode_Up: u32 = 0xfffffffc;
pub const keycode_Down: u32 = 0xfffffffb;
pub const keycode_Return: u32 = 0xfffffffa;
pub const keycode_Delete: u32 = 0xfffffff9;
pub const keycode_Escape: u32 = 0xfffffff8;
pub const keycode_Tab: u32 = 0xfffffff7;
pub const keycode_PageUp: u32 = 0xfffffff6;
pub const keycode_PageDown: u32 = 0xfffffff5;
pub const keycode_Home: u32 = 0xfffffff4;
pub const keycode_End: u32 = 0xfffffff3;
pub const keycode_Func1: u32 = 0xffffffef;
pub const keycode_Func2: u32 = 0xffffffee;
pub const keycode_Func3: u32 = 0xffffffed;
pub const keycode_Func4: u32 = 0xffffffec;
pub const keycode_Func5: u32 = 0xffffffeb;
pub const keycode_Func6: u32 = 0xffffffea;
pub const keycode_Func7: u32 = 0xffffffe9;
pub const keycode_Func8: u32 = 0xffffffe8;
pub const keycode_Func9: u32 = 0xffffffe7;
pub const keycode_Func10: u32 = 0xffffffe6;
pub const keycode_Func11: u32 = 0xffffffe5;
pub const keycode_Func12: u32 = 0xffffffe4;
pub const keycode_MAXVAL: u32 = 28;

pub const style_Normal: u32 = 0;
pub const style_Emphasized: u32 = 1;
pub const style_Preformatted: u32 = 2;
pub const style_Header: u32 = 3;
pub const style_Subheader: u32 = 4;
pub const style_Alert: u32 = 5;
pub const style_Note: u32 = 6;
pub const style_BlockQuote: u32 = 7;
pub const style_Input: u32 = 8;
pub const style_User1: u32 = 9;
pub const style_User2: u32 = 10;
pub const style_NUMSTYLES: u32 = 11;

pub const wintype_AllTypes: u32 = 0;
pub const wintype_Pair: u32 = 1;
pub const wintype_Blank: u32 = 2;
pub const wintype_TextBuffer: u32 = 3;
pub const wintype_TextGrid: u32 = 4;
pub const wintype_Graphics: u32 = 5;

pub const winmethod_Left: u32 = 0x00;
pub const winmethod_Right: u32 = 0x01;
pub const winmethod_Above: u32 = 0x02;
pub const winmethod_Below: u32 = 0x03;
pub const winmethod_DirMask: u32 = 0x0f;

pub const winmethod_Fixed: u32 = 0x10;
pub const winmethod_Proportional: u32 = 0x20;
pub const winmethod_DivisionMask: u32 = 0xf0;

pub const winmethod_Border: u32 = 0x000;
pub const winmethod_NoBorder: u32 = 0x100;
pub const winmethod_BorderMask: u32 = 0x100;

pub const fileusage_Data: u32 = 0x00;
pub const fileusage_SavedGame: u32 = 0x01;
pub const fileusage_Transcript: u32 = 0x02;
pub const fileusage_InputRecord: u32 = 0x03;
pub const fileusage_TypeMask: u32 = 0x0f;

pub const fileusage_TextMode: u32 = 0x100;
pub const fileusage_BinaryMode: u32 = 0x000;

pub const filemode_Write: u32 = 0x01;
pub const filemode_Read: u32 = 0x02;
pub const filemode_ReadWrite: u32 = 0x03;
pub const filemode_WriteAppend: u32 = 0x05;

pub const seekmode_Start: u32 = 0;
pub const seekmode_Current: u32 = 1;
pub const seekmode_End: u32 = 2;

pub const stylehint_Indentation: u32 = 0;
pub const stylehint_ParaIndentation: u32 = 1;
pub const stylehint_Justification: u32 = 2;
pub const stylehint_Size: u32 = 3;
pub const stylehint_Weight: u32 = 4;
pub const stylehint_Oblique: u32 = 5;
pub const stylehint_Proportional: u32 = 6;
pub const stylehint_TextColor: u32 = 7;
pub const stylehint_BackColor: u32 = 8;
pub const stylehint_ReverseColor: u32 = 9;
pub const stylehint_NUMHINTS: u32 = 10;

pub const stylehint_just_LeftFlush: u32 = 0;
pub const stylehint_just_LeftRight: u32 = 1;
pub const stylehint_just_Centered: u32 = 2;
pub const stylehint_just_RightFlush: u32 = 3;

pub const giblorb_err_None: u32 = 0;
pub const giblorb_err_CompileTime: u32 = 1;
pub const giblorb_err_Alloc: u32 = 2;
pub const giblorb_err_Read: u32 = 3;
pub const giblorb_err_NotAMap: u32 = 4;
pub const giblorb_err_Format: u32 = 5;
pub const giblorb_err_NotFound: u32 = 6;
