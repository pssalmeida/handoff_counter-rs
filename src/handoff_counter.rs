use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::max;

#[derive(Clone, Debug)]
pub struct Counter<Id: Hash + Eq + Copy> {
    id: Id,
    tier: usize,
    val: u64,
    below: u64,
    vals: HashMap<Id, u64>,
    sck: u64,
    dck: u64,
    slots: HashMap<Id, (u64, u64)>,
    tokens: HashMap<(Id, Id), (u64, u64, u64)>
}

impl<Id: Hash + Eq + Copy> Counter<Id> {

    pub fn new(id: Id, tier: usize) -> Counter<Id> {
        let mut c = Counter {
            id: id,
            tier: tier,
            val: 0,
            below: 0,
            vals: if tier == 0 { HashMap::with_capacity(8) }
                  else { HashMap::with_capacity(1) },
            sck: 0,
            dck: 0,
            slots: HashMap::new(),
            tokens: HashMap::with_capacity(1)
        };
        c.vals.insert(id, 0);
        c
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn tier(&self) -> usize {
        self.tier
    }

    pub fn value(&self) -> u64 {
        self.val
    }

    pub fn slots(&self) -> &HashMap<Id, (u64, u64)> {
        &self.slots
    }

    pub fn incr(&mut self) {
        self.val += 1;
        let mut v = self.vals.get_mut(&self.id).unwrap();
        *v += 1;
    }

    pub fn view(&self, id: Id, tier: usize) -> Self {
        let mut slots: HashMap<Id, (u64, u64)> = HashMap::with_capacity(1);
        if self.tier < tier {
            match self.slots.get(&id) {
                Some(&v) => { slots.insert(id, v); }
                None => {}
            }
        } else if self.tier == tier {
            slots = self.slots.clone();
        }
        Counter {
            id : self.id,
            tier: self.tier,
            val: self.val,
            below: self.below,
            vals: self.vals.clone(),
            sck: self.sck,
            dck: self.dck,
            slots: slots,
            tokens: self.tokens.clone(),
        }
    }

    pub fn merge(&mut self, other: &Self) {
        self.fill_slots(other);
        self.discard_slot(other);
        self.create_slot(other);
        self.merge_vectors(other);
        self.aggregate(other);
        self.discard_tokens(other);
        self.create_token(other);
        self.cache_tokens(other);
    }

    fn fill_slots(&mut self, other: &Self) {
        let v = self.vals.get_mut(&self.id).unwrap();
        for (&(src, dst), &(sck, dck, n)) in &other.tokens {
            if dst == self.id {
                if let Some(&(s,d)) = self.slots.get(&src) {
                    if (s,d) == (sck, dck) {
                        *v += n;
                        self.slots.remove(&src);
                    }
                };
            }
        }
    }

    fn discard_slot(&mut self, other: &Self) {
        if let Some(&(sck, _)) = self.slots.get(&other.id) {
            if other.sck > sck {
                self.slots.remove(&other.id);
            }
        }
    }

    fn create_slot(&mut self, other: &Self) {
        if self.tier < other.tier &&
           *other.vals.get(&other.id).unwrap() > 0 &&
           !self.slots.contains_key(&other.id) {
               self.slots.insert(other.id, (other.sck, self.dck));
               self.dck += 1;
        }
    }

    fn merge_vectors(&mut self, other: &Self) {
        if self.tier == 0 && other.tier == 0 {
            for (&k2,&v2) in &other.vals {
                let v = self.vals.entry(k2).or_insert(0);
                *v = max(*v, v2);
            }
        }
    }

    fn aggregate(&mut self, other: &Self) {
        let b = if self.tier == other.tier {
            max(self.below, other.below)
        } else if self.tier > other.tier {
            max(self.below, other.val)
        } else {
            self.below
        };
        let v = if self.tier == 0 {
            self.vals.values().fold(0, |s,&v| s+v)
        } else if self.tier == other.tier {
            max(max(self.val, other.val), b +
                self.vals.get(&self.id).unwrap() +
                other.vals.get(&other.id).unwrap())
        } else {
            max(self.val, b + self.vals.get(&self.id).unwrap())
        };
        self.below = b;
        self.val = v;
    }

    fn discard_tokens(&mut self, other: &Self) {
        let tok =
            self.tokens.drain().filter(|&((src, dst), (_, dck, _))| {
                !(dst == other.id &&
                  match other.slots.get(&src) {
                      Some(&(_, d)) => d > dck,
                      None => other.dck > dck
                  })
            }).collect();
        self.tokens = tok;
    }

    fn create_token(&mut self, other: &Self) {
        match other.slots.get(&self.id) {
            Some(&(s, d)) if s == self.sck => {
                let v = self.vals.insert(self.id, 0).unwrap();
                self.tokens.insert((self.id, other.id), (s, d, v));
                self.sck += 1;
            }
            _ => {}
        }
    }

    fn cache_tokens(&mut self, other: &Self) {
        if self.tier < other.tier {
            for (&(src,dst), &v) in &other.tokens {
                if src == other.id && dst != self.id {
                    let e = self.tokens.entry((src, dst)).or_insert(v);
                    if e.0 < v.0 {
                        *e = v;
                    }
                }
            }
        }
    }

}

