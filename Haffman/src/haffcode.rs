use std::fs::File;
use std::collections::HashMap;
use std::io::{Read, Write, Seek};
use std::io::SeekFrom::Start;

#[derive(Clone)]
pub struct Node {
    sym: usize,
    freq: usize,
    //because here is no pointers in rust ¯\_(ツ)_/¯
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}


pub struct Haffman {
    pub file: File,
    pub stats: Option<HashMap<usize, usize>>,
    pub dict: Option<HashMap<usize, String>>,
}

impl Haffman {

    fn collect_stats(&mut self){
        let tmp = &self.file;
        let mut stats: HashMap<usize, usize> = HashMap::new();
        for byte in tmp.bytes() {
            let i = byte.unwrap() as usize;
            let stat = stats.entry(i).or_insert(0);
            *stat += 1;
        }
        self.stats = Some(stats);
    }

    fn build_dict(& mut self, tree: Node, code: String) {
        //let Tree = self.build_tree();
        let mut have_r = true;
        let mut have_l = true;
        
        match tree.right {
            Some(node) => { self.build_dict(*node , code.clone() + "1"); }
            None => { have_r = false; }
        }

        match tree.left {
            Some(node) => { self.build_dict(*node, code.clone() + "0"); }
            None => { have_l = false; }
        }

        //already at leaf
        if !have_r && !have_l {
            let mut some_dict: HashMap<usize, String> = HashMap::new();
            {
                let kek = self.dict.as_mut();
                //cause of need to break borrowing
                {
                    let mut tmp = kek.unwrap_or(&mut some_dict);
                    tmp.insert(tree.sym as usize, code);
                }
            }
            match self.dict {
                None => { self.dict = Some(some_dict); }
                _ => (),
            }
        }
    }

    //actually a wrapper
    fn build_tree(&self) -> Node {
        let mut pairs = Vec::new();
        
        match self.stats {
            Some(ref val) => {  
                for (i, j) in val.iter() { 
                    pairs.push(( Node{ sym: *i, freq: *j, right: None, left: None } ));
                } 
            }
            None => println!("Не сукец ¯\\_(ツ)_/¯")
        }

        let kek = self.build_tree_rec(pairs);

        kek
    }

    //why i called vector - `pairs` wtf?
    fn build_tree_rec (&self, mut pairs: Vec<Node>) -> Node {

        if pairs.len() == 1 {
          
            pairs[0].clone()

        } else {

            for _ in 0..pairs.len() {
                for i in 0..(pairs.len()-1) {
                    if pairs[i].freq > pairs[i+1].freq {
                        pairs.swap(i, i+1);
                    }
                }
            }

            //lol nice bug(feature)
            //let l = pairs.remove(0);
            //fucking removing second element after removing first element and shifting vector -___-
            //let r = pairs.remove(1);
            //dont forget to add this idea to trello ;C
            
            let l = pairs.remove(0);
            let r = pairs.remove(0);
            pairs.push(Node{ sym: 255, freq: l.freq + r.freq, right: Some(Box::new(r)), left: Some(Box::new(l)) });
            
            self.build_tree_rec(pairs)
        }
    }

    pub fn show_stats(&self) {
        match self.stats {
            Some(ref val) => {  for (i, j) in val.iter() { let tmp = *i as u8 as char; println!("{} - {}", tmp, j);} }
            None => println!("Не сукец ¯\\_(ツ)_/¯")
        }
    }

    pub fn show_dict(&self) {
        match self.dict {
            Some(ref dict) => {
                for (i, j) in dict.iter() {
                    let tmp = *i as u8 as char;
                    println!("`{}` - {}", tmp, j);
                }
            }

            None => println!("Не сукец ¯\\_(ツ)_/¯")
        }
    }

    pub fn encode(& mut self) {
        self.collect_stats();

        //check for empty file -___-
        //but its a bit retarded
        match self.stats {
            Some(ref val) => { if val.len() == 0 { panic!("Are you retarded?"); } },
            None => ()
        }

        let tree = self.build_tree();
        self.build_dict(tree, String::from(""));
        
        let mut out = File::create("haff.enc").unwrap();
        let dict = match self.dict {
            Some(ref dict) => dict,
            None => panic!("kek")
        };

        let file  = &mut self.file;
        let _ = file.seek(Start(0));
        for i in file.bytes() {
            let i = i.unwrap() as usize;
            out.write(&(dict[&i].clone().into_bytes())[..]).unwrap();
            let _ = out.flush();
        }
    }

    pub fn decode(&self, to_decode: File) {
        let mut dec_dict = HashMap::new();
        let dict = match self.dict {
            Some(ref val) => val,
            None => panic!("lolwut")
        };

        for (key, val) in dict.iter() {
            dec_dict.insert(val.clone(), key.clone());
        }

        let mut out = File::create("haff.dec").unwrap();
        let mut buf = String::new();
        for i in to_decode.bytes() {
            match i {
                Ok(49) => { buf.push('1'); },
                Ok(48) => { buf.push('0'); },
                _ => panic!("Error while decoding"),
            };

            if dec_dict.contains_key(&buf) {
                //fuck you fucking rust -___- DAFUCK IS THIS
                let tmp = dec_dict[&buf] as u8 as char; //<-----
                write!(out, "{}", tmp).unwrap();
                let _ = out.flush();
                buf.clear();
            }
        }
    }
}
