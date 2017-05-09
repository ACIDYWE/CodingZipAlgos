extern crate getopts;

use std::env;
use getopts::Options;
use std::fs::File;

mod haffcode;
use haffcode::Haffman;



fn print_usage(binary: &str, opts: &Options) {
    let brief = format!("💩 💩 💩  Usage: {} [options]", binary);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let bin_name = args[0].clone();
    let mut opts = Options::new();
    
    opts.optopt("f", "file", "input file", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(v) => { v }
        Err(r) => { panic!(r.to_string()) }
    };
    
    let mut haffy: Haffman;
    if matches.opt_present("f") {
        haffy = Haffman{file: File::open(matches.opt_str("f").unwrap()).unwrap(), stats: None, dict: None};
        haffy.encode();
        //haffy.show_stats();
        haffy.show_dict();

    } else {
        print_usage(&bin_name, &opts);
        return;
    }
    
}
