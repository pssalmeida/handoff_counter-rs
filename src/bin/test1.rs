extern crate handoff_counter;

use handoff_counter::handoff_counter::Counter;

type Cnt = Counter<&'static str>;

pub fn main() {
    let mut c = Cnt::new("aa", 0);
    c.incr();
    println!("{}", c.id());
    println!("{}", c.value());
    println!("{:#?}", c);
}
