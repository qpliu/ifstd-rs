extern crate glktest;
extern crate glulx;

use glktest::TestOutput::Match;

mod common;

#[test]
fn test() {
    const INTRO: &'static str = "\nMemCopyTest\nNot a game.\nRelease 1 / Serial number 070211 / Inform v6.30 Library 6/11 S\n\nHeap Room\nA voice booooms out: Try \"say WORD\", \"zero POS LEN\", or \"copy SRCPOS DESTPOS LEN\", where POS and LEN are numbers from 0 to 20.\nArray: (0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)\n\n>";
    assert!(common::run_test("memcopytest.ulx", vec![
                (Match(INTRO), "say WORD"),
                (Match("Array: WORD(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)\n\n>"), "zero 1 2"),
                (Match("Zeroing 2 bytes at 1.\nArray: W(0)(0)D(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)\n\n>"), "copy 0 16 4"),
                (Match("Copying 4 bytes from 0 to 16.\nArray: W(0)(0)D(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)(0)W(0)(0)D\n\n>"), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
}
