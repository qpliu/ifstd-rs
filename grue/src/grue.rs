extern crate iff;
extern crate glk;
extern crate glkterm;
extern crate glulx;

use glkterm::{init,set_arguments,Argument,GlkTerm};

mod run;

fn main() {
    set_arguments(vec![Argument::ValueFollows("".to_string(), "STORY-FILE".to_string())]);
    init(glk_main);
}

fn glk_main(glk: GlkTerm, args: Vec<String>) {
    run::grue(glk, args).unwrap();
}

