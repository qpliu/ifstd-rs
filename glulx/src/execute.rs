use rand;
use glk::Glk;

use super::{accel,call,gestalt,glk_dispatch,iosys,malloc,opcode,operand,search,state};
use super::state::{read_u16,read_u32,write_u16,write_u32,State};

pub struct Execute<G: Glk> {
    pub state: State,

    pub undo_state: state::UndoState<operand::Mode>,
    pub protected_range: (usize,usize),
    pub protected_tmp: Vec<u8>,
    pub rng: rand::XorShiftRng,
    pub stringtbl: usize,
    pub call_args: Vec<u32>,
    pub iosys: iosys::IOSys,
    pub accel: accel::Accel,
    pub dispatch: glk_dispatch::Dispatch<G>,

    // Cache repeatedly used values
    pub ram_start: usize,
    pub frame_locals: usize,
    pub frame_end: usize,

    pub glk: G,
}

impl<G: Glk> Execute<G> {
    pub fn new(state: State, glk: G) -> Self {
        let stringtbl = read_u32(&state.rom, 28) as usize;
        let ram_start = read_u32(&state.rom, 8) as usize;
        Execute{
            state: state,

            undo_state: state::UndoState::new(),
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
        }
    }

    pub fn next(&mut self) -> bool {
        let opcode_addr = self.state.pc;
        let mut opcode = self.state.mem[self.state.pc] as u32;
        match opcode & 0xc0 {
            0xc0 => {
                opcode = (opcode & 0x03f) << 24
                    | (self.state.mem[self.state.pc+1] as u32) << 16
                    | (self.state.mem[self.state.pc+2] as u32) << 8
                    | self.state.mem[self.state.pc+3] as u32;
                self.state.pc += 4;
            },
            0x80 => {
                opcode = (opcode & 0x03f) << 8 | self.state.mem[self.state.pc+1] as u32;
                self.state.pc += 2;
            },
            _ => self.state.pc += 1,
        }
        match opcode {
            opcode::NOP => (),
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
                if !self.jump(l1) {
                    return false;
                }
            },
            opcode::JZ => {
                let (l1,l2) = self.l1l2();
                if l1 == 0 {
                    if !self.jump(l2) {
                        return false;
                    }
                }
            },
            opcode::JNZ => {
                let (l1,l2) = self.l1l2();
                if l1 != 0 {
                    if !self.jump(l2) {
                        return false;
                    }
                }
            },
            opcode::JEQ => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 == l2 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JNE => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 != l2 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JLT => {
                let (l1,l2,l3) = self.l1l2l3();
                if (l1 as i32) < l2 as i32 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JGE => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 as i32 >= l2 as i32 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JGT => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 as i32 > l2 as i32 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JLE => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 as i32 <= l2 as i32 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JLTU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 < l2 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JGEU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 >= l2 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JGTU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 > l2 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JLEU => {
                let (l1,l2,l3) = self.l1l2l3();
                if l1 <= l2 {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::CALL => {
                let (l1,l2,s1) = self.l1l2s1();
                let addr = l1 as usize;
                let (dest_type,dest_addr) = s1.result_dest(self);
                self.call_args.clear();
                let args = self.state.stack.len() - l2 as usize .. self.state.stack.len();
                self.call_args.extend(self.state.stack.drain(args));
                call::call(self, addr, dest_type, dest_addr);
                self.tick();
            },
            opcode::RETURN => {
                let l1 = self.l1();
                if !call::ret(self, l1) {
                    return false;
                }
                self.tick();
            },
            opcode::CATCH => {
                let (s1,l1) = self.s1l1();
                let (dest_type,dest_addr) = s1.result_dest(self);
                call::push_stub(&mut self.state, dest_type, dest_addr);
                let catch_token = self.state.stack.len() as u32;
                s1.store(self, catch_token);
                if !self.jump(l1) {
                    return false;
                }
            },
            opcode::THROW => {
                let (l1,l2) = self.l1l2();
                self.state.stack.truncate(l2 as usize);
                let return_status = call::ret(self, l1);
                assert!(return_status, "{:x}:throw {:x} {:x}", self.state.pc, l1, l2);
                self.tick();
            },
            opcode::TAILCALL => {
                let (l1,l2) = self.l1l2();
                let addr = l1 as usize;
                self.call_args.clear();
                let args = self.state.stack.len() - l2 as usize .. self.state.stack.len();
                self.call_args.extend_from_slice(&self.state.stack[args]);
                self.state.stack.truncate(self.state.frame_ptr);
                if !call::tailcall(self, addr) {
                    return false;
                }
                self.tick();
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
                let val = read_u32(&self.state.mem, (l1+4*l2) as usize);
                s1.store(self, val);
            },
            opcode::ALOADS => {
                let (l1,l2,s1) = self.l1l2s1();
                let val = read_u16(&self.state.mem, (l1+2*l2) as usize);
                s1.store(self, val);
            },
            opcode::ALOADB => {
                let (l1,l2,s1) = self.l1l2s1();
                let val = self.state.mem[(l1+l2) as usize] as u32;
                s1.store(self, val);
            },
            opcode::ALOADBIT => {
                let (l1,l2,s1) = self.l1l2s1();
                let val = self.state.mem[(l1+l2/8) as usize];
                s1.store(self, if val & (1 << (l2 % 8) as u8) == 0 { 0 } else { 1 });
            },
            opcode::ASTORE => {
                let (l1,l2,l3) = self.l1l2l3();
                write_u32(&mut self.state.mem, (l1+4*l2) as usize, l3);
            },
            opcode::ASTORES => {
                let (l1,l2,l3) = self.l1l2l3();
                write_u16(&mut self.state.mem, (l1+2*l2) as usize, l3);
            },
            opcode::ASTOREB => {
                let (l1,l2,l3) = self.l1l2l3();
                self.state.mem[(l1+l2) as usize] = l3 as u8;
            },
            opcode::ASTOREBIT => {
                let (l1,l2,l3) = self.l1l2l3();
                if l3 == 0 {
                    self.state.mem[(l1+l2/8) as usize] &= !(1 << (l2 % 8) as u8);
                } else {
                    self.state.mem[(l1+l2/8) as usize] |= 1 << (l2 % 8) as u8;
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
                assert!(self.frame_end + 2 <= self.state.stack.len(), "{:x}:stkswap underflow", self.state.pc);
                let index = self.state.stack.len()-1;
                self.state.stack.swap(index-1,index-2);
            },
            opcode::STKROLL => {
                let (l1,l2) = self.l1l2();
                let len = self.state.stack.len();
                assert!(self.frame_end + l1 as usize <= len, "{:x}:stkroll {:x} underflow {:x}", self.state.pc, l1, l2);
                let count = (l1 as i32 - l2 as i32 % l1 as i32) % l1 as i32;
                for _ in 0 .. count {
                    let val = self.state.stack.remove(len - l1 as usize);
                    self.state.stack.push(val);
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
                iosys::streamchar(self, l1 as u8, false);
            },
            opcode::STREAMNUM => {
                let l1 = self.l1();
                iosys::streamnum(self, l1 as i32, false);
            },
            opcode::STREAMSTR => {
                let l1 = self.l1();
                iosys::streamstr(self, l1 as usize, false);
            },
            opcode::STREAMUNICHAR => {
                let l1 = self.l1();
                iosys::streamunichar(self, l1, false);
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
                    self.rng.gen_range(l1 - 1, 0xffffffff) + 1
                };
                s1.store(self, val);
            },
            opcode::SETRANDOM => {
                use rand::SeedableRng;
                let l1 = self.l1();
                self.rng.reseed(if l1 == 0 { rand::random() } else { [l1; 4] });
            },
            opcode::QUIT => {
                return false;
            },
            opcode::VERIFY => {
                let s1 = self.s1();
                s1.store(self, 0);
            },
            opcode::RESTART => {
                self.stash_protected_range();
                self.state.reset_mem();
                self.unstash_protected_range();
                let start_func = read_u32(&self.state.rom, 24) as usize;
                self.call_args.clear();
                call::tailcall(self, start_func);
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
                        if !call::ret(self, 0xffffffff) {
                            return false;
                        }
                    },
                    _ => s1.store(self, 1),
                }
            },
            opcode::SAVEUNDO => {
                let s1 = self.s1();
                self.undo_state.save(&self.state, s1);
                s1.store(self, 0);
            },
            opcode::RESTOREUNDO => {
                let s1 = self.s1();
                self.stash_protected_range();
                match self.undo_state.restore(&mut self.state) {
                    None => s1.store(self, 1),
                    Some(s1) => {
                        self.unstash_protected_range();
                        s1.store(self, (-1i32) as u32);
                    },
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
                s1.store(self, if f.is_finite() {
                        f.trunc() as i32 as u32
                    } else if l1 >= 0x8000000 {
                        0x80000000
                    } else {
                        0x7fffffff
                    });
            },
            opcode::FTONUMN => {
                let (l1,s1) = self.l1s1();
                let f = to_f32(l1);
                s1.store(self, if f.is_finite() {
                        f.round() as i32 as u32
                    } else if l1 >= 0x8000000 {
                        0x80000000
                    } else {
                        0x7fffffff
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
                if (to_f32(l1) - to_f32(l2)).abs() <= to_f32(l3).abs() {
                    if !self.jump(l4) {
                        return false;
                    }
                }
            },
            opcode::JFNE => {
                let (l1,l2,l3,l4) = self.l1l2l3l4();
                if !((to_f32(l1) - to_f32(l2)).abs() <= to_f32(l3).abs()) {
                    if !self.jump(l4) {
                        return false;
                    }
                }
            },
            opcode::JFLT => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) < to_f32(l2) {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JFLE => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) <= to_f32(l2) {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JFGT => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) > to_f32(l2) {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JFGE => {
                let (l1,l2,l3) = self.l1l2l3();
                if to_f32(l1) >= to_f32(l2) {
                    if !self.jump(l3) {
                        return false;
                    }
                }
            },
            opcode::JISNAN => {
                let (l1,l2) = self.l1l2();
                if to_f32(l1).is_nan() {
                    if !self.jump(l2) {
                        return false;
                    }
                }
            },
            opcode::JISINF => {
                let (l1,l2) = self.l1l2();
                if to_f32(l1).is_infinite() {
                    if !self.jump(l2) {
                        return false;
                    }
                }
            },
            _ => {
                panic!("{:x}: unknown opcode {:x}", opcode_addr, opcode);
            },
        }
        true
    }

    fn l1(&mut self) -> u32 {
        let (m1,_) = operand::next_mode(&mut self.state);
        m1.load(self)
    }

    fn l1l2(&mut self) -> (u32,u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        (l1,l2)
    }

    fn l1l2l3(&mut self) -> (u32,u32,u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,_) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        (l1,l2,l3)
    }

    fn l1l2l3l4(&mut self) -> (u32,u32,u32,u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        let l4 = m4.load(self);
        (l1,l2,l3,l4)
    }

    fn l1s1(&mut self) -> (u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        (m2.load(self), m1)
    }

    fn l1l2s1(&mut self) -> (u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,_) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        (l1,l2,m3)
    }

    fn l1l2l3s1(&mut self) -> (u32,u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        (l1,l2,l3,m4)
    }

    fn l1l2s1s2(&mut self) -> (u32,u32,operand::Mode,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        (l1,l2,m3,m4)
    }

    fn l1l2l3l4s1(&mut self) -> (u32,u32,u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let (m5,_) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        let l4 = m4.load(self);
        (l1,l2,l3,l4,m5)
    }

    fn l1l2l3l4l5l6s1(&mut self) -> (u32,u32,u32,u32,u32,u32,operand::Mode) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let (m3,m4) = operand::next_mode(&mut self.state);
        let (m5,m6) = operand::next_mode(&mut self.state);
        let (m7,_) = operand::next_mode(&mut self.state);
        let l1 = m1.load(self);
        let l2 = m2.load(self);
        let l3 = m3.load(self);
        let l4 = m4.load(self);
        let l5 = m5.load(self);
        let l6 = m6.load(self);
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
        (l1,l2,l3,l4,l5,l6,l7,m8)
    }

    fn s1(&mut self) -> operand::Mode {
        let (m1,_) = operand::next_mode(&mut self.state);
        m1
    }

    fn s1l1(&mut self) -> (operand::Mode,u32) {
        let (m1,m2) = operand::next_mode(&mut self.state);
        let l2 = m2.load(self);
        (m1,l2)
    }

    fn s1s2(&mut self) -> (operand::Mode,operand::Mode) {
        operand::next_mode(&mut self.state)
    }

    fn jump(&mut self, offset: u32) -> bool {
        self.tick();
        match offset {
            0 => call::ret(self, 0),
            1 => call::ret(self, 1),
            _ => {
                self.state.pc += offset as usize - 2;
                true
            }
        }
    }

    fn stash_protected_range(&mut self) {
        let (start, len) = self.protected_range;
        self.protected_tmp.clear();
        self.protected_tmp.extend_from_slice(&self.state.mem[start .. start+len]);
    }

    fn unstash_protected_range(&mut self) {
        let (start, len) = self.protected_range;
        for i in 0 .. len {
            self.state.mem[start + i] = self.protected_tmp[i];
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
