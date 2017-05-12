// An awful mechanism for passing arrays in and out of Glk's retained
// arrays using static variables.  Lots of unsafe crap.

use std;

use super::Glk;

static mut arrays8_initialized: bool = false;
static mut arrays8: *const u8 = &0;
static mut arrays32_initialized: bool = false;
static mut arrays32: *const u8 = &0;

unsafe fn get_arrays<T>(arrays: &mut *const u8, initialized: &mut bool) -> &'static mut Vec<Option<Option<Box<[T]>>>> {
    if !*initialized {
        *initialized = true;
        let vec: *mut Vec<Option<Option<Box<[T]>>>> = &mut Vec::new();
        *arrays = std::mem::transmute(vec);
    }
    let vec: *mut Vec<Option<Option<Box<[T]>>>> = std::mem::transmute(*arrays);
    vec.as_mut().unwrap()
}

fn unregister_callback<T>(arrays: &mut Vec<Option<Option<Box<[T]>>>>, array: Box<[T]>, index: u32) {
    let i = index as usize;
    if i >= arrays.len() {
        return;
    }
    if arrays[i].is_none() {
        return;
    }
    arrays[i] = Some(Some(array));
}

fn take<T>(arrays: &mut Vec<Option<Option<Box<[T]>>>>, index: u32) -> Option<Box<[T]>> {
    let i = index as usize;
    if i >= arrays.len() {
        return None;
    }
    arrays[i].take().unwrap_or(None)
}

fn register<T>(arrays: &mut Vec<Option<Option<Box<[T]>>>>) -> u32 {
    for i in 0 .. arrays.len() {
        if arrays[i].is_none() {
            arrays[i] = Some(None);
            return i as u32;
        }
    }
    arrays.push(Some(None));
    arrays.len() as u32 - 1
}

fn unregister_callback8(array: Box<[u8]>, index: u32) {
    unsafe {
        let arrays: &mut Vec<Option<Option<Box<[u8]>>>> = get_arrays(&mut arrays8, &mut arrays8_initialized);
        unregister_callback(arrays, array, index);
    }
}

pub unsafe fn register8<G: Glk>(glk: &G, array: Box<[u8]>) -> (u32,G::RetainedMem8) {
    let arrays: &mut Vec<Option<Option<Box<[u8]>>>> = get_arrays(&mut arrays8, &mut arrays8_initialized);
    let index = register(arrays);
    (index,glk.retain_mem8(array, index, unregister_callback8))
}

pub unsafe fn take8(index: u32) -> Option<Box<[u8]>> {
    take(get_arrays(&mut arrays8, &mut arrays8_initialized), index)
}

fn unregister_callback32(array: Box<[u32]>, index: u32) {
    unsafe {
        unregister_callback(get_arrays(&mut arrays32, &mut arrays32_initialized), array, index);
    }
}

pub unsafe fn register32<G: Glk>(glk: &G, array: Box<[u32]>) -> (u32,G::RetainedMem32) {
    let arrays: &mut Vec<Option<Option<Box<[u32]>>>> = get_arrays(&mut arrays32, &mut arrays32_initialized);
    let index = register(arrays);
    (index,glk.retain_mem32(array, index, unregister_callback32))
}

pub unsafe fn take32(index: u32) -> Option<Box<[u32]>> {
    take(get_arrays(&mut arrays32, &mut arrays32_initialized), index)
}
