
mod handoff_counter;

pub use handoff_counter::*;

#[cfg(test)]
mod tests {

    type Cnt = Counter<&'static str>;

    #[test]
    fn create_counter() {
        let mut c = Cnt::new("aa", 0);
        c.incr();
        assert!(c.value() == 1);
    }
}
