use glk::{Glk,DateType,EventType,IdType,TimeValType};

use super::glk_selector;
use super::execute::Execute;
use super::state::{cstr,read_arr8,read_arr32,read_u32,write_arr8,write_arr32,write_u32};

pub struct Dispatch<'a,G: Glk<'a>> {
    winids: Registry<G::WinId>,
    strids: Registry<G::StrId>,
    frefids: Registry<G::FRefId>,
    schanids: Registry<G::SChanId>,

    buffer8: Option<Vec<u8>>,
    buffer32: Option<Vec<u32>>,
}

impl<'a,G: Glk<'a>> Dispatch<'a,G> {
    pub fn new() -> Self {
        Dispatch{
            winids: Registry::new(),
            strids: Registry::new(),
            frefids: Registry::new(),
            schanids: Registry::new(),

            buffer8: None,
            buffer32: None,
        }
    }

    pub fn get_strid(&self, index: u32) -> G::StrId {
        self.strids.get(index)
    }

    fn get_buffer<T: Clone + Default>(buffer_store: &mut Option<Vec<T>>, size: usize) -> Box<[T]> {
        let mut vec = buffer_store.take().unwrap_or(Vec::with_capacity(size));
        vec.resize(size, Default::default());
        vec.into_boxed_slice()
    }

    fn get_buffer8(&mut self, size: usize) -> Box<[u8]> {
        Self::get_buffer(&mut self.buffer8, size)
    }

    fn get_buffer32(&mut self, size: usize) -> Box<[u32]> {
        Self::get_buffer(&mut self.buffer32, size)
    }

    fn put_buffer<T: Clone>(buffer_store: &mut Option<Vec<T>>, buffer: Box<[T]>) {
        if let &mut Some(ref vec) = buffer_store {
            if vec.capacity() >= buffer.len() {
                return;
            }
        }
        *buffer_store = Some(buffer.to_vec());
    }

    fn put_buffer8(&mut self, buffer: Box<[u8]>) {
        Self::put_buffer(&mut self.buffer8, buffer);
    }

    fn put_buffer32(&mut self, buffer: Box<[u32]>) {
        Self::put_buffer(&mut self.buffer32, buffer);
    }
}

struct Registry<T> {
    list: Vec<T>,
}

impl<T: IdType> Registry<T> {
    fn new() -> Self {
        Registry{ list: Vec::new() }
    }

    fn get(&self, i: u32) -> T {
        if i == 0 {
            return T::null();
        }
        let index = i as usize - 1;
        if index >= self.list.len() {
            T::null()
        } else {
            self.list[index].clone()
        }
    }

    fn get_index(&mut self, item: T) -> u32 {
        if item.is_null() {
            return 0;
        }
        let mut free_index = self.list.len();
        for i in 0 .. self.list.len() {
            if self.list[i] == item {
                return i as u32 + 1;
            }
            if self.list[i].is_null() {
                free_index = i;
            }
        }
        if free_index == self.list.len() {
            self.list.push(item);
        } else {
            self.list[free_index] = item;
        }
        free_index as u32 + 1
    }

    fn remove(&mut self, i: u32) -> T {
        let index = i as usize - 1;
        if index >= self.list.len() {
            T::null()
        } else {
            self.list.push(T::null());
            self.list.swap_remove(index)
        }
    }
}

