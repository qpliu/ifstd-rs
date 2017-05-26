extern crate glktest;
extern crate glulx;
extern crate iff;

use glktest::TestOutput::{Check,Match};

mod common;

const INTRO: &'static str = "\nGlulxercise: A Glulx interpreter unit test\nRelease 9 / Serial number 161114 / Inform v6.34, compiler options S\nInterpreter version 0.1.0 / VM 3.1.2 / game file format 3.1.3\n\nA voice booooms out: Welcome to the test chamber.\n\nType \"help\" to repeat this message, \"quit\" to exit, \"all\" to run all tests, or one of the following test options: \"operand\", \"arith\", \"bigmul\", \"comvar\", \"comarith\", \"bitwise\", \"shift\", \"trunc\", \"extend\", \"aload\", \"astore\", \"arraybit\", \"call\", \"callstack\", \"jump\", \"jumpform\", \"compare\", \"stack\", \"gestalt\", \"throw\", \"streamnum\", \"strings\", \"ramstring\", \"iosys\", \"iosys2\", \"filter\", \"nullio\", \"glk\", \"gidispa\", \"random\", \"nonrandom\", \"search\", \"mzero\", \"mcopy\", \"undo\", \"multiundo\", \"extundo\", \"restore\", \"verify\", \"protect\", \"memsize\", \"undomemsize\", \"undorestart\", \"heap\", \"undoheap\", \"acceleration\", \"floatconv\", \"floatarith\", \"floatmod\", \"floatround\", \"floatexp\", \"floattrig\", \"floatatan2\", \"fjumpform\", \"fjump\", \"fcompare\", \"fprint\", \"safari5\".\n\n>";

fn test(test: &str, output: &str) {
    let result = common::run_test("glulxercise.ulx", vec![
            (Match(INTRO), test),
            (Match(output), "quit"),
            ]);
    assert!(result.is_ok());
    assert_eq!("\nExiting via return. (Try \"opquit\" for @quit, \"glkquit\" for glk_exit().)\n\nGoodbye.\n", result.unwrap());
}

#[test]
fn operand() {
    test("operand", "Basic operand access:\n\nstkcount=0\nConstants: zero=0, -1=-1, 16=16, -$81=$FFFFFF7F, $100=$100, -$8000=$FFFF8000, $10000=$10000, $7FFFFFFF=$7FFFFFFF, $80000000=$80000000, $CDEF1234=$CDEF1234\nConstants: zero=0, -1=-1, 16=16, -$81=$FFFFFF7F, $100=$100, -$8000=$FFFF8000, $10000=$10000, $7FFFFFFF=$7FFFFFFF, $80000000=$80000000, $CDEF1234=$CDEF1234\nGlobal to local 123=123=123, local to global 321=321=321\nStack: 456=456, Stack: 933=933\nGlobal to stack: 123=123\nStack to stack: 789=789, Stack to stack: 1234=1234\n\nPassed.\n\n>");
}

#[test]
fn arith() {
    test("arith", "Integer arithmetic:\n\n2+2=4, -2+-3=-5, 3+-4=-1, -4+5=1, $7FFFFFFF+$7FFFFFFE=-3, $80000000+$80000000=0\nGlobals 6+8=14\n2-2=0, -2-3=-5, 3-4=-1, -4-(-5)=1, $7FFFFFFF-$7FFFFFFE=1, $80000000-$80000001=-1, $7FFFFFFF-$80000001=-2\nGlobals 6-8=-2\n2*2=4, -2*-3=6, 3*-4=-12, -4*5=-20, $10000*$10000 (trunc)=0, 311537*335117 (trunc)=$4ECE193D\nGlobals -6*-8=48\n12/3=4, 11/2=5, -11/2=-5, 11/-2=-5, -11/-2=5, $7fffffff/2=$3FFFFFFF, $7fffffff/-2=$C0000001, -$7fffffff/2=$C0000001, -$7fffffff/-2=$3FFFFFFF, $80000000/2=$C0000000, $80000000/(-2)=$40000000, $80000000/1=$80000000\nGlobals -48/-8=6, 48/7=6, 48/-7=-6, -48/7=-6, -48/-7=6\n12%3=0, 13%5=3, -13%5=-3, 13%-5=3, -13%-5=-3, $7fffffff%7=1, -$7fffffff%7=-1, $7fffffff%-7=1, -$7fffffff%-7=-1, $80000000%7=-2, $80000000%-7=-2, $80000000%2=0, $80000000%-2=0, $80000000%1=0\nGlobals 49%8=1, 49%-8=1, -49%8=-1, -49%-8=-1\n-(0)=0, -(5)=-5, -(-5)=5, -($7FFFFFFF)=-2147483647, -($80000001)=2147483647, -($80000000)=$80000000\nglobal -($80000001)=2147483647\n\nPassed.\n\n>");
}

#[test]
fn bigmul() {
    test("bigmul", "Large integer multiplication:\n\n51537*35117=$6BDFBC3D\n-51539*35117=$941F3169\n51537*-35119=$941EB121\n-51539*-35119=$6BE2613D\n51537*35117 (loc)=$6BDFBC3D\n51537*35117 (glob)=$6BDFBC3D\n$5432FEDB*-1 (loc)=$ABCD0125\n$1C10FF9E*-3 (glob,loc)=$ABCD0126\n\n$7654321*1=$7654321\n$7654321*2=$ECA8642\n$7654321*4=$1D950C84\n$7654321*5=$24FA4FA5\n$7654321*8=$3B2A1908\n$7654321*16=$76543210\n$7654321*32=$ECA86420\n$7654321*64=$D950C840\n$7654321*128=$B2A19080\n$7654321*256=$65432100\n$7654321*1024=$950C8400\n$7654321*32768=$A1908000\n$7654321*65536=$43210000\n\n1*$7654321=$7654321\n2*$7654321=$ECA8642\n4*$7654321=$1D950C84\n5*$7654321=$24FA4FA5\n8*$7654321=$3B2A1908\n16*$7654321=$76543210\n32*$7654321=$ECA86420\n64*$7654321=$D950C840\n128*$7654321=$B2A19080\n256*$7654321=$65432100\n1024*$7654321=$950C8400\n32768*$7654321=$A1908000\n65536*$7654321=$43210000\n\n$7FFFFFFF*$7FFFFFFF (trunc)=$1\n$7FFFFFFF*$7FFFFFFE (glob,trunc)=$80000002\n-$7FFFFFFE*$7FFFFFFE (loc,trunc)=$FFFFFFFC\n$10000003*$10000007 (trunc)=$A0000015\n$10000001*$10000003 (glob,trunc)=$40000003\n-$10000005*$10000007 (loc,trunc)=$3FFFFFDD\n\nPassed.\n\n>");
}

#[test]
fn comvar() {
    test("comvar", "Compound variable juggling:\n\n6=6, 5=5\n12=12, 16=16, 36=36, 32=32\n7=7, 6=6, 6=6\n7=7, 7=7\n8=8, 8=8\n\nPassed.\n\n>");
}

#[test]
fn comarith() {
    test("comarith", "Compound arithmetic expressions:\n\n(7+2)*-4=-36, (7+2)*-4=-36\n($10000*$10000)/16+1=1, ($10000*$10000)/16+1=1\n($7FFFFFFF+2)/16=-134217727, ($7FFFFFFF+2)/16=-134217727\n(-$7FFFFFFF-2)/16=134217727, (-$7FFFFFFF-2)/16=134217727\n\nPassed.\n\n>");
}

#[test]
fn bitwise() {
    test("bitwise", "Bitwise arithmetic:\n\n0&0=$0, $FFFFFFFF&0=$0, $FFFFFFFF&$FFFFFFFF=$FFFFFFFF, $0137FFFF&$FFFF7310=$1377310, $35&56=$14\n0|0=$0, $FFFFFFFF|0=$FFFFFFFF, $FFFFFFFF|$FFFFFFFF=$FFFFFFFF, $01370000|$00007310=$1377310, $35|56=$77\n0^0=$0, $FFFFFFFF^0=$FFFFFFFF, $FFFFFFFF^$FFFFFFFF=$0, $0137FFFF^$00007310=$1378CEF, $35^56=$63\n!0=$FFFFFFFF, !1=$FFFFFFFE, !$F=$FFFFFFF0, !$80000000=$7FFFFFFF\n\nPassed.\n\n>");
}

#[test]
fn shift() {
    test("shift", "Bit shifts:\n\n$1001<<0=$1001, $1001<<1=$2002, $1001<<4=$10010, $1001<<10=$400400, $1001<<16=$10010000, $1001<<24=$1000000, $1001<<31=$80000000, $1001<<32=$0, $1001<<-1=$0\n-2<<0=-2, -2<<1=-4, -2<<7=-256, -2<<31=0\n1<<0=$1, 1<<1=$2, 1<<2=$4, 1<<3=$8, 1<<4=$10, 1<<5=$20, 1<<6=$40, 1<<7=$80, 1<<8=$100, 1<<9=$200, 1<<10=$400, 1<<11=$800, 1<<12=$1000, 1<<13=$2000, 1<<14=$4000, 1<<15=$8000, 1<<16=$10000, 1<<17=$20000, 1<<18=$40000, 1<<19=$80000, 1<<20=$100000, 1<<21=$200000, 1<<22=$400000, 1<<23=$800000, 1<<24=$1000000, 1<<25=$2000000, 1<<26=$4000000, 1<<27=$8000000, 1<<28=$10000000, 1<<29=$20000000, 1<<30=$40000000, 1<<31=$80000000, 1<<32=0, 1<<-1=0\n$1001u>>0=$1001, $1001u>>1=$800, $1001u>>2=$400, $1001u>>6=$40, $1001u>>12=$1, $1001u>>13=$0, $1001u>>31=$0, $1001u>>32=$0\n$7FFFFFFFu>>0=$7FFFFFFF, $7FFFFFFFu>>1=$3FFFFFFF, $7FFFFFFFu>>2=$1FFFFFFF, $7FFFFFFFu>>6=$1FFFFFF, $7FFFFFFFu>>12=$7FFFF, $7FFFFFFFu>>13=$3FFFF, $7FFFFFFFu>>30=$1, $7FFFFFFFu>>31=$0, $7FFFFFFFu>>32=$0\n-1u>>0=$FFFFFFFF, -1u>>1=$7FFFFFFF, -1u>>2=$3FFFFFFF, -1u>>6=$3FFFFFF, -1u>>12=$FFFFF, -1u>>13=$7FFFF, -1u>>30=$3, -1u>>31=$1, -1u>>32=$0, -1u>>33=$0, -1u>>-1=$0\n-1u>>1=$7FFFFFFF, -1u>>2=$3FFFFFFF, -1u>>3=$1FFFFFFF, -1u>>4=$FFFFFFF, -1u>>5=$7FFFFFF, -1u>>6=$3FFFFFF, -1u>>7=$1FFFFFF, -1u>>8=$FFFFFF, -1u>>9=$7FFFFF, -1u>>10=$3FFFFF, -1u>>11=$1FFFFF, -1u>>12=$FFFFF, -1u>>13=$7FFFF, -1u>>14=$3FFFF, -1u>>15=$1FFFF, -1u>>16=$FFFF, -1u>>17=$7FFF, -1u>>18=$3FFF, -1u>>19=$1FFF, -1u>>20=$FFF, -1u>>21=$7FF, -1u>>22=$3FF, -1u>>23=$1FF, -1u>>24=$FF, -1u>>25=$7F, -1u>>26=$3F, -1u>>27=$1F, -1u>>28=$F, -1u>>29=$7, -1u>>30=$3, -1u>>31=$1, -1u>>32=0, -1u>>-1=0\n$1001s>>0=$1001, $1001s>>1=$800, $1001s>>2=$400, $1001s>>6=$40, $1001s>>12=$1, $1001s>>13=$0, $1001s>>31=$0, $1001s>>32=$0\n$7FFFFFFFs>>0=$7FFFFFFF, $7FFFFFFFs>>1=$3FFFFFFF, $7FFFFFFFs>>2=$1FFFFFFF, $7FFFFFFFs>>6=$1FFFFFF, $7FFFFFFFs>>12=$7FFFF, $7FFFFFFFs>>13=$3FFFF, $7FFFFFFFs>>30=$1, $7FFFFFFFs>>31=$0, $7FFFFFFFs>>32=$0\n-1s>>0=-1, -1s>>1=-1, -1s>>31=-1, -1s>>32=-1, -1s>>33=-1, -1s>>-1=-1\n-1000s>>0=-1000, -1000s>>1=-500, -1000s>>2=-250, -1000s>>4=-63, -1000s>>6=-16, -1000s>>9=-2, -1000s>>31=-1, -1000s>>32=-1, -1000s>>33=-1, -1000s>>-1=-1\n-1s>>0=-1, -1s>>1=-1, -1s>>2=-1, -1s>>3=-1, -1s>>4=-1, -1s>>5=-1, -1s>>6=-1, -1s>>7=-1, -1s>>8=-1, -1s>>9=-1, -1s>>10=-1, -1s>>11=-1, -1s>>12=-1, -1s>>13=-1, -1s>>14=-1, -1s>>15=-1, -1s>>16=-1, -1s>>17=-1, -1s>>18=-1, -1s>>19=-1, -1s>>20=-1, -1s>>21=-1, -1s>>22=-1, -1s>>23=-1, -1s>>24=-1, -1s>>25=-1, -1s>>26=-1, -1s>>27=-1, -1s>>28=-1, -1s>>29=-1, -1s>>30=-1, -1s>>31=-1, -1s>>32=-1, -1s>>-1=-1\n\nPassed.\n\n>");
}

