# coffeetrove

[![Crates.io](https://img.shields.io/crates/v/coffeetrove.svg)](https://crates.io/crates/coffeetrove)
[![docs.rs](https://img.shields.io/docsrs/coffeetrove)](https://docs.rs/coffeetrove)

Data models and brewing utilities for the CoffeeTrove coffee discovery platform. CoffeeTrove indexes over 440,000 cafes worldwide alongside brewing guides, origin profiles, and a Golden Drop scoring system that rates cafes on data completeness and quality signals.

## Installation

```toml
[dependencies]
coffeetrove = "0.1.2"
```

## Usage

### Cafe Records

The `Cafe` struct mirrors the fields stored in the CoffeeTrove database. Construct records with the builder pattern and compute their Golden Drop score:

```rust
use coffeetrove::{Cafe, ChainType};

let cafe = Cafe::builder("blue-bottle-hayes", "Blue Bottle Coffee")
    .city("San Francisco")
    .country("US")
    .chain_type(ChainType::Local)
    .rating(4.5)
    .build();

println!("{} -- score {}", cafe.name, cafe.golden_drop_score());
```

### Brewing Method Lookup

A const table of common brewing methods ships with the crate. Each entry includes the recommended water temperature range and typical brew time in seconds:

```rust
use coffeetrove::BREWING_METHODS;

for method in BREWING_METHODS {
    println!(
        "{}: {}--{}C, {} min",
        method.name,
        method.temp_min_c,
        method.temp_max_c,
        method.brew_seconds / 60
    );
}
```

### Origin Profiles

Use the `Origin` enum when you need type-safe representation of coffee-producing regions:

```rust
use coffeetrove::Origin;

let origin = Origin::Ethiopia;
println!(
    "{} -- altitude {}--{}m, notes: {}",
    origin.country(),
    origin.altitude_range().0,
    origin.altitude_range().1,
    origin.tasting_notes()
);
```

### Grind Size Estimation

Convert a brewing method name into a recommended grind level:

```rust
use coffeetrove::recommend_grind;

let grind = recommend_grind("French Press");
assert_eq!(grind, Some("Coarse"));
```

## Available Data

CoffeeTrove covers 440,000+ cafes across 195 countries, 15 brewing method guides, 23 bean origin profiles, 17 coffee drink recipes, and 15 equipment reviews. The Golden Drop scoring algorithm evaluates cafes on a 0--100 scale using data completeness, independence bonus (non-chain cafes score higher), and user review signals. Cafes are classified into three tiers: global chains, local chains, and independent shops. Each cafe record includes geolocation, opening hours where available, and a normalized rating that allows cross-region comparison.

## Links

- [CoffeeTrove](https://coffeetrove.com) -- discover cafes, origins, and brewing guides
- [Source Code](https://github.com/arnaudleroy-studio/coffeetrove-rust)

## License

MIT
