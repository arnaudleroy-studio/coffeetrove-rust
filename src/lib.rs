//! # coffeetrove
//!
//! Data models and brewing utilities for the CoffeeTrove coffee discovery
//! platform. 440,000+ cafes, brewing guides, origin profiles, and Golden
//! Drop scoring.
//!
//! Homepage: <https://coffeetrove.com>

/// Library version.
pub const VERSION: &str = "0.1.2";

/// Base URL for CoffeeTrove.
pub const BASE_URL: &str = "https://coffeetrove.com";

// ---------------------------------------------------------------------------
// Brewing methods
// ---------------------------------------------------------------------------

/// A brewing method with recommended parameters.
#[derive(Debug, Clone, Copy)]
pub struct BrewingMethod {
    pub name: &'static str,
    pub temp_min_c: u8,
    pub temp_max_c: u8,
    pub brew_seconds: u16,
    pub grind: &'static str,
}

/// Built-in table of common brewing methods.
pub const BREWING_METHODS: &[BrewingMethod] = &[
    BrewingMethod { name: "Pour Over",     temp_min_c: 90, temp_max_c: 96, brew_seconds: 210, grind: "Medium-Fine" },
    BrewingMethod { name: "French Press",  temp_min_c: 93, temp_max_c: 96, brew_seconds: 240, grind: "Coarse" },
    BrewingMethod { name: "Espresso",      temp_min_c: 90, temp_max_c: 94, brew_seconds: 28,  grind: "Fine" },
    BrewingMethod { name: "AeroPress",     temp_min_c: 80, temp_max_c: 92, brew_seconds: 120, grind: "Medium" },
    BrewingMethod { name: "Cold Brew",     temp_min_c: 2,  temp_max_c: 8,  brew_seconds: 43200, grind: "Extra Coarse" },
    BrewingMethod { name: "Moka Pot",      temp_min_c: 90, temp_max_c: 95, brew_seconds: 300, grind: "Fine" },
    BrewingMethod { name: "Turkish",       temp_min_c: 90, temp_max_c: 98, brew_seconds: 180, grind: "Extra Fine" },
    BrewingMethod { name: "Chemex",        temp_min_c: 90, temp_max_c: 96, brew_seconds: 240, grind: "Medium-Coarse" },
    BrewingMethod { name: "Siphon",        temp_min_c: 90, temp_max_c: 96, brew_seconds: 150, grind: "Medium" },
];

/// Look up the recommended grind size for a brewing method (case-insensitive).
pub fn recommend_grind(method: &str) -> Option<&'static str> {
    let lower = method.to_lowercase();
    BREWING_METHODS
        .iter()
        .find(|m| m.name.to_lowercase() == lower)
        .map(|m| m.grind)
}

// ---------------------------------------------------------------------------
// Origins
// ---------------------------------------------------------------------------

/// Major coffee-producing origins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    Ethiopia,
    Colombia,
    Brazil,
    Guatemala,
    Kenya,
    CostaRica,
    Indonesia,
    Vietnam,
    Jamaica,
    Yemen,
}

