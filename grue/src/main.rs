extern crate glk;
extern crate cheapglk;
extern crate glulx;

use cheapglk::{init,set_arguments,Argument,CheapGlk};

fn main() {
    set_arguments(vec![Argument::ValueFollows("".to_string(), "STORY-FILE".to_string())]);
    init(glk_main);
}

fn glk_main(glk: CheapGlk, args: Vec<String>) {
    if args.len() < 2 {
        return;
    }
    let mut file = std::fs::File::open(&args[1]).unwrap();
    glulx::run(glk, &mut file, None);
}
