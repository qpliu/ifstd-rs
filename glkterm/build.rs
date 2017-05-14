extern crate gcc;
extern crate pkg_config;

fn main() {
    gcc::Config::new()
        .define("main", Some("glkterm_main"))
        .include("glkterm")
        .file("glkterm/main.c")
        .file("glkterm/gtevent.c")
        .file("glkterm/gtfref.c")
        .file("glkterm/gtgestal.c")
        .file("glkterm/gtinput.c")
        .file("glkterm/gtmessag.c")
        .file("glkterm/gtmessin.c")
        .file("glkterm/gtmisc.c")
        .file("glkterm/gtstream.c")
        .file("glkterm/gtstyle.c")
        .file("glkterm/gtw_blnk.c")
        .file("glkterm/gtw_buf.c")
        .file("glkterm/gtw_grid.c")
        .file("glkterm/gtw_pair.c")
        .file("glkterm/gtwindow.c")
        .file("glkterm/gtschan.c")
        .file("glkterm/gtblorb.c")
        .file("glkterm/cgunicod.c")
        .file("glkterm/cgdate.c")
        .file("glkterm/gi_dispa.c")
        .file("glkterm/gi_blorb.c")
        .compile("libglkterm.a");
    println!("cargo:rustc-link-lib=ncurses");
    println!("cargo:rustc-link-search=/usr/lib");
}
