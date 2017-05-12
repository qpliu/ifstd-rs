use std;

static mut MAIN_FUNC: *const u8 = &0;

pub fn init(main_func: fn(super::CheapGlk), args: Vec<String>) {
    unsafe {
        MAIN_FUNC = std::mem::transmute(main_func);
    }

    let mut cargs = Vec::new();
    for arg in args {
        if let Ok(carg) = std::ffi::CString::new(arg) {
            cargs.push(carg);
        }
    }
    let argc = cargs.len() as std::os::raw::c_int;
    let mut argv = Vec::new();
    for carg in &cargs {
        argv.push(carg.as_ptr());
    }
    argv.push(std::ptr::null());
    unsafe {
        cheapglk_main(argc, argv.as_ptr());
    }
}

#[no_mangle]
pub extern fn glk_main() {
    let main_func: fn(super::CheapGlk) = unsafe { std::mem::transmute(MAIN_FUNC) };
    main_func(super::CheapGlk{});
}

#[link(name="cheapglk")]
extern {
    fn cheapglk_main(argc: std::os::raw::c_int, argv: *const *const std::os::raw::c_char);
}