#[test]
fn trunc() {
    test("trunc", "Truncating copies:\n\n$12345678 s:> glob $01020304=$56780304, $80818283 s:> stack=$8283, glob $fedcba98 s:> glob $02030405=$FEDC0405, glob $fedcba98 s:> stack=$FEDC, stack $7654321f s:> glob $03040506=$321F0506, stack $654321fe s:> glob $04050607=$21FE0607, stack $674523f1 s:> stack=$23F1, stack $67452301 s:> stack=$2301\n$12345678 b:> glob $01020304=$78020304, $80818283 b:> stack=$83, glob $fedcba98 b:> glob $02030405=$FE030405, glob $fedcba98 b:> stack=$FE, stack $7654321f b:> glob $03040506=$1F040506, stack $654321fe b:> glob $04050607=$FE050607, stack $674523f1 b:> stack=$F1, stack $67452301 b:> stack=$1\n\nPassed.\n\n>");
}

#[test]
fn extend() {
    test("extend", "Sign-extend:\n\nsexb($00)=$0, sexb($01)=$1, sexb($7f)=$7F, sexb($80)=$FFFFFF80, sexb($fe)=$FFFFFFFE, sexb($ff)=$FFFFFFFF, sexb($100)=$0, sexb($ffffff01)=$1, sexb($7f0f0ff0)=$FFFFFFF0\nsexb($02)=$2, sexb($0f)=$F, sexb($ff)=$FFFFFFFF, sexb($100)=$0, sexb($ffffff01)=$1, sexb($7f1f1ff1)=$FFFFFFF1\nsexs($00)=$0, sexs($01)=$1, sexs($7fff)=$7FFF, sexs($8000)=$FFFF8000, sexs($fffe)=$FFFFFFFE, sexs($ffff)=$FFFFFFFF, sexs($10000)=$0, sexs($ffff0001)=$1, sexs($7f0ff00f)=$FFFFF00F\nsexs($102)=$102, sexs($0fffff)=$FFFFFFFF, sexs($fffe)=$FFFFFFFE, sexs($10000)=$0, sexs($ffff0001)=$1, sexs($7f1ffff1)=$FFFFFFF1\n\nPassed.\n\n>");
}

#[test]
fn aload() {
    test("aload", "Array loads:\n\nArray sequence: $C\narr-->0=$1020304, arr-->1=$FFFEFDFC, arr-->2=$F\narr+1-->0=$20304FF, arr+1-->1=$FEFDFC00, arr+2-->0=$304FFFE, arr+2-->1=$FDFC0000, arr+3-->0=$4FFFEFD, arr+3-->1=$FC000000, arr+4-->-1=$1020304, arr+4-->0=$FFFEFDFC, arr+4-->1=$F\narr2-->-1=$F, arr2-->-1=$F, arr2-->-1=$F, arr2-->-1=$F, arr2-->-1=$F, arr2-->-1=$F, arr2-->-1=$F, arr2-->-1=$F\narr2-->0=$7F008002, arr2-->0=$7F008002, arr2-->0=$7F008002, arr2-->0=$7F008002, arr2-->0=$7F008002, arr2-->0=$7F008002, arr2-->0=$7F008002, arr2-->0=$7F008002\narr2-->1=$100FFFF, arr2-->1=$100FFFF, arr2-->1=$100FFFF, arr2-->1=$100FFFF, arr2-->1=$100FFFF, arr2-->1=$100FFFF, arr2-->1=$100FFFF, arr2-->1=$100FFFF\narr=>0=$102, arr=>1=$304, arr=>2=$FFFE, arr=>3=$FDFC, arr=>4=$0, arr=>5=$F\narr+1=>0=$203, arr+1=>1=$4FF, arr+2=>0=$304, arr+2=>1=$FFFE, arr+3=>0=$4FF, arr+3=>1=$FEFD, arr+4=>-1=$304, arr+4=>0=$FFFE, arr+4=>1=$FDFC\narr2=>-1=$F, arr2=>-1=$F, arr2=>-1=$F, arr2=>-1=$F, arr2=>-1=$F, arr2=>-1=$F, arr2=>-1=$F, arr2=>-1=$F\narr2=>0=$7F00, arr2=>0=$7F00, arr2=>0=$7F00, arr2=>0=$7F00, arr2=>0=$7F00, arr2=>0=$7F00, arr2=>0=$7F00, arr2=>0=$7F00\narr2=>1=$8002, arr2=>1=$8002, arr2=>1=$8002, arr2=>1=$8002, arr2=>1=$8002, arr2=>1=$8002, arr2=>1=$8002, arr2=>1=$8002\narr->0=$1, arr->1=$2, arr->4=$FF, arr->5=$FE, arr->11=$F\narr+1->0=$2, arr+1->1=$3, arr+2->0=$3, arr+2->1=$4, arr+3->0=$4, arr+3->1=$FF, arr+4->-1=$4, arr+4->0=$FF, arr+4->1=$FE\narr2->-1=$F, arr2->-1=$F, arr2->-1=$F, arr2->-1=$F, arr2->-1=$F, arr2->-1=$F, arr2->-1=$F, arr2->-1=$F\narr2->0=$7F, arr2->0=$7F, arr2->0=$7F, arr2->0=$7F, arr2->0=$7F, arr2->0=$7F, arr2->0=$7F, arr2->0=$7F\narr2->2=$80, arr2->2=$80, arr2->2=$80, arr2->2=$80, arr2->2=$80, arr2->2=$80, arr2->2=$80, arr2->2=$80\n\nPassed.\n\n>");
}

#[test]
fn astore() {
    test("astore", "Array stores:\n\nArray sequence: $C\narr-->0=$2030405, arr-->1=$FEFDFCDB, arr-->2=$E0F\narr+1-->0=$12131415, $2121314/$15FDFCDB, arr+1-->1=$E0E1E2E3, $2121314/$15E0E1E2/$E3000E0F, arr+2-->0=$12345678, $2121234/$5678E1E2, arr+2-->1=$FEDCBA99, arr+3-->0=$44556677, $2121244/$556677DC, arr+3-->1=$51413121, $2121244/$55667751/$4131210F, arr+4-->-1=$21436587, arr+4-->0=$31425364, arr+4-->1=$41526374, $21436587/$31425364/$41526374\narr2-->-1=$D0000001, arr2-->-1=$D1000002, arr2-->-1=$D2000003, arr2-->-1=$D3000004, arr2-->-1=$E0000001, arr2-->-1=$E1000011, arr2-->-1=$E2000021, arr2-->-1=$E3000031\narr2-->0=$F1223310, arr2-->0=$F2223311, arr2-->0=$F3223312, arr2-->0=$F4223313, arr2-->0=$F5223315, arr2-->0=$F6223316, arr2-->0=$F7223317, arr2-->0=$F8223318\narr2-->1=$1, arr2-->1=$2, arr2-->1=$3, arr2-->1=$4, arr2-->1=$5, arr2-->1=$6, arr2-->1=$7, arr2-->1=$8\narr=>0=$4050000, arr=>1=$405FCDB, arr=>2=$E0F0000\narr+1=>0=$1415, $41415DB/$E0F0000, arr+1=>1=$E2E3, $41415E2/$E30F0000, arr+2=>0=$5678, $4145678/$E30F0000, arr+2=>1=$BA99, $4145678/$BA990000, arr+3=>0=$6677, $4145666/$77990000, arr+3=>1=$3121, $4145666/$77312100, arr+4=>-1=$6587, arr+4=>0=$5364, arr+4=>1=$6374, $4146587/$53646374\narr2=>-1=$F001, arr2=>-1=$E002, arr2=>-1=$D003, arr2=>-1=$C004, arr2=>-1=$F001, arr2=>-1=$E011, arr2=>-1=$D021, arr2=>-1=$C031\npre-guard=$9999C031, post-guard=$98979695\narr2=>0=$3310, arr2=>0=$3311, arr2=>0=$3312, arr2=>0=$3313, arr2=>0=$3315, arr2=>0=$3316, arr2=>0=$3317, arr2=>0=$3318\npost-guard=$33189695\narr2=>2=$1, arr2=>2=$2, arr2=>2=$3, arr2=>2=$4, arr2=>2=$5, arr2=>2=$6, arr2=>2=$7, arr2=>2=$8\npost-guard=$89695\narr=>0=$5000000, arr=>1=$5DB0000, arr=>2=$5DB0F00, arr=>3=$5DB0F63\narr+1=>0=$15, $5150F63/$0, arr+1=>1=$E3, $515E363/$0, arr+2=>0=$78, $5157863/$0, arr+2=>1=$99, $5157899/$0, arr+3=>0=$77, $5157877/$0, arr+3=>1=$21, $5157877/$21000000, arr+4=>-1=$87, arr+4=>0=$64, arr+4=>1=$74, $5157887/$64740000\narr2=>-1=$1, arr2=>-1=$2, arr2=>-1=$3, arr2=>-1=$4, arr2=>-1=$1, arr2=>-1=$11, arr2=>-1=$21, arr2=>-1=$31\npre-guard=$99999931, post-guard=$98979695\narr2=>0=$10, arr2=>0=$11, arr2=>0=$12, arr2=>0=$13, arr2=>0=$15, arr2=>0=$16, arr2=>0=$17, arr2=>0=$18\npost-guard=$18979695\narr2=>2=$1, arr2=>2=$2, arr2=>2=$3, arr2=>2=$4, arr2=>2=$5, arr2=>2=$6, arr2=>2=$7, arr2=>2=$8\npost-guard=$8979695\n\nPassed.\n\n>");
}

#[test]
fn arraybit() {
    test("arraybit", "Aloadbit and astorebit:\n\nbit 0=1, bit 1=0, bit 2=0, bit 3=0, bit 4=0, bit 5=0, bit 6=0, bit 7=0, bit 8=0, bit 9=1, bit 10=0, bit 11=0, bit 12=0, bit 13=0, bit 14=0, bit 15=0, \nbit -8=1, bit -7=1, bit -6=1, bit -5=1, bit -4=0, bit -3=0, bit -2=0, bit -1=0, bit 0=1, bit 1=1, bit 2=1, bit 3=1, bit 4=1, bit 5=1, bit 6=1, bit 7=0, bit 8=0, bit 9=0, bit 10=0, bit 11=0, bit 12=0, bit 13=0, bit 14=0, bit 15=0, \nbit -8=1, bit -7=1, bit -6=1, bit -5=1, bit -4=0, bit -3=0, bit -2=0, bit -1=0, bit 0=1, bit 1=1, bit 2=1, bit 3=1, bit 4=1, bit 5=1, bit 6=1, bit 7=0, bit 8=0, bit 9=0, bit 10=0, bit 11=0, bit 12=0, bit 13=0, bit 14=0, bit 15=0, \nbit 22=0, bit 23=1, bit 24=0, bit 25=1\nbit -31=0, bit -32=0, bit -33=1, bit -34=1\nbit 22=0, bit 23=1, bit 24=0, bit 25=1\nbit -31=0, bit -32=0, bit -33=1, bit -34=1\nbit 1 on=$2, bit 6 on=$42, bit 3 on=$4A, bit 0 off=$4A, bit 6 off=$A\nbit 15 off=$7F, bit 12 on=$7F, bit 8 off=$7E\nbit -1 on=$80, bit -8 on=$81\n$1000000, $3000000, $7000000, $F000000, $1F000000, $3F000000, $7F000000, $FF000000, $FF010000, $FF030000, $FF070000, $FF0F0000, $FF1F0000, $FF3F0000, $FF7F0000, $FFFF0000, $FFFF0100, $FFFF0300, $FFFF0700, $FFFF0F00, $FFFF1F00, $FFFF3F00, $FFFF7F00, $FFFFFF00, $FFFFFF01, $FFFFFF03, $FFFFFF07, $FFFFFF0F, $FFFFFF1F, $FFFFFF3F, $FFFFFF7F, $FFFFFFFF, \n$FEFFFFFF, $FCFFFFFF, $F8FFFFFF, $F0FFFFFF, $E0FFFFFF, $C0FFFFFF, $80FFFFFF, $FFFFFF, $FEFFFF, $FCFFFF, $F8FFFF, $F0FFFF, $E0FFFF, $C0FFFF, $80FFFF, $FFFF, $FEFF, $FCFF, $F8FF, $F0FF, $E0FF, $C0FF, $80FF, $FF, $FE, $FC, $F8, $F0, $E0, $C0, $80, $0, \n\nPassed.\n\n>");
}

