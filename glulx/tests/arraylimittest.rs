extern crate glktest;
extern crate glulx;
extern crate iff;

mod common;

#[test]
#[should_panic(expected="Memory access out of range")]
fn test() {
    const OUTPUT: &'static str = "ArrayLimitTest\n\nThis test commits a sin: it calls print_to_array(buf) with one argument, to see how the interpreter reacts. This form is deprecated in Glulx; you should be using two arguments.\n\nA correct interpreter should display either a warning or a fatal error. (If 2009 is recent, a warning is sufficient.) If the error is not fatal, it will also display the intended output of the test, which is two identical lines:\n\n7 chars: Hello.^J (1013*^@)\n7 chars: Hello.^J (1013*^@)\n\nAn old interpreter might print that output; or it might print only one such line, which is wrong. (This uncertainty is why this is a sin, and deprecated.) Conceivably it might crash, which is even worse.\n\n(If the interpreter displays the error \"[** Programming error: tried to call Glulx print_to_array with only one argument **]\", then the compiler has caught the sin and prevented it from happening at all! Which is virtuous of the compiler, but it makes this test pointless. You must compile this source code with an older Inform compiler, or download the arraylimittest.ulx file from the Glk web site.)\n\nOkay, enough explanation. Test time. Ready? Go!\n\n---------------------------------------\n\n7 chars: Hello.^J (1013*^@) \n7 chars: Hello.^J (1013*^@) \n";
    let result = common::run_test("arraylimittest.ulx", vec![]);
    assert!(result.is_ok());
    assert_eq!(OUTPUT, &result.unwrap());
}
