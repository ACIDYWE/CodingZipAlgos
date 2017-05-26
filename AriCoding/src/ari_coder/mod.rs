pub struct Ari {
    pub stats: Vec<usize>,
    cum_freq: Vec<usize>,
    high: usize,
    low: usize,
    bits_to_follow: isize,
    value: usize,
    counter: usize,
}

//i dont want to think about this code, so i just copy-paste most of it 
const MAX_VAL: usize = 65535;
const BITS_N: usize = 16;
const FIRST_Q: usize = (MAX_VAL >> 2) + 1;
const HALF: usize = FIRST_Q << 1;
const THIRD_Q: usize = FIRST_Q * 3;


impl Ari {
    pub fn new() -> Ari {
        let stats = Vec::new();
        let cum_freq = Vec::new();
        Ari{ stats: stats, cum_freq: cum_freq, low: 0, high: MAX_VAL, bits_to_follow: 0, value: 0, counter: 0}
    }

    #[allow(dead_code)]
    fn refresh_stats(&mut self, buf: &str) {
        let mut stats = vec![1; 256];
        for i in buf.bytes() {
            let i = i as usize;
            stats[i] += 1;
        }
        self.stats = stats;
    }

    fn refresh_cumfreq(&mut self) {
        let mut cum_freq = self.stats.clone();
        //freaking idiotism
        //fn (&mut self) -> () WHY????!!!1111
        cum_freq.reverse();
        let mut cum_freq = cum_freq.iter().scan(0, |state, &x| { 
                                        *state = *state + x;
                                        Some(*state)
                                    }).collect::<Vec<usize>>();
         cum_freq.reverse();
         self.cum_freq = cum_freq;

    }


    pub fn encode(&mut self, buf: &str) -> String {
        self.refresh_stats(buf);
        self.refresh_cumfreq();
        //println!("{:?}", self.cum_freq);
        let mut acc = String::new();

        for byte in buf.bytes() {
            self.encode_symbol(byte, &mut acc);
        }

        //encode termination null-byte
        //self.encode_symbol(0u8, &mut acc);
        //inlined `done_encoding` func
        self.bits_to_follow+= 1;
        //println!("self.bits_to_follow = {}", self.bits_to_follow);
        if self.low <  FIRST_Q {
            self.bits_plus_follow(0, &mut acc);
        } else {
            self.bits_plus_follow(1, &mut acc);
        }

        acc
    }

    #[allow(dead_code)] 
    fn encode_symbol(&mut self, byte: u8, acc: &mut String) {
        let byte = byte as usize; 
        let range = (self.high - self.low) + 1; 

        //println!("range = {}, byte = {}, self.low = {}, self.high = {}", range, byte, self.low, self.high);
        //println!("byte = {}", byte);
        if byte == 0 {
            self.high = self.low + (range * self.cum_freq[0]) / MAX_VAL - 1; 
            self.low  = self.low + (range * self.cum_freq[0]) / MAX_VAL;
        } else {
            self.high = self.low + (range * self.cum_freq[byte-1]) / self.cum_freq[0] - 1; 
            self.low  = self.low + (range * self.cum_freq[byte]) / self.cum_freq[0];
        }
        //println!("range = {}, byte = {}, self.low = {}, self.high = {}", range, byte, self.low, self.high);
        
        loop {
            if self.high < HALF {
                self.bits_plus_follow(0, acc);
            } else if self.low >= HALF { 
                self.bits_plus_follow(1, acc);
                self.low -= HALF;
                self.high -= HALF;
            } else if self.low >= FIRST_Q && self.high < THIRD_Q {
                self.bits_to_follow += 1;
                self.low -= FIRST_Q;
                self.high -= FIRST_Q;
            } else { 
                break; 
            }
            self.low = self.low << 1;
            self.high = (self.high << 1) + 1;
            //println!("low = {}, high = {}", self.low, self.high);//self.bits_to_follow);
        }
    }

    pub fn decode(&mut self, buf: &str) -> String {
        //init decoding
        self.low = 0;
        self.high = MAX_VAL;
        self.value = 0;

        for i in buf[0..BITS_N].bytes() {
            self.value <<= 1;
            match i {
                48 => (),
                49 => { self.value += 1; },
                _ => { panic!("KEK"); }
            }
        }
        self.counter = BITS_N;
        let mut res = String::new();
        let mut tmp = self.decode_symbol(buf);
        while tmp != '\x7f' {
            res.push(tmp);
            tmp = self.decode_symbol(buf);
        }

        res
    }

    fn decode_symbol(&mut self, buf: &str) -> char {
        let range = (self.high - self.low) + 1;
        let cum = ((self.value - self.low + 1) * self.cum_freq[0] - 1) / range;
        let mut symbol = 0;
        //println!("cum = {}, value = {}", cum, self.value);
        for i in 0..256 {
            if self.cum_freq[i] <= cum {
                symbol = i;
                break;
            }   
        }
        self.high = self.low + (range * self.cum_freq[symbol-1]) / self.cum_freq[0] - 1;
        self.low = self.low + (range * self.cum_freq[symbol]) / self.cum_freq[0];

        //println!("Value = {} low = {}", self.value, self.low);

        loop {
            //println!("KEKLOOP");
            if self.high < HALF {

            }
            else if self.low >= HALF {
                self.value -= HALF;
                self.low -= HALF;
                self.high -= HALF;
            } else  if self.low >= FIRST_Q && self.high < THIRD_Q {
                self.value -= FIRST_Q;
                self.low -= FIRST_Q;
                self.high -= FIRST_Q;
            } else {
                break;
            }
            self.low <<= 1;
            self.high = (self.high << 1) + 1;

            let temp = String::from(buf).into_bytes();
            //println!("So i think here is problem");
            let kek;
            if temp.len() <= self.counter {
                kek = 48;
            } else {
                kek = temp[self.counter] as usize;
            }

            self.value = (self.value << 1) + (kek - 48);       
            self.counter += 1;

            //println!("Value = {} low = {}", self.value, self.low);
        }
        //println!("Value = {} low = {}", self.value, self.low);
            //println!("end debug");
            symbol as u8 as char
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