#[test]
fn call() {
    test("call", "Call and tailcall:\n\narg2adder()=0, arg2adder(4)=4, arg2adder(4,6)=10, arg2adder(4,6,1)=10\nhash(4,6,1)=19\ntailcalltest(2,3,4)=18, testglobal=2\n\nPassed.\n\n>");
}

#[test]
fn callstack() {
    test("callstack", "Call with various stack arrangements:\n\nhash(6,3,5,4,2)=53\nhash(4,5,2,3,1)=37, guard value=99\nhash(6,3,5,4,2)=53, guard value=98\nhash(4,5,2,3,1)=37\nhash(6,3,5,4,2)=53\nhash()=0\nhash(7)=7, hash(8)=8\nhash(9)=9, guard value=99\nguard value=98\nhash(6,7)=20, hash(5,7,2)=25\nhash(4,5,2,3,1)=37, guard value=99\nhash(6,3,5,4,2)=53, guard value=98\nhash(4,5,2,3,1)=37\nhash(6,3,5,4,2)=53\nhash()=0\nhash(7)=7, hash(8)=8\nhash(9)=9, guard value=99\nguard value=98\n\nPassed.\n\n>");
}

#[test]
fn jump() {
    test("jump", "Jumps and branches:\n\nJump loop 5=5\njz 0=1, jz 1=0, jz -1=0\njnz 0=0, jnz $1000000=1, jnz 1=1, jnz -1=1\njumpabs test=33, 44\n\nPassed.\n\n>");
}

#[test]
fn jumpform() {
    test("jumpform", "Jump with various operand forms:\n\nTest A0=2\nTest A1=5, guard val=91\nTest B0=0, B1=1\nTest C0=0, C1=1\nTest D0=0, D1=1\nTest E0=2, E1=3, E2=4, E3=5, E4=6\nTest F0=2, F1=3, F2=9\n\nJump-if-zero with various operand forms:\n\nTest B0=0, B1=1, B2=99, B3=99\nTest F0=2, F1=3, F2=9, F3=5, F4=2, F5=0, F6=1\n\nJump-if-equal with various operand forms:\n\nTest B0=0, B1=1, B2=99\nTest F0=2, F1=3, F2=9, F3=5, F4=2, F5=1, F6=0\n\nPassed.\n\n>");
}

#[test]
fn compare() {
    test("compare", "Compare branches:\n\njgt 2: 1, 1, 1, 0, 0, 0, 1\njgt -2: 1, 0, 0, 0, 0, 0, 0, 1\njge 2: 1, 1, 1, 1, 0, 0, 1\njge -2: 1, 1, 0, 0, 0, 0, 0, 1\njlt 2: 0, 0, 0, 0, 1, 1, 0\njlt -2: 0, 0, 1, 1, 1, 1, 1, 0\njle 2: 0, 0, 0, 1, 1, 1, 0\njle -2: 0, 1, 1, 1, 1, 1, 1, 0\njgtu 2: 0, 1, 1, 0, 0, 0, 0\njgtu -2: 1, 0, 0, 1, 1, 1, 1, 1\njgeu 2: 0, 1, 1, 1, 0, 0, 0\njgeu -2: 1, 1, 0, 1, 1, 1, 1, 1\njltu 2: 1, 0, 0, 0, 1, 1, 1\njltu -2: 0, 0, 1, 0, 0, 0, 0, 0\njleu 2: 1, 0, 0, 1, 1, 1, 1\njleu -2: 0, 1, 1, 0, 0, 0, 0, 0\n\nPassed.\n\n>");
}

#[test]
fn stack() {
    test("stack", "Stack operations:\n\nstkcount=0, stkcount=1, stkcount=2, stkcount=3, stkcount=3, stkcount=3, stkcount=3\nsp-sp=-1, sp-sp=3, sp-sp=1\npeek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7\ncount=0\npeek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7\ncount=0\npeek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7\ncount=0\npeek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7, peek 0=9, peek 1=8, peek 2=7\ncount=0\npeek 1=8, peek 1=8, peek 1=8, peek 0=9, peek 0=9, peek 0=9, peek 2=7, peek 2=7, peek 2=7\ncount=0\nhash(4,3,2)=16, hash(5,3,2,5,3,2)=64, hash(4,3,2,4,3,2)=59, hash(5,3,2,5,3,2)=64\nstkcount=0\nhash(5,3,2,5,3,2)=64, hash(4,3,2,4,3,2)=59, hash(5,3,2,5,3,2)=64\nstkcount=0\nhash(3,4,3,4,5)=61, hash(3,4,3,4,5)=61, hash(3,4,3,4,5)=61, hash(3,4,3,4,5)=61\nstkcount=0\nhash(6,4,3,2,1)=36, hash(6,4,3,2,1)=36, hash(6,4,3,2,1)=36, hash(4,3,6,2,1)=41, hash(3,6,4,2,1)=40, hash(4,3,6,2,1)=41, hash(3,6,4,2,1)=40, hash(6,4,3,2,1)=36, hash(6,4,3,2,1)=36\nhash(4,3,2,6,1)=45, hash(2,6,4,3,1)=43\nhash(4,3,2,6,1)=45, hash(2,6,4,3,1)=43\n\nPassed.\n\n>");
}

#[test]
fn gestalt() {
    test("gestalt", "Gestalt:\n\ngestalt(4,0)=1, gestalt(4,1)=1, gestalt(4,99)=0\ngestalt(1234,5678)=0\ngestalt(4,1)=1\ngestalt(4,1)=1\nguard=99\n\nPassed.\n\n>");
}

#[test]
fn throw() {
    test("throw", "Catch/throw:\n\ncatch 0=0, token=248\ncatch 1=1, token=248\ncatch 1 sp=1, token=248\ncatch discard=100, token=252, guard=77\ncatch computed=99, thrown=97, token=256\nglobal catch=105, token=220, guard=88\nglobal catch=106, token=220, guard=87\nglobal catch=107, token=220, guard=86\nlocal catch=105, token=220, guard=88\nlocal catch=104, token=220, count=1, guard=87\n\nPassed.\n\n>");
}

#[test]
fn streamnum() {
    test("streamnum", "Printing integers:\n\n\"0\" len 1\n\"1\" len 1\n\"-1\" len 2\n\"9999\" len 4\n\"-9999\" len 5\n\"1234579\" len 7\n\"-97654321\" len 9\n\"2147483647\" len 10\n\"-2147483647\" len 11\n\"-2147483648\" len 11\n\nPassed.\n\n>");
}

#[test]
fn strings() {
    test("strings", "String table decoding:\n\nBasic strings: \"hello\" len 5 is len 5, \"bye\" len 3, \"\" len 0, \"abcdefghijklmnopqrstuvwxyz\" len 26, \"àèìòù\" len 5, \"\n\" len 1, \"This contains several node types.\n\" len 34\nUnicode strings: \"hello\" len 5, \"abcdefghijklmnopqrstuvwxyz\" len 26, \"aàαォ\" len 4\nC-style strings: \"C string.\" len 9\n\"C Ünïcoδe “ォ”\" len 13\nSubstrings: \"\"substring\"\" len 11, \"\"substring\"\" len 11\nReferences: \"[hello]\" len 7, \"[hello]\" len 7, \"[]\" len 2, \"[1]\" len 3, \"[1 2]\" len 5, \"[foo bar]\" len 9\nIndirect references: \"{0:bye:0}\" len 9, \"{0:\"substring\":0}\" len 17, \"{1:{0:hello:0}:1}\" len 17, \"{1:{0:bye hello:0}:1}\" len 21, \"{0:1:0}\" len 7, \"{1:2 3:1}\" len 9, \"{2:hello bye:2}\" len 15, \"{\"\'``\'\"}\" len 8\nMultiple references: \"{hello...bye...C string.}\" len 25, \"{{1:+0:1}...+1...bye hello}\" len 27, counter=2\nIndirect references with unicode: \"{2:aàαォ:2}\" len 10\n\nPassed.\n\n>");
}

#[test]
fn ramstring() {
    test("ramstring", "String table decoding in RAM:\n\n\"Decode test.\" len 12\n\"Another test.\" len 13\n\"Third test.\" len 11\n\"\" len 0\n\nPassed.\n\n>");
}

#[test]
fn iosys() {
    test("iosys", "I/O mode switching:\n\n\"\" len 0, \"static glk\" len 10, \"=s=t=a=t=i=c= =f=i=l=t=e=r\" len 26\n\"=C= =Ü=n=ï=c=o=δ=e= =“=ォ=”\" len 26\n\"\" len 0, guard=99\n\"string, chr 0, -100 -2\" len 22, guard=99\n\"<s><t><r><i><n><g><,>< ><c><h><r>< ><0><,>< ><-><1><0><0>< ><-><2>\" len 66, guard=99\n\"<s><t><r><i><n><g><,>< ><c><h><r>< ><0><,>< ><-><1><0><0>< ><-><2>\" len 66, guard=99\ncurrent=2, 0\ncurrent=1, 41418\nChanging in mid-string: current=2, 0, \"<#>.\" len 4, current=2, 0, \"<a>bcde\" len 7, current=2, 0, \"<1>23\" len 5\n\nPassed.\n\n>");
}

#[test]
fn iosys2() {
    test("iosys2", "I/O mode with different store operands:\nThis tests for a bug in older Glulxe (version 0.4.5 and earlier). Calling @getiosys with two different store operands (e.g., a local variable and a global variable) did the wrong thing in those releases. Because the bug has been around for so long, the spec recommends not doing what this test does.\n\ncurrent=2, 0\n\nPassed.\n\n>");
}

#[test]
fn filter() {
    test("filter", "Filter iosys mode:\n\nBasic strings: \"=h=e=l=l=o\" len 10, \"b=y=e=\" len 6, \"=T=h=i=s= =c=o=n=t=a=i=n=s= =s=e=v=e=r=a=l= =n=o=d=e= =t=y=p=e=s=.=\n\" len 68, \"=C= =Ü=n=ï=c=o=´=e= =\u{1c}=©=\u{1d}\" len 26\nReferences: \"=[=h=e=l=l=o=]\" len 14, \"=[=1= =2=]\" len 10, \"=[=f=o=o= =b=a=r=]\" len 18\nMultiple references: \"={=h=e=l=l=o=.=.=.=b=y=e=.=.=.=C= =s=t=r=i=n=g=.=}\" len 50, \"={<.><.><.><[><h><e><l><l><o><]><.><.><.><[><1>< ><2><]><}>\" len 59, \"={={=1=:=+=0=:=1=}=.=.=.=+=1=.=.=.=b=y=e= =h=e=l=l=o=}\" len 54, counter=2, \"={=C= =s=t=r=i=n=g=.=.=.=.=+=0=.=.=.=C= =Ü=n=ï=c=o=´=e= =\u{1c}=©=\u{1d}=}\" len 64, counter=1\n\nPassed.\n\n>");
}

#[test]
fn nullio() {
    test("nullio", "Null iosys mode:\n\nBasic strings: \"\" len 0, \"\" len 0, \"\" len 0, \"\" len 0\nReferences: \"\" len 0, \"\" len 0, \"\" len 0\nMultiple references: \"\" len 0, \"<.><.><.><[><1>< ><2><]><}>\" len 27, \"\" len 0, counter=2, \"\" len 0, counter=1\n\nPassed.\n\n>");
}

