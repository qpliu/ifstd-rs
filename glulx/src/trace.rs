pub use self::internal::{Trace,opcode,iosys,operand,frame,push_call_stub,call_stub};

#[cfg(not(debug_assertions))]
mod internal {
    use glk::Glk;
    use super::super::execute::Execute;
    use super::super::operand::Mode;

    pub struct Trace {
    }

    impl Trace {
        pub fn new() -> Self {
            Trace{}
        }
    }

    #[inline]
    pub fn opcode<'a,G: Glk<'a>>(_exec: &mut Execute<'a,G>, _addr: usize, _opcode: u32) {
    }

    #[inline]
    pub fn iosys<'a,G: Glk<'a>>(_exec: &mut Execute<'a,G>, _op: &'static str) {
    }

    #[inline]
    pub fn operand<'a,G: Glk<'a>>(_exec: &mut Execute<'a,G>, _mode: &Mode, _val: Option<u32>) {
    }

    #[inline]
    pub fn frame<'a,G: Glk<'a>>(_exec: &mut Execute<'a,G>) {
    }

    #[inline]
    pub fn push_call_stub<'a,G: Glk<'a>>(_exec: &mut Execute<'a,G>) {
    }

    #[inline]
    pub fn call_stub<'a,G: Glk<'a>>(_exec: &mut Execute<'a,G>) {
    }
}

#[cfg(debug_assertions)]
mod internal {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::time::Instant;
    use glk::Glk;
    use super::super::execute::Execute;
    use super::super::operand::Mode;
    use super::super::opcode;

    pub struct Trace {
        out: Option<File>,
        start: Instant,
        count: u64,
        skip: u64,
    }

    impl Trace {
        pub fn new() -> Self {
            let out = if let Ok(file) = env::var("GLULX_TRACE") {
                if let Ok(f) = File::create(file) {
                    Some(f)
                } else {
                    None
                }
            } else {
                None
            };
            let skip = if let Ok(skip) = env::var("GLULX_TRACE_SKIP") {
                skip.parse().unwrap_or(0)
            } else {
                0
            };
            Trace{ out, start: Instant::now(), count: 0, skip }
        }
    }

    fn timestamp<W: Write>(out: &mut W, start: Instant) {
        let t = Instant::now().duration_since(start);
        out.write(format!("{:03}.{:06}:", t.as_secs(), t.subsec_nanos()/1000).as_bytes()).unwrap();
    }

