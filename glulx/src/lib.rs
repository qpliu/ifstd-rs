extern crate glk;
extern crate iff;
extern crate rand;

use glk::Glk;

mod accel;
mod call;
mod execute;
mod gestalt;
mod glk_dispatch;
mod glk_selector;
mod iosys;
mod malloc;
mod opcode;
mod operand;
mod save;
mod search;
mod state;
mod trace;

pub fn run<'a,G: Glk<'a>, R: std::io::Read>(glk: G, r: &mut R) -> (G,Result<(),std::io::Error>) {
    match state::State::new(r) {
        Err(cause) => (glk,Err(cause)),
        Ok(state) => {
            let mut exec = execute::Execute::new(state, glk);
            let mut next = execute::NEXT_EXEC;
            while next != execute::NEXT_QUIT {
                next = exec.next(next);
            }
            (exec.glk,Ok(()))
        },
    }
}
