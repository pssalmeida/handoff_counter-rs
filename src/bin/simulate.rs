extern crate handoff_counter;
extern crate rand;

use std::collections::HashMap;
use handoff_counter::Counter;
use rand::Rng;


fn vars(prefix: &str, n: usize) -> Vec<String> {
    (0..n).map(|i| prefix.to_string() + &i.to_string()).collect()
}

type Cnt<'a> = Counter<&'a str>;
type Env<'a> = HashMap<&'a str, Cnt<'a>>;

fn rand_trace(n: usize, c: usize, s: usize, r: usize) {
    let clients = vars("c", c);
    let servers = vars("s", s);
    let roots = vars("r", r);

    let mut nodes: Vec<String> = Vec::with_capacity(c+s+r);
    nodes.extend_from_slice(&clients);
    nodes.extend_from_slice(&servers);
    nodes.extend_from_slice(&roots);

    let mut env = HashMap::new();
    for id in &clients { let id = id.as_str(); env.insert(id, Cnt::new(id, 2)); }
    for id in &servers { let id = id.as_str(); env.insert(id, Cnt::new(id, 1)); }
    for id in &roots { let id = id.as_str(); env.insert(id, Cnt::new(id, 0)); }

    let mut rng = rand::thread_rng();
    let mut incrs = 0;

    for k in 0..n {
        let i = rng.gen::<usize>() % nodes.len();
        if rng.gen::<u64>() % 2 == 0 && k < n / 2 {
            let id = nodes[i].as_str();
            let c = env.get_mut(id).unwrap();
            c.incr();
            incrs += 1;
        } else {
            let mut j;
            loop {
                j = rng.gen::<usize>() % nodes.len();
                if i != j { break; }
            }
            let id = nodes[i].as_str();
            let mut c1 = env.remove(&id).unwrap();
            let id2 = nodes[j].as_str();
            c1.merge(env.get(&id2).unwrap());
            env.insert(id, c1);
        }
    }

    println!("incrs: {}", incrs);
    let mut wrong = 0;
    for c in env.values() {
        if c.value() != incrs { wrong += 1; }
    }
    println!("wrong values: {}", wrong);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let c: usize = args[2].parse().unwrap();
    let s: usize = args[3].parse().unwrap();
    let r: usize = args[4].parse().unwrap();
    rand_trace(n, c, s, r);
}

