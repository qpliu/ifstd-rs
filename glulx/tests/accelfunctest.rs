extern crate glktest;
extern crate glulx;

use glktest::TestOutput::Match;

mod common;

const INTRO: &'static str = "\nAccelFuncTest\nNot a game.\nRelease 3 / Serial number 140313 / Inform v6.33 Library 6/11 S\n\nAcceleration Room\nA voice booooms out: Try \"slow test\" or \"fast test\", to run a sequence with the standard or accelerated function. The results should be identical. Type \"which\" to list the functions that your interpreter can accelerate.\n\nNUM_ATTR_BYTES is 7 (the default). The old and new accel functions should behave identically.\n\nYou can see a core test, a ZRegion test, a CPTab (old) test, a RAPr (old) test, a RLPr (old) test, an OCCl (old) test, a RVPr (old) test, an OPPr (old) test, a CPTab (new) test, a RAPr (new) test, a RLPr (new) test, an OCCl (new) test, a RVPr (new) test and an OPPr (new) test here.\n\n>";

fn test(test: &str, output: &str) {
    assert!(common::run_test("accelfunctest.ulx", vec![
                (Match(INTRO), test),
                (Match(output), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
}

#[test]
fn core() {
    const OUTPUT: &'static str = "test of core opcode funtionality, using Z__Region (function 1)\n\nToo few arguments: 0\nToo many arguments: 1\nSo many arguments that they go on the stack: 2\nTo local: 1\nTo global: 2\nTo stack: 1\nIn tailcall: 2\n\n>";
    test("slow core", &format!("Unaccelerated {}", OUTPUT));
    test("fast core", &format!("ACCELERATED {}", OUTPUT));
}

#[test]
fn zregion() {
    const OUTPUT: &'static str = "test of Z__Region (function 1)\n\n(Note: objects return 1, functions return 2, strings return 3)\nKitchen: 1; test_zregion(): 2; \"foo\": 3\nVarious zeroes: 0 0 0 0 0 0\nZ__Region itself: 2\n\n>";
    test("slow zregion", &format!("Unaccelerated {}", OUTPUT));
    test("fast zregion", &format!("ACCELERATED {}", OUTPUT));
}

fn accel(func: &str, func_name: &str, old_num: u8, new_num: u8, output: &str) {
    test(&format!("slow {} old", func), &format!("Unaccelerated test of {} (old) (function {})\n\n{}", func_name, old_num, output));
    test(&format!("fast {} old", func), &format!("ACCELERATED test of {} (old) (function {})\n\n{}", func_name, old_num, output));
    test(&format!("slow {} new", func), &format!("Unaccelerated test of {} (new) (function {})\n\n{}", func_name, new_num, output));
    test(&format!("fast {} new", func), &format!("ACCELERATED test of {} (new) (function {})\n\n{}", func_name, new_num, output));
}

#[test]
fn cptab() {
    accel("cptab", "CP__Tab", 2, 8, "Kitchen.description:\nProperty 35, flags 0, 1 words:\n  0: 80529 <func>\n\nKitchen.testfunc:\nNo such property.\n\nTestCPTab.testfunc:\nProperty 277, flags 0, 1 words:\n  0: 81038 <func>\n\nTestCPTab.name:\nProperty 1, flags 0, 5 words:\n  0: 123435 \'cptab\'\n  1: 123419 \'cp\'\n  2: 127195 \'tab\'\n  3: 125179 \'new\'\n  4: 127291 \'test\'\n\nKitchen.321:\nNo such property.\n\nbareobject.name:\nNo such property.\n\nKitchen.(0):\nNo such property.\n\nThree errors (might be fatal):\nThe exact phrasing of errors need not be identical between fast and slow.\n\n[** Programming error: tried to find the \".\" of nothing **]\n\n[** Programming error: tried to find the \".\" of <illegal object number 1> **]\n\n[** Programming error: tried to find the \".\" of <routine 81038> **]\n\n>");
}

#[test]
fn rapr() {
    accel("rapr", "RA__Pr", 3, 9, "bareobject.name: No such property.\nCoreRegion.name: 123387 \'core\'\nCoreRegion.indexnum: 1\nbareobject.TestObj::indexnum: No such property.\nCoreRegion.TestObj::indexnum: 0\nbotspecial.TestObj::indexnum: No such property.\ntopobj.gloop: 11\nmidobj.gloop: 22\nmidobj.TopClass::gloop: 11\nmidobj.BotClass::gloop: No such property.\nbotspecial.glark: 127003 \'special\'\nbotspecial.BotClass::glark: 124443 \'goodbye\'\nbotspecial.MidClass::glark: 124507 \'hello\'\nmidobj.password: 125691 \'pass\'\nbotspecial.password (foreign): No such property.\nmidobj.password (foreign): No such property.\nbotspecial.password: 127179 \'swordfish\'\nTopClass.gloop: No such property.\nTopClass.create: No such property.\n\n>");
}

#[test]
fn rlpr() {
    accel("rlpr", "RL__Pr", 4, 10, "bareobject.name: 0 bytes\nCoreRegion.name: 12 bytes\nCoreRegion.indexnum: 4 bytes\nbareobject.TestObj::indexnum: 0 bytes\nCoreRegion.TestObj::indexnum: 4 bytes\nbotspecial.TestObj::indexnum: 0 bytes\ntopobj.gloop: 4 bytes\nmidobj.gloop: 8 bytes\nmidobj.TopClass::gloop: 4 bytes\nmidobj.BotClass::gloop: 0 bytes\nbotspecial.glark: 12 bytes\nbotspecial.BotClass::glark: 4 bytes\nbotspecial.MidClass::glark: 8 bytes\nmidobj.password: 4 bytes\nbotspecial.password (foreign): 0 bytes\nmidobj.password (foreign): 0 bytes\nbotspecial.password: 4 bytes\nTopClass.gloop: 0 bytes\nTopClass.create: 0 bytes\n\n>");
}

#[test]
fn occl() {
    accel("occl", "OC__Cl", 5, 11, "\"str\" ofclass String: yes\n\"str\" ofclass Routine: no\n\"str\" ofclass Object: no\n\"str\" ofclass Class: no\n\"str\" ofclass TopClass: no\nprintbool() ofclass String: no\nprintbool() ofclass Routine: yes\nprintbool() ofclass Object: no\nprintbool() ofclass Class: no\nprintbool() ofclass TopClass: no\n\'word\' ofclass String: no\n\'word\' ofclass Routine: no\n\'word\' ofclass Object: no\n\'word\' ofclass Class: no\n\'word\' ofclass TopClass: no\nString ofclass Class: yes\nRoutine ofclass Class: yes\nObject ofclass Class: yes\nClass ofclass Class: yes\nTopClass ofclass Class: yes\nbareobject ofclass Class: no\nString ofclass Object: no\nRoutine ofclass Object: no\nObject ofclass Object: no\nClass ofclass Object: no\nTopClass ofclass Object: no\nbareobject ofclass Object: yes\nTopClass ofclass String: no\nbareobject ofclass String: no\nTopClass ofclass Routine: no\nbareobject ofclass Routine: no\nbareobject ofclass TopClass: no\nbareobject ofclass BotClass: no\ntopobj ofclass TopClass: yes\ntopobj ofclass MidClass: no\ntopobj ofclass BotClass: no\nmidobj ofclass TopClass: yes\nmidobj ofclass MidClass: yes\nmidobj ofclass BotClass: no\nbotobj ofclass TopClass: yes\nbotobj ofclass MidClass: yes\nbotobj ofclass BotClass: yes\n\nThree errors (might be fatal):\nThe exact phrasing of errors need not be identical between fast and slow.\n\n[** Programming error: (topobj) (object number 113513) is not of class <illegal object number 1> to apply \'ofclass\' for **]\nbareobject ofclass topobj: no\n\n[** Programming error: (object number 103379) is not of class <illegal object number 1> to apply \'ofclass\' for **]\ntopobj ofclass \"str\": no\n\n[** Programming error: (object number 83088) is not of class <illegal object number 1> to apply \'ofclass\' for **]\nTopClass ofclass printbool(): no\n\n>");
}

#[test]
fn rvpr() {
    accel("rvpr", "RV__Pr", 6, 12, "bareobject.name: 0\nbotobj.gloop: 22\nmidobj.gloop: 22\ntopobj.gloop: 11\nbotobj.comprop: 123\nmidobj.comprop: 123\ntopobj.comprop: 99\nTopClass.comprop: 99\n\nTwo errors (might be fatal):\nThe exact phrasing of errors need not be identical between fast and slow.\n\n[** Programming error: (topobj) (object number 113513)  has no property glark to read **]\ntopobj.glark: 0\n\n[** Programming error: class TopClass (object number 113417)  has no property gloop to read **]\nTopClass.gloop: 0\n\n>");
}

#[test]
fn oppr() {
    accel("oppr", "OP__Pr", 7, 13, "\"str\" provides name: no\n\"str\" provides gloop: no\n\"str\" provides print: yes\n\"str\" provides print_to_array: yes\n\"str\" provides create: no\n\"str\" provides call: no\nprintbool() provides name: no\nprintbool() provides gloop: no\nprintbool() provides print: no\nprintbool() provides print_to_array: no\nprintbool() provides create: no\nprintbool() provides call: yes\n\'word\' provides name: no\n\'word\' provides gloop: no\n\'word\' provides print: no\n\'word\' provides print_to_array: no\n\'word\' provides create: no\n\'word\' provides call: no\nTopClass provides name: no\nTopClass provides gloop: no\nTopClass provides glark: no\nTopClass provides print: yes\nTopClass provides print_to_array: yes\nTopClass provides create: yes\nTopClass provides call: yes\ntopobj provides gloop: yes\nmidobj provides gloop: yes\nmidobj provides TopClass::gloop: yes\nmidobj provides BotClass::gloop: no\nbotspecial provides glark: yes\nbotspecial provides BotClass::glark: yes\nbotspecial provides MidClass::glark: yes\nbotspecial provides TopClass::glark: no\n\n>");
}
