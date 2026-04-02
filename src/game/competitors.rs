use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competitor {
    pub name: String,
    pub store_count: u32,
    pub strength: f64,
    pub strategy: String,
    pub market_share: f64,
    pub recent_action: String,
    pub quarters_since_action: i32,
}

pub fn default_competitors() -> Vec<Competitor> {
    vec![
        Competitor {
            name: "Wilcon Depot".into(),
            store_count: 45,
            strength: 85.0,
            strategy: "Premium stores in major cities".into(),
            market_share: 18.0,
            recent_action: "Opened 3 new Mega stores".into(),
            quarters_since_action: 0,
        },
        Competitor {
            name: "CW Home Depot".into(),
            store_count: 35,
            strength: 72.0,
            strategy: "Aggressive nationwide expansion".into(),
            market_share: 12.0,
            recent_action: "Launched e-commerce platform".into(),
            quarters_since_action: 0,
        },
        Competitor {
            name: "AllHome".into(),
            store_count: 25,
            strength: 60.0,
            strategy: "Budget-friendly positioning".into(),
            market_share: 8.0,
            recent_action: "Offered seasonal discounts".into(),
            quarters_since_action: 0,
        },
    ]
}

pub fn update_competitors(
    competitors: &mut [Competitor],
    rng: &mut rand::rngs::ThreadRng,
    player_market_share: f64,
    messages: &mut Vec<String>,
) {
    let actions = [
        "Opened a new store in Cebu City",
        "Launched a customer loyalty program",
        "Hired a new executive leadership team",
        "Reduced prices on key product categories",
        "Invested P50M in supply chain upgrades",
        "Launched a nationwide marketing campaign",
        "Upgraded POS systems across all stores",
        "Entered the online retail space",
        "Partnered with major construction firms",
        "Opened a new distribution center in Laguna",
        "Closed 2 underperforming stores",
        "Acquired a smaller hardware chain",
    ];

    for comp in competitors.iter_mut() {
        comp.store_count = (comp.store_count as i32 + rng.gen_range(-1..3)).max(10) as u32;
        comp.strength = (comp.strength + rng.gen_range(-3.0..4.0)).clamp(40.0, 95.0);
        comp.quarters_since_action += 1;

        if player_market_share > 15.0 {
            comp.market_share = (comp.market_share - rng.gen_range(0.1..0.5)).max(3.0);
        } else if player_market_share > 10.0 {
            comp.market_share = (comp.market_share + rng.gen_range(-0.3..0.1)).max(3.0);
        } else {
            comp.market_share = (comp.market_share + rng.gen_range(-0.2..0.3)).clamp(3.0, 25.0);
        }

        if rng.gen_bool(0.25) {
            let action = actions[rng.gen_range(0..actions.len())];
            comp.recent_action = action.to_string();
            comp.quarters_since_action = 0;
            if rng.gen_bool(0.3) {
                messages.push(format!("[INTEL] {}: {}", comp.name, comp.recent_action));
            }
        }
    }
}

pub fn total_competitor_market_share(competitors: &[Competitor]) -> f64 {
    competitors.iter().map(|c| c.market_share).sum()
}

pub fn average_competitor_strength(competitors: &[Competitor]) -> f64 {
    if competitors.is_empty() {
        return 50.0;
    }
    competitors.iter().map(|c| c.strength).sum::<f64>() / competitors.len() as f64
}
