pub struct Ari {
    pub stats: Vec<usize>,
    high: usize,
    low: usize,
    bits_to_follow: isize,
}

//i dont want to think about this code, so i just copy-paste most of it 
const MAX_VAL: usize = 65535;
const BITS_N: usize = 16;
const FIRST_Q: usize = MAX_VAL / 4 + 1;
const HALF: usize = FIRST_Q << 1;
const THIRD_Q: usize = FIRST_Q * 3;


impl Ari {
    pub fn new() -> Ari {
        let stats = Vec::new();
        Ari{ stats: stats, low: 0, high: MAX_VAL, bits_to_follow: 0 }
    }

    #[allow(dead_code)]
    pub fn refresh_stats(&mut self, buf: &str) {
        let mut stats = vec![1; 256];
        for i in buf.bytes() {
            let i = i as usize;
            stats[i] += 1;
        }
        self.stats = stats;
    }

    pub fn encode(&mut self, buf: &str) -> String{
        self.refresh_stats(buf);
        let mut acc = String::new();

        for byte in buf.bytes() {
            self.encode_symbol(byte, &mut acc);
        }
        
        //inlined `done_encoding` func
        self.bits_to_follow+= 1;
        if self.low <  FIRST_Q {
            self.bits_plus_follow(0, &mut acc);
        } else {
            self.bits_plus_follow(1, &mut acc);
        }

        String::from("asdf")
    }

    #[allow(dead_code)] 
    fn encode_symbol(&mut self, byte: u8, acc: &mut String) {
        let byte = byte as usize;
        let range = (self.high - self.low) + 1;
        //********************************************
        //* don't forget to change `self.stats` here *
        //********************************************
        self.high = self.low + (range * self.stats[byte-1]) / self.stats[0] - 1;
        self.low  = self.low + (range * self.stats[byte]) / self.stats[0];
        loop {
            if self.high < HALF {
                self.bits_plus_follow(0, acc);
            }
            else if self.low >= HALF { 
                self.bits_plus_follow(1, acc);
                self.low -= HALF;
                self.high -= HALF;
            }
            else if self.low >= FIRST_Q && self.high < THIRD_Q {
                self.bits_to_follow += 1;
                self.low -= FIRST_Q;
                self.high -= FIRST_Q;
            }
            else { 
                break; 
            }
            self.low <<= 1;
            self.high <<= 1;
        }
    }

    //may be shoud rename this method to `output` or something same
    fn bits_plus_follow(&mut self, bit: usize, acc: &mut String) {
        
        match bit {
            0 => { acc.push('0'); },
            1 => { acc.push('1'); },
            _ => panic!("Ti chto durak?")
        }

        while self.bits_to_follow > 0 {
            match bit {
                0 => { acc.push('1'); },
                1 => { acc.push('0'); },
                _ => panic!("Ti chto durak?")
            }
            self.bits_to_follow-= 1;
        }
    }
}