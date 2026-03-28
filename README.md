# coffeetrove

Rust client for [CoffeeTrove](https://coffeetrove.com) coffee data.

Access 440K+ cafes, brewing guides, and coffee data through a simple Rust interface.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
coffeetrove = "0.1.0"
```

## Usage

```rust
use coffeetrove::{VERSION, BASE_URL};

fn main() {
    println!("CoffeeTrove client v{}", VERSION);
    println!("API: {}", BASE_URL);
}
```

## Links

- **Homepage**: [https://coffeetrove.com](https://coffeetrove.com)
- **Knowledge Base**: [https://coffeetrove.com/knowledge](https://coffeetrove.com/knowledge)
- **Repository**: [https://github.com/arnaudleroy-studio/coffeetrove-rust](https://github.com/arnaudleroy-studio/coffeetrove-rust)

## License

MIT - See [LICENSE](LICENSE) for details.