#[test]
fn glk() {
    test("glk", "Glk opcode:\n\nlowercase \'A\'=97, lowercase \'B\'=98, lowercase \'C\'=99\nlowercase \'A\'=97, lowercase \'B\'=98, lowercase \'C\'=99\nlowercase \'D\'=100, lowercase \'E\'=101, lowercase \'F\'=102\nlowercase \'D\'=100, lowercase \'E\'=101, lowercase \'F\'=102\nlowercase \'G\'=103, lowercase \'H\'=104, lowercase \'I\'=105\nguard=999\nwindow=1, rock=201\nwindow=1, rock=201\nwindow=1, rock=201\nwindow=1, rock=201\nselect_poll=0, result=0000\nselect_poll=0, result=0000\nguard=999\nlen=14 \"up-case ãäδεд.\" len 14\nPassed.\n\n>");
}

#[test]
fn gidispa() {
    test("gidispa", "Glk dispatch layer:\n\n\"XYZЯ C string. C Ünïcoδe “ォ” 123 ⅕⅖⅗.\" len 37\nlength: 37\n\nPassed.\n\n>");
}

#[test]
fn random() {
    assert!(common::run_test("glulxercise.ulx", vec![
                (Match(INTRO), "random"),
                (Check(&|output| output.starts_with("Random-number generator:\nNOTE: Tests may, very occasionally, fail through sheer bad luck. If so, try this test again.\n\n") && output.ends_with("\n\nPassed.\n\n>")), "quit"),
                ]).is_ok());
}

#[test]
fn nonrandom() {
    test("nonrandom", "Random numbers in deterministic mode:\n\nsetrandom 1: 2056, 1, 2056, 1, 4196417, 4194368, 2056, 1, 131648, 4210688, 18504, 16449, 268456451, 306335811, 268572747, 302141506, \nsetrandom 100: 205600, 100, 205600, 100, 419641703, 419436803, 205600, 100, 13158658, 421068825, 1718841, 1644926, 1189036198, -1760061012, 1175302789, -1911062061, \nSequences different: 1\n\nPassed.\n\n>");
}

#[test]
fn search() {
    test("search", "Search opcodes:\n\nLinear:\ngot 166946, got 13, got 0, got -1\ngot 166946, got 13\ngot 166948, got 15, guard=999\ngot 166946, got 13, got 0, got -1\ngot 0, got -1, got 166950, got 17\ngot 0, got -1, got 166943, got 10\ngot 166945, got 6, got 6, got -1, got 166945, got 3\ngot 166945, got 6, got 166945, got 3\ngot 166957, got 6, got 0, got -1\ngot 166937, got 1, got 0, got -1\ngot 166941, got 2, got 0, got -1\ngot 166958, got 5, got 166958, got 5\ngot 166957, got 3, got 0, got -1, got 0, got -1, got 0, got -1\ngot 166933, got 0, got 0, got -1, got 166933, got 0\nBinary:\ngot 166978, got 13, got 0, got -1\ngot 166978, got 13\ngot 166980, got 15, guard=999\ngot 166965, 166966, 166967, 166968, 166969, 166970, 166971, 166972, 166973, 166974, 166975, 166976, 166977, 166978, 166979, 166980, 166981, 166982, 166983, 166984, 166985, 166986, 166987, 166988, 166989, 166990, 166991, 166992, 166993, 166994, 166995, 166996, \ngot 166965, 166966, 166967, 166968, 166969, 166970, 166971, 166972, 166973, 166974, 166975, 166976, 166977, 166978, 166979, 166980, 166981, 166982, 166983, 166984, 166985, 166986, 166987, 166988, 166989, 166990, 166991, 166992, 166993, 166994, 166995, \ngot 166978, got 13, got 0, got -1\ngot 166977, got 6, got 6, got -1, got 166977, got 3\ngot 166977, got 6, got 166977, got 3\ngot 166989, got 6\ngot 166969, got 1, got 0, got -1\ngot 166973, got 2, got 0, got -1\ngot 166990, got 5, got 166990, got 5\ngot 166989, got 3, got 0, got -1, got 0, got -1, got 0, got -1\ngot 166965, got 0, got 0, got -1, got 166965, got 0\nLinked:\ngot 166997, got 166997, got 167045, got 167053, got 167005, got 0\ngot 166997, got 0, got 167053, got 0, got 0\ngot 166997, got 166997, got 167045, got 167053, got 167005, got 0, got 0, got 0\ngot 166997, got 0, got 167053, got 0, got 0\ngot 166997, got 167053, got 167045, got 167021, got 0\ngot 166997, got 167053, got 167045, got 0, got 0\ngot 166997, got 167045, got 167053, got 167005, got 0\ngot 166997, got 167045, got 167053, got 167005, got 0\ngot 166997, got 166997, got 167045, got 167053, got 167005, got 0\ngot 166997, got 0, got 167053, got 0, got 0\ngot 166997, got 166997, got 167045, got 167053, got 167005, got 0, got 0, got 0\ngot 166997, got 0, got 167053, got 0, got 0\ngot 166997, got 167053, got 167045, got 167021, got 0\ngot 166997, got 167053, got 167045, got 0, got 0\ngot 166997, got 167045, got 167053, got 167005, got 0\ngot 166997, got 167045, got 167053, got 167005, got 0\ngot 167045, got 167005, guard=999\n\nPassed.\n\n>");
}

#[test]
fn mzero() {
    test("mzero", "mzero opcode:\n\n0, arr+4: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12\n1, arr+4: 1, 2, 3, 4, 0, 6, 7, 8, 9, 10, 11, 12\n6, arr+0: 0, 0, 0, 0, 0, 0, 7, 8, 9, 10, 11, 12\n3, arr+2: 1, 2, 0, 0, 0, 6, 7, 8, 9, 10, 11, 12\n4, arr+3: 1, 2, 3, 0, 0, 0, 0, 8, 9, 10, 11, 12\n\nPassed.\n\n>");
}

#[test]
fn mcopy() {
    test("mcopy", "mcopy opcode:\n\n0, arr+4, arr+6: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12\n0, arr+8, arr+6: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12\n4, arr+4, arr+4: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12\n5, arr+4, arr+6: 1, 2, 3, 4, 5, 6, 5, 6, 7, 8, 9, 12\n5, arr+6, arr+4: 1, 2, 3, 4, 7, 8, 9, 10, 11, 10, 11, 12\n3, arr+1, arr+8: 1, 2, 3, 4, 5, 6, 7, 8, 2, 3, 4, 12\n3, arr+8, arr+1: 1, 9, 10, 11, 5, 6, 7, 8, 9, 10, 11, 12\n2, arr+8, arr+1: 1, 9, 10, 4, 5, 6, 7, 8, 9, 10, 11, 12\n\nPassed.\n\n>");
}

#[test]
fn undo() {
    test("undo", "Undo:\n\nInterpreter claims to support undo.\n\nRestore without saveundo: 1\nRestore without saveundo: 1\nRestore without saveundo: 1\nUndo saved...\nRestoring undo...\nUndo succeeded, return value -1.\nloc=99 glob=999\nUndo saved...\nRestoring undo...\nUndo succeeded, return value -1.\nloc=98 glob=998\nUndo saved...\nRestoring undo...\nUndo succeeded, return value -1.\nloc=97 glob=997\nUndo saved...\nRestoring undo...\nUndo succeeded, return value -1.\nloc=98 glob=998\nUndo saved...\nguard=9\nRestoring undo...\nUndo succeeded, return value -1.\nloc=99 glob=999 glob2=-999\nguard=9\n\nPassed.\n\n>");
}

#[test]
fn multiundo() {
    test("multiundo", "Multi-level undo:\n\nInterpreter claims to support undo.\n\nUndo 1 saved...\nUndo 2 saved...\nRestoring undo 2...\nUndo 2 succeeded, return value -1.\nloc=77 glob=777\nRestoring undo 1...\nUndo 1 succeeded, return value -1.\nloc=99 glob=999\n\nPassed.\n\n>");
}

#[test]
fn extundo() {
    test("extundo", "ExtUndo:\n\nInterpreter claims to not support extended undo. Skipping test.\n\n\nPassed.\n\n>");
}

#[test]
fn restore() {
    test("restore", "Restore:\n\n(Deleting existing save file)\nSimple restore.\nSaving...\nSaved.\nRestoring...\n\n>");
}

#[test]
fn verify() {
    test("verify", "Verify:\n\nverify=0\nverify=0\nverify=0\n\nPassed.\n\n>");
}

#[test]
fn protect() {
    test("protect", "Protect:\n\nUndo saved...\nRestoring undo...\nUndo succeeded, return value -1.\nProtected 3,6: 1, 2, 3, 99, 99, 99, 99, 99, 99, 10, 11, 12\nUndo saved...\nRestoring undo...\nUndo succeeded, return value -1.\nProtected 6,1: 1, 2, 3, 4, 5, 6, 99, 8, 9, 10, 11, 12\nUndo saved...\nRestoring undo...\nUndo succeeded, return value -1.\nUnprotected: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12\n\nPassed.\n\n>");
}

#[test]
fn memsize() {
    test("memsize", "Memory-size extension:\n\nExtStart=$2B400; EndMem=$2B500\nInitial memsize=$2B500\nValue at $2B4FE=0\nWrite/read at $2B4FF=75\n@setmemsize=0\nNew memsize=$2B600\nWrite/read at $2B4FF=75\nWrite/read at $2B5FF=234\n@setmemsize=0\nRestored memsize=$2B500\n@setmemsize=0\nNew memsize=$2B600\nWrite/read at $2B5FF=0\n@setmemsize=0\nRestored memsize=$2B500\n\nPassed.\n\n>");
}

#[test]
fn undomemsize() {
    test("undomemsize", "Undo of memory-size extension:\n\nExtStart=$2B400; EndMem=$2B500\nOriginal memsize=$2B500\n@setmemsize=0\nNew memsize=$2B600\nWrote: $2B580: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12\nUndo saved...\n@setmemsize=0\nShrunk memsize=$2B500\nRestoring undo...\nUndo succeeded, return value -1.\nRestored memsize=$2B600\nRestored: $2B580: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12\nUndo saved...\n@setmemsize=0\nShrunk memsize=$2B500\nRestoring undo...\nUndo succeeded, return value -1.\nRestored memsize=$2B600\nRestored: $2B580: 1, 0, 0, 0, 0, 0, 7, 8, 9, 10, 11, 12\n@setmemsize=0\nRestored memsize=$2B500\n\nPassed.\n\n>");
}

#[test]
fn undorestart() {
    test("undorestart", "Undo of restart:\n\nUndo saved...\n\nA voice booooms out: You\'ve been here before!\n\nRestoring undo...\nUndo succeeded, return value -1.\nMagic number 1234, 3, 1\n\nPassed.\n\n>");
}

#[test]
fn heap() {
    test("heap", "Heap:\n\nOriginal memsize=$2B500\nCurrent heap: $0\nAllocating 16...\nHeap starts at $2B500, ends at $2B510\nAllocating 512...\nHeap starts at $2B500, ends at $2B710\nFreeing 16...\nHeap ends at $2B710\nFreeing 512...\nFinal heap: $0\nFinal memsize=$2B500\nblk1(19)=$2B500, blk2(23)=$2B513, blk3(17)=$2B52A\nfree blk2, blk2(23)=$2B513\nfree blk1, blk1(19)=$2B500\nfree blk2, blk2(23)=$2B513\nfree blk1, free blk2\nblk1(25)=$2B500, blk2(17)=$2B519\nfree blk2, blk2(41)=$2B53B\nfree blk1, free blk2, free blk3\nFinal heap: $0\nFinal memsize=$2B500\n\nPassed.\n\n>");
}

#[test]
fn undoheap() {
    test("undoheap", "Heap:\n\nOriginal memsize=$2B500\nCurrent heap: $0\nAllocating 16...\nAllocating 512...\nHeap starts at $2B500, ends at $2B710\nUndo saved...\nFreeing 16...\nFreeing 512...\nFinal heap: $0\nRestoring undo...\nUndo succeeded, return value -1.\nHeap starts at $2B500, ends at $2B710\nFreeing 16...\nFreeing 512...\nFinal heap: $0\nFinal memsize=$2B500\n\nPassed.\n\n>");
}

