extern crate glktest;
extern crate glulx;
extern crate iff;

use glktest::TestOutput::Match;

mod common;

#[test]
fn test() {
    const INTRO: &'static str = "\nMemHeapTest\nNot a game.\nRelease 1 / Serial number 070211 / Inform v6.30 Library 6/11 S\n\nHeap Room\nA voice booooms out: Try \"status\", \"alloc LEN\", and \"free ADDR\", where LEN and ADDR are numbers.\n\n>";
    assert!(common::run_test("memheaptest.ulx", vec![
                (Match(INTRO), "status"),
                (Match("Heap inactive.\n\n>"), "alloc 10"),
                (Match("Allocating 10 bytes...\nAllocated block at 114432.\n\n>"), "alloc 20"),
                (Match("Allocating 20 bytes...\nAllocated block at 114442.\n\n>"), "status"),
                (Match("Heap exists from 114432 to 114462.\n\n>"), "free 114432"),
                (Match("Freeing block at 114432...\n\n>"), "alloc 5"),
                (Match("Allocating 5 bytes...\nAllocated block at 114432.\n\n>"), "alloc 5"),
                (Match("Allocating 5 bytes...\nAllocated block at 114437.\n\n>"), "free 114442"),
                (Match("Freeing block at 114442...\n\n>"), "status"),
                (Match("Heap exists from 114432 to 114462.\n\n>"), "free 114432"),
                (Match("Freeing block at 114432...\n\n>"), "free 114437"),
                (Match("Freeing block at 114437...\n\n>"), "status"),
                (Match("Heap inactive.\n\n>"), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
}
