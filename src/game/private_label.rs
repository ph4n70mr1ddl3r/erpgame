use rand::Rng;
use serde::{Deserialize, Serialize};

use super::state::{ExecutivePosition, GameState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateLabelBrand {
    pub id: String,
    pub category_id: String,
    pub brand_name: String,
    pub development_progress: f64,
    pub development_quarters: i32,
    pub quarters_remaining: i32,
    pub brand_power: f64,
    pub quarterly_revenue: f64,
    pub total_revenue: f64,
    pub margin_rate: f64,
    pub development_cost: f64,
    pub quarterly_marketing_cost: f64,
    pub status: PrivateLabelStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PrivateLabelStatus {
    Developing,
    Active,
    Discontinued,
}

pub struct PrivateLabelConfig {
    pub category_id: String,
    pub category_name: String,
    pub development_cost: f64,
    pub development_quarters: i32,
    pub base_margin: f64,
    pub quarterly_marketing_cost: f64,
}

impl PrivateLabelConfig {}

pub fn get_category_configs() -> Vec<PrivateLabelConfig> {
    vec![
        PrivateLabelConfig {
            category_id: "building".into(),
            category_name: "Building Materials".into(),
            development_cost: 15_000_000.0,
            development_quarters: 4,
            base_margin: 0.40,
            quarterly_marketing_cost: 500_000.0,
        },
        PrivateLabelConfig {
            category_id: "plumbing".into(),
            category_name: "Plumbing & Pipes".into(),
            development_cost: 10_000_000.0,
            development_quarters: 3,
            base_margin: 0.45,
            quarterly_marketing_cost: 400_000.0,
        },
        PrivateLabelConfig {
            category_id: "electrical".into(),
            category_name: "Electrical & Wiring".into(),
            development_cost: 12_000_000.0,
            development_quarters: 3,
            base_margin: 0.42,
            quarterly_marketing_cost: 450_000.0,
        },
        PrivateLabelConfig {
            category_id: "paint".into(),
            category_name: "Paint & Finishes".into(),
            development_cost: 8_000_000.0,
            development_quarters: 2,
            base_margin: 0.50,
            quarterly_marketing_cost: 350_000.0,
        },
        PrivateLabelConfig {
            category_id: "tools".into(),
            category_name: "Tools & Equipment".into(),
            development_cost: 20_000_000.0,
            development_quarters: 5,
            base_margin: 0.48,
            quarterly_marketing_cost: 600_000.0,
        },
        PrivateLabelConfig {
            category_id: "garden".into(),
            category_name: "Garden & Outdoor".into(),
            development_cost: 7_000_000.0,
            development_quarters: 2,
            base_margin: 0.44,
            quarterly_marketing_cost: 300_000.0,
        },
        PrivateLabelConfig {
            category_id: "kitchen".into(),
            category_name: "Kitchen & Bath".into(),
            development_cost: 18_000_000.0,
            development_quarters: 4,
            base_margin: 0.46,
            quarterly_marketing_cost: 550_000.0,
        },
        PrivateLabelConfig {
            category_id: "hardware".into(),
            category_name: "Hardware & Fasteners".into(),
            development_cost: 5_000_000.0,
            development_quarters: 2,
            base_margin: 0.38,
            quarterly_marketing_cost: 250_000.0,
        },
    ]
}

pub fn find_config(category_id: &str) -> Option<PrivateLabelConfig> {
    get_category_configs()
        .into_iter()
        .find(|c| c.category_id == category_id)
}

pub fn start_development(state: &mut GameState, category_id: &str) -> Result<f64, &'static str> {
    if is_category_developing(state, category_id) {
        return Err(
            "A private label brand is already being developed or active for this category.",
        );
    }

    let config = find_config(category_id).ok_or("Invalid category.")?;

    if state.company.cash < config.development_cost {
        return Err("Not enough cash to start development.");
    }

    state.company.cash -= config.development_cost;

    let brand = PrivateLabelBrand {
        id: uuid::Uuid::new_v4().to_string(),
        category_id: category_id.to_string(),
        brand_name: format!("Bahay Depot {}", config.category_name),
        development_progress: 0.0,
        development_quarters: config.development_quarters,
        quarters_remaining: config.development_quarters,
        brand_power: 0.0,
        quarterly_revenue: 0.0,
        total_revenue: 0.0,
        margin_rate: config.base_margin,
        development_cost: config.development_cost,
        quarterly_marketing_cost: config.quarterly_marketing_cost,
        status: PrivateLabelStatus::Developing,
    };

    state.push_message(format!(
        "Started developing '{}' private label brand! Investment: {}. Expected launch in {} quarters.",
        brand.brand_name,
        super::state::format_currency(config.development_cost),
        config.development_quarters,
    ));

    state.private_labels.push(brand);
    Ok(config.development_cost)
}

pub fn is_category_developing(state: &GameState, category_id: &str) -> bool {
    state
        .private_labels
        .iter()
        .any(|pl| pl.category_id == category_id && pl.status != PrivateLabelStatus::Discontinued)
}

pub fn process_private_labels(state: &mut GameState, rng: &mut rand::rngs::ThreadRng) -> f64 {
    let operating_count = state.operating_store_count();
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);
    let satisfaction_factor = state.company.customer_satisfaction / 100.0;
    let reputation_factor = state.company.brand_reputation / 100.0;

    let mut total_revenue = 0.0;
    let mut total_cost = 0.0;
    let mut messages: Vec<String> = Vec::new();

    for brand in state.private_labels.iter_mut() {
        match brand.status {
            PrivateLabelStatus::Developing => {
                let step = 100.0 / brand.development_quarters as f64;
                brand.development_progress =
                    (brand.development_progress + step + rng.gen_range(-5.0..5.0)).min(100.0);
                brand.quarters_remaining -= 1;

                if brand.quarters_remaining <= 0 || brand.development_progress >= 100.0 {
                    brand.status = PrivateLabelStatus::Active;
                    brand.development_progress = 100.0;
                    brand.brand_power = 30.0;
                    messages.push(format!(
                        "[PRIVATE LABEL] '{}' has launched! The brand is now active and generating revenue.",
                        brand.brand_name
                    ));
                }
            }
            PrivateLabelStatus::Active => {
                let brand_power_growth = 2.0 + cmo_skill * 0.03 + rng.gen_range(-1.0..2.0);
                brand.brand_power = (brand.brand_power + brand_power_growth).clamp(20.0, 95.0);

                let base_revenue_per_store = 800_000.0;
                let revenue_per_store = base_revenue_per_store
                    * (brand.brand_power / 50.0)
                    * satisfaction_factor
                    * (0.8 + reputation_factor * 0.4)
                    * (1.0 + cmo_skill * 0.003);

                let store_factor = if operating_count == 0 {
                    0.0
                } else {
                    (operating_count as f64).min(20.0)
                };
                let noise = rng.gen_range(0.92..1.08);

                brand.quarterly_revenue = revenue_per_store * store_factor * noise;
                brand.quarterly_revenue = brand.quarterly_revenue.max(0.0);
                brand.total_revenue += brand.quarterly_revenue;
                total_revenue += brand.quarterly_revenue;

                total_cost += brand.quarterly_marketing_cost;

                if brand.brand_power < 25.0 && rng.gen_bool(0.15) {
                    brand.status = PrivateLabelStatus::Discontinued;
                    messages.push(format!(
                        "[PRIVATE LABEL] '{}' has been discontinued due to low brand power.",
                        brand.brand_name
                    ));
                }
            }
            PrivateLabelStatus::Discontinued => {}
        }
    }

    for msg in messages {
        state.push_message(msg);
    }

    total_revenue - total_cost
}

pub fn private_label_revenue(state: &GameState) -> f64 {
    state
        .private_labels
        .iter()
        .filter(|b| b.status == PrivateLabelStatus::Active)
        .map(|b| b.quarterly_revenue)
        .sum()
}
