use std::{collections::HashSet,io::BufRead};use scan_fmt::*;
fn main() {
  let(mut d,mut c)=(HashSet::new(),Vec::new());
  for l in std::io::stdin().lock().lines().flatten(){
    scan_fmt!(&l,"{},{}",i16,i16).map(|w|d.insert([w.0,w.1])).ok();
    scan_fmt!(&l,"fold along {}={}",char,i16).map(|w|c.push(((w.0=='y') as usize,w.1))).ok();}
  for&(a,p)in&c{d=d.drain().map(|mut c|{if c[a]>p{c[a]=2*p-c[a]}c}).collect();}
  print!("\x1b[2J");for x in&d{print!("\x1b[{};{}H#",x[1]+1,x[0]+1)};print!("\x1b[20;1H");
}
