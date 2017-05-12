extern crate gcc;

fn main() {
    gcc::Config::new()
        .define("main", Some("cheapglk_main"))
        .include("cheapglk")
        .file("cheapglk/cgfref.c")
        .file("cheapglk/cggestal.c")
        .file("cheapglk/cgmisc.c")
        .file("cheapglk/cgstream.c")
        .file("cheapglk/cgstyle.c")
        .file("cheapglk/cgwindow.c")
        .file("cheapglk/cgschan.c")
        .file("cheapglk/cgdate.c")
        .file("cheapglk/cgunicod.c")
        .file("cheapglk/main.c")
        .file("cheapglk/gi_dispa.c")
        .file("cheapglk/gi_blorb.c")
        .file("cheapglk/gi_debug.c")
        .file("cheapglk/cgblorb.c")
        .file("cheapglk/glkstart.c")
        .compile("libcheapglk.a");
}
