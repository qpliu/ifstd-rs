use std::env::current_exe;
use std::fs::File;
use std::io::{Error,ErrorKind,Read,Result};

use glktest;
use glulx;
use iff;

fn testdata(name: &'static str) -> Result<File> {
    let mut path = current_exe()?;
    while path.file_name().unwrap() != "target" {
        path.pop();
    }
    path.pop();
    path.push("glulx");
    path.push("tests");
    path.push("testdata");
    path.push(name);
    File::open(path)
}

#[allow(dead_code)]
pub fn run_test<'a>(ulx_file: &'static str, test: Vec<(glktest::TestOutput<'a>,&'a str)>) -> Result<String> {
    let mut ulx = testdata(ulx_file)?;
    let glk = glktest::GlkTest::new(test);
    let (glk,result) = glulx::run(glk, &mut ulx);
    result?;
    Ok(glk.output())
}

#[allow(dead_code)]
pub fn run_blorb<'a>(blorb_file: &'static str, test: Vec<(glktest::TestOutput<'a>,&'a str)>) -> Result<String> {
    let mut blorb = testdata(blorb_file)?;
    let mut buf = vec![];
    blorb.read_to_end(&mut buf)?;
    if let iff::Chunk::Envelope { envelope_id:_, id, chunks } = iff::Chunk::new(&buf)? {
        if id == From::from(b"IFRS") {
            for chunk in chunks {
                if let iff::Chunk::Data { id, mut data } = chunk {
                    if id == From::from(b"GLUL") {
                        let glk = glktest::GlkTest::new(test);
                        let (glk,result) = glulx::run(glk, &mut data);
                        result?;
                        return Ok(glk.output());
                    }
                }
            }
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "invalid file"))
}
