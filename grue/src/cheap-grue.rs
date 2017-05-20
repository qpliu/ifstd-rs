extern crate iff;
extern crate glk;
extern crate cheapglk;
extern crate glulx;

use cheapglk::{init,set_arguments,Argument,CheapGlk};

mod run;

fn main() {
    set_arguments(vec![Argument::ValueFollows("".to_string(), "STORY-FILE".to_string())]);
    init(glk_main);
}

fn glk_main(glk: CheapGlk, args: Vec<String>) {
    run::grue(glk, args).unwrap();
}