#[test]
fn acceleration() {
    test("acceleration", "Acceleration:\n(This tests only the operands. For a complete test of the accelfunc and accelparam opcodes, see accelfunctest.ulx.)\n\nguard=987\n\nPassed.\n\n>");
}

#[test]
fn floatconv() {
    test("floatconv", "Floating-point conversion:\n\n0=$0, -0=$80000000, 1=$3F800000, 9.2e-41=$10000, 2.9e-39=$200001, 1.2e-38=$7FFFFF, 1.2e-38=$800000, 3.8e-34=$8000000, 3.4e+38=$7F7FFFFF, Inf=$7F800000, -Inf=$FF800000\n\nnumtof 0=$0, numtof 1=$3F800000, numtof -1=$BF800000, numtof 2=$40000000, numtof -2=$C0000000, numtof 33=$42040000, numtof -33=$C2040000, numtof 100=$42C80000, numtof -100=$C2C80000, numtof 12345=$4640E400, numtof -12345=$C640E400, numtof 9876543=$4B16B43F, numtof -9876543=$CB16B43F, numtof $1000000=$4B800000, numtof -$1000000=$CB800000, numtof $1000001=$4B800000, numtof -$1000001=$CB800000, numtof $1234CDEF=$4D91A66F, numtof -$1234CDEF=$CD91A66F, numtof $7FFFFFFF=$4F000000, numtof -$7FFFFFFF=$CF000000, numtof $80000000=$CF000000\nnumtof 0=$0, numtof 1=$3F800000, numtof -1=$BF800000, numtof 2=$40000000, numtof -2=$C0000000, numtof 33=$42040000, numtof -33=$C2040000, numtof 100=$42C80000, numtof -100=$C2C80000, numtof 12345=$4640E400, numtof -12345=$C640E400, numtof 9876543=$4B16B43F, numtof -9876543=$CB16B43F, numtof $1000000=$4B800000, numtof -$1000000=$CB800000, numtof $1000001=$4B800000, numtof -$1000001=$CB800000, numtof $1234CDEF=$4D91A66F, numtof -$1234CDEF=$CD91A66F, numtof $7FFFFFFF=$4F000000, numtof -$7FFFFFFF=$CF000000, numtof $80000000=$CF000000\nnumtof 0=$0, numtof 1=$3F800000, numtof -1=$BF800000, numtof 2=$40000000, numtof -2=$C0000000, numtof 33=$42040000, numtof -33=$C2040000, numtof 100=$42C80000, numtof -100=$C2C80000, numtof 12345=$4640E400, numtof -12345=$C640E400, numtof 9876543=$4B16B43F, numtof -9876543=$CB16B43F, numtof $1000000=$4B800000, numtof -$1000000=$CB800000, numtof $1000001=$4B800000, numtof -$1000001=$CB800000, numtof $1234CDEF=$4D91A66F, numtof -$1234CDEF=$CD91A66F, numtof $7FFFFFFF=$4F000000, numtof -$7FFFFFFF=$CF000000, numtof $80000000=$CF000000\nnumtof 0=$0, numtof 1=$3F800000, numtof -1=$BF800000, numtof 2=$40000000, numtof -2=$C0000000, numtof 33=$42040000, numtof -33=$C2040000, numtof 100=$42C80000, numtof -100=$C2C80000, numtof 12345=$4640E400, numtof -12345=$C640E400, numtof 9876543=$4B16B43F, numtof -9876543=$CB16B43F, numtof $1000000=$4B800000, numtof -$1000000=$CB800000, numtof $1000001=$4B800000, numtof -$1000001=$CB800000, numtof $1234CDEF=$4D91A66F, numtof -$1234CDEF=$CD91A66F, numtof $7FFFFFFF=$4F000000, numtof -$7FFFFFFF=$CF000000, numtof $80000000=$CF000000\n\nftonumz 0.0=$0, ftonumz -0.0=$0, ftonumz 0.9=$0, ftonumz -0.9=$0, ftonumz 1.0=$1, ftonumz -1.0=$FFFFFFFF, ftonumz 1.75=$1, ftonumz -1.75=$FFFFFFFF, ftonumz 2.0=$2, ftonumz -2.0=$FFFFFFFE, ftonumz 10.1=$A, ftonumz -10.1=$FFFFFFF6, ftonumz 999.99995=$3E7, ftonumz -999.99995=$FFFFFC19, ftonumz $1000000=$1000000, ftonumz -$1000000=$FF000000, ftonumz $7FFFFF00=$7FFFFF00, ftonumz -$7FFFFF00=$80000100, ftonumz $80000000=$7FFFFFFF, ftonumz -$80000000=$80000000, ftonumz $90000000=$7FFFFFFF, ftonumz -$90000000=$80000000, ftonumz $C1234500=$7FFFFFFF, ftonumz -$C1234500=$80000000, ftonumz $100000000=$7FFFFFFF, ftonumz -$100000000=$80000000, ftonumz 3.4e+34=$7FFFFFFF, ftonumz -3.4e+34=$80000000, ftonumz +Inf=$7FFFFFFF, ftonumz -Inf=$80000000, ftonumz +NaN=$7FFFFFFF, ftonumz -NaN=$80000000\nftonumz 0.0=$0, ftonumz -0.0=$0, ftonumz 0.9=$0, ftonumz -0.9=$0, ftonumz 1.0=$1, ftonumz -1.0=$FFFFFFFF, ftonumz 1.75=$1, ftonumz -1.75=$FFFFFFFF, ftonumz 2.0=$2, ftonumz -2.0=$FFFFFFFE, ftonumz 10.1=$A, ftonumz -10.1=$FFFFFFF6, ftonumz 999.99995=$3E7, ftonumz -999.99995=$FFFFFC19, ftonumz $1000000=$1000000, ftonumz -$1000000=$FF000000, ftonumz $7FFFFF00=$7FFFFF00, ftonumz -$7FFFFF00=$80000100, ftonumz $80000000=$7FFFFFFF, ftonumz -$80000000=$80000000, ftonumz $90000000=$7FFFFFFF, ftonumz -$90000000=$80000000, ftonumz $C1234500=$7FFFFFFF, ftonumz -$C1234500=$80000000, ftonumz $100000000=$7FFFFFFF, ftonumz -$100000000=$80000000, ftonumz 3.4e+34=$7FFFFFFF, ftonumz -3.4e+34=$80000000, ftonumz +Inf=$7FFFFFFF, ftonumz -Inf=$80000000, ftonumz +NaN=$7FFFFFFF, ftonumz -NaN=$80000000\nftonumz 0.0=$0, ftonumz -0.0=$0, ftonumz 0.9=$0, ftonumz -0.9=$0, ftonumz 1.0=$1, ftonumz -1.0=$FFFFFFFF, ftonumz 1.75=$1, ftonumz -1.75=$FFFFFFFF, ftonumz 2.0=$2, ftonumz -2.0=$FFFFFFFE, ftonumz 10.1=$A, ftonumz -10.1=$FFFFFFF6, ftonumz 999.99995=$3E7, ftonumz -999.99995=$FFFFFC19, ftonumz $1000000=$1000000, ftonumz -$1000000=$FF000000, ftonumz $7FFFFF00=$7FFFFF00, ftonumz -$7FFFFF00=$80000100, ftonumz $80000000=$7FFFFFFF, ftonumz -$80000000=$80000000, ftonumz $90000000=$7FFFFFFF, ftonumz -$90000000=$80000000, ftonumz $C1234500=$7FFFFFFF, ftonumz -$C1234500=$80000000, ftonumz $100000000=$7FFFFFFF, ftonumz -$100000000=$80000000, ftonumz 3.4e+34=$7FFFFFFF, ftonumz -3.4e+34=$80000000, ftonumz +Inf=$7FFFFFFF, ftonumz -Inf=$80000000, ftonumz +NaN=$7FFFFFFF, ftonumz -NaN=$80000000\nftonumz 0.0=$0, ftonumz -0.0=$0, ftonumz 0.9=$0, ftonumz -0.9=$0, ftonumz 1.0=$1, ftonumz -1.0=$FFFFFFFF, ftonumz 1.75=$1, ftonumz -1.75=$FFFFFFFF, ftonumz 2.0=$2, ftonumz -2.0=$FFFFFFFE, ftonumz 10.1=$A, ftonumz -10.1=$FFFFFFF6, ftonumz 999.99995=$3E7, ftonumz -999.99995=$FFFFFC19, ftonumz $1000000=$1000000, ftonumz -$1000000=$FF000000, ftonumz $7FFFFF00=$7FFFFF00, ftonumz -$7FFFFF00=$80000100, ftonumz $80000000=$7FFFFFFF, ftonumz -$80000000=$80000000, ftonumz $90000000=$7FFFFFFF, ftonumz -$90000000=$80000000, ftonumz $C1234500=$7FFFFFFF, ftonumz -$C1234500=$80000000, ftonumz $100000000=$7FFFFFFF, ftonumz -$100000000=$80000000, ftonumz 3.4e+34=$7FFFFFFF, ftonumz -3.4e+34=$80000000, ftonumz +Inf=$7FFFFFFF, ftonumz -Inf=$80000000, ftonumz +NaN=$7FFFFFFF, ftonumz -NaN=$80000000\n\nftonumn 0.0=$0, ftonumn -0.0=$0, ftonumn 0.9=$1, ftonumn -0.9=$FFFFFFFF, ftonumn 1.0=$1, ftonumn -1.0=$FFFFFFFF, ftonumn 1.75=$2, ftonumn -1.75=$FFFFFFFE, ftonumn 2.0=$2, ftonumn -2.0=$FFFFFFFE, ftonumn 10.1=$A, ftonumn -10.1=$FFFFFFF6, ftonumn 999.99995=$3E8, ftonumn -999.99995=$FFFFFC18, ftonumn $1000000=$1000000, ftonumn -$1000000=$FF000000, ftonumn $7FFFFF00=$7FFFFF00, ftonumn -$7FFFFF00=$80000100, ftonumn $80000000=$7FFFFFFF, ftonumn -$80000000=$80000000, ftonumn $90000000=$7FFFFFFF, ftonumn -$90000000=$80000000, ftonumn $C1234500=$7FFFFFFF, ftonumn -$C1234500=$80000000, ftonumn $100000000=$7FFFFFFF, ftonumn -$100000000=$80000000, ftonumn 3.4e+34=$7FFFFFFF, ftonumn -3.4e+34=$80000000, ftonumn +Inf=$7FFFFFFF, ftonumn -Inf=$80000000, ftonumn +NaN=$7FFFFFFF, ftonumn -NaN=$80000000\nftonumn 0.0=$0, ftonumn -0.0=$0, ftonumn 0.9=$1, ftonumn -0.9=$FFFFFFFF, ftonumn 1.0=$1, ftonumn -1.0=$FFFFFFFF, ftonumn 1.75=$2, ftonumn -1.75=$FFFFFFFE, ftonumn 2.0=$2, ftonumn -2.0=$FFFFFFFE, ftonumn 10.1=$A, ftonumn -10.1=$FFFFFFF6, ftonumn 999.99995=$3E8, ftonumn -999.99995=$FFFFFC18, ftonumn $1000000=$1000000, ftonumn -$1000000=$FF000000, ftonumn $7FFFFF00=$7FFFFF00, ftonumn -$7FFFFF00=$80000100, ftonumn $80000000=$7FFFFFFF, ftonumn -$80000000=$80000000, ftonumn -$90000000=$80000000, ftonumn $C1234500=$7FFFFFFF, ftonumn -$C1234500=$80000000, ftonumn $100000000=$7FFFFFFF, ftonumn -$100000000=$80000000, ftonumn 3.4e+34=$7FFFFFFF, ftonumn -3.4e+34=$80000000, ftonumn +Inf=$7FFFFFFF, ftonumn -Inf=$80000000, ftonumn +NaN=$7FFFFFFF, ftonumn -NaN=$80000000\nftonumn 0.0=$0, ftonumn -0.0=$0, ftonumn 0.9=$1, ftonumn -0.9=$FFFFFFFF, ftonumn 1.0=$1, ftonumn -1.0=$FFFFFFFF, ftonumn 1.75=$2, ftonumn -1.75=$FFFFFFFE, ftonumn 2.0=$2, ftonumn -2.0=$FFFFFFFE, ftonumn 10.1=$A, ftonumn -10.1=$FFFFFFF6, ftonumn 999.99995=$3E8, ftonumn -999.99995=$FFFFFC18, ftonumn $1000000=$1000000, ftonumn -$1000000=$FF000000, ftonumn $7FFFFF00=$7FFFFF00, ftonumn -$7FFFFF00=$80000100, ftonumn $80000000=$7FFFFFFF, ftonumn -$80000000=$80000000, ftonumn $90000000=$7FFFFFFF, ftonumn -$90000000=$80000000, ftonumn $C1234500=$7FFFFFFF, ftonumn -$C1234500=$80000000, ftonumn $100000000=$7FFFFFFF, ftonumn -$100000000=$80000000, ftonumn 3.4e+34=$7FFFFFFF, ftonumn -3.4e+34=$80000000, ftonumn +Inf=$7FFFFFFF, ftonumn -Inf=$80000000, ftonumn +NaN=$7FFFFFFF, ftonumn -NaN=$80000000\nftonumn 0.0=$0, ftonumn -0.0=$0, ftonumn 0.9=$1, ftonumn -0.9=$FFFFFFFF, ftonumn 1.0=$1, ftonumn -1.0=$FFFFFFFF, ftonumn 1.75=$2, ftonumn -1.75=$FFFFFFFE, ftonumn 2.0=$2, ftonumn -2.0=$FFFFFFFE, ftonumn 10.1=$A, ftonumn -10.1=$FFFFFFF6, ftonumn 999.99995=$3E8, ftonumn -999.99995=$FFFFFC18, ftonumn $1000000=$1000000, ftonumn -$1000000=$FF000000, ftonumn $7FFFFF00=$7FFFFF00, ftonumn -$7FFFFF00=$80000100, ftonumn $80000000=$7FFFFFFF, ftonumn -$80000000=$80000000, ftonumn $90000000=$7FFFFFFF, ftonumn -$90000000=$80000000, ftonumn $C1234500=$7FFFFFFF, ftonumn -$C1234500=$80000000, ftonumn $100000000=$7FFFFFFF, ftonumn -$100000000=$80000000, ftonumn 3.4e+34=$7FFFFFFF, ftonumn -3.4e+34=$80000000, ftonumn +Inf=$7FFFFFFF, ftonumn -Inf=$80000000, ftonumn +NaN=$7FFFFFFF, ftonumn -NaN=$80000000\n\nPassed.\n\n>");
}

