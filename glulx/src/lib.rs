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

pub fn run<G: Glk, R: std::io::Read>(mut glk: G, r: &mut R, resources: Option<G::StrId>) -> (G,Result<(),std::io::Error>) {
    if let Some(res) = resources {
        glk.set_resource_map(res);
    }
    match state::State::new(r) {
        Err(cause) => (glk,Err(cause)),
        Ok(state) => {
            let mut exec = execute::Execute::new(state, glk);
            let mut next = execute::Next::Exec;
            while next != execute::Next::Quit {
                next = exec.next(next);
            }
            (exec.glk,Ok(()))
        },
    }
}
