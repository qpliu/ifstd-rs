extern crate glktest;
extern crate glulx;

use glktest::TestOutput::Match;

mod common;

#[test]
fn test() {
    const INTRO: &'static str = "\nMemStreamTest\nNot a game.\nRelease 3 / Serial number 110824 / Inform v6.32 Library 6/11 S\n\nStream Room\nA voice booooms out: Try \"stream OBJECT\" to send its description to a byte array (glk_stream_open_memory). Try \"streamuni OBJECT\" to send its description to a Unicode character array (glk_stream_open_memory_uni). \"nullstream OBJECT\" and \"uninullstream OBJECT\" send to a null stream, which gets the description's length. \"pos\" and \"unipos\" test positioning in byte/Unicode streams. \"read\" and \"uniread\" test reading from byte/Unicode streams.\n\nYou can see a grape, some umlauts, a Greek pie, a Russian restaurant, some quotes and a long string here.\n\n>";
    assert!(common::run_test("memstreamtest.ulx", vec![
                (Match(INTRO), "stream grape"),
                (Match("(Sent to char stream: 14 chars)\nIt\'s a grape.\n\nCharacter by character:  I t \' s   a   g r a p e . (newline)\n\n>"), "unistream grape"),
                (Match("(Sent to unicode char stream: 14 chars)\nIt\'s a grape.\n\nCharacter by character:  I t \' s   a   g r a p e . (newline)\n\n>"), "nullstream grapes"),
                (Match("((Sent to char stream: 14 chars)\n\n>"), "uninullstream grape"),
                (Match("((Sent to unicode char stream: 14 chars)\n\n>"), "stream umlauts"),
                (Match(""), "unistream umlauts"),
                (Match(""), "nullstream umlauts"),
                (Match(""), "uninullstream umlauts"),
                (Match(""), "stream pi"),
                (Match(""), "unistream pi"),
                (Match(""), "nullstream pi"),
                (Match(""), "uninullstream pi"),
                (Match(""), "stream restaurant"),
                (Match(""), "unistream restaurant"),
                (Match(""), "nullstream restaurant"),
                (Match(""), "uninullstream restaurant"),
                (Match(""), "stream quotes"),
                (Match(""), "unistream quotes"),
                (Match(""), "nullstream quotes"),
                (Match(""), "uninullstream quotes"),
                (Match(""), "stream string"),
                (Match(""), "unistream string"),
                (Match(""), "nullstream string"),
                (Match(""), "uninullstream string"),
                (Match(""), "pos"),
                (Match(""), "unipos"),
                (Match(""), "read"),
                (Match(""), "uniread"),
                (Match(""), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
}