impl Origin {
    /// Country name.
    pub fn country(self) -> &'static str {
        match self {
            Self::Ethiopia  => "Ethiopia",
            Self::Colombia  => "Colombia",
            Self::Brazil    => "Brazil",
            Self::Guatemala => "Guatemala",
            Self::Kenya     => "Kenya",
            Self::CostaRica => "Costa Rica",
            Self::Indonesia => "Indonesia",
            Self::Vietnam   => "Vietnam",
            Self::Jamaica   => "Jamaica",
            Self::Yemen     => "Yemen",
        }
    }

    /// Typical growing altitude range in metres.
    pub fn altitude_range(self) -> (u16, u16) {
        match self {
            Self::Ethiopia  => (1500, 2200),
            Self::Colombia  => (1200, 2000),
            Self::Brazil    => (800, 1400),
            Self::Guatemala => (1300, 1800),
            Self::Kenya     => (1400, 2100),
            Self::CostaRica => (1200, 1800),
            Self::Indonesia => (900, 1700),
            Self::Vietnam   => (500, 1600),
            Self::Jamaica   => (900, 1500),
            Self::Yemen     => (1500, 2500),
        }
    }

    /// Common tasting notes associated with this origin.
    pub fn tasting_notes(self) -> &'static str {
        match self {
            Self::Ethiopia  => "floral, blueberry, citrus",
            Self::Colombia  => "caramel, nutty, balanced",
            Self::Brazil    => "chocolate, nutty, low acidity",
            Self::Guatemala => "cocoa, spice, full body",
            Self::Kenya     => "blackcurrant, bright acidity, wine",
            Self::CostaRica => "honey, citrus, clean",
            Self::Indonesia => "earthy, herbal, heavy body",
            Self::Vietnam   => "bold, dark chocolate, smoky",
            Self::Jamaica   => "mild, sweet, floral",
            Self::Yemen     => "winey, dried fruit, complex",
        }
    }
}

// ---------------------------------------------------------------------------
// Cafe
// ---------------------------------------------------------------------------

/// Chain classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainType {
    /// Global chains (e.g. Starbucks).
    Global,
    /// Regional or local chains.
    Local,
    /// Independent single-location cafes.
    Independent,
}

/// A cafe record.
#[derive(Debug, Clone)]
pub struct Cafe {
    pub slug: String,
    pub name: String,
    pub city: Option<String>,
    pub country: Option<String>,
    pub chain_type: ChainType,
    pub rating: Option<f32>,
}

/// Builder for [`Cafe`].
pub struct CafeBuilder {
    slug: String,
    name: String,
    city: Option<String>,
    country: Option<String>,
    chain_type: ChainType,
    rating: Option<f32>,
}

impl Cafe {
    /// Start building a cafe record.
    pub fn builder(slug: &str, name: &str) -> CafeBuilder {
        CafeBuilder {
            slug: slug.to_owned(),
            name: name.to_owned(),
            city: None,
            country: None,
            chain_type: ChainType::Independent,
            rating: None,
        }
    }

    /// Compute the Golden Drop score (0--100).
    ///
    /// Points are awarded for data completeness: city, country, rating,
    /// and chain type each contribute. Independent cafes receive a bonus.
    pub fn golden_drop_score(&self) -> u8 {
        let mut score: u8 = 20; // base
        if self.city.is_some() { score += 15; }
        if self.country.is_some() { score += 10; }
        if self.rating.is_some() { score += 15; }
        if self.chain_type == ChainType::Independent { score += 10; }
        // Rating quality bonus
        if let Some(r) = self.rating {
            if r >= 4.0 { score += 10; }
            if r >= 4.5 { score += 10; }
        }
        score.min(100)
    }
}

impl CafeBuilder {
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }

    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }

    pub fn chain_type(mut self, ct: ChainType) -> Self {
        self.chain_type = ct;
        self
    }

    pub fn rating(mut self, rating: f32) -> Self {
        self.rating = Some(rating);
        self
    }

    pub fn build(self) -> Cafe {
        Cafe {
            slug: self.slug,
            name: self.name,
            city: self.city,
            country: self.country,
            chain_type: self.chain_type,
            rating: self.rating,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_drop_independent() {
        let cafe = Cafe::builder("test", "Test Cafe")
            .city("Berlin")
            .country("DE")
            .rating(4.6)
            .build();
        assert!(cafe.golden_drop_score() >= 70);
    }

    #[test]
    fn grind_lookup() {
        assert_eq!(recommend_grind("French Press"), Some("Coarse"));
        assert_eq!(recommend_grind("nonexistent"), None);
    }

    #[test]
    fn origin_data() {
        let (lo, hi) = Origin::Ethiopia.altitude_range();
        assert!(lo < hi);
    }
}
