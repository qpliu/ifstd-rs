extern crate glktest;
extern crate glulx;
extern crate iff;

use std::time::Instant;

use glktest::TestOutput::Match;

mod common;

#[test]
#[ignore]
fn counterfeit() {
    let start = Instant::now();
    assert!(common::run_blorb("CounterfeitMonkey-6.gblorb", vec![
                (Match("\n\n\nCan you hear me? >> "), "yes"),
                (Match("\nGood, you\'re conscious. We\'re conscious. I\'ve heard urban legends about synthesis going wrong, one half person getting lost.\n\nDo you remember our name?\n\n>"), "yes"),
                (Match("Right, we're Alexandra now. Before the synthesis, I was Alex. You were...\n\n>"), "andra"),
                (Match("...yes! Okay. We\'re both here, neither of us lost our minds in the synthesis process. As far as I can tell, the operation was a success. We\'re meant to be one person now, unrecognizable to anyone who knew us before.\n\n"), " "),
                (Match("Counterfeit Monkey\nA Removal by Emily Short\nRelease 6 / Serial number 160520 / Inform 7 build 6G60 (I6/v6.32 lib 6/12N) \n\n\nLet\'s try to get a look around. I haven\'t been able to run our body without your help, but maybe now you\'re awake, it\'ll work better.\n\nTo get a look around, type LOOK and press return. If you do not want help getting started, type TUTORIAL OFF.\n>"), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
    let t = Instant::now().duration_since(start);
    println!("Time: {}.{:09}", t.as_secs(), t.subsec_nanos());
}
