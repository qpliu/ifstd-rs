extern crate glktest;
extern crate glulx;
extern crate iff;

mod common;

#[test]
fn test() {
    const OUTPUT: &'static str = "ResizeMemStreamTest: Not a game.\nRelease 1 / Serial number 121018 / Inform v6.32, compiler options S\nInterpreter version 0.1.0 / VM 3.1.2 / game file format 3.0.0\n\nA voice booooms out: Welcome to the test chamber.\n\nThis tests a memory-pointer bug in Glulxe (which has also been imported into Git, because it's in glkop.c). If a char array is held open while memory is resized, the underlying realloc() may yank the memory map out from under the array. (Examples of an open char array: an open char memory stream or an open char line input request.) This can result in writing to random memory and corrupting the (libc) heap.\n\nThis bug should be fixed in Glulxe 0.5.0 and later.\n\nOpening a unichar stream. This should not fail even on old interpreters.\nResizing to 8960...\nChars written: 4: 88 89 90 87\nOpening a char stream. This will probably fail on older interpreters.\nResizing to 9984...\nChars written: 4: 120 121 122 119\n\nTest passed.\n";
    let result = common::run_test("resizememstreamtest.ulx", vec![]);
    assert!(result.is_ok());
    assert_eq!(OUTPUT, &result.unwrap());
}