pub fn dispatch<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, glksel: u32) -> u32 {
    match glksel {
        glk_selector::EXIT => exec.glk.exit(),
        glk_selector::SET_INTERRUPT_HANDLER => 0,
        glk_selector::TICK => {
            exec.glk.tick();
            0
        },
        glk_selector::GESTALT => {
            let sel = exec.call_args[0];
            let val = exec.call_args[1];
            exec.glk.gestalt(sel, val)
        },
        glk_selector::GESTALT_EXT => {
            let sel = exec.call_args[0];
            let val = exec.call_args[1];
            let arraddr = exec.call_args[2] as usize;
            let arrlen = exec.call_args[3] as usize;
            let mut arr = read_arrayref32(exec, arraddr, arrlen);
            let result = exec.glk.gestalt_ext(sel, val, &mut arr);
            write_arrayref32(exec, arraddr, arr);
            result
        },
        glk_selector::WINDOW_ITERATE => {
            let winid = exec.dispatch.winids.get(exec.call_args[0]);
            let rockaddr = exec.call_args[1] as usize;
            let (nextwinid,rock) = exec.glk.window_iterate(&winid);
            write_ref(exec, rockaddr, rock);
            exec.dispatch.winids.get_index(nextwinid)
        },
        glk_selector::WINDOW_GET_ROCK => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.window_get_rock(&win)
        },
        glk_selector::WINDOW_GET_ROOT => {
            let win = exec.glk.window_get_root();
            exec.dispatch.winids.get_index(win)
        },
        glk_selector::WINDOW_OPEN => {
            let split = exec.dispatch.winids.get(exec.call_args[0]);
            let method = exec.call_args[1];
            let size = exec.call_args[2];
            let wintype = exec.call_args[3];
            let rock = exec.call_args[4];
            let win = exec.glk.window_open(&split, method, size, wintype, rock);
            exec.dispatch.winids.get_index(win)
        },
        glk_selector::WINDOW_CLOSE => {
            let mut win = exec.dispatch.winids.remove(exec.call_args[0]);
            let addr = exec.call_args[1] as usize;
            let stream_result = exec.glk.window_close(&mut win);
            write_stream_result(exec, addr, stream_result);
            0
        },
        glk_selector::WINDOW_GET_SIZE => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let widthaddr = *exec.call_args.get(1).unwrap_or(&0) as usize;
            let heightaddr = *exec.call_args.get(2).unwrap_or(&0) as usize;
            let (width,height) = exec.glk.window_get_size(&win);
            write_ref(exec, widthaddr, width);
            write_ref(exec, heightaddr, height);
            0
        },
        glk_selector::WINDOW_SET_ARRANGEMENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let method = exec.call_args[1];
            let size = exec.call_args[2];
            let keywin = exec.dispatch.winids.get(exec.call_args[3]);
            exec.glk.window_set_arrangement(&win, method, size, &keywin);
            0
        },
        glk_selector::WINDOW_GET_ARRANGEMENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let methodaddr = *exec.call_args.get(1).unwrap_or(&0) as usize;
            let sizeaddr = *exec.call_args.get(2).unwrap_or(&0) as usize;
            let keywinaddr = *exec.call_args.get(3).unwrap_or(&0) as usize;
            let (method,size,keywin) = exec.glk.window_get_arrangement(&win);
            write_ref(exec, methodaddr, method);
            write_ref(exec, sizeaddr, size);
            let keywinindex = exec.dispatch.winids.get_index(keywin);
            write_ref(exec, keywinaddr, keywinindex);
            0
        },
        glk_selector::WINDOW_GET_TYPE => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.window_get_type(&win)
        },
        glk_selector::WINDOW_GET_PARENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let parent = exec.glk.window_get_parent(&win);
            exec.dispatch.winids.get_index(parent)
        },
        glk_selector::WINDOW_CLEAR => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.window_clear(&win);
            0
        },
        glk_selector::WINDOW_MOVE_CURSOR => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let xpos = exec.call_args[1];
            let ypos = exec.call_args[2];
            exec.glk.window_move_cursor(&win, xpos, ypos);
            0
        },
        glk_selector::WINDOW_GET_STREAM => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let str = exec.glk.window_get_stream(&win);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::WINDOW_SET_ECHO_STREAM => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let str = exec.dispatch.strids.get(exec.call_args[1]);
            exec.glk.window_set_echo_stream(&win, &str);
            0
        },
        glk_selector::WINDOW_GET_ECHO_STREAM => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let str = exec.glk.window_get_echo_stream(&win);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::SET_WINDOW => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.set_window(&win);
            0
        },
        glk_selector::WINDOW_GET_SIBLING => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let sib = exec.glk.window_get_sibling(&win);
            exec.dispatch.winids.get_index(sib)
        },
        glk_selector::STREAM_ITERATE => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let rockaddr = exec.call_args[1] as usize;
            let (nextstr,rock) = exec.glk.stream_iterate(&str);
            write_ref(exec, rockaddr, rock);
            exec.dispatch.strids.get_index(nextstr)
        },
        glk_selector::STREAM_GET_ROCK => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            exec.glk.stream_get_rock(&str)
        },
        glk_selector::STREAM_OPEN_FILE => {
            let fileref = exec.dispatch.frefids.get(exec.call_args[0]);
            let fmode = exec.call_args[1];
            let rock = exec.call_args[2];
            let str = exec.glk.stream_open_file(&fileref, fmode, rock);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::STREAM_OPEN_MEMORY => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let fmode = exec.call_args[2];
            let rock = exec.call_args[3];
            if bufaddr+buflen > exec.state.mem.len() {
                panic!("Memory access out of range");
            }
            let buf = exec.dispatch.get_buffer8(buflen);
            let str = exec.glk.stream_open_memory((bufaddr as u32,buf), fmode, rock);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::STREAM_CLOSE => {
            let mut str = exec.dispatch.strids.remove(exec.call_args[0]);
            let addr = exec.call_args[1] as usize;
            let stream_result = exec.glk.stream_close(&mut str);
            write_stream_result(exec, addr, stream_result);
            0
        },
        glk_selector::STREAM_SET_POSITION => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let pos = exec.call_args[1] as i32;
            let seekmode = exec.call_args[2];
            exec.glk.stream_set_position(&str, pos, seekmode);
            0
        },
        glk_selector::STREAM_GET_POSITION => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            exec.glk.stream_get_position(&str)
        },
        glk_selector::STREAM_SET_CURRENT => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            exec.glk.stream_set_current(&str);
            0
        },
        glk_selector::STREAM_GET_CURRENT => {
            let str = exec.glk.stream_get_current();
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::STREAM_OPEN_RESOURCE => {
            let filenum = exec.call_args[0];
            let rock = exec.call_args[1];
            let str = exec.glk.stream_open_resource(filenum, rock);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::FILEREF_CREATE_TEMP => {
            let usage = exec.call_args[0];
            let rock = exec.call_args[1];
            let frefid = exec.glk.fileref_create_temp(usage, rock);
            exec.dispatch.frefids.get_index(frefid)
        },
        glk_selector::FILEREF_CREATE_BY_NAME => {
            let usage = exec.call_args[0];
            let nameaddr = exec.call_args[1] as usize;
            let rock = exec.call_args[2];
            let fref = exec.glk.fileref_create_by_name(usage, cstr(&exec.state.mem, nameaddr), rock);
            exec.dispatch.frefids.get_index(fref)
        },
        glk_selector::FILEREF_CREATE_BY_PROMPT => {
            let usage = exec.call_args[0];
            let fmode = exec.call_args[1];
            let rock = exec.call_args[2];
            let fref = exec.glk.fileref_create_by_prompt(usage, fmode, rock);
            exec.dispatch.frefids.get_index(fref)
        },
        glk_selector::FILEREF_DESTROY => {
            let mut fref = exec.dispatch.frefids.remove(exec.call_args[0]);
            exec.glk.fileref_destroy(&mut fref);
            0
        },
        glk_selector::FILEREF_ITERATE => {
            let fref = exec.dispatch.frefids.get(exec.call_args[0]);
            let rockaddr = exec.call_args[1] as usize;
            let (nextfileref,rock) = exec.glk.fileref_iterate(&fref);
            write_ref(exec, rockaddr, rock);
            exec.dispatch.frefids.get_index(nextfileref)
        },
        glk_selector::FILEREF_GET_ROCK => {
            let fref = exec.dispatch.frefids.get(exec.call_args[0]);
            exec.glk.fileref_get_rock(&fref)
        },
        glk_selector::FILEREF_DELETE_FILE => {
            let fref = exec.dispatch.frefids.get(exec.call_args[0]);
            exec.glk.fileref_delete_file(&fref);
            0
        },
        glk_selector::FILEREF_DOES_FILE_EXIST => {
            let fref = exec.dispatch.frefids.get(exec.call_args[0]);
            if exec.glk.fileref_does_file_exist(&fref) { 1 } else { 0 }
        },
        glk_selector::FILEREF_CREATE_FROM_FILEREF => {
            let usage = exec.call_args[0];
            let fref = exec.dispatch.frefids.get(exec.call_args[1]);
            let rock = exec.call_args[2];
            let newfref = exec.glk.fileref_create_from_fileref(usage, &fref, rock);
            exec.dispatch.frefids.get_index(newfref)
        },
        glk_selector::PUT_CHAR => {
            let ch = exec.call_args[0] as u8;
            exec.glk.put_char(ch);
            0
        },
        glk_selector::PUT_CHAR_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let ch = exec.call_args[1] as u8;
            exec.glk.put_char_stream(&str, ch);
            0
        },
        glk_selector::PUT_STRING => {
            let straddr = exec.call_args[0] as usize;
            assert_eq!(exec.state.mem[straddr], 0xe0);
            exec.glk.put_string(cstr(&exec.state.mem, straddr+1));
            0
        },
        glk_selector::PUT_STRING_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let straddr = exec.call_args[1] as usize;
            assert_eq!(exec.state.mem[straddr], 0xe0);
            exec.glk.put_string_stream(&str, cstr(&exec.state.mem, straddr+1));
            0
        },
        glk_selector::PUT_BUFFER => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            assert_ne!(bufaddr, 0xffffffff);
            exec.glk.put_buffer(&exec.state.mem[bufaddr .. bufaddr+buflen]);
            0
        },
        glk_selector::PUT_BUFFER_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            assert_ne!(bufaddr, 0xffffffff);
            exec.glk.put_buffer_stream(&str, &exec.state.mem[bufaddr .. bufaddr+buflen]);
            0
        },
        glk_selector::SET_STYLE => {
            let styl = exec.call_args[0];
            exec.glk.set_style(styl);
            0
        },
        glk_selector::SET_STYLE_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let styl = exec.call_args[1];
            exec.glk.set_style_stream(&str, styl);
            0
        },
        glk_selector::GET_CHAR_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            exec.glk.get_char_stream(&str) as u32
        },
        glk_selector::GET_LINE_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            let mut arr = read_arrayref8(exec, bufaddr, buflen);
            let result = exec.glk.get_line_stream(&str, &mut arr);
            write_arrayref8(exec, bufaddr, arr);
            result
        },
        glk_selector::GET_BUFFER_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            let mut arr = read_arrayref8(exec, bufaddr, buflen);
            let result = exec.glk.get_buffer_stream(&str, &mut arr);
            write_arrayref8(exec, bufaddr, arr);
            result
        },
        glk_selector::CHAR_TO_LOWER => {
            let ch = exec.call_args[0] as u8;
            exec.glk.char_to_lower(ch) as u32
        },
        glk_selector::CHAR_TO_UPPER => {
            let ch = exec.call_args[0] as u8;
            exec.glk.char_to_upper(ch) as u32
        },
        glk_selector::STYLEHINT_SET => {
            let wintype = exec.call_args[0];
            let styl = exec.call_args[1];
            let hint = exec.call_args[2];
            let val = exec.call_args[3] as i32;
            exec.glk.stylehint_set(wintype, styl, hint, val);
            0
        },
        glk_selector::STYLEHINT_CLEAR => {
            let wintype = exec.call_args[0];
            let styl = exec.call_args[1];
            let hint = exec.call_args[2];
            exec.glk.stylehint_clear(wintype, styl, hint);
            0
        },
        glk_selector::STYLE_DISTINGUISH => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let styl1 = exec.call_args[1];
            let styl2 = exec.call_args[2];
            if exec.glk.style_distinguish(&win, styl1, styl2) { 1 } else { 0 }
        },
        glk_selector::STYLE_MEASURE => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let styl = exec.call_args[1];
            let hint = exec.call_args[2];
            let resultaddr = exec.call_args[3] as usize;
            let (success,result) = exec.glk.style_measure(&win, styl, hint);
            write_ref(exec, resultaddr, result);
            if success { 1 } else { 0 }
        },
        glk_selector::SELECT => {
            let addr = exec.call_args[0] as usize;
            let event = exec.glk.select();
            write_event(exec, addr, event);
            0
        },
        glk_selector::SELECT_POLL => {
            let addr = exec.call_args[0] as usize;
            let event = exec.glk.select_poll();
            write_event(exec, addr, event);
            0
        },
        glk_selector::REQUEST_LINE_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            let initlen = exec.call_args[3];
            let buf = read_arrayref8(exec, bufaddr, buflen);
            exec.glk.request_line_event(&win, (bufaddr as u32,buf), initlen);
            0
        },
        glk_selector::CANCEL_LINE_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let addr = exec.call_args[1] as usize;
            let event = exec.glk.cancel_line_event(&win);
            write_event(exec, addr, event);
            0
        },
        glk_selector::REQUEST_CHAR_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.request_char_event(&win);
            0
        },
        glk_selector::CANCEL_CHAR_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.cancel_char_event(&win);
            0
        },
        glk_selector::REQUEST_MOUSE_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.request_mouse_event(&win);
            0
        },
        glk_selector::CANCEL_MOUSE_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.cancel_mouse_event(&win);
            0
        },
        glk_selector::REQUEST_TIMER_EVENTS => {
            let millisecs = exec.call_args[0];
            exec.glk.request_timer_events(millisecs);
            0
        },
        glk_selector::IMAGE_GET_INFO => {
            let image = exec.call_args[0];
            let widthaddr = exec.call_args[1] as usize;
            let heightaddr = exec.call_args[2] as usize;
            let (result,width,height) = exec.glk.image_get_info(image);
            write_ref(exec, widthaddr, width);
            write_ref(exec, heightaddr, height);
            if result { 1 } else { 0 }
        },
        glk_selector::IMAGE_DRAW => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let image = exec.call_args[1];
            let val1 = exec.call_args[2] as i32;
            let val2 = exec.call_args[3] as i32;
            if exec.glk.image_draw(&win, image, val1, val2) { 1 } else { 0 }
        },
        glk_selector::IMAGE_DRAW_SCALED => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let image = exec.call_args[1];
            let val1 = exec.call_args[2] as i32;
            let val2 = exec.call_args[3] as i32;
            let width = exec.call_args[4];
            let height = exec.call_args[5];
            if exec.glk.image_draw_scaled(&win, image, val1, val2, width, height) { 1 } else { 0 }
        },
        glk_selector::WINDOW_FLOW_BREAK => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.window_flow_break(&win);
            0
        },
        glk_selector::WINDOW_ERASE_RECT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let left = exec.call_args[1] as i32;
            let top = exec.call_args[2] as i32;
            let width = exec.call_args[3];
            let height = exec.call_args[4];
            exec.glk.window_erase_rect(&win, left, top, width, height);
            0
        },
        glk_selector::WINDOW_FILL_RECT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let color = exec.call_args[1];
            let left = exec.call_args[2] as i32;
            let top = exec.call_args[3] as i32;
            let width = exec.call_args[4];
            let height = exec.call_args[5];
            exec.glk.window_fill_rect(&win, color, left, top, width, height);
            0
        },
        glk_selector::WINDOW_SET_BACKGROUND_COLOR => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let color = exec.call_args[1];
            exec.glk.window_set_background_color(&win, color);
            0
        },
        glk_selector::SCHANNEL_ITERATE => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            let rockaddr = exec.call_args[1] as usize;
            let (nextchan,rock) = exec.glk.schannel_iterate(&chan);
            write_ref(exec, rockaddr, rock);
            exec.dispatch.schanids.get_index(nextchan)
        },
        glk_selector::SCHANNEL_GET_ROCK => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            exec.glk.schannel_get_rock(&chan)
        },
        glk_selector::SCHANNEL_CREATE => {
            let rock = exec.call_args[0];
            let chan = exec.glk.schannel_create(rock);
            exec.dispatch.schanids.get_index(chan)
        },
        glk_selector::SCHANNEL_DESTROY => {
            let mut chan = exec.dispatch.schanids.remove(exec.call_args[0]);
            exec.glk.schannel_destroy(&mut chan);
            0
        },
        glk_selector::SCHANNEL_CREATE_EXT => {
            let rock = exec.call_args[0];
            let volume = exec.call_args[1];
            exec.glk.schannel_create_ext(rock, volume);
            0
        },
        glk_selector::SCHANNEL_PLAY_MULTI => {
            let chanarrayaddr = exec.call_args[0] as usize;
            let chanarraylen = exec.call_args[1] as usize;
            let sndarrayaddr = exec.call_args[2] as usize;
            let sndarraylen = exec.call_args[3] as usize;
            let notify = exec.call_args[4];
            let chanarray = read_arrayref32(exec, chanarrayaddr, chanarraylen);
            let mut chans = Vec::with_capacity(chanarray.len());
            for chanindex in &chanarray[..] {
                chans.push(exec.dispatch.schanids.get(*chanindex));
            }
            exec.dispatch.put_buffer32(chanarray);
            let sndarray = read_arrayref32(exec, sndarrayaddr, sndarraylen);
            exec.glk.schannel_play_multi(&chans, &sndarray, notify);
            exec.dispatch.put_buffer32(sndarray);
            0
        },
        glk_selector::SCHANNEL_PLAY => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            let snd = exec.call_args[1];
            if exec.glk.schannel_play(&chan, snd) { 1 } else { 0 }
        },
        glk_selector::SCHANNEL_PLAY_EXT => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            let snd = exec.call_args[1];
            let repeat = exec.call_args[2];
            let notify = exec.call_args[3];
            if exec.glk.schannel_play_ext(&chan, snd, repeat, notify) { 1 } else { 0 }
        },
        glk_selector::SCHANNEL_STOP => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            exec.glk.schannel_stop(&chan);
            0
        },
        glk_selector::SCHANNEL_SET_VOLUME => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            let vol = exec.call_args[1];
            exec.glk.schannel_set_volume(&chan, vol);
            0
        },
        glk_selector::SOUND_LOAD_HINT => {
            let snd = exec.call_args[0];
            let flag = exec.call_args[1];
            exec.glk.sound_load_hint(snd, flag);
            0
        },
        glk_selector::SCHANNEL_SET_VOLUME_EXT => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            let vol = exec.call_args[1];
            let duration = exec.call_args[2];
            let notify = exec.call_args[3];
            exec.glk.schannel_set_volume_ext(&chan, vol, duration, notify);
            0
        },
        glk_selector::SCHANNEL_PAUSE => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            exec.glk.schannel_pause(&chan);
            0
        },
        glk_selector::SCHANNEL_UNPAUSE => {
            let chan = exec.dispatch.schanids.get(exec.call_args[0]);
            exec.glk.schannel_unpause(&chan);
            0
        },
        glk_selector::SET_HYPERLINK => {
            let linkval = exec.call_args[0];
            exec.glk.set_hyperlink(linkval);
            0
        },
        glk_selector::SET_HYPERLINK_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let linkval = exec.call_args[1];
            exec.glk.set_hyperlink_stream(&str, linkval);
            0
        },
        glk_selector::REQUEST_HYPERLINK_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.request_hyperlink_event(&win);
            0
        },
        glk_selector::CANCEL_HYPERLINK_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.cancel_hyperlink_event(&win);
            0
        },
        glk_selector::BUFFER_TO_LOWER_CASE_UNI => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let numchars = exec.call_args[2];
            let mut buf = read_arrayref32(exec, bufaddr, buflen);
            let result = exec.glk.buffer_to_lower_case_uni(&mut buf, numchars);
            write_arrayref32(exec, bufaddr, buf);
            result
        },
        glk_selector::BUFFER_TO_UPPER_CASE_UNI => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let numchars = exec.call_args[2];
            let mut buf = read_arrayref32(exec, bufaddr, buflen);
            let result = exec.glk.buffer_to_upper_case_uni(&mut buf, numchars);
            write_arrayref32(exec, bufaddr, buf);
            result
        },
        glk_selector::BUFFER_TO_TITLE_CASE_UNI => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let numchars = exec.call_args[2];
            let lowerrest = exec.call_args[3];
            let mut buf = read_arrayref32(exec, bufaddr, buflen);
            let result = exec.glk.buffer_to_title_case_uni(&mut buf, numchars, lowerrest);
            write_arrayref32(exec, bufaddr, buf);
            result
        },
        glk_selector::BUFFER_CANON_DECOMPOSE_UNI => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let numchars = exec.call_args[2];
            let mut buf = read_arrayref32(exec, bufaddr, buflen);
            let result = exec.glk.buffer_canon_decompose_uni(&mut buf, numchars);
            write_arrayref32(exec, bufaddr, buf);
            result
        },
        glk_selector::BUFFER_CANON_NORMALIZE_UNI => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let numchars = exec.call_args[2];
            let mut buf = read_arrayref32(exec, bufaddr, buflen);
            let result = exec.glk.buffer_canon_normalize_uni(&mut buf, numchars);
            write_arrayref32(exec, bufaddr, buf);
            result
        },
        glk_selector::PUT_CHAR_UNI => {
            let ch = exec.call_args[0];
            exec.glk.put_char_uni(ch);
            0
        },
        glk_selector::PUT_STRING_UNI => {
            let straddr = exec.call_args[0] as usize;
            assert_eq!(exec.state.mem[straddr], 0xe2);
            let s = read_cstr_uni(exec, straddr+4);
            exec.glk.put_string_uni(&s);
            exec.dispatch.put_buffer32(s);
            0
        },
        glk_selector::PUT_BUFFER_UNI => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let buf = read_arrayref32(exec, bufaddr, buflen);
            exec.glk.put_buffer_uni(&buf);
            exec.dispatch.put_buffer32(buf);
            0
        },
        glk_selector::PUT_CHAR_STREAM_UNI => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let ch = exec.call_args[1];
            exec.glk.put_char_stream_uni(&str, ch);
            0
        },
        glk_selector::PUT_STRING_STREAM_UNI => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let straddr = exec.call_args[1] as usize;
            assert_eq!(exec.state.mem[straddr], 0xe2);
            let s = read_cstr_uni(exec, straddr+4);
            exec.glk.put_string_stream_uni(&str, &s);
            exec.dispatch.put_buffer32(s);
            0
        },
        glk_selector::PUT_BUFFER_STREAM_UNI => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            let buf = read_arrayref32(exec, bufaddr, buflen);
            exec.glk.put_buffer_stream_uni(&str, &buf);
            exec.dispatch.put_buffer32(buf);
            0
        },
        glk_selector::GET_CHAR_STREAM_UNI => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            exec.glk.get_char_stream_uni(&str) as u32
        },
        glk_selector::GET_BUFFER_STREAM_UNI => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            let mut buf = exec.dispatch.get_buffer32(buflen);
            let result = exec.glk.get_buffer_stream_uni(&str, &mut buf);
            write_arrayref32(exec, bufaddr, buf);
            result
        },
        glk_selector::GET_LINE_STREAM_UNI => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            let mut buf = exec.dispatch.get_buffer32(buflen);
            let result = exec.glk.get_line_stream_uni(&str, &mut buf);
            write_arrayref32(exec, bufaddr, buf);
            result
        },
        glk_selector::STREAM_OPEN_FILE_UNI => {
            let fileref = exec.dispatch.frefids.get(exec.call_args[0]);
            let fmode = exec.call_args[1];
            let rock = exec.call_args[2];
            let str = exec.glk.stream_open_file_uni(&fileref, fmode, rock);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::STREAM_OPEN_MEMORY_UNI => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let fmode = exec.call_args[2];
            let rock = exec.call_args[3];
            if bufaddr + buflen*4 > exec.state.mem.len() {
                panic!("Memory access out of range");
            }
            let buf = exec.dispatch.get_buffer32(buflen);
            let str = exec.glk.stream_open_memory_uni((bufaddr as u32,buf), fmode, rock);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::STREAM_OPEN_RESOURCE_UNI => {
            let filenum = exec.call_args[0];
            let rock = exec.call_args[1];
            let str = exec.glk.stream_open_resource_uni(filenum, rock);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::REQUEST_CHAR_EVENT_UNI => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            exec.glk.request_char_event_uni(&win);
            0
        },
        glk_selector::REQUEST_LINE_EVENT_UNI => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            let initlen = exec.call_args[3];
            let buf = read_arrayref32(exec, bufaddr, buflen);
            exec.glk.request_line_event_uni(&win, (bufaddr as u32,buf), initlen);
            0
        },
        glk_selector::SET_ECHO_LINE_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let val = exec.call_args[1];
            exec.glk.set_echo_line_event(&win, val);
            0
        },
        glk_selector::SET_TERMINATORS_LINE_EVENT => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let keycodesaddr = exec.call_args[1] as usize;
            let keycodeslen = exec.call_args[2] as usize;
            let keycodes = read_arrayref32(exec, keycodesaddr, keycodeslen);
            exec.glk.set_terminators_line_event(&win, &keycodes);
            exec.dispatch.put_buffer32(keycodes);
            0
        },
        glk_selector::CURRENT_TIME => {
            let addr = exec.call_args[0] as usize;
            let time = exec.glk.current_time();
            write_time(exec, addr, time);
            0
        },
        glk_selector::CURRENT_SIMPLE_TIME => {
            let factor = exec.call_args[0];
            exec.glk.current_simple_time(factor) as u32
        },
        glk_selector::TIME_TO_DATE_UTC => {
            let timeaddr = exec.call_args[0] as usize;
            let dateaddr = exec.call_args[1] as usize;
            let time = read_time(exec, timeaddr);
            let date = exec.glk.time_to_date_utc(&time);
            write_date(exec, dateaddr, date);
            0
        },
        glk_selector::TIME_TO_DATE_LOCAL => {
            let timeaddr = exec.call_args[0] as usize;
            let dateaddr = exec.call_args[1] as usize;
            let time = read_time(exec, timeaddr);
            let date = exec.glk.time_to_date_local(&time);
            write_date(exec, dateaddr, date);
            0
        },
        glk_selector::SIMPLE_TIME_TO_DATE_UTC => {
            let time = exec.call_args[0] as i32;
            let factor = exec.call_args[1];
            let dateaddr = exec.call_args[2] as usize;
            let date = exec.glk.simple_time_to_date_utc(time, factor);
            write_date(exec, dateaddr, date);
            0
        },
        glk_selector::SIMPLE_TIME_TO_DATE_LOCAL => {
            let time = exec.call_args[0] as i32;
            let factor = exec.call_args[1];
            let dateaddr = exec.call_args[2] as usize;
            let date = exec.glk.simple_time_to_date_local(time, factor);
            write_date(exec, dateaddr, date);
            0
        },
        glk_selector::DATE_TO_TIME_UTC => {
            let dateaddr = exec.call_args[0] as usize;
            let timeaddr = exec.call_args[1] as usize;
            let date = read_date(exec, dateaddr);
            let time = exec.glk.date_to_time_utc(&date);
            write_time(exec, timeaddr, time);
            0
        },
        glk_selector::DATE_TO_TIME_LOCAL => {
            let dateaddr = exec.call_args[0] as usize;
            let timeaddr = exec.call_args[1] as usize;
            let date = read_date(exec, dateaddr);
            let time = exec.glk.date_to_time_local(&date);
            write_time(exec, timeaddr, time);
            0
        },
        glk_selector::DATE_TO_SIMPLE_TIME_UTC => {
            let dateaddr = exec.call_args[0] as usize;
            let factor = exec.call_args[1];
            let date = read_date(exec, dateaddr);
            exec.glk.date_to_simple_time_utc(&date, factor) as u32
        },
        glk_selector::DATE_TO_SIMPLE_TIME_LOCAL => {
            let dateaddr = exec.call_args[0] as usize;
            let factor = exec.call_args[1];
            let date = read_date(exec, dateaddr);
            exec.glk.date_to_simple_time_local(&date, factor) as u32
        },
        _ => 0,
    }
}

