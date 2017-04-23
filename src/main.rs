extern crate getopts;

use std::env;
use getopts::Options;
use std::fs::File;


fn print_usage(binary: &str, opts: &Options) {
    let brief = format!("💩 💩 💩  Usage: {} [options]", binary);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let bin_name = args[0].clone();
    let mut opts = Options::new();
    let mut IN_FILE;
    
    opts.optopt("f", "file", "input file", "NAME");
    opts.optopt("d", "decode", "decoding mode (encoding as default)", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(v) => { v }
        Err(r) => { panic!(r.to_string()) }
    };
    
    if matches.opt_present("f") {
        IN_FILE = File::open(matches.opt_str("f").unwrap()).unwrap();
    } else {
        print_usage(&bin_name, &opts);
        return;
    }

    if matches.opt_present("d") {
        let mut statistic = File::open(matches.opt_str("d").unwrap()).unwrap();
        haff_decode(& mut IN_FILE, & mut statistic);
    } else {
        haff_encode(& mut IN_FILE);
    }

    //let mut OUT_FILE = File::create("haff.out").unwrap();


}

fn haff_decode(file: & mut File, stats: & mut File) {

}

fn haff_encode(file: & mut File) {

}