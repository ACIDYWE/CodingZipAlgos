extern crate getopts;

use std::env;
use getopts::Options;
use std::fs::File;

struct State {
    file: String,
    mode: bool,
}

fn print_usage(binary: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", binary);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut state = State {file: String::from(""), mode: false};
    let args: Vec<String> = env::args().collect();
    let bin_name = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("f", "file", "input file", "NAME");
    opts.optflag("d", "decode", "decoding mode (encoding as default}");
    let matches = match opts.parse(&args[1..]) {
        Ok(v) => { v }
        Err(r) => { panic!(r.to_string()) }
    };
    
    if matches.opt_present("f") {
        state.file = matches.opt_str("f").unwrap();
    } else {
        print_usage(&bin_name, &opts);
        return;
    }

    //if matched '-d' flag, then enable decode mode
    state.mode = matches.opt_present("d");

    let mut IN_FILE = File::open(state.file).unwrap();
    let mut OUT_FILE = File::create("haff.out").unwrap();


    
}