fn read_arrayref8<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, len: usize) -> Box<[u8]> {
    let mut arr = exec.dispatch.get_buffer8(len);
    if addr == 0xffffffff {
        for i in 0 .. len {
            arr[i] = exec.state.stack.pop().unwrap() as u8;
        }
    } else {
        read_arr8(&exec.state.mem, addr, &mut arr);
    }
    arr
}

fn write_arrayref8<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, arr: Box<[u8]>) {
    if addr == 0xffffffff {
        for i in 0 .. arr.len() {
            exec.state.stack.push(arr[i] as u32);
        }
    } else if addr >= exec.ram_start {
        if addr + arr.len() <= exec.state.mem.len() {
            write_arr8(&mut exec.state.mem, addr, &arr);
        } else if addr < exec.state.mem.len() {
            let len = exec.state.mem.len() - addr;
            write_arr8(&mut exec.state.mem, addr, &arr[0..len]);
        }
    }
    exec.dispatch.put_buffer8(arr);
}

fn read_arrayref32<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, len: usize) -> Box<[u32]> {
    let mut arr = exec.dispatch.get_buffer32(len);
    if addr == 0xffffffff {
        for i in 0 .. len {
            arr[i] = exec.state.stack.pop().unwrap();
        }
    } else {
        read_arr32(&exec.state.mem, addr, &mut arr);
    }
    arr
}