#[test]
fn floatarith() {
    test("floatarith", "Floating-point arithmetic:\n\nadd(1,1.5)=2.50000, add(0.5,-1.5)=-1.00000, add(-0.5,-1.5)=-2.00000, add(-0.5,1.5)=1.00000, add(0,2.5)=2.50000\nsub(1,1.5)=-0.50000, sub(0.5,-1.5)=2.00000, sub(-0.5,-1.5)=1.00000, sub(-0.5,1.5)=-2.00000, sub(0,2.5)=-2.50000\nmul(1.25,1.5)=1.87500, mul(0.5,-1.5)=-0.75000, mul(-0.75,-1.5)=1.12500, mul(-0.5,2)=-1.00000, mul(4,2.5)=10.00000\ndiv(1.25,1.5)=0.83333, div(0.5,-1.5)=-0.33333, div(-0.75,-1.5)=0.50000, div(-0.5,2)=-0.25000, div(4,2.5)=1.60000\n\nadd(1,1)=2.00000, add(-1,1)=0.00000, add(-1,-1)=-2.00000, add(1,0)=1.00000, add(-0,1)=1.00000, add(-0,0)=0.00000, add(123,-0)=123.00000, add(0,123)=123.00000, add(1.0000001,-1)=1.19209e-07, add(3.4e38,3.4e38)=Inf, add(-3.4e38,-3.4e38)=-Inf, add(3.4e38,-3.4e38)=0.00000, add(Inf,123)=Inf, add(-Inf,123)=-Inf, add(Inf,Inf)=Inf, add(-Inf,Inf)=-NaN\nadd(1,NaN)=NaN, add(NaN,-0)=NaN, add(Inf,NaN)=NaN, add(-Inf,NaN)=NaN, add(NaN,NaN)=NaN\n\nsub(1,1)=0.00000, sub(-1,1)=-2.00000, sub(-1,-1)=0.00000, sub(1,0)=1.00000, sub(-0,1)=-1.00000, sub(123,-0)=123.00000, sub(0,123)=-123.00000, sub(1.0000001,1)=1.19209e-07, sub(3.4e38,3.4e38)=0.00000, sub(-3.4e38,-3.4e38)=0.00000, sub(3.4e38,-3.4e38)=Inf, sub(-3.4e38,3.4e38)=-Inf, sub(Inf,123)=Inf, sub(-Inf,123)=-Inf, sub(123,Inf)=-Inf, sub(123,-Inf)=Inf, sub(Inf,-Inf)=Inf, sub(-Inf,Inf)=-Inf, sub(-Inf,-Inf)=-NaN, sub(Inf,Inf)=-NaN\nsub(1,NaN)=NaN, sub(NaN,-0)=NaN, sub(Inf,NaN)=NaN, sub(-Inf,NaN)=NaN, sub(NaN,NaN)=NaN\n\nmul(1,1)=1.00000, mul(-1,1)=-1.00000, mul(-1,-1)=1.00000, mul(1,0)=0.00000, mul(-0,1)=-0.00000, mul(-0,-1)=0.00000, mul(123,-1)=-123.00000, mul(1,123)=123.00000, mul(3.4e38,2.9e-39)=1.00000, mul(2.9e-39,2.9e-39)=0.00000, mul(-2.9e-39,2.9e-39)=-0.00000, mul(1e20,1e20)=Inf, mul(1e20,-1e20)=-Inf, mul(-1e20,-1e20)=Inf, mul(Inf,0.0001)=Inf, mul(-Inf,0.0001)=-Inf, mul(Inf,Inf)=Inf, mul(-Inf,Inf)=-Inf, mul(-Inf,-Inf)=Inf, mul(Inf,0)=-NaN, mul(-0,Inf)=-NaN\nmul(1,NaN)=NaN, mul(NaN,-0)=NaN, mul(Inf,NaN)=NaN, mul(-Inf,NaN)=NaN, mul(NaN,NaN)=NaN\n\ndiv(1,1)=1.00000, div(-1,1)=-1.00000, div(-1,-1)=1.00000, div(1,0)=Inf, div(1,-0)=-Inf, div(-0,1)=-0.00000, div(-0,-1)=0.00000, div(123,-1)=-123.00000, div(123,1)=123.00000, div(3.4e38,2.9e-39)=Inf, div(2.9e-39,2.9e-39)=1.00000, div(-2.9e-39,2.9e-39)=-1.00000, div(1e20,1e20)=1.00000, div(1e20,-1e20)=-1.00000, div(Inf,10000)=Inf, div(-Inf,10000)=-Inf, div(Inf,0)=Inf, div(Inf,-0)=-Inf, div(Inf,Inf)=-NaN, div(-Inf,Inf)=-NaN, div(0,0)=-NaN, div(-0,0)=-NaN\ndiv(1,NaN)=NaN, div(NaN,-0)=NaN, div(Inf,NaN)=NaN, div(-Inf,NaN)=NaN, div(NaN,NaN)=NaN\n\nPassed.\n\n>");
}

#[test]
fn floatmod() {
    test("floatmod", "Floating-point modulo:\n\nmod(4.125,2)=rem 0.12500 quo 2.00000, mod(5,1.5)=rem 0.50000 quo 3.00000, mod(7.125,1)=rem 0.12500 quo 7.00000, mod(6,1.75)=rem 0.75000 quo 3.00000, mod(5.125,0.5)=rem 0.12500 quo 10.00000, mod(4,0.75)=rem 0.25000 quo 5.00000\n\nmod(2.5,1)=rem 0.50000 quo 2.00000, mod(2.5,-1)=rem 0.50000 quo -2.00000, mod(-2.5,1)=rem -0.50000 quo -2.00000, mod(-2.5,-1)=rem -0.50000 quo 2.00000, mod(0,1)=rem 0.00000 quo 0.00000, mod(0,-1)=rem 0.00000 quo -0.00000, mod(-0,1)=rem -0.00000 quo -0.00000, mod(-0,-1)=rem -0.00000 quo 0.00000\n\nmod(5.125,2)=rem 1.12500 quo 2.00000, mod(5.125,-2)=rem 1.12500 quo -2.00000, mod(-5.125,2)=rem -1.12500 quo -2.00000, mod(-5.125,-2)=rem -1.12500 quo 2.00000\nmod(5.125,1)=rem 0.12500 quo 5.00000, mod(5.125,-1)=rem 0.12500 quo -5.00000, mod(-5.125,1)=rem -0.12500 quo -5.00000, mod(-5.125,-1)=rem -0.12500 quo 5.00000\nmod(1.5,0.75)=rem 0.00000 quo 2.00000, mod(1.5,-0.75)=rem 0.00000 quo -2.00000, mod(-1.5,0.75)=rem -0.00000 quo -2.00000, mod(-1.5,-0.75)=rem -0.00000 quo 2.00000\n\nmod(1e-20,1)=rem 1.00000e-20 quo 0.00000, mod(1e20,1)=rem 0.00000 quo 1.00000e+20, mod(8388607.5,1)=rem 0.50000 quo 8.38861e+06, mod(-8388607.5,1)=rem -0.50000 quo -8.38861e+06\nmod(2.5e11,1e10)=rem 15360.00000 quo 25.00000, mod(2.5e10,1e10)=rem 5.00000e+09 quo 2.00000, mod(2.5e10,0.0123)=rem 0.00301 quo 2.03252e+12\n\nmod(0,0)=rem -NaN quo NaN, mod(-0,0)=rem -NaN quo NaN, mod(1,0)=rem -NaN quo NaN, mod(Inf,1)=rem -NaN quo NaN, mod(-Inf,1)=rem -NaN quo NaN, mod(Inf,Inf)=rem -NaN quo NaN, mod(Inf,-Inf)=rem -NaN quo NaN, mod(-Inf,Inf)=rem -NaN quo NaN, mod(0,1)=rem 0.00000 quo 0.00000, mod(-0,1)=rem -0.00000 quo -0.00000, mod(0,-1)=rem 0.00000 quo -0.00000, mod(-0,-1)=rem -0.00000 quo 0.00000, mod(1,Inf)=rem 1.00000 quo 0.00000, mod(1,-Inf)=rem 1.00000 quo -0.00000, mod(-2,Inf)=rem -2.00000 quo -0.00000, mod(-0.125,Inf)=rem -0.12500 quo -0.00000\n\nmod(1,NaN)=NaN quo NaN, mod(NaN,-1)=NaN quo NaN, mod(0,NaN)=NaN quo NaN, mod(N0,NaN)=NaN quo NaN, mod(Inf,NaN)=NaN quo NaN, mod(NaN,Inf)=NaN quo NaN, mod(NaN,-Inf)=NaN quo NaN, mod(-Inf,NaN)=NaN quo NaN, mod(NaN,NaN)=NaN quo NaN\n\nPassed.\n\n>");
}

#[test]
fn floatround() {
    test("floatround", "Floating-point rounding:\n\nfloor 3.5=3.00000, floor -3.5=-4.00000, floor 3.5=3.00000, floor -3.5=-4.00000, floor 3.5=3.00000, floor -3.5=-4.00000, floor 3.5=3.00000, floor -3.5=-4.00000, floor 3.5=3.00000, floor -3.5=-4.00000\n\nceil 3.5=4.00000, ceil -3.5=-3.00000, ceil 3.5=4.00000, ceil -3.5=-3.00000, ceil 3.5=4.00000, ceil -3.5=-3.00000, ceil 3.5=4.00000, ceil -3.5=-3.00000, ceil 3.5=4.00000, ceil -3.5=-3.00000\n\nfloor 0.0=0.00000, floor -0.0=-0.00000, floor 0.9=0.00000, floor -0.9=-1.00000, floor 1.0=1.00000, floor -1.0=-1.00000, floor 1.75=1.00000, floor -1.75=-2.00000, floor 2.0=2.00000, floor -2.0=-2.00000, floor 10.1=10.00000, floor -10.1=-11.00000, floor 999.99995=999.00000, floor -999.99995=-1000.00000, floor $1000000=1.67772e+07, floor -$1000000=-1.67772e+07, floor $7FFFFF00=2.14748e+09, floor -$7FFFFF00=-2.14748e+09, floor $80000000=2.14748e+09, floor -$80000000=-2.14748e+09, floor +Inf=Inf, floor -Inf=-Inf, floor +NaN=NaN, floor -NaN=-NaN\n\nceil 0.0=0.00000, ceil -0.0=-0.00000, ceil 0.9=1.00000, ceil -0.9=-0.00000, ceil 1.0=1.00000, ceil -1.0=-1.00000, ceil 1.75=2.00000, ceil -1.75=-1.00000, ceil 2.0=2.00000, ceil -2.0=-2.00000, ceil 10.1=11.00000, ceil -10.1=-10.00000, ceil 999.99995=1000.00000, ceil -999.99995=-999.00000, ceil $1000000=1.67772e+07, ceil -$1000000=-1.67772e+07, ceil $7FFFFF00=2.14748e+09, ceil -$7FFFFF00=-2.14748e+09, ceil $80000000=2.14748e+09, ceil -$80000000=-2.14748e+09, ceil +Inf=Inf, ceil -Inf=-Inf, ceil +NaN=NaN, ceil -NaN=-NaN\n\nPassed.\n\n>");
}

