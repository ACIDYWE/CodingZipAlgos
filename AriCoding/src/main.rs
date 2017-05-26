use std::io::{Read, self};

mod ari_coder;
use ari_coder::Ari;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.push('\x7f');
    
    let mut coder = Ari::new();
    let str_enc = coder.encode(&buffer);
    println!("\n{}", str_enc);
    
    let str_dec = coder.decode(&str_enc);
    println!("{}", str_dec);
}