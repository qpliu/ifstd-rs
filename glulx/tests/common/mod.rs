use std::env::current_exe;
use std::fs::File;
use std::io::Result;

use glktest;
use glulx;

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

pub fn run_test<'a>(ulx_file: &'static str, test: Vec<(glktest::TestOutput<'a>,&'a str)>) -> Result<String> {
    let mut ulx = testdata(ulx_file)?;
    let glk = glktest::GlkTest::new(test);
    let (glk,result) = glulx::run(glk, &mut ulx);
    result?;
    Ok(glk.output())
}
