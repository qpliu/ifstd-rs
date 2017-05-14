use glk::{Glk,IdType,EventType};

use super::glk_selector;
use super::execute::Execute;
use super::state::{cstr,read_arr8,read_arr32,write_arr8,write_arr32,write_u32};

pub struct Dispatch<G: Glk> {
    winids: Registry<G::WinId>,
    strids: Registry<G::StrId>,
    frefids: Registry<G::FRefId>,
    schanids: Registry<G::SChanId>,

    buffer8: Option<Vec<u8>>,
    buffer32: Option<Vec<u32>>,
}

impl<G: Glk> Dispatch<G> {
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

pub fn dispatch<G: Glk>(exec: &mut Execute<G>, glksel: u32) -> u32 {
    match glksel {
        glk_selector::EXIT => exec.glk.exit(),
        glk_selector::SET_INTERRUPT_HANDLER => 0,
        glk_selector::TICK => {
            exec.glk.tick();
            0
        },
        glk_selector::GESTALT => {
            let sel = *exec.call_args.get(0).unwrap();
            let val = *exec.call_args.get(1).unwrap();
            exec.glk.gestalt(sel, val)
        },
        glk_selector::GESTALT_EXT => {
            let sel = *exec.call_args.get(0).unwrap();
            let val = *exec.call_args.get(1).unwrap();
            let arr = *exec.call_args.get(2).unwrap_or(&0) as usize;
            let arrlen = *exec.call_args.get(3).unwrap_or(&0) as usize;
            let mut buffer = exec.dispatch.get_buffer32(arrlen);
            read_arr32(&exec.state.mem, arr, &mut buffer);
            let result = exec.glk.gestalt_ext(sel, val, &mut buffer);
            write_arr32(&mut exec.state.mem, arr, &buffer);
            exec.dispatch.put_buffer32(buffer);
            result
        },
        glk_selector::WINDOW_ITERATE => {
            let winid = exec.dispatch.winids.get(exec.call_args[0]);
            let rockaddr = exec.call_args[1] as usize;
            let (nextwinid,rock) = exec.glk.window_iterate(&winid);
            if rockaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, rockaddr, rock);
            }
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
            exec.glk.window_close(&mut win);
            0
        },
        glk_selector::WINDOW_GET_SIZE => {
            let win = exec.dispatch.winids.get(exec.call_args[0]);
            let widthaddr = *exec.call_args.get(1).unwrap_or(&0) as usize;
            let heightaddr = *exec.call_args.get(2).unwrap_or(&0) as usize;
            let (width,height) = exec.glk.window_get_size(&win);
            if widthaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, widthaddr, width);
            }
            if heightaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, heightaddr, height);
            }
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
            if methodaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, methodaddr, method);
            }
            if sizeaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, sizeaddr, size);
            }
            if keywinaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, keywinaddr, exec.dispatch.winids.get_index(keywin));
            }
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
            if rockaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, rockaddr, rock);
            }
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
            let addr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            let fmode = exec.call_args[2];
            let rock = exec.call_args[3];
            let mut buffer = exec.dispatch.get_buffer8(buflen);
            read_arr8(&exec.state.mem, addr, &mut buffer);
            let str = exec.glk.stream_open_memory((addr as u32,buffer), fmode, rock);
            exec.dispatch.strids.get_index(str)
        },
        glk_selector::STREAM_CLOSE => {
            let mut str = exec.dispatch.strids.remove(exec.call_args[0]);
            let readcountaddr = *exec.call_args.get(1).unwrap_or(&0) as usize;
            let writecountaddr = *exec.call_args.get(1).unwrap_or(&0) as usize;
            let (readcount,writecount,buf8,buf32) = exec.glk.stream_close(&mut str);
            if readcountaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, readcountaddr, readcount);
            }
            if writecountaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, writecountaddr, writecount);
            }
            if let Some((addr,buf)) = buf8 {
                if addr as usize > exec.ram_start {
                    write_arr8(&mut exec.state.mem, addr as usize, &buf);
                }
                exec.dispatch.put_buffer8(buf);
            }
            if let Some((addr,buf)) = buf32 {
                if addr as usize > exec.ram_start {
                    write_arr32(&mut exec.state.mem, addr as usize, &buf);
                }
                exec.dispatch.put_buffer32(buf);
            }
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
            let (sib,rock) = exec.glk.fileref_iterate(&fref);
            if rockaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, rockaddr, rock);
            }
            exec.dispatch.frefids.get_index(sib)
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
            exec.glk.put_string(cstr(&exec.state.mem, straddr));
            0
        },
        glk_selector::PUT_STRING_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let straddr = exec.call_args[1] as usize;
            exec.glk.put_string_stream(&str, cstr(&exec.state.mem, straddr));
            0
        },
        glk_selector::PUT_BUFFER => {
            let bufaddr = exec.call_args[0] as usize;
            let buflen = exec.call_args[1] as usize;
            exec.glk.put_buffer(&exec.state.mem[bufaddr .. bufaddr+buflen]);
            0
        },
        glk_selector::PUT_BUFFER_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
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
            exec.glk.get_line_stream(&str, &mut exec.state.mem[bufaddr .. bufaddr+buflen])
        },
        glk_selector::GET_BUFFER_STREAM => {
            let str = exec.dispatch.strids.get(exec.call_args[0]);
            let bufaddr = exec.call_args[1] as usize;
            let buflen = exec.call_args[2] as usize;
            exec.glk.get_buffer_stream(&str, &mut exec.state.mem[bufaddr .. bufaddr+buflen])
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
            if resultaddr >= exec.ram_start {
                write_u32(&mut exec.state.mem, resultaddr, result);
            }
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
            unimplemented!()
        },
        glk_selector::CANCEL_LINE_EVENT => {
            unimplemented!()
        },
        glk_selector::REQUEST_CHAR_EVENT => {
            unimplemented!()
        },
        glk_selector::CANCEL_CHAR_EVENT => {
            unimplemented!()
        },
        glk_selector::REQUEST_MOUSE_EVENT => {
            unimplemented!()
        },
        glk_selector::CANCEL_MOUSE_EVENT => {
            unimplemented!()
        },
        glk_selector::REQUEST_TIMER_EVENTS => {
            unimplemented!()
        },
        glk_selector::IMAGE_GET_INFO => {
            unimplemented!()
        },
        glk_selector::IMAGE_DRAW => {
            unimplemented!()
        },
        glk_selector::IMAGE_DRAW_SCALED => {
            unimplemented!()
        },
        glk_selector::WINDOW_FLOW_BREAK => {
            unimplemented!()
        },
        glk_selector::WINDOW_ERASE_RECT => {
            unimplemented!()
        },
        glk_selector::WINDOW_FILL_RECT => {
            unimplemented!()
        },
        glk_selector::WINDOW_SET_BACKGROUND_COLOR => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_ITERATE => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_GET_ROCK => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_CREATE => {
            let rock = exec.call_args[0];
            let schanid = exec.glk.schannel_create(rock);
            exec.dispatch.schanids.get_index(schanid)
        },
        glk_selector::SCHANNEL_DESTROY => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_CREATE_EXT => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_PLAY_MULTI => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_PLAY => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_PLAY_EXT => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_STOP => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_SET_VOLUME => {
            unimplemented!()
        },
        glk_selector::SOUND_LOAD_HINT => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_SET_VOLUME_EXT => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_PAUSE => {
            unimplemented!()
        },
        glk_selector::SCHANNEL_UNPAUSE => {
            unimplemented!()
        },
        glk_selector::SET_HYPERLINK => {
            unimplemented!()
        },
        glk_selector::SET_HYPERLINK_STREAM => {
            unimplemented!()
        },
        glk_selector::REQUEST_HYPERLINK_EVENT => {
            unimplemented!()
        },
        glk_selector::CANCEL_HYPERLINK_EVENT => {
            unimplemented!()
        },
        glk_selector::BUFFER_TO_LOWER_CASE_UNI => {
            unimplemented!()
        },
        glk_selector::BUFFER_TO_UPPER_CASE_UNI => {
            unimplemented!()
        },
        glk_selector::BUFFER_TO_TITLE_CASE_UNI => {
            unimplemented!()
        },
        glk_selector::BUFFER_CANON_DECOMPOSE_UNI => {
            unimplemented!()
        },
        glk_selector::BUFFER_CANON_NORMALIZE_UNI => {
            unimplemented!()
        },
        glk_selector::PUT_CHAR_UNI => {
            unimplemented!()
        },
        glk_selector::PUT_STRING_UNI => {
            unimplemented!()
        },
        glk_selector::PUT_BUFFER_UNI => {
            unimplemented!()
        },
        glk_selector::PUT_CHAR_STREAM_UNI => {
            unimplemented!()
        },
        glk_selector::PUT_STRING_STREAM_UNI => {
            unimplemented!()
        },
        glk_selector::PUT_BUFFER_STREAM_UNI => {
            unimplemented!()
        },
        glk_selector::GET_CHAR_STREAM_UNI => {
            unimplemented!()
        },
        glk_selector::GET_BUFFER_STREAM_UNI => {
            unimplemented!()
        },
        glk_selector::GET_LINE_STREAM_UNI => {
            unimplemented!()
        },
        glk_selector::STREAM_OPEN_FILE_UNI => {
            unimplemented!()
        },
        glk_selector::STREAM_OPEN_MEMORY_UNI => {
            unimplemented!()
        },
        glk_selector::STREAM_OPEN_RESOURCE_UNI => {
            unimplemented!()
        },
        glk_selector::REQUEST_CHAR_EVENT_UNI => {
            unimplemented!()
        },
        glk_selector::REQUEST_LINE_EVENT_UNI => {
            unimplemented!()
        },
        glk_selector::SET_ECHO_LINE_EVENT => {
            unimplemented!()
        },
        glk_selector::SET_TERMINATORS_LINE_EVENT => {
            unimplemented!()
        },
        glk_selector::CURRENT_TIME => {
            unimplemented!()
        },
        glk_selector::CURRENT_SIMPLE_TIME => {
            unimplemented!()
        },
        glk_selector::TIME_TO_DATE_UTC => {
            unimplemented!()
        },
        glk_selector::TIME_TO_DATE_LOCAL => {
            unimplemented!()
        },
        glk_selector::SIMPLE_TIME_TO_DATE_UTC => {
            unimplemented!()
        },
        glk_selector::SIMPLE_TIME_TO_DATE_LOCAL => {
            unimplemented!()
        },
        glk_selector::DATE_TO_TIME_UTC => {
            unimplemented!()
        },
        glk_selector::DATE_TO_TIME_LOCAL => {
            unimplemented!()
        },
        glk_selector::DATE_TO_SIMPLE_TIME_UTC => {
            unimplemented!()
        },
        glk_selector::DATE_TO_SIMPLE_TIME_LOCAL => {
            unimplemented!()
        },
        _ => 0,
    }
}

fn write_event<G: Glk>(exec: &mut Execute<G>, addr: usize, mut event: G::Event) {
    if addr >= exec.ram_start {
        write_u32(&mut exec.state.mem, addr, event.evtype());
        write_u32(&mut exec.state.mem, addr+4, exec.dispatch.winids.get_index(event.win()));
        write_u32(&mut exec.state.mem, addr+8, event.val1());
        write_u32(&mut exec.state.mem, addr+12, event.val1());
    }
    if let Some((bufaddr,buf)) = event.buf() {
        if bufaddr as usize > exec.ram_start {
            write_arr8(&mut exec.state.mem, bufaddr as usize, &buf);
        }
        exec.dispatch.put_buffer8(buf);
    }
    if let Some((bufaddr,buf)) = event.buf_uni() {
        if bufaddr as usize > exec.ram_start {
            write_arr32(&mut exec.state.mem, bufaddr as usize, &buf);
        }
        exec.dispatch.put_buffer32(buf);
    }
}
