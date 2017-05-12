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

    fn exit(&self) -> !;
    fn set_interrupt_handler(&self, handler: fn());
    fn tick(&self);

    fn gestalt(&self, sel: u32, val: u32) -> u32;
    fn gestalt_ext(&self, sel: u32, val: u32, arr: &mut [u32]) -> u32;

    fn char_to_lower(&self, ch: u8) -> u8;
    fn char_to_upper(&self, ch: u8) -> u8;

    fn window_get_root(&self) -> Self::WinId;
    fn window_open(&self, split: &Self::WinId, method: u32, size: u32, wintype: u32, rock: u32) -> Self::WinId;
    fn window_close(&self, win: &mut Self::WinId) -> (u32,u32);
    fn window_get_size(&self, win: &Self::WinId) -> (u32,u32);
    fn window_set_arrangement(&self, win: &Self::WinId, method: u32, size: u32, keywin: &Self::WinId);
    fn window_get_arrangement(&self, win: &Self::WinId) -> (u32,u32,Self::WinId);
    fn window_iterate(&self, win: &Self::WinId) -> Self::WinId;
    fn window_get_rock(&self, win: &Self::WinId) -> u32;
    fn window_get_type(&self, win: &Self::WinId) -> u32;
    fn window_get_parent(&self, win: &Self::WinId) -> Self::WinId;
    fn window_get_sibling(&self, win: &Self::WinId) -> Self::WinId;
    fn window_clear(&self, win: &Self::WinId);
    fn window_move_cursor(&self, win: &Self::WinId, xpos: u32, ypos: u32);

    fn window_get_stream(&self, win: &Self::WinId) -> Self::StrId;
    fn window_set_echo_stream(&self, win: &Self::WinId, str: &Self::StrId);
    fn window_get_echo_stream(&self, win: &Self::WinId) -> Self::StrId;
    fn set_window(&self, win: &Self::WinId);

    fn stream_open_file(&self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId;
    fn stream_open_memory(&self, buf: Box<[u8]>, fmode: u32, rock: u32) -> Self::StrId;
    fn stream_close(&self, str: &mut Self::StrId) -> (u32,u32,Option<Box<[u8]>>);
    fn stream_iterate(&self, str: &Self::StrId) -> (Self::StrId,u32);
    fn stream_get_rock(&self, str: &Self::StrId) -> u32;
    fn stream_set_position(&self, str: &Self::StrId, pos: i32, seekmode: u32);
    fn stream_get_position(&self, str: &Self::StrId) -> u32;
    fn stream_set_current(&self, str: &Self::StrId);
    fn stream_get_current(&self) -> Self::StrId;

    fn put_char(&self, ch: u8);
    fn put_char_stream(&self, str: &Self::StrId, ch: u8);
    fn put_string<S: AsRef<[u8]>>(&self, s: S);
    fn put_string_stream<S: AsRef<[u8]>>(&self, str: &Self::StrId, s: S);
    fn put_buffer(&self, buf: &[u8]);
    fn put_buffer_stream(&self, str: &Self::StrId, buf: &[u8]);
    fn set_style(&self, styl: u32);
    fn set_style_stream(&self, str: &Self::StrId, styl: u32);

    fn get_char_stream(&self, str: &Self::StrId) -> i32;
    fn get_line_stream(&self, str: &Self::StrId, buf: &mut [u8]) -> u32;
    fn get_buffer_stream(&self, str: &Self::StrId, buf: &mut [u8]) -> u32;

    fn stylehint_set(&self, wintype: u32, styl: u32, hint: u32, val: i32);
    fn stylehint_clear(&self, wintype: u32, styl: u32, hint: u32);
    fn style_distinguish(&self, win: &Self::WinId, styl1: u32, styl2: u32) -> u32;
    fn style_measure(&self, win: &Self::WinId, styl: u32, hint: u32) -> (u32,u32);

    fn fileref_create_temp(&self, usage: u32, rock: u32) -> Self::FRefId;
    fn fileref_create_by_name<S: AsRef<[u8]>>(&self, usage: u32, name: S, rock: u32) -> Self::FRefId;
    fn fileref_create_by_prompt(&self, usage: u32, fmode: u32, rock: u32) -> Self::FRefId;
    fn fileref_create_from_fileref(&self, usage: u32, fref: &Self::FRefId, rock: u32) -> Self::FRefId;
    fn fileref_destroy(&self, fref: &mut Self::FRefId);
    fn fileref_iterate(&self, fref: &Self::FRefId) -> (Self::FRefId,u32);
    fn fileref_get_rock(&self, fref: &Self::FRefId) -> u32;
    fn fileref_delete_file(&self, fref: &Self::FRefId);
    fn fileref_does_file_exist(&self, fref: &Self::FRefId) -> u32;

    fn select(&self) -> Self::Event;
    fn select_poll(&self) -> Self::Event;

    fn request_timer_events(&self, millisecs: u32);

    fn request_line_event(&self, win: &Self::WinId, buf: Box<[u8]>, initlen: u32);
    fn request_char_event(&self, win: &Self::WinId);
    fn request_mouse_event(&self, win: &Self::WinId);

    fn cancel_line_event(&self, win: &Self::WinId) -> Self::Event;
    fn cancel_char_event(&self, win: &Self::WinId);
    fn cancel_mouse_event(&self, win: &Self::WinId);

    fn set_echo_line_event(&self, win: &Self::WinId, val: u32);

    fn set_terminators_line_event(&self, win: &Self::WinId, keycodes: &[u32]);

    fn buffer_to_lower_case_uni(&self, buf: &mut [u32], numchars: u32) -> u32;
    fn buffer_to_upper_case_uni(&self, buf: &mut [u32], numchars: u32) -> u32;
    fn buffer_to_title_case_uni(&self, buf: &mut [u32], numchars: u32, lowerrest: u32) -> u32;

    fn put_char_uni(&self, ch: u32);
    fn put_string_uni<SU: AsRef<[u32]>>(&mut self, s: SU);
    fn put_buffer_uni(&self, buf: &[u32]);
    fn put_char_stream_uni(&self, str: &Self::StrId, ch: u32);
    fn put_string_stream_uni<SU: AsRef<[u32]>>(&mut self, str: &Self::StrId, s: SU);
    fn put_buffer_stream_uni(&self, str: &Self::StrId, buf: &[u32]);

    fn get_char_stream_uni(&self, str: &Self::StrId) -> i32;
    fn get_buffer_stream_uni(&self, str: &Self::StrId, buf: &mut [u32]) -> u32;
    fn get_line_stream_uni(&self, str: &Self::StrId, buf: &mut [u32]) -> u32;

    fn stream_open_file_uni(&self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId;
    fn stream_open_memory_uni(&self, fileref: &Self::FRefId, fmode: u32, rock: u32) -> Self::StrId;

    fn request_char_event_uni(&self, win: &Self::WinId);
    fn request_line_event_uni(&self, win: &Self::WinId, buf: Box<[u32]>, initlen: u32);

    fn buffer_canon_decompose_uni(&self, buf: &mut [u32], numchars: u32) -> u32;
    fn buffer_canon_normalize_uni(&self, buf: &mut [u32], numchars: u32) -> u32;

    fn image_draw(&self, win: &Self::WinId, image: u32, val1: i32, val2: i32) -> u32;
    fn image_draw_scaled(&self, win: &Self::WinId, image: u32, val1: i32, val2: i32, width: u32, height: u32) -> u32;
    fn image_get_info(&self, image: u32) -> (u32,u32);

    fn window_flow_break(&self, win: &Self::WinId);

    fn window_erase_rect(&self, win: &Self::WinId, left: i32, top: i32, width: u32, height: u32);
    fn window_fill_rect(&self, win: &Self::WinId, color: u32, left: i32, top: i32, width: u32, height: u32);
    fn window_set_background_color(&self, win: &Self::WinId, color: u32);

    fn schannel_create(&self, rock: u32) -> Self::SChanId;
    fn schannel_destroy(&self, chan: &mut Self::SChanId);
    fn schannel_iterate(&self, chan: &Self::SChanId) -> (Self::SChanId,u32);
    fn schannel_get_rock(&self, chan: &Self::SChanId) -> u32;

    fn schannel_play(&self, chan: &Self::SChanId, snd: u32) -> u32;
    fn schannel_play_ext(&self, chan: &Self::SChanId, snd: u32, repeat: u32, notify: u32) -> u32;
    fn schannel_stop(&self, chan: &Self::SChanId);
    fn schannel_set_volume(&self, chan: &Self::SChanId, vol: u32);

    fn sound_load_hint(&self, snd: u32, flag: u32);

    fn schannel_create_ext(&self, rock: u32, volume: u32) -> Self::SChanId;
    fn schannel_play_multi(&self, chanarray: &[Self::SChanId], sndarray: &[u32], notify: u32) -> u32;
    fn schannel_pause(&self, chan: &Self::SChanId);
    fn schannel_unpause(&self, chan: &Self::SChanId);
    fn schannel_set_volume_ext(&self, chan: &Self::SChanId, vol: u32, duration: u32, notify: u32);

    fn set_hyperlink(&self, linkval: u32);
    fn set_hyperlink_stream(&self, str: &Self::StrId, linkval: u32);
    fn request_hyperlink_event(&self, win: &Self::WinId);
    fn cancel_hyperlink_event(&self, win: &Self::WinId);

    fn current_time(&self) -> Self::TimeVal;
    fn current_simple_time(&self, factor: u32) -> i32;
    fn time_to_date_utc(&self, time: &Self::TimeVal) -> Self::Date;
    fn time_to_date_local(&self, time: &Self::TimeVal) -> Self::Date;
    fn simple_time_to_date_utc(&self, time: i32, factor: u32) -> Self::Date;
    fn simple_time_to_date_local(&self, time: i32, factor: u32) -> Self::Date;
    fn date_to_time_utc(&self, date: &Self::Date) -> Self::TimeVal;
    fn date_to_time_local(&self, date: &Self::Date) -> Self::TimeVal;
    fn date_to_simple_time_utc(&self, date: &Self::Date, factor: u32) -> i32;
    fn date_to_simple_time_local(&self, date: &Self::Date, factor: u32) -> i32;

    fn stream_open_resource(&self, filenum: u32, rock: u32) -> Self::StrId;
    fn stream_open_resource_uni(&self, filenum: u32, rock: u32) -> Self::StrId;
}

pub trait IdType: Eq {
    fn null() -> Self;
    fn is_null(&self) -> bool;
    fn id(&self) -> u32;
}

pub trait EventType<WinId> {
    fn evtype(&self) -> u32;
    fn win(&self) -> WinId;
    fn val1(&self) -> u32;
    fn val2(&self) -> u32;
    fn buf(&self) -> Option<Box<[u8]>>;
    fn buf_uni(&self) -> Option<Box<[u32]>>;
}

pub trait TimeValType {
    fn high_sec(&self) -> i32;
    fn low_sec(&self) -> u32;
    fn microsec(&self) -> i32;
}

pub trait DateType {
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
