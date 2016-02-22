extern crate rustbox;
extern crate vex;
extern crate getopts;

use getopts::Options;
use std::env;
use std::path::Path;

use self::rustbox::{RustBox};

use vex::vex_editor::{Editor};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut editor = Editor::new(rustbox.width(), rustbox.height());
    if matches.free.is_empty() {
        editor.open_empty();
    } else {
        for name in matches.free {
            editor.open(Path::new(&name));
        }
    }
    editor.edit(&rustbox);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}