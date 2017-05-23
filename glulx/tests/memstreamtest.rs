extern crate glktest;
extern crate glulx;

use glktest::TestOutput::Match;

mod common;

const INTRO: &'static str = "\nMemStreamTest\nNot a game.\nRelease 3 / Serial number 110824 / Inform v6.32 Library 6/11 S\n\nStream Room\nA voice booooms out: Try \"stream OBJECT\" to send its description to a byte array (glk_stream_open_memory). Try \"streamuni OBJECT\" to send its description to a Unicode character array (glk_stream_open_memory_uni). \"nullstream OBJECT\" and \"uninullstream OBJECT\" send to a null stream, which gets the description's length. \"pos\" and \"unipos\" test positioning in byte/Unicode streams. \"read\" and \"uniread\" test reading from byte/Unicode streams.\n\nYou can see a grape, some umlauts, a Greek pie, a Russian restaurant, some quotes and a long string here.\n\n>";

#[test]
fn test() {
    assert!(common::run_test("memstreamtest.ulx", vec![
                (Match(INTRO), "stream grape"),
                (Match("(Sent to char stream: 14 chars)\nIt\'s a grape.\n\nCharacter by character:  I t \' s   a   g r a p e . (newline)\n\n>"), "unistream grape"),
                (Match("(Sent to unicode char stream: 14 chars)\nIt\'s a grape.\n\nCharacter by character:  I t \' s   a   g r a p e . (newline)\n\n>"), "nullstream grape"),
                (Match("(Sent to char stream: 14 chars)\n\n>"), "uninullstream grape"),
                (Match("(Sent to unicode char stream: 14 chars)\n\n>"), "stream umlauts"),
                (Match("(Sent to char stream: 34 chars)\nCapital AEIOU with umlauts: ÄËÏÖÜ\n\nCharacter by character:  C a p i t a l   A E I O U   w i t h   u m l a u t s :   (196) (203) (207) (214) (220) (newline)\n\n>"), "unistream umlauts"),
                (Match("(Sent to unicode char stream: 34 chars)\nCapital AEIOU with umlauts: ÄËÏÖÜ\n\nCharacter by character:  C a p i t a l   A E I O U   w i t h   u m l a u t s :   (196) (203) (207) (214) (220) (newline)\n\n>"), "nullstream umlauts"),
                (Match("(Sent to char stream: 34 chars)\n\n>"), "uninullstream umlauts"),
                (Match("(Sent to unicode char stream: 34 chars)\n\n>"), "stream pi"),
                (Match("(Sent to char stream: 40 chars)\nIt\'s a Greek À, made with feta and ink.\n\nCharacter by character:  I t \' s   a   G r e e k   (192) ,   m a d e   w i t h   f e t a   a n d   i n k . (newline)\n\n>"), "unistream pi"),
                (Match("(Sent to unicode char stream: 40 chars)\nIt\'s a Greek π, made with feta and ink.\n\nCharacter by character:  I t \' s   a   G r e e k   (960) ,   m a d e   w i t h   f e t a   a n d   i n k . (newline)\n\n>"), "nullstream pi"),
                (Match("(Sent to char stream: 40 chars)\n\n>"), "uninullstream pi"),
                (Match("(Sent to unicode char stream: 40 chars)\n\n>"), "stream restaurant"),
                (Match("(Sent to char stream: 47 chars)\nIt\'s a Russian @5AB>@0=. The food smells good.\n\nCharacter by character:  I t \' s   a   R u s s i a n   @ 5 A B > @ 0 = .   T h e   f o o d   s m e l l s   g o o d . (newline)\n\n>"), "unistream restaurant"),
                (Match("(Sent to unicode char stream: 47 chars)\nIt\'s a Russian ресторан. The food smells good.\n\nCharacter by character:  I t \' s   a   R u s s i a n   (1088) (1077) (1089) (1090) (1086) (1088) (1072) (1085) .   T h e   f o o d   s m e l l s   g o o d . (newline)\n\n>"), "nullstream restaurant"),
                (Match("(Sent to char stream: 47 chars)\n\n>"), "uninullstream restaurant"),
                (Match("(Sent to unicode char stream: 47 chars)\n\n>"), "stream quotes"),
                (Match("(Sent to char stream: 73 chars)\nSome text with curly quotes: \u{18}single curly quotes\u{19} \u{1c}double curly quotes\u{1d}\n\nCharacter by character:  S o m e   t e x t   w i t h   c u r l y   q u o t e s :   (24) s i n g l e   c u r l y   q u o t e s (25)   (28) d o u b l e   c u r l y   q u o t e s (29) (newline)\n\n>"), "unistream quotes"),
                (Match("(Sent to unicode char stream: 73 chars)\nSome text with curly quotes: ‘single curly quotes’ “double curly quotes”\n\nCharacter by character:  S o m e   t e x t   w i t h   c u r l y   q u o t e s :   (8216) s i n g l e   c u r l y   q u o t e s (8217)   (8220) d o u b l e   c u r l y   q u o t e s (8221) (newline)\n\n>"), "nullstream quotes"),
                (Match("(Sent to char stream: 73 chars)\n\n>"), "uninullstream quotes"),
                (Match("(Sent to unicode char stream: 73 chars)\n\n>"), "stream string"),
                (Match("(Sent to char stream: 161 chars)\nIt\'s a very long piece of string. So long, in fact, that its description will certainly overflow the 128-character output array.\nCharacter by character:  I t \' s   a   v e r y   l o n g   p i e c e   o f   s t r i n g .   S o   l o n g ,   i n   f a c t ,   t h a t   i t s   d e s c r i p t i o n   w i l l   c e r t a i n l y   o v e r f l o w   t h e   1 2 8 - c h a r a c t e r   o u t p u t   a r r a y .\n\n>"), "unistream string"),
                (Match("(Sent to unicode char stream: 161 chars)\nIt\'s a very long piece of string. So long, in fact, that its description will certainly overflow the 128-character output array.\nCharacter by character:  I t \' s   a   v e r y   l o n g   p i e c e   o f   s t r i n g .   S o   l o n g ,   i n   f a c t ,   t h a t   i t s   d e s c r i p t i o n   w i l l   c e r t a i n l y   o v e r f l o w   t h e   1 2 8 - c h a r a c t e r   o u t p u t   a r r a y .\n\n>"), "nullstream string"),
                (Match("(Sent to char stream: 161 chars)\n\n>"), "uninullstream string"),
                (Match("(Sent to unicode char stream: 161 chars)\n\n>"), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
}

#[test]
fn pos() {
    assert!(common::run_test("memstreamtest.ulx", vec![
                (Match(INTRO), "pos"),
                (Match(""), "unipos"),
                (Match(""), "read"),
                (Match(""), "uniread"),
                (Match(""), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
}

#[test]
fn read() {
    assert!(common::run_test("memstreamtest.ulx", vec![
                (Match(INTRO), "read"),
                (Match(""), "uniread"),
                (Match(""), "quit"),
                (Match("Are you sure you want to quit? "), "y"),
                ]).is_ok());
}
