use std;
use super::{glk,glulx,iff};

pub fn grue<'a,G: glk::Glk<'a>>(glk: G, args: Vec<String>) -> std::io::Result<()> {
    if args.len() < 2 {
        return Ok(());
    }
    let mut file = std::fs::File::open(&args[1])?;
    let mut buf = vec![0,0,0,0];
    use std::io::{Read,Seek};
    file.read(&mut buf)?;
    if buf[..] == b"Glul"[..] {
        file.seek(std::io::SeekFrom::Current(-4))?;
        glulx::run(glk, &mut file);
    } else if buf[..] == b"FORM"[..] {
        file.read_to_end(&mut buf)?;
        if let iff::Chunk::Envelope { envelope_id:_, id, chunks } = iff::Chunk::new(&buf)? {
            if id == From::from(b"IFRS") {
                for chunk in chunks {
                    if let iff::Chunk::Data { id, mut data } = chunk {
                        if id == From::from(b"GLUL") {
                            glulx::run(glk, &mut data);
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
