use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategory {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub margin_rate: f64,
    pub demand_factor: f64,
    pub investment_level: f64,
    pub trend: f64,
}

impl ProductCategory {
    pub fn margin_modifier(&self) -> f64 {
        self.margin_rate * (0.8 + (self.investment_level / 100.0) * 0.4)
    }

    pub fn investment_display(&self) -> String {
        format!("{:.0}%", self.investment_level)
    }

    pub fn margin_display(&self) -> String {
        format!("{:.1}%", self.margin_modifier() * 100.0)
    }

    pub fn demand_display(&self) -> String {
        format!("{:.2}x", self.demand_factor)
    }

    pub fn trend_display(&self) -> String {
        if self.trend > 0.02 {
            format!("+{:.1}%", self.trend * 100.0)
        } else if self.trend < -0.02 {
            format!("{:.1}%", self.trend * 100.0)
        } else {
            "Stable".into()
        }
    }

    pub fn trend_class(&self) -> &'static str {
        if self.trend > 0.02 {
            "text-green-400"
        } else if self.trend < -0.02 {
            "text-red-400"
        } else {
            "text-yellow-400"
        }
    }
}

pub fn default_product_categories() -> Vec<ProductCategory> {
    vec![
        ProductCategory {
            id: "building".into(),
            name: "Building Materials".into(),
            icon: "Hammer".into(),
            margin_rate: 0.25,
            demand_factor: 1.2,
            investment_level: 30.0,
            trend: 0.0,
        },
        ProductCategory {
            id: "plumbing".into(),
            name: "Plumbing & Pipes".into(),
            icon: "Droplets".into(),
            margin_rate: 0.30,
            demand_factor: 0.9,
            investment_level: 25.0,
            trend: 0.0,
        },
        ProductCategory {
            id: "electrical".into(),
            name: "Electrical & Wiring".into(),
            icon: "Zap".into(),
            margin_rate: 0.32,
            demand_factor: 0.85,
            investment_level: 20.0,
            trend: 0.0,
        },
        ProductCategory {
            id: "paint".into(),
            name: "Paint & Finishes".into(),
            icon: "Palette".into(),
            margin_rate: 0.35,
            demand_factor: 1.1,
            investment_level: 35.0,
            trend: 0.0,
        },
        ProductCategory {
            id: "tools".into(),
            name: "Tools & Equipment".into(),
            icon: "Wrench".into(),
            margin_rate: 0.40,
            demand_factor: 0.7,
            investment_level: 25.0,
            trend: 0.0,
        },
        ProductCategory {
            id: "garden".into(),
            name: "Garden & Outdoor".into(),
            icon: "Flower2".into(),
            margin_rate: 0.30,
            demand_factor: 0.6,
            investment_level: 15.0,
            trend: 0.0,
        },
        ProductCategory {
            id: "kitchen".into(),
            name: "Kitchen & Bath".into(),
            icon: "CookingPot".into(),
            margin_rate: 0.38,
            demand_factor: 0.65,
            investment_level: 20.0,
            trend: 0.0,
        },
        ProductCategory {
            id: "hardware".into(),
            name: "Hardware & Fasteners".into(),
            icon: "Cog".into(),
            margin_rate: 0.22,
            demand_factor: 1.3,
            investment_level: 40.0,
            trend: 0.0,
        },
    ]
}

pub fn update_product_categories(
    categories: &mut [ProductCategory],
    rng: &mut rand::rngs::ThreadRng,
    construction_index: f64,
    quarter: i32,
) {
    for cat in categories.iter_mut() {
        cat.trend = (cat.trend + rng.gen_range(-0.02..0.02)).clamp(-0.1, 0.1);

        if cat.id == "building" {
            cat.demand_factor = 0.8 + (construction_index / 100.0) * 0.8;
        }

        if cat.id == "garden" {
            let seasonal = match quarter {
                1 => 0.5,
                2 => 0.9,
                3 => 1.0,
                4 => 0.6,
                _ => 0.7,
            };
            cat.demand_factor = seasonal + rng.gen_range(-0.05..0.05);
        }

        if cat.id == "paint" && quarter == 4 {
            cat.demand_factor = 1.3 + rng.gen_range(-0.1..0.1);
        }

        if cat.id != "building" && cat.id != "garden" && cat.id != "paint" {
            cat.demand_factor = (cat.demand_factor + rng.gen_range(-0.03..0.03)).clamp(0.3, 1.5);
        }

        cat.investment_level = (cat.investment_level - 0.5).max(0.0);
    }
}

pub fn total_product_revenue_modifier(categories: &[ProductCategory]) -> f64 {
    let avg_demand: f64 =
        categories.iter().map(|c| c.demand_factor).sum::<f64>() / categories.len().max(1) as f64;
    let avg_investment: f64 =
        categories.iter().map(|c| c.investment_level).sum::<f64>() / categories.len().max(1) as f64;
    0.85 + (avg_demand - 0.9) * 0.15 + (avg_investment / 100.0) * 0.2
}

pub fn total_product_margin_modifier(categories: &[ProductCategory]) -> f64 {
    let avg_investment: f64 =
        categories.iter().map(|c| c.investment_level).sum::<f64>() / categories.len().max(1) as f64;
    0.9 + (avg_investment / 100.0) * 0.15
}

pub fn invest_in_category(
    categories: &mut [ProductCategory],
    category_id: &str,
    amount: f64,
) -> (f64, f64) {
    if let Some(cat) = categories.iter_mut().find(|c| c.id == category_id) {
        let headroom = 100.0 - cat.investment_level;
        if headroom <= 0.0 {
            return (0.0, 0.0);
        }
        let desired_increase = (amount / 500_000.0).min(5.0);
        let actual_increase = desired_increase.min(headroom);
        let actual_cost = (actual_increase / 5.0) * amount.min(5.0 * 500_000.0).min(amount);
        cat.investment_level = (cat.investment_level + actual_increase).min(100.0);
        (actual_increase, actual_cost)
    } else {
        (0.0, 0.0)
    }
}