#[test]
fn floatexp() {
    test("floatexp", "Floating-point exponent functions:\n\nsqrt 2.25=1.50000, sqrt -2.25=NaN, sqrt 2.25=1.50000, sqrt -2.25=NaN, sqrt 2.25=1.50000, sqrt -2.25=NaN, sqrt 2.25=1.50000, sqrt -2.25=NaN\n\nlog e^2=2.00000, log -1.0=-NaN, log e^2=2.00000, log -1.0=-NaN, log e^2=2.00000, log -1.0=-NaN, log e^2=2.00000, log -1.0=-NaN\n\nexp 2.0=7.38906, exp -2.0=0.13534, exp 2.0=7.38906, exp -2.0=0.13534, exp 2.0=7.38906, exp -2.0=0.13534, exp 2.0=7.38906, exp -2.0=0.13534\n\npow(1.75,1.5)=2.31503, pow(1.75,-1.5)=0.43196, pow(-1.75,2)=3.06250, pow(-1.75,1.5)=-NaN\npow(2.25,2.0)=5.06250, pow(2.25,-2.0)=0.19753, pow(-2.25,3.0)=-11.39063, pow(-2.25,-3.0)=-0.08779\n\nsqrt 0=0.00000, sqrt -0=-0.00000, sqrt 1=1.00000, sqrt -1=NaN, sqrt 0.6=0.77460, sqrt 100.0000076=10.00000, sqrt 123456789.0=11111.11133, sqrt 9.8765e+35=9.93805e+17, sqrt Inf=Inf, sqrt -Inf=NaN, sqrt +NaN=NaN, sqrt -NaN=-NaN\n\nexp 0=1.00000, exp -0=1.00000, exp 1=2.71828, exp -1=0.36788, exp 0.6=1.82212, exp -0.6=0.54881, exp 88.0=1.65164e+38, exp 100.0=Inf, exp -100.0=3.78352e-44, exp -104.0=0.00000, exp Inf=Inf, exp -Inf=0.00000, exp +NaN=NaN, exp -NaN=-NaN\n\nlog 0=-Inf, log -0=-Inf, log 1=0.00000, log -1=-NaN, log e=~0.99999, log 0.6=~-0.51083, log 65536=~11.09035, log 123456789.0=~18.63140, log 9.8765e+37=~87.48581, log Inf=Inf, log -Inf=-NaN, log +NaN=NaN, log -NaN=-NaN\n\npow(-1,1)=-1.00000, pow(-1,-1)=-1.00000, pow(-1,1.5)=-NaN, pow(0,1)=0.00000, pow(-0,1)=-0.00000, pow(2,127)=1.70141e+38, pow(2,128)=Inf, pow(2,-149)=1.40130e-45, pow(2,-150)=0.00000, pow(2,NaN)=NaN, pow(NaN,2)=NaN, pow(NaN,NaN)=NaN\npow(0,-1)=Inf, pow(-0,-1)=-Inf, pow(0,-2)=Inf, pow(-0,-2)=Inf, pow(0,-1.5)=Inf, pow(-0,-1.5)=Inf, pow(0,1)=0.00000, pow(-0,1)=-0.00000, pow(0,2)=0.00000, pow(-0,2)=0.00000, pow(0,1.5)=0.00000, pow(-0,1.5)=0.00000\npow(-1,Inf)=1.00000, pow(-1,-Inf)=1.00000, pow(1,1)=1.00000, pow(1,-1)=1.00000, pow(1,0)=1.00000, pow(1,Inf)=1.00000, pow(1,-Inf)=1.00000, pow(1,NaN)=1.00000, pow(1,-NaN)=1.00000\npow(4,0)=1.00000, pow(-4,0)=1.00000, pow(0,0)=1.00000, pow(-0,0)=1.00000, pow(Inf,0)=1.00000, pow(-Inf,0)=1.00000, pow(NaN,0)=1.00000, pow(-NaN,0)=1.00000\npow(-1,1.5)=-NaN, pow(0.5,-Inf)=Inf, pow(-0.5,-Inf)=Inf, pow(1.5,-Inf)=0.00000, pow(-1.5,-Inf)=0.00000, pow(0.5,Inf)=0.00000, pow(-0.5,Inf)=0.00000, pow(1.5,Inf)=Inf, pow(-1.5,Inf)=Inf, pow(-Inf,-1)=-0.00000, pow(-Inf,-2)=0.00000, pow(-Inf,-1.5)=0.00000, pow(-Inf,1)=-Inf, pow(-Inf,2)=Inf, pow(-Inf,1.5)=Inf, pow(Inf,2)=Inf, pow(Inf,1.5)=Inf, pow(Inf,-2)=0.00000, pow(Inf,-1.5)=0.00000\n\nPassed.\n\n>");
}

#[test]
fn floattrig() {
    test("floattrig", "Floating-point trig functions:\n\nsin(pi/6)=~0.50000, sin(-pi/3)=~-0.86603, sin(pi/4)=~0.70711\ncos(pi/6)=~0.86603, cos(-pi/3)=~0.50000, cos(pi/4)=~0.70711\ntan(pi/6)=~0.57735, tan(-pi/3)=~-1.73205, tan(pi/4)=1.00000\nasin(1/2)=~0.52360, asin(-sqrt(3)/2)=~-1.04720, asin(sqrt(2)/2)=~0.78540\nacos(sqrt(3)/2)=~0.52360, acos(-0.5)=~2.09440, acos(sqrt(2)/2)=~0.78540\natan(sqrt(3)/3)=~0.52360, atan(-sqrt(3))=~-1.04720, atan(1)=~0.78540\n\nsin(0)=0.00000, sin(-0)=-0.00000, sin(pi)=~-8.74228e-08, sin(2pi)=~1.74846e-07, sin(Inf)=-NaN, sin(-Inf)=-NaN, sin(NaN)=NaN\ncos(0)=1.00000, cos(-0)=1.00000, cos(pi)=~-1.00000, cos(2pi)=~1.00000, cos(Inf)=-NaN, cos(-Inf)=-NaN, cos(NaN)=NaN\ntan(0)=0.00000, tan(-0)=-0.00000, tan(pi)=~8.74228e-08, tan(2pi)=~1.74846e-07, tan(Inf)=-NaN, tan(-Inf)=-NaN, tan(NaN)=NaN\nasin(0)=0.00000, asin(-0)=-0.00000, asin(1)=~1.57080, asin(-1)=~-1.57080, asin(2)=-NaN, asin(-2)=-NaN, asin(Inf)=-NaN, asin(-Inf)=-NaN, asin(NaN)=NaN\nacos(1)=0.00000, acos(-1)=~3.14159, acos(0)=~1.57080, acos(-0)=~1.57080, acos(2)=-NaN, acos(-2)=-NaN, acos(Inf)=-NaN, acos(-Inf)=-NaN, acos(NaN)=NaN\natan(0)=0.00000, atan(-0)=-0.00000, atan(1)=~0.78540, atan(-1)=~-0.78540, atan(Inf)=~1.57080, atan(-Inf)=~-1.57080, atan(NaN)=NaN\n\nPassed.\n\n>");
}

#[test]
fn floatatan2() {
    test("floatatan2", "Floating-point atan2 function:\n\natan2(1,1)=~0.78540, atan2(1,-1)=~2.35619, atan2(-1,-1)=~-2.35619, atan2(-1,1)=~-0.78540\n\natan2(1,2)=~0.46365, atan2(2,-0.5)=~1.81578, atan2(-0.125,-8)=~-3.12597, atan2(-2,3)=~-0.58800\n\natan2(0,0)=0.00000, atan2(-0,0)=-0.00000, atan2(0,-0)=~3.14159, atan2(-0,-0)=~-3.14159, atan2(0,1)=0.00000, atan2(-0,1)=-0.00000, atan2(0,-1)=~3.14159, atan2(-0,-1)=~-3.14159, atan2(1,0)=~1.57080, atan2(1,-0)=~1.57080, atan2(-1,0)=~-1.57080, atan2(-1,-0)=~-1.57080\natan2(1,Inf)=0.00000, atan2(-1,Inf)=-0.00000, atan2(1,-Inf)=~3.14159, atan2(-1,-Inf)=~-3.14159, atan2(Inf,Inf)=~0.78540, atan2(-Inf,Inf)=~-0.78540, atan2(Inf,-Inf)=~2.35619, atan2(-Inf,-Inf)=~-2.35619\natan2(1,NaN)=NaN, atan2(NaN,-0)=NaN, atan2(Inf,NaN)=NaN, atan2(-Inf,NaN)=NaN, atan2(NaN,NaN)=NaN\n\nPassed.\n\n>");
}

#[test]
fn fjumpform() {
    test("fjumpform", "Floating-point jump with various operand forms:\n\nTest A0=33, Test A1=44, Test A2=33, Test A3=44, Test A4=33, Test A5=44\nTest B0=11, Test B1=22, Test B2=11, Test B3=22, Test B4=11, Test B5=22\nTest C0=55, Test C1=66, Test C2=55, Test C3=66, Test C4=55, Test C5=66\nTest E0=0, E1=1, E2=99\nTest F0=2, F1=3, F2=9, F3=5, F4=2, F5=1, F6=0\n\nPassed.\n\n>");
}

