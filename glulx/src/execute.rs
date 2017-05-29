use rand;
use std::cmp::min;
use glk::Glk;

use super::{accel,call,gestalt,glk_dispatch,iosys,malloc,opcode,operand,search,state};
use super::state::{read_u8,read_u16,read_u32,write_u16,write_u32,State};

#[derive(Eq,PartialEq)]
pub enum Next {
    Quit,
    Exec,
    Ret(u32),
}

const MAX_UNDO_DEPTH: usize = 2;

pub struct Execute<'a,G: Glk<'a>> {
    pub state: State,

    pub undo_state: Vec<state::UndoState<operand::Mode>>,
    pub protected_range: (usize,usize),
    pub protected_tmp: Vec<u8>,
    pub rng: rand::XorShiftRng,
    pub stringtbl: usize,
    pub call_args: Vec<u32>,
    pub iosys: iosys::IOSys,
    pub accel: accel::Accel,
    pub dispatch: glk_dispatch::Dispatch<'a,G>,

    // Cache repeatedly used values
    pub ram_start: usize,
    pub frame_locals: usize,
    pub frame_end: usize,

    pub glk: G,

    pub trace: super::trace::Trace,
}

impl<'a,G: Glk<'a>> Execute<'a,G> {
    pub fn new(state: State, glk: G) -> Self {
        let stringtbl = read_u32(&state.rom, 28) as usize;
        let ram_start = read_u32(&state.rom, 8) as usize;
        let mut exec = Execute{
            state: state,

            undo_state: Vec::new(),
            protected_range: (0,0),
            protected_tmp: Vec::new(),
            rng: rand::SeedableRng::from_seed(rand::random()),
            stringtbl: stringtbl,
            call_args: Vec::new(),
            iosys: iosys::IOSys::new(),
            accel: accel::Accel::new(),
            dispatch: glk_dispatch::Dispatch::new(),

            ram_start: ram_start,
            frame_locals: 0,
            frame_end: 0,

            glk: glk,

            trace: super::trace::Trace::new(),
        };
        exec.start();
        exec
    }

    pub fn next(&mut self, next: Next) -> Next {
        match next {
            Next::Quit => Next::Quit,
            Next::Exec => self.exec_next(),
            Next::Ret(val) => call::ret(self, val),
        }
    }

    fn exec_next(&mut self) -> Next {
        let opcode_addr = self.state.pc;
        let mut opcode = read_u8(&self.state.mem, self.state.pc);
        match opcode & 0xc0 {
            0xc0 => {
                opcode = read_u32(&self.state.mem, self.state.pc) & 0x3fffffff;
                self.state.pc += 4;
            },
            0x80 => {
                opcode = read_u16(&self.state.mem, self.state.pc) & 0x3fff;
                self.state.pc += 2;
            },
            _ => self.state.pc += 1,
        }
        super::trace::opcode(self, opcode_addr, opcode);
        match opcode {
            opcode::NOP => {
                super::trace::frame(self);
            },
            opcode::ADD => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, l1.wrapping_add(l2));
            },
            opcode::SUB => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, l1.wrapping_sub(l2));
            },
            opcode::MUL => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, l1.wrapping_mul(l2));
            },
            opcode::DIV => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, (l1 as i32).wrapping_div(l2 as i32) as u32);
            },
            opcode::MOD => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, (l1 as i32).wrapping_rem(l2 as i32) as u32);
            },
            opcode::NEG => {
                let (l1,s1) = self.l1s1();
                s1.store(self, (l1 as i32).wrapping_neg() as u32);
            },
            opcode::BITAND => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, l1 & l2);
            },
            opcode::BITOR => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, l1 | l2);
            },
            opcode::BITXOR => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, l1 ^ l2);
            },
            opcode::BITNOT => {
                let (l1,s1) = self.l1s1();
                s1.store(self, !l1);
            },
            opcode::SHIFTL => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, if l2 >= 32 {
                        0
                    } else {
                        l1.wrapping_shl(l2)
                    });
            },
            opcode::SSHIFTR => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, if l2 >= 32 {
                        if l1 >= 0x80000000 {
                            0xffffffff
                        } else {
                            0
                        }
                    } else {
                        (l1 as i32).wrapping_shr(l2) as u32
                    });
            },
            opcode::USHIFTR => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, if l2 >= 32 {
                        0
                    } else {
                        l1.wrapping_shr(l2)
                    });
            },
            opcode::JUMP => {
                let l1 = self.l1();
                return self.jump(l1);
            },
            opcode::JZ => {
                let (l1,l2) = self.l1l2();
                if l1 == 0 {
                    return self.jump(l2);
                }
            },
            opcode::JNZ => {
                let (l1,l2) = self.l1l2();
                if l1 != 0 {
                    return self.jump(l2);
                }
            },
            opcode::JEQ => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 == l2 {
                    return self.jump(l3);
                }
            },
            opcode::JNE => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 != l2 {
                    return self.jump(l3);
                }
            },
            opcode::JLT => {
                let (l1,l2,l3) = self.l1l2l3();
                if (l1 as i32) < l2 as i32 {
                    return self.jump(l3);
                }
            },
            opcode::JGE => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 as i32 >= l2 as i32 {
                    return self.jump(l3);
                }
            },
            opcode::JGT => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 as i32 > l2 as i32 {
                    return self.jump(l3);
                }
            },
            opcode::JLE => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 as i32 <= l2 as i32 {
                    return self.jump(l3);
                }
            },
            opcode::JLTU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 < l2 {
                    return self.jump(l3);
                }
            },
            opcode::JGEU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 >= l2 {
                    return self.jump(l3);
                }
            },
            opcode::JGTU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 > l2 {
                    return self.jump(l3);
                }
            },
            opcode::JLEU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 <= l2 {
                    return self.jump(l3);
                }
            },
            opcode::CALL => {
                let (l1,l2,s1) = self.l1l2s1();
                let addr = l1 as usize;
                let (dest_type,dest_addr) = s1.result_dest(self);
                self.call_args.clear();
                for _ in 0 .. l2 {
                    self.call_args.push(self.state.stack.pop().unwrap());
                }
                call::call(self, addr, dest_type, dest_addr);
                self.tick();
            },
            opcode::RETURN => {
                let l1 = self.l1();
                self.tick();
                return call::ret(self, l1);
            },
            opcode::CATCH => {
                let ((dest_type,dest_addr),l1) = self.s1l1();
                call::push_stub(self, dest_type, dest_addr);
                let catch_token = 4 * self.state.stack.len() as u32;
                call::store_ret_result(self, catch_token, dest_type, dest_addr as usize);
                return self.jump(l1);
            },
            opcode::THROW => {
                let (l1,l2) = self.l1l2();
                self.state.stack.truncate((l2 / 4) as usize);
                self.state.frame_ptr = self.state.stack.len();
                self.tick();
                return Next::Ret(l1);
            },
            opcode::TAILCALL => {
                let (l1,l2) = self.l1l2();
                let addr = l1 as usize;
                self.call_args.clear();
                for _ in 0 .. l2 {
                    self.call_args.push(self.state.stack.pop().unwrap());
                }
                self.state.stack.truncate(self.state.frame_ptr);
                self.tick();
                return call::tailcall(self, addr);
            },
            opcode::COPY => {
                let (l1,s1) = self.l1s1();
                s1.store(self, l1);
            },
            opcode::COPYS => {
                let (m1,m2) = operand::next_mode(&mut self.state);
                let val = m1.load16(self) & 0x0000ffff;
                m2.store16(self, val);
            },
            opcode::COPYB => {
                let (m1,m2) = operand::next_mode(&mut self.state);
                let val = m1.load8(self) & 0x000000ff;
                m2.store8(self, val);
            },
            opcode::SEXS => {
                let (l1,s1) = self.l1s1();
                if l1 & 0x8000 == 0 {
                    s1.store(self, l1 & 0xffff);
                } else {
                    s1.store(self, l1 | 0xffff0000);
                }
            },
            opcode::SEXB => {
                let (l1,s1) = self.l1s1();
                if l1 & 0x80 == 0 {
                    s1.store(self, l1 & 0xff);
                } else {
                    s1.store(self, l1 | 0xffffff00);
                }
            },
            opcode::ALOAD => {
                let (l1,l2,s1) = self.l1l2s1();
                let addr = l1.wrapping_add(l2.wrapping_mul(4));
                let val = read_u32(&self.state.mem, addr as usize);
                s1.store(self, val);
            },
            opcode::ALOADS => {
                let (l1,l2,s1) = self.l1l2s1();
                let addr = l1.wrapping_add(l2.wrapping_mul(2));
                let val = read_u16(&self.state.mem, addr as usize);
                s1.store(self, val);
            },
            opcode::ALOADB => {
                let (l1,l2,s1) = self.l1l2s1();
                let val = self.state.mem[l1.wrapping_add(l2) as usize] as u32;
                s1.store(self, val);
            },
            opcode::ALOADBIT => {
                let (l1,l2,s1) = self.l1l2s1();
                let byteoffset = (l2 as i32 >> 3) as u32;
                let bitoffset = (l2 & 7) as u8;
                let addr = l1.wrapping_add(byteoffset);
                let val = self.state.mem[addr as usize];
                s1.store(self, if val & (1 << bitoffset) == 0 { 0 } else { 1 });
            },
            opcode::ASTORE => {
                let (l1,l2,l3) = self.l1l2l3();
                let addr = l1.wrapping_add((l2 as i32).wrapping_mul(4) as u32);
                write_u32(&mut self.state.mem, addr as usize, l3);
            },
            opcode::ASTORES => {
                let (l1,l2,l3) = self.l1l2l3();
                let addr = l1.wrapping_add(l2.wrapping_mul(2));
                write_u16(&mut self.state.mem, addr as usize, l3);
            },
            opcode::ASTOREB => {
                let (l1,l2,l3) = self.l1l2l3();
                self.state.mem[l1.wrapping_add(l2) as usize] = l3 as u8;
            },
            opcode::ASTOREBIT => {
                let (l1,l2,l3) = self.l1l2l3();
                let byteoffset = (l2 as i32 >> 3) as u32;
                let bitoffset = (l2 & 7) as u8;
                let addr = l1.wrapping_add(byteoffset);
                if l3 == 0 {
                    self.state.mem[addr as usize] &= !(1 << bitoffset as u8);
                } else {
                    self.state.mem[addr as usize] |= 1 << bitoffset as u8;
                }
            },
            opcode::STKCOUNT => {
                let s1 = self.s1();
                let count = (self.state.stack.len() - self.frame_end) as u32;
                s1.store(self, count);
            },
            opcode::STKPEEK => {
                let (l1,s1) = self.l1s1();
                assert!(self.frame_end + (l1 as usize) < self.state.stack.len(), "{:x}:stkpeek {:x} underflow", self.state.pc, l1);
                let val = self.state.stack[self.state.stack.len() - 1 - l1 as usize];
                s1.store(self, val);
            },
            opcode::STKSWAP => {
                super::trace::frame(self);
                assert!(self.frame_end + 2 <= self.state.stack.len(), "{:x}:stkswap underflow", self.state.pc);
                let index = self.state.stack.len()-1;
                self.state.stack.swap(index,index-1);
            },
            opcode::STKROLL => {
                let (l1,l2) = self.l1l2();
                let len = self.state.stack.len();
                assert!(self.frame_end + l1 as usize <= len, "{:x}:stkroll {:x} underflow {:x}", self.state.pc, l1, l2);
                if l1 > 0 {
                    let count = (l1 as i32 - l2 as i32 % l1 as i32) % l1 as i32;
                    for _ in 0 .. count {
                        let val = self.state.stack.remove(len - l1 as usize);
                        self.state.stack.push(val);

                    }
                }
            },
            opcode::STKCOPY => {
                let l1 = self.l1();
                let len = self.state.stack.len();
                assert!(self.frame_end + l1 as usize <= len, "{:x}:stkcopy {:x} underflow", self.state.pc, l1);
                for i in len - l1 as usize .. len {
                    let val = self.state.stack[i];
                    self.state.stack.push(val);
                }
            },
            opcode::STREAMCHAR => {
                let l1 = self.l1();
                return iosys::streamchar(self, l1 as u8, false);
            },
            opcode::STREAMNUM => {
                let l1 = self.l1();
                return iosys::streamnum(self, l1 as i32, false);
            },
            opcode::STREAMSTR => {
                let l1 = self.l1();
                return iosys::streamstr(self, l1 as usize, false);
            },
            opcode::STREAMUNICHAR => {
                let l1 = self.l1();
                return iosys::streamunichar(self, l1, false);
            },
            opcode::GESTALT => {
                let (l1,l2,s1) = self.l1l2s1();
                let val = gestalt::get(&self, l1, l2);
                s1.store(self, val);
            },
            opcode::DEBUGTRAP => {
                let l1 = self.l1();
                panic!("debugtrap {:x} pc={:x}", l1, self.state.pc);
            },
            opcode::GETMEMSIZE => {
                let s1 = self.s1();
                let memsize = self.state.mem.len() as u32;
                s1.store(self, memsize);
            },
            opcode::SETMEMSIZE => {
                let (l1,s1) = self.l1s1();
                let end_mem = read_u32(&self.state.mem, 16);
                if !self.state.heap.is_empty() && l1 < end_mem {
                    s1.store(self, 1);
                } else {
                    self.state.mem.resize(l1 as usize, 0);
                    s1.store(self, 0);
                }
            },
            opcode::JUMPABS => {
                let l1 = self.l1();
                self.tick();
                self.state.pc = l1 as usize;
            },
            opcode::RANDOM => {
                use rand::Rng;
                let (l1,s1) = self.l1s1();
                let val = if l1 == 0 {
                    self.rng.next_u32()
                } else if l1 as i32 > 0 {
                    self.rng.gen_range(0, l1)
                } else {
                    self.rng.gen_range(l1-1, 0xffffffff).wrapping_add(2)
                };
                s1.store(self, val);
            },
            opcode::SETRANDOM => {
                use rand::SeedableRng;
                let l1 = self.l1();
                self.rng.reseed(if l1 == 0 { rand::random() } else { [l1; 4] });
            },
            opcode::QUIT => {
                super::trace::frame(self);
                return Next::Quit;
            },
            opcode::VERIFY => {
                let s1 = self.s1();
                s1.store(self, 0);
            },
            opcode::RESTART => {
                super::trace::frame(self);
                self.stash_protected_range();
                self.state.reset_mem();
                self.unstash_protected_range();
                self.start();
            },
            opcode::SAVE => {
                let (l1,s1) = self.l1s1();
                match iosys::save(self, l1) {
                    Ok(()) => s1.store(self, 0),
                    _ => s1.store(self, 1),
                }
            },
            opcode::RESTORE => {
                let (l1,s1) = self.l1s1();
                match iosys::restore(self, l1) {
                    Ok(()) => {
                        return Next::Ret(0xffffffff);
                    },
                    _ => s1.store(self, 1),
                }
            },
            opcode::SAVEUNDO => {
                let s1 = self.s1();
                let mut result = 1;
                for mut undo_state in &mut self.undo_state {
                    if undo_state.save(&self.state, s1) {
                        result = 0;
                        break;
                    }
                }
                if result != 0 {
                    let mut undo_state = if self.undo_state.len() < MAX_UNDO_DEPTH {
                        state::UndoState::new()
                    } else {
                        self.undo_state.remove(0)
                    };
                    if undo_state.save(&self.state, s1) {
                        self.undo_state.push(undo_state);
                        result = 0;
                    }
                }
                s1.store(self, result);
            },
            opcode::RESTOREUNDO => {
                let s1 = self.s1();
                self.stash_protected_range();
                let len = self.undo_state.len();
                let mut failed = true;
                for i in 0 .. len {
                    if let Some(s1) = self.undo_state[len - i - 1].restore(&mut self.state) {
                        self.unstash_protected_range();
                        self.frame_locals = self.state.frame_ptr + self.state.stack[self.state.frame_ptr] as usize / 4;
                        self.frame_end = self.state.frame_ptr + self.state.stack[self.state.frame_ptr+1] as usize / 4;
                        s1.store(self, 0xffffffff);
                        failed = false;
                        break;
                    }
                }
                if failed {
                    s1.store(self, 1);
                }
            },
            opcode::PROTECT => {
                let (l1,l2) = self.l1l2();
                self.protected_range = (l1 as usize,l2 as usize);
            },
            opcode::GLK => {
                let (l1,l2,s1) = self.l1l2s1();
                self.call_args.clear();
                for _ in 0 .. l2 {
                    self.call_args.push(self.state.stack.pop().unwrap());
                }
                let result = glk_dispatch::dispatch(self, l1);
                s1.store(self, result);
            },
            opcode::GETSTRINGTBL => {
                let s1 = self.s1();
                let stringtbl = self.stringtbl as u32;
                s1.store(self, stringtbl);
            },
            opcode::SETSTRINGTBL => {
                let l1 = self.l1();
                self.stringtbl = l1 as usize;
            },
            opcode::GETIOSYS => {
                let (s1,s2) = self.s1s2();
                let (mode,rock) = self.iosys.get();
                s1.store(self, mode);
                s2.store(self, rock);
            },
            opcode::SETIOSYS => {
                let (l1,l2) = self.l1l2();
                self.iosys.set(l1, l2);
            },
            opcode::LINEARSEARCH => {
                let (l1,l2,l3,l4,l5,l6,l7,s1) = self.l1l2l3l4l5l6l7s1();
                let val = search::linear(&self.state, l1, l2 as usize, l3 as usize, l4 as usize, l5 as usize, l6 as usize, l7);
                s1.store(self, val);
            },
            opcode::BINARYSEARCH => {
                let (l1,l2,l3,l4,l5,l6,l7,s1) = self.l1l2l3l4l5l6l7s1();
                let val = search::binary(&self.state, l1, l2 as usize, l3 as usize, l4 as usize, l5 as usize, l6 as usize, l7);
                s1.store(self, val);
            },
            opcode::LINKEDSEARCH => {
                let (l1,l2,l3,l4,l5,l6,s1) = self.l1l2l3l4l5l6s1();
                let val = search::linked(&self.state, l1, l2 as usize, l3 as usize, l4 as usize, l5 as usize, l6);
                s1.store(self, val);
            },
            opcode::CALLF => {
                let (l1,s1) = self.l1s1();
                let addr = l1 as usize;
                let (dest_type,dest_addr) = s1.result_dest(self);
                self.call_args.clear();
                call::call(self, addr, dest_type, dest_addr);
                self.tick();
            },
            opcode::CALLFI => {
                let (l1,l2,s1) = self.l1l2s1();
                let addr = l1 as usize;
                let (dest_type,dest_addr) = s1.result_dest(self);
                self.call_args.clear();
                self.call_args.push(l2);
                call::call(self, addr, dest_type, dest_addr);
                self.tick();
            },
            opcode::CALLFII => {
                let (l1,l2,l3,s1) = self.l1l2l3s1();
                let addr = l1 as usize;
                let (dest_type,dest_addr) = s1.result_dest(self);
                self.call_args.clear();
                self.call_args.push(l2);
                self.call_args.push(l3);
                call::call(self, addr, dest_type, dest_addr);
                self.tick();
            },
            opcode::CALLFIII => {
                let (l1,l2,l3,l4,s1) = self.l1l2l3l4s1();
                let addr = l1 as usize;
                let (dest_type,dest_addr) = s1.result_dest(self);
                self.call_args.clear();
                self.call_args.push(l2);
                self.call_args.push(l3);
                self.call_args.push(l4);
                call::call(self, addr, dest_type, dest_addr);
                self.tick();
            },
            opcode::MZERO => {
                let (l1,l2) = self.l1l2();
                for i in l2 .. l2 + l1 {
                    self.state.mem[i as usize] = 0;
                }
            },
            opcode::MCOPY => {
                let (l1,l2,l3) = self.l1l2l3();
                if l2 >= l3 {
                    for i in 0 .. l1 as usize {
                        let b = self.state.mem[l2 as usize + i];
                        self.state.mem[l3 as usize + i] = b;
                    }
                } else {
                    for i in 0 .. l1 as usize {
                        let b = self.state.mem[(l1 + l2 - 1) as usize - i];
                        self.state.mem[(l1 + l3 - 1) as usize - i] = b;
                    }
                }
            },
            opcode::MALLOC => {
                let (l1,s1) = self.l1s1();
                let addr = malloc::malloc(&mut self.state, l1 as usize);
                s1.store(self, addr as u32);
            },
            opcode::MFREE => {
                let l1 = self.l1();
                malloc::free(&mut self.state, l1 as usize);
            },
            opcode::ACCELFUNC => {
                let (l1,l2) = self.l1l2();
                self.accel.func(l1, l2 as usize);
            },
            opcode::ACCELPARAM => {
                let (l1,l2) = self.l1l2();
                self.accel.param(l1, l2);
            },
            opcode::NUMTOF => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(l1 as i32 as f32));
            },
            opcode::FTONUMZ => {
                let (l1,s1) = self.l1s1();
                let f = to_f32(l1);
                s1.store(self, if f.is_finite() && f > i32::min_value() as f32 && f < i32::max_value() as f32 {
                        f.trunc() as i32 as u32
                    } else if l1 & 0x80000000 == 0 {
                        0x7fffffff
                    } else {
                        0x80000000
                    });
            },
            opcode::FTONUMN => {
                let (l1,s1) = self.l1s1();
                let f = to_f32(l1);
                s1.store(self, if f.is_finite() && f > i32::min_value() as f32 && f < i32::max_value() as f32 {
                        f.round() as i32 as u32
                    } else if l1 & 0x80000000 == 0 {
                        0x7fffffff
                    } else {
                        0x80000000
                    });
            },
            opcode::CEIL => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).ceil()));
            },
            opcode::FLOOR => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).floor()));
            },
            opcode::FADD => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, from_f32(to_f32(l1) + to_f32(l2)));
            },
            opcode::FSUB => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, from_f32(to_f32(l1) - to_f32(l2)));
            },
            opcode::FMUL => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, from_f32(to_f32(l1) * to_f32(l2)));
            },
            opcode::FDIV => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, from_f32(to_f32(l1) / to_f32(l2)));
            },
            opcode::FMOD => {
                let (l1,l2,s1,s2) = self.l1l2s1s2();
                let f1 = to_f32(l1);
                let f2 = to_f32(l2);
                let f3 = f1 % f2;
                s1.store(self, from_f32(f3));
                s2.store(self, from_f32((f1 - f3).abs()/f2.abs()*f1.signum()*f2.signum()));
            },
            opcode::SQRT => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).sqrt()));
            },
            opcode::EXP => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).exp()));
            },
            opcode::LOG => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).ln()));
            },
            opcode::POW => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, from_f32(to_f32(l1).powf(to_f32(l2))));
            },
            opcode::SIN => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).sin()));
            },
            opcode::COS => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).cos()));
            },
            opcode::TAN => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).tan()));
            },
            opcode::ASIN => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).asin()));
            },
            opcode::ACOS => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).acos()));
            },
            opcode::ATAN => {
                let (l1,s1) = self.l1s1();
                s1.store(self, from_f32(to_f32(l1).atan()));
            },
            opcode::ATAN2 => {
                let (l1,l2,s1) = self.l1l2s1();
                s1.store(self, from_f32(to_f32(l1).atan2(to_f32(l2))));
            },
            opcode::JFEQ => {
                let (l1,l2,l3,l4) = self.l1l2l3l4();
                let f1 = to_f32(l1);
                let f2 = to_f32(l2);
                let f3 = to_f32(l3);
                if f1.is_nan() || f2.is_nan() || f3.is_nan() {
                } else if f3.is_infinite() {
                    if !f1.is_infinite() || !f2.is_infinite() || f1.signum() == f2.signum() {
                        return self.jump(l4);
                    }
                } else if f1.is_infinite() && f2.is_infinite() {
                    if f1.signum() == f2.signum() {
                        return self.jump(l4);
                    }
                } else if (f1 - f2).abs() <= f3.abs() {
                    return self.jump(l4);
                }
            },
            opcode::JFNE => {
                let (l1,l2,l3,l4) = self.l1l2l3l4();
                let f1 = to_f32(l1);
                let f2 = to_f32(l2);
                let f3 = to_f32(l3);
                if f1.is_nan() || f2.is_nan() || f3.is_nan() {
                    return self.jump(l4);
                } else if f3.is_infinite() {
                    if f1.is_infinite() && f2.is_infinite() && f1.signum() != f2.signum() {
                        return self.jump(l4);
                    }
                } else if f1.is_infinite() && f2.is_infinite() {
                    if f1.signum() != f2.signum() {
                        return self.jump(l4);
                    }
                } else if (f1 - f2).abs() > f3.abs() {
                    return self.jump(l4);
                }
            },
            opcode::JFLT => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) < to_f32(l2) {
                    return self.jump(l3);
                }
            },
            opcode::JFLE => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) <= to_f32(l2) {
                    return self.jump(l3);
                }
            },
            opcode::JFGT => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) > to_f32(l2) {
                    return self.jump(l3);
                }
            },
            opcode::JFGE => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) >= to_f32(l2) {
                    return self.jump(l3);
                }
            },
            opcode::JISNAN => {
                let (l1,l2) = self.l1l2();
                if to_f32(l1).is_nan() {
                    return self.jump(l2);
                }
            },
            opcode::JISINF => {
                let (l1,l2) = self.l1l2();
                if to_f32(l1).is_infinite() {
                    return self.jump(l2);
                }
            },
            _ => {
                panic!("{:x}: unknown opcode {:x}", opcode_addr, opcode);
            },
        }
        Next::Exec
    }

    fn start(&mut self) {
        let start_func = read_u32(&self.state.rom, 24) as usize;
        self.call_args.clear();
        call::tailcall(self, start_func);
    }

    fn l1(&mut self) -> u32 {
        let m1 = operand::last_mode(&mut self.state);
        let l1 = m1.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::frame(self);
        l1
    }

    fn l1l2(&mut self) -> (u32,u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::frame(self);
        (l1,l2)
    }

    fn l1l2l3(&mut self) -> (u32,u32,u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let m3 = operand::last_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, Some(l3));
        super::trace::frame(self);
        (l1,l2,l3)
    }

    fn l1l2l3l4(&mut self) -> (u32,u32,u32,u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        let l4 = m4.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, Some(l3));
        super::trace::operand(self, &m4, Some(l4));
        super::trace::frame(self);
        (l1,l2,l3,l4)
    }

    fn l1s1(&mut self) -> (u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, None);
        super::trace::frame(self);
        (l1,m2)
    }

    fn l1l2s1(&mut self) -> (u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,_) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, None);
        super::trace::frame(self);
        (l1,l2,m3)
    }

    fn l1l2l3s1(&mut self) -> (u32,u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, Some(l3));
        super::trace::operand(self, &m4, None);
        super::trace::frame(self);
        (l1,l2,l3,m4)
    }

    fn l1l2s1s2(&mut self) -> (u32,u32,operand::Mode,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, None);
        super::trace::operand(self, &m4, None);
        super::trace::frame(self);
        (l1,l2,m3,m4)
    }

    fn l1l2l3l4s1(&mut self) -> (u32,u32,u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let m5 = operand::last_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        let l4 = m4.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, Some(l3));
        super::trace::operand(self, &m4, Some(l4));
        super::trace::operand(self, &m5, None);
        super::trace::frame(self);
        (l1,l2,l3,l4,m5)
    }

    fn l1l2l3l4l5l6s1(&mut self) -> (u32,u32,u32,u32,u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let (m5,m6) = operand::next_mode(&mut self.state);
        let m7 = operand::last_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        let l4 = m4.load(self);
        let l5 = m5.load(self);
        let l6 = m6.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, Some(l3));
        super::trace::operand(self, &m4, Some(l4));
        super::trace::operand(self, &m5, Some(l5));
        super::trace::operand(self, &m6, Some(l6));
        super::trace::operand(self, &m7, None);
        super::trace::frame(self);
        (l1,l2,l3,l4,l5,l6,m7)
    }

    fn l1l2l3l4l5l6l7s1(&mut self) -> (u32,u32,u32,u32,u32,u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let (m5,m6) = operand::next_mode(&mut self.state);
        let (m7,m8) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        let l4 = m4.load(self);
        let l5 = m5.load(self);
        let l6 = m6.load(self);
        let l7 = m7.load(self);
        super::trace::operand(self, &m1, Some(l1));
        super::trace::operand(self, &m2, Some(l2));
        super::trace::operand(self, &m3, Some(l3));
        super::trace::operand(self, &m4, Some(l4));
        super::trace::operand(self, &m5, Some(l5));
        super::trace::operand(self, &m6, Some(l6));
        super::trace::operand(self, &m7, Some(l7));
        super::trace::operand(self, &m8, None);
        super::trace::frame(self);
        (l1,l2,l3,l4,l5,l6,l7,m8)
    }

    fn s1(&mut self) -> operand::Mode {
        let m1 = operand::last_mode(&mut self.state);
        super::trace::operand(self, &m1, None);
        super::trace::frame(self);
        m1
    }

    fn s1l1(&mut self) -> ((u32,u32),u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let result_dest = m1.result_dest(self);
        let l1 = m2.load(self);
        super::trace::operand(self, &m1, None);
        super::trace::operand(self, &m2, Some(l1));
        super::trace::frame(self);
        (result_dest,l1)
    }

    fn s1s2(&mut self) -> (operand::Mode,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        super::trace::operand(self, &m1, None);
        super::trace::operand(self, &m2, None);
        super::trace::frame(self);
        (m1,m2)
    }

    fn jump(&mut self, offset: u32) -> Next {
        self.tick();
        match offset {
            0 => Next::Ret(0),
            1 => Next::Ret(1),
            _ => {
                let pc = self.state.pc as i32 + offset as i32 - 2;
                self.state.pc = pc as usize & 0xffffffff;
                Next::Exec
            }
        }
    }

    fn stash_protected_range(&mut self) {
        let (start, len) = self.protected_range;

        self.protected_tmp.clear();
        if start >= self.state.mem.len() {
            self.protected_tmp.resize(len, 0);
            return;
        }
        let end = min(start+len, self.state.mem.len());
        self.protected_tmp.extend_from_slice(&self.state.mem[start .. end]);
        self.protected_tmp.resize(len, 0);
    }

    fn unstash_protected_range(&mut self) {
        let (start, len) = self.protected_range;
        if start >= self.state.mem.len() {
            return;
        }
        let safe_len = min(len, min(self.protected_tmp.len(), self.state.mem.len() - start));
        for i in 0 .. safe_len {
            self.state.mem[start+i] = self.protected_tmp[i];
        }
    }

    fn tick(&mut self) {
        self.glk.tick();
    }
}

fn to_f32(val: u32) -> f32 {
    use std;
    unsafe { std::mem::transmute(val) }
}

fn from_f32(val: f32) -> u32 {
    use std;
    unsafe { std::mem::transmute(val) }
}
