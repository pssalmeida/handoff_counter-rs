# crdt

Handoff Counters in Rust - eventually consistent distributed counters, as presented in
http://arxiv.org/abs/1307.3207


## Usage

To use Handoff Counters first add this to your `Cargo.toml`:

```toml
[dependencies]
handoff_counter = "0.8"
```

Then, add this to your crate root:

```rust
extern crate handoff_counter;
```

