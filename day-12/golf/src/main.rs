use std::collections::{HashMap, HashSet};
use scan_fmt::*;
fn main() {
  let mut c=HashMap::new();
  while let Ok((c1, c2))=scanln_fmt!("{}-{}",String,String) {
    c.entry(c1.clone()).or_insert(Vec::<String>::new()).push(c2.clone());
    c.entry(c2).or_insert(Vec::new()).push(c1);
  }
  fn e(c:&HashMap<String,Vec<String>>,n:&str,t:bool,mut v:HashSet<String>)->u32 {
    if n=="end"{return 1};if n.as_bytes()[0]&32>0{v.insert(n.to_string());}
    c[n].iter().filter_map(|k|{let x=!v.contains(k);((x||t)&&k!="start").then(||e(c,k,x&&t,v.clone()))}).sum()
  }
  println!("1. {}\n2. {}",e(&c,"start",false,HashSet::new()),e(&c,"start",true,HashSet::new()));
}
