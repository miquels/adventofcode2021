use std::{collections::{HashSet as H,BinaryHeap as B,Vec as V},cmp::Reverse as R,io::{self, BufRead}};
fn main() {
    let m=io::stdin().lock().lines().flatten().map(|l|l.bytes().map(|b|b as u16-48)
        .collect::<V<_>>()).collect::<V<_>>();let(lx,ly)=(m[0].len(),m.len());
    let mut m5=m.clone();m5.resize(5*ly,V::new());for y in 0..m5.len(){m5[y].resize(5*lx,0)}
    for y in 0..5*ly{for x in 0..5*lx{m5[y][x]=(m5[y%ly][x%lx]-1+(y/ly+x/lx)as u16)%9+1;}}
    sp(&m);sp(&m5);
}
fn sp(m:&V<V<u16>>){
    let(mx,my)=(m[0].len()-1,m.len()-1);
    let a=|x:usize,y:usize|[(y>0).then(||(x,y-1)),(x>0).then(||(x-1,y)),(x<mx).then(||(x+1,y)),
        (y<my).then(||(x,y+1))].into_iter().flatten().map(|(x,y)|(m[y][x],(x,y)));
    let mut d=V::new();d.resize_with(my+1,||{let mut v=V::new();v.resize(mx+1,u16::MAX);v});d[0][0]=0;
    let mut tv=B::new();tv.push(R((0,(0,0))));let mut vs=H::new();
    while let Some(R((r,(x,y))))=tv.pop() {
        if vs.insert((x,y)){for(h,(x,y))in a(x,y){if r+h<d[y][x]{d[y][x]=r+h;tv.push(R((r+h,(x,y))))}}}
    }
    println!("{}",d[my][mx]);
}
