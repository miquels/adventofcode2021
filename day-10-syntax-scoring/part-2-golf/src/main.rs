use std::io::{self, BufRead};

fn main() {
    let mut s = io::stdin().lock().lines().flatten()
        .filter_map(|l| {let (mut v,mut z)=(Vec::new(),0);for x in l.bytes() {
            if (x&1)==(x&2)/2{v.push(x)}else{z|=(x^v.pop().unwrap())&112}}
            (z==0).then(||v.iter().rev().fold(0,|a,x|5*a+((x+24*(x&4))>>5)as u64))
         }).collect::<Vec<_>>();
    s.sort();println!("{}",s[s.len()/2]);
}