#[test]
fn fjump() {
    test("fjump", "Floating-point equality comparisons:\n\njisnan(0)=0, jisnan(-0)=0, jisnan(1)=0, jisnan(3.4e38)=0, jisnan(Inf)=0, jisnan(-Inf)=0, jisnan(NaN)=1, jisnan(-NaN)=1, jisnan(other NaN)=1, jisnan(other -NaN)=1\njisinf(0)=0, jisinf(-0)=0, jisinf(1)=0, jisinf(3.4e+38)=0, jisinf(Inf)=1, jisinf(-Inf)=1, jisinf(NaN)=0, jisinf(-NaN)=0, jisinf(other NaN)=0, jisinf(other -NaN)=0\njfeq(0,0,0)=1, jfeq(0,-0,0)=1, jfeq(0,0,-0)=1, jfeq(0,1.4e-45,0)=0, jfeq(0,-1.4e-45,0)=0, jfeq(3.4e+38,3.4e+38,0)=1, jfeq(3.4e+38,3.4e+38,0)=0, jfeq(Inf,Inf,0)=1, jfeq(-Inf,-Inf,0)=1, jfeq(Inf,-Inf,0)=0\njfeq(0,0,1.4e-45)=1, jfeq(0,-0,1.4e-45)=1, jfeq(0,0,-1.4e-45)=1, jfeq(0,1.4e-45,1.4e-45)=1, jfeq(0,-1.4e-45,1.4e-45)=1, jfeq(0,1.4e-45,-1.4e-45)=1, jfeq(0,2.8e-45,1.4e-45)=0, jfeq(3.4e+38,3.4e+38,1.4e-45)=1, jfeq(3.4e+38,3.4e+38,1.4e-45)=0, jfeq(Inf,Inf,1.4e-45)=1, jfeq(-Inf,-Inf,1.4e-45)=1, jfeq(Inf,-Inf,1.4e-45)=0\njfeq(0,0,1)=1, jfeq(0,-2,1)=0, jfeq(0,-2,1.5)=0, jfeq(0,-2,2)=1, jfeq(0,-2,-2)=1, jfeq(1.5,2,1.5)=1, jfeq(1.5,3,1.5)=1, jfeq(1.5,3+,1.5)=0\njfeq(0,3.4e+38,3.4e+38-)=0, jfeq(0,3.4e+38,3.4e+38)=1, jfeq(-1,3.4e+38,3.4e+38)=1, jfeq(-3.4e+38,3.4e+38,3.4e+38)=0,jfeq(Inf,3.4e+38,3.4e+38)=0, jfeq(-Inf,-3.4e+38,3.4e+38)=0, jfeq(Inf,Inf,3.4e+38)=1, jfeq(-Inf,Inf,3.4e+38)=0\njfeq(0,0,Inf)=1, jfeq(0,3.4e+38,Inf)=1, jfeq(0,3.4e+38,-Inf)=1, jfeq(0,-3.4e+38,-Inf)=1, jfeq(-3.4e+38,3.4e+38,Inf)=1, jfeq(-3.4e+38,3.4e+38,-Inf)=1, jfeq(0,Inf,Inf)=1, jfeq(-3.4e+38,-Inf,Inf)=1, jfeq(0,-Inf,Inf)=1, jfeq(-Inf,-Inf,Inf)=1, jfeq(Inf,-Inf,Inf)=0\njfeq(NaN,0,0)=0, jfeq(0,NaN,0)=0, jfeq(0,0,NaN)=0, jfeq(0,NaN,NAN)=0, jfeq(NaN,0,NaN)=0, jfeq(NaN,NaN,0)=0, jfeq(NaN,NaN,NaN)=0, jfeq(Inf,Inf,NaN)=0, jfeq(Inf,-Inf,NaN)=0, jfeq(Inf,0,NaN)=0, jfeq(0,NaN,Inf)=0, jfeq(NaN,NaN,Inf)=0\njfne(0,0,0)=0, jfne(0,-0,0)=0, jfne(0,0,-0)=0, jfne(0,1.4e-45,0)=1, jfne(0,-1.4e-45,0)=1, jfne(3.4e+38,3.4e+38,0)=0, jfne(3.4e+38,3.4e+38,0)=1, jfne(Inf,Inf,0)=0, jfne(-Inf,-Inf,0)=0, jfne(Inf,-Inf,0)=1\njfne(0,0,1.4e-45)=0, jfne(0,-0,1.4e-45)=0, jfne(0,0,-1.4e-45)=0, jfne(0,1.4e-45,1.4e-45)=0, jfne(0,-1.4e-45,1.4e-45)=0, jfne(0,1.4e-45,-1.4e-45)=0, jfne(0,2.8e-45,1.4e-45)=1, jfne(3.4e+38,3.4e+38,1.4e-45)=0, jfne(3.4e+38,3.4e+38,1.4e-45)=1, jfne(Inf,Inf,1.4e-45)=0, jfne(-Inf,-Inf,1.4e-45)=0, jfne(Inf,-Inf,1.4e-45)=1\njfne(0,0,1)=0, jfne(0,-2,1)=1, jfne(0,-2,1.5)=1, jfne(0,-2,2)=0, jfne(0,-2,-2)=0, jfne(1.5,2,1.5)=0, jfne(1.5,3,1.5)=0, jfne(1.5,3+,1.5)=1\njfne(0,3.4e+38,3.4e+38-)=1, jfne(0,3.4e+38,3.4e+38)=0, jfne(-1,3.4e+38,3.4e+38)=0, jfne(-3.4e+38,3.4e+38,3.4e+38)=1,jfne(Inf,3.4e+38,3.4e+38)=1, jfne(-Inf,-3.4e+38,3.4e+38)=1, jfne(Inf,Inf,3.4e+38)=0, jfne(-Inf,Inf,3.4e+38)=1\njfne(0,0,Inf)=0, jfne(0,3.4e+38,Inf)=0, jfne(0,3.4e+38,-Inf)=0, jfne(0,-3.4e+38,-Inf)=0, jfne(-3.4e+38,3.4e+38,Inf)=0, jfne(-3.4e+38,3.4e+38,-Inf)=0, jfne(0,Inf,Inf)=0, jfne(-3.4e+38,-Inf,Inf)=0, jfne(0,-Inf,Inf)=0, jfne(-Inf,-Inf,Inf)=0, jfne(Inf,-Inf,Inf)=1\njfne(NaN,0,0)=1, jfne(0,NaN,0)=1, jfne(0,0,NaN)=1, jfne(0,NaN,NAN)=1, jfne(NaN,0,NaN)=1, jfne(NaN,NaN,0)=1, jfne(NaN,NaN,NaN)=1, jfne(Inf,Inf,NaN)=1, jfne(Inf,-Inf,NaN)=1, jfne(Inf,0,NaN)=1, jfne(0,NaN,Inf)=1, jfne(NaN,NaN,Inf)=1\n\nPassed.\n\n>");
}

#[test]
fn fcompare() {
    test("fcompare", "Floating-point inequality comparisons:\n\njflt(0,0)=0, jflt(0,1)=1, jflt(0,-1)=0, jflt(-0,0)=0, jflt(-0,1)=1, jflt(-0,-1)=0, jflt(-0,-0)=0, jflt(1,1)=0, jflt(pi,pi)=0, jflt(0,1.4e-45)=1, jflt(0,-1.4e-45)=0, jflt(-1.4e-45,-0)=1, jflt(1.4e-45,3.4e+38)=1, jflt(1.4e-45,-3.4e+38)=0\njflt(0,Inf)=1, jflt(0,-Inf)=0, jflt(3.4e+38,Inf)=1, jflt(3.4e+38,-Inf)=0, jflt(-Inf,-3.4e+38)=1, jflt(-Inf,Inf)=1, jflt(Inf,-Inf)=0, jflt(Inf,Inf)=0, jflt(-Inf,-Inf)=0\njflt(0,NaN)=0, jflt(NaN,0)=0, jflt(Inf,NaN)=0, jflt(-Inf,NaN)=0, jflt(NaN,Inf)=0, jflt(NaN,-Inf)=0, jflt(NaN,NaN)=0, jflt(-NaN,NaN)=0\njfle(0,0)=1, jfle(0,1)=1, jfle(0,-1)=0, jfle(-0,0)=1, jfle(-0,1)=1, jfle(-0,-1)=0, jfle(-0,-0)=1, jfle(1,1)=1, jfle(pi,pi)=1, jfle(0,1.4e-45)=1, jfle(0,-1.4e-45)=0, jfle(-1.4e-45,-0)=1, jfle(1.4e-45,3.4e+38)=1, jfle(1.4e-45,-3.4e+38)=0\njfle(0,Inf)=1, jfle(0,-Inf)=0, jfle(3.4e+38,Inf)=1, jfle(3.4e+38,-Inf)=0, jfle(-Inf,-3.4e+38)=1, jfle(-Inf,Inf)=1, jfle(Inf,-Inf)=0, jfle(Inf,Inf)=1, jfle(-Inf,-Inf)=1\njfle(0,NaN)=0, jfle(NaN,0)=0, jfle(Inf,NaN)=0, jfle(-Inf,NaN)=0, jfle(NaN,Inf)=0, jfle(NaN,-Inf)=0, jfle(NaN,NaN)=0, jfle(-NaN,NaN)=0\njfgt(0,0)=0, jfgt(0,1)=0, jfgt(0,-1)=1, jfgt(-0,0)=0, jfgt(-0,1)=0, jfgt(-0,-1)=1, jfgt(-0,-0)=0, jfgt(1,1)=0, jfgt(pi,pi)=0, jfgt(0,1.4e-45)=0, jfgt(0,-1.4e-45)=1, jfgt(-1.4e-45,-0)=0, jfgt(1.4e-45,3.4e+38)=0, jfgt(1.4e-45,-3.4e+38)=1\njfgt(0,Inf)=0, jfgt(0,-Inf)=1, jfgt(3.4e+38,Inf)=0, jfgt(3.4e+38,-Inf)=1, jfgt(-Inf,-3.4e+38)=0, jfgt(-Inf,Inf)=0, jfgt(Inf,-Inf)=1, jfgt(Inf,Inf)=0, jfgt(-Inf,-Inf)=0\njfgt(0,NaN)=0, jfgt(NaN,0)=0, jfgt(Inf,NaN)=0, jfgt(-Inf,NaN)=0, jfgt(NaN,Inf)=0, jfgt(NaN,-Inf)=0, jfgt(NaN,NaN)=0, jfgt(-NaN,NaN)=0\njfge(0,0)=1, jfge(0,1)=0, jfge(0,-1)=1, jfge(-0,0)=1, jfge(-0,1)=0, jfge(-0,-1)=1, jfge(-0,-0)=1, jfge(1,1)=1, jfge(pi,pi)=1, jfge(0,1.4e-45)=0, jfge(0,-1.4e-45)=1, jfge(-1.4e-45,-0)=0, jfge(1.4e-45,3.4e+38)=0, jfge(1.4e-45,-3.4e+38)=1\njfge(0,Inf)=0, jfge(0,-Inf)=1, jfge(3.4e+38,Inf)=0, jfge(3.4e+38,-Inf)=1, jfge(-Inf,-3.4e+38)=0, jfge(-Inf,Inf)=0, jfge(Inf,-Inf)=1, jfge(Inf,Inf)=1, jfge(-Inf,-Inf)=1\njfge(0,NaN)=0, jfge(NaN,0)=0, jfge(Inf,NaN)=0, jfge(-Inf,NaN)=0, jfge(NaN,Inf)=0, jfge(NaN,-Inf)=0, jfge(NaN,NaN)=0, jfge(-NaN,NaN)=0\n\nPassed.\n\n>");
}

#[test]
fn fprint() {
    test("fprint", "Print floating-point numbers:\nNote: this does not test an opcode. It tests the FloatExp function, which is included in this test suite. You are welcome to use that function in your Glulx program or library.\n\n\"0.00000e+00\" len 11, \"-0.00000e+00\" len 12, \"1.00000e+00\" len 11, \"-1.00000e+00\" len 12\n\"1.00000e-01\" len 11, \"3.33333e-02\" len 11, \"2.00000e+00\" len 11, \"1.00000e+02\" len 11, \"1.00000e+02\" len 11, \"1.00000e+02\" len 11, \"9.99999e+01\" len 11, \"1.25000e+02\" len 11\n\"3.00000e-30\" len 11, \"6.99998e+30\" len 11, \"3.00004e-40\" len 11, \"6.99998e+34\" len 11\n\"1.0e+00\" len 7, \"1.0000e+00\" len 10, \"1.00000024e+00\" len 14, \"6.8e+00\" len 7, \"6.7898e+00\" len 10, \"6.78979520e+00\" len 14\n\"Inf\" len 3, \"-Inf\" len 4, \"NaN\" len 3, \"-NaN\" len 4\n\n\"0.00000\" len 7, \"-0.00000\" len 8, \"1.00000\" len 7, \"-1.00000\" len 8\n\"0.10000\" len 7, \"0.02000\" len 7, \"0.03333\" len 7, \"12.34568\" len 8, \"-120.34568\" len 10, \"100000.34375\" len 12, \"1000000.37500\" len 13, \"10000000.00000\" len 14\n\"0.00000\" len 7, \"-0.00000\" len 8, \"4294965440.00000\" len 16, \"1000000240000000000000000000000.00000\" len 37\n\"1.0\" len 3, \"1.0000\" len 6, \"1.00000000\" len 10, \"6.8\" len 3, \"6.7898\" len 6, \"6.78978832\" len 10\n\"Inf\" len 3, \"-Inf\" len 4, \"NaN\" len 3, \"-NaN\" len 4\n\n\"1.00\" len 4, \"1.00000\" len 7, \"999999.00000\" len 12, \"-999999.00000\" len 13, \"1.00001e+06\" len 11, \"-1.00001e+06\" len 12, \"0.00010\" len 7, \"-0.00010\" len 8, \"9.0000e-05\" len 10, \"-9.0000e-05\" len 11\n\nPassed.\n\n>");
}

#[test]
fn safari5() {
    test("safari5", "Safari 5 bug:\nThis tests for a known Javascript bug in Safari 5, MacOS 10.5.8, Intel (not PPC). You should only see this fail on Quixe on that browser setup. This failure does not represent a Glulx error. I just want to be able to track it.\n\nFolded: $FFFFFF\nStack: $FFFFFF\nLocals: $FFFFFF\n\nPassed.\n\n>");
}