fn write_arrayref32<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, arr: Box<[u32]>) {
    if addr == 0xffffffff {
        for i in 0 .. arr.len() {
            exec.state.stack.push(arr[i]);
        }
    } else if addr >= exec.ram_start {
        if addr + 4*arr.len() <= exec.state.mem.len() {
            write_arr32(&mut exec.state.mem, addr, &arr);
        } else if addr < exec.state.mem.len() {
            let len = (exec.state.mem.len() - addr)/4;
            write_arr32(&mut exec.state.mem, addr, &arr[0..len]);
        }
    }
    exec.dispatch.put_buffer32(arr);
}

fn read_cstr_uni<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize) -> Box<[u32]> {
    for len in 0 .. {
        if read_u32(&exec.state.mem, addr + 4*len) == 0 {
            return read_arrayref32(exec, addr, len);
        }
    }
    read_arrayref32(exec, addr, 0)
}


fn write_ref<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, val: u32) {
    if addr == 0xffffffff {
        exec.state.stack.push(val);
    } else if addr >= exec.ram_start {
        write_u32(&mut exec.state.mem, addr, val);
    }
}

fn write_stream_result<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, stream_result: (u32,u32,Option<(u32,Box<[u8]>)>,Option<(u32,Box<[u32]>)>)) {
    let (readcount,writecount,buf,buf_uni) = stream_result;
    let mut arr = exec.dispatch.get_buffer32(2);
    arr[0] = readcount;
    arr[1] = writecount;
    write_arrayref32(exec, addr, arr);
    if let Some((bufaddr,buf8)) = buf {
        write_arrayref8(exec, bufaddr as usize, buf8);
    }
    if let Some((bufaddr,buf32)) = buf_uni {
        write_arrayref32(exec, bufaddr as usize, buf32);
    }
}