    pub fn opcode<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, addr: usize, opcode: u32) {
        if let &mut Trace{out: Some(ref mut out), start, ref mut count, skip} = &mut exec.trace {
            *count += 1;
            if *count >= skip {
                *count = 0;
            } else {
                return;
            }
            timestamp(out, start);
            out.write(format!("{:06x}:{:03x} {:10.10}", addr, opcode, opcode_name(opcode)).as_bytes()).unwrap();
        }
    }

    pub fn iosys<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, op: &'static str) {
        if let &mut Trace{out: Some(ref mut out), start, ref mut count, skip} = &mut exec.trace {
            *count += 1;
            if *count >= skip {
                *count = 0;
            } else {
                return;
            }
            timestamp(out, start);
            out.write(format!("{:06x}: {:10.10} Fr:{:x}/{:x}\n", exec.state.pc, op, exec.state.frame_ptr, exec.state.stack.len()).as_bytes()).unwrap();

        }
    }

    pub fn operand<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>, mode: &Mode, val: Option<u32>) {
        if exec.trace.count != 0 {
            return;
        }
        if let &mut Some(ref mut out) = &mut exec.trace.out {
            out.write(format!(" {:?}", mode).as_bytes()).unwrap();
            if let Some(v) = val {
                out.write(format!(":{:x}", v).as_bytes()).unwrap();
            }
        }
    }

    pub fn frame<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>) {
        if exec.trace.count != 0 {
            return;
        }
        if let &mut Some(ref mut out) = &mut exec.trace.out {
            out.write(format!(" === Fr:{:x}:{:x}:{:x} [", exec.state.frame_ptr, exec.frame_locals, exec.frame_end).as_bytes()).unwrap();
            for i in exec.frame_locals .. exec.frame_end {
                out.write(format!(" {:x}", exec.state.stack[i]).as_bytes()).unwrap();
            }
            out.write(format!("] St:[").as_bytes()).unwrap();
            for i in exec.frame_end .. exec.state.stack.len() {
                out.write(format!(" {:x}", exec.state.stack[i]).as_bytes()).unwrap();
            }
            out.write(format!("]\n").as_bytes()).unwrap();
        }
    }

    pub fn push_call_stub<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>) {
        if let &mut Trace{out: Some(ref mut out), start, ref mut count, skip} = &mut exec.trace {
            *count += 1;
            if *count >= skip {
                *count = 0;
            } else {
                return;
            }
            timestamp(out, start);
            if exec.state.stack.len() < 4 {
                return;
            }
            timestamp(out, start);
            let i = exec.state.stack.len();
            out.write(format!("push_call_stub: dest: type:{:x} addr:{:x} pc:{:x} fr:{:x} St:{:x}\n", exec.state.stack[i-4], exec.state.stack[i-3], exec.state.stack[i-2], exec.state.stack[i-1], exec.state.stack.len()).as_bytes()).unwrap();
        }
    }

    pub fn call_stub<'a,G: Glk<'a>>(exec: &mut Execute<'a,G>) {
        if let &mut Trace{out: Some(ref mut out), start, ref mut count, skip} = &mut exec.trace {
            *count += 1;
            if *count >= skip {
                *count = 0;
            } else {
                return;
            }
            if exec.state.stack.len() < 4 {
                return;
            }
            timestamp(out, start);
            let i = exec.state.stack.len();
            out.write(format!("call_stub: dest: type:{:x} addr:{:x} pc:{:x} fr:{:x} St:{:x}\n", exec.state.stack[i-4], exec.state.stack[i-3], exec.state.stack[i-2], exec.state.stack[i-1], exec.state.stack.len()).as_bytes()).unwrap();
        }
    }

    fn opcode_name(opcode: u32) -> &'static str {
        match opcode {
            opcode::NOP => "nop",
            opcode::ADD => "add",
            opcode::SUB => "sub",
            opcode::MUL => "mul",
            opcode::DIV => "div",
            opcode::MOD => "mod",
            opcode::NEG => "neg",
            opcode::BITAND => "bitand",
            opcode::BITOR => "bitor",
            opcode::BITXOR => "bitxor",
            opcode::BITNOT => "bitnot",
            opcode::SHIFTL => "shiftl",
            opcode::SSHIFTR => "sshiftr",
            opcode::USHIFTR => "ushiftr",
            opcode::JUMP => "jump",
            opcode::JZ => "jz",
            opcode::JNZ => "jnz",
            opcode::JEQ => "jeq",
            opcode::JNE => "jne",
            opcode::JLT => "jlt",
            opcode::JGE => "jge",
            opcode::JGT => "jgt",
            opcode::JLE => "jle",
            opcode::JLTU => "jltu",
            opcode::JGEU => "jgeu",
            opcode::JGTU => "jgtu",
            opcode::JLEU => "jleu",
            opcode::CALL => "call",
            opcode::RETURN => "return",
            opcode::CATCH => "catch",
            opcode::THROW => "throw",
            opcode::TAILCALL => "tailcall",
            opcode::COPY => "copy",
            opcode::COPYS => "copys",
            opcode::COPYB => "copyb",
            opcode::SEXS => "sexs",
            opcode::SEXB => "sexb",
            opcode::ALOAD => "aload",
            opcode::ALOADS => "aloads",
            opcode::ALOADB => "aloadb",
            opcode::ALOADBIT => "aloadbit",
            opcode::ASTORE => "astore",
            opcode::ASTORES => "astores",
            opcode::ASTOREB => "astoreb",
            opcode::ASTOREBIT => "astorebit",
            opcode::STKCOUNT => "stkcount",
            opcode::STKPEEK => "stkpeek",
            opcode::STKSWAP => "stkswap",
            opcode::STKROLL => "stkroll",
            opcode::STKCOPY => "stkcopy",
            opcode::STREAMCHAR => "streamchar",
            opcode::STREAMNUM => "streamnum",
            opcode::STREAMSTR => "streamstr",
            opcode::STREAMUNICHAR => "streamunichar",
            opcode::GESTALT => "gestalt",
            opcode::DEBUGTRAP => "debugtrap",
            opcode::GETMEMSIZE => "getmemsize",
            opcode::SETMEMSIZE => "setmemsize",
            opcode::JUMPABS => "jumpabs",
            opcode::RANDOM => "random",
            opcode::SETRANDOM => "setrandom",
            opcode::QUIT => "quit",
            opcode::VERIFY => "verify",
            opcode::RESTART => "restart",
            opcode::SAVE => "save",
            opcode::RESTORE => "restore",
            opcode::SAVEUNDO => "saveundo",
            opcode::RESTOREUNDO => "restoreundo",
            opcode::PROTECT => "protect",
            opcode::GLK => "glk",
            opcode::GETSTRINGTBL => "getstringtbl",
            opcode::SETSTRINGTBL => "setstringtbl",
            opcode::GETIOSYS => "getiosys",
            opcode::SETIOSYS => "setiosys",
            opcode::LINEARSEARCH => "linearsearch",
            opcode::BINARYSEARCH => "binarysearch",
            opcode::LINKEDSEARCH => "linkedsearch",
            opcode::CALLF => "callf",
            opcode::CALLFI => "callfi",
            opcode::CALLFII => "callfii",
            opcode::CALLFIII => "callfiii",
            opcode::MZERO => "mzero",
            opcode::MCOPY => "mcopy",
            opcode::MALLOC => "malloc",
            opcode::MFREE => "mfree",
            opcode::ACCELFUNC => "accelfunc",
            opcode::ACCELPARAM => "accelparam",
            opcode::NUMTOF => "numtof",
            opcode::FTONUMZ => "ftonumz",
            opcode::FTONUMN => "ftonumn",
            opcode::CEIL => "ceil",
            opcode::FLOOR => "floor",
            opcode::FADD => "fadd",
            opcode::FSUB => "fsub",
            opcode::FMUL => "fmul",
            opcode::FDIV => "fdiv",
            opcode::FMOD => "fmod",
            opcode::SQRT => "sqrt",
            opcode::EXP => "exp",
            opcode::LOG => "log",
            opcode::POW => "pow",
            opcode::SIN => "sin",
            opcode::COS => "cos",
            opcode::TAN => "tan",
            opcode::ASIN => "asin",
            opcode::ACOS => "acos",
            opcode::ATAN => "atan",
            opcode::ATAN2 => "atan2",
            opcode::JFEQ => "jfeq",
            opcode::JFNE => "jfne",
            opcode::JFLT => "jflt",
            opcode::JFLE => "jfle",
            opcode::JFGT => "jfgt",
            opcode::JFGE => "jfge",
            opcode::JISNAN => "jisnan",
            opcode::JISINF => "jisinf",
            _ => "<unknown>",
        }
    }
}