fn write_event<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, mut event: G::Event) {
    let mut arr = exec.dispatch.get_buffer32(4);
    arr[0] = event.evtype();
    arr[1] = exec.dispatch.winids.get_index(event.win());
    arr[2] = event.val1();
    arr[3] = event.val2();
    write_arrayref32(exec, addr, arr);
    if let Some((bufaddr,buf)) = event.buf() {
        write_arrayref8(exec, bufaddr as usize, buf);
    }
    if let Some((bufaddr,buf)) = event.buf_uni() {
        write_arrayref32(exec, bufaddr as usize, buf);
    }
}

fn read_time<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize) -> G::TimeVal {
    let arr = read_arrayref32(exec, addr, 3);
    let high_sec = arr[0] as i32;
    let low_sec = arr[1];
    let microsec = arr[2] as i32;
    exec.dispatch.put_buffer32(arr);
    G::TimeVal::new(high_sec, low_sec, microsec)
}

fn write_time<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, time: G::TimeVal) {
    let mut arr = exec.dispatch.get_buffer32(3);
    arr[0] = time.high_sec() as u32;
    arr[1] = time.low_sec();
    arr[2] = time.microsec() as u32;
    write_arrayref32(exec, addr, arr);
}

fn read_date<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize) -> G::Date {
    let arr = read_arrayref32(exec, addr, 8);
    let year = arr[0] as i32;
    let month = arr[1] as i32;
    let day = arr[2] as i32;
    let weekday = arr[3] as i32;
    let hour = arr[4] as i32;
    let minute = arr[5] as i32;
    let second = arr[6] as i32;
    let microsec = arr[7] as i32;
    exec.dispatch.put_buffer32(arr);
    G::Date::new(year, month, day, weekday, hour, minute, second, microsec)
}

fn write_date<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, date: G::Date) {
    let mut arr = exec.dispatch.get_buffer32(8);
    arr[0] = date.year() as u32;
    arr[1] = date.month() as u32;
    arr[2] = date.day() as u32;
    arr[3] = date.weekday() as u32;
    arr[4] = date.hour() as u32;
    arr[5] = date.minute() as u32;
    arr[6] = date.second() as u32;
    arr[7] = date.microsec() as u32;
    write_arrayref32(exec, addr, arr);
}
