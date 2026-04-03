use rand::Rng;
use serde::{Deserialize, Serialize};

use super::state::{ExpansionPolicy, MarketingPolicy, PricingPolicy};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competitor {
    pub name: String,
    pub store_count: u32,
    pub strength: f64,
    pub strategy: String,
    pub market_share: f64,
    pub recent_action: String,
    pub quarters_since_action: i32,
    pub aggression: f64,
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
            aggression: 0.6,
        },
        Competitor {
            name: "CW Home Depot".into(),
            store_count: 35,
            strength: 72.0,
            strategy: "Aggressive nationwide expansion".into(),
            market_share: 12.0,
            recent_action: "Launched e-commerce platform".into(),
            quarters_since_action: 0,
            aggression: 0.8,
        },
        Competitor {
            name: "AllHome".into(),
            store_count: 25,
            strength: 60.0,
            strategy: "Budget-friendly positioning".into(),
            market_share: 8.0,
            recent_action: "Offered seasonal discounts".into(),
            quarters_since_action: 0,
            aggression: 0.5,
        },
    ]
}

pub struct PlayerActions {
    pub player_market_share: f64,
    #[allow(dead_code)]
    pub player_store_count: u32,
    pub player_pricing: PricingPolicy,
    #[allow(dead_code)]
    pub player_marketing: MarketingPolicy,
    pub player_expansion: ExpansionPolicy,
    pub opened_new_store: bool,
}

pub fn update_competitors_with_actions(
    competitors: &mut [Competitor],
    rng: &mut rand::rngs::ThreadRng,
    player: &PlayerActions,
    messages: &mut VecDeque<String>,
) {
    let normal_actions = [
        "Opened a new store in Cebu City",
        "Launched a customer loyalty program",
        "Hired a new executive leadership team",
        "Invested P50M in supply chain upgrades",
        "Upgraded POS systems across all stores",
        "Entered the online retail space",
        "Partnered with major construction firms",
        "Opened a new distribution center in Laguna",
        "Closed 2 underperforming stores",
        "Acquired a smaller hardware chain",
    ];

    let defensive_actions = [
        "Launched aggressive counter-promotion against competitor",
        "Increased marketing spend to defend market share",
        "Matched competitor pricing on key product lines",
        "Opened new stores near competitor locations",
        "Enhanced loyalty rewards to retain customers",
        "Invested in major store renovations to compete",
    ];

    let aggressive_actions = [
        "Cut prices by 15% on key product categories",
        "Launched massive nationwide marketing campaign",
        "Poached senior staff from competitor",
        "Opened 3 new stores in rapid expansion",
        "Undercut competitor pricing in major cities",
        "Offered 0% installment plans to customers",
    ];

    let player_threat = player.player_market_share > 10.0 || player.opened_new_store;

    for comp in competitors.iter_mut() {
        comp.quarters_since_action += 1;
        comp.aggression = (comp.aggression + rng.gen_range(-0.05..0.05)).clamp(0.2, 1.0);

        let base_growth = if comp.aggression > 0.7 {
            rng.gen_range(0..3)
        } else {
            rng.gen_range(-1..2)
        };
        comp.store_count = (comp.store_count as i32 + base_growth).max(10) as u32;

        if player_threat && comp.aggression > 0.5 {
            comp.strength = (comp.strength + rng.gen_range(0.0..3.0)).clamp(40.0, 98.0);
        } else {
            comp.strength = (comp.strength + rng.gen_range(-3.0..4.0)).clamp(40.0, 95.0);
        }

        if player.player_market_share > 15.0 {
            comp.market_share = (comp.market_share - rng.gen_range(0.2..0.8)).max(3.0);
        } else if player.player_market_share > 10.0 {
            comp.market_share = (comp.market_share + rng.gen_range(-0.3..0.1)).max(3.0);
        } else {
            comp.market_share = (comp.market_share + rng.gen_range(-0.1..0.4)).clamp(3.0, 25.0);
        }

        if player_threat
            && matches!(player.player_pricing, PricingPolicy::Budget)
            && comp.aggression > 0.6
            && rng.gen_bool(0.4)
        {
            let action = aggressive_actions[rng.gen_range(0..aggressive_actions.len())];
            comp.recent_action = action.to_string();
            comp.quarters_since_action = 0;
            if rng.gen_bool(0.4) {
                messages.push_back(format!("[INTEL] {}: {}", comp.name, comp.recent_action));
            }
            continue;
        }

        if player_threat
            && matches!(
                player.player_expansion,
                ExpansionPolicy::Blitz | ExpansionPolicy::Aggressive
            )
            && comp.aggression > 0.6
            && rng.gen_bool(comp.aggression * 0.4)
        {
            let action = defensive_actions[rng.gen_range(0..defensive_actions.len())];
            comp.recent_action = action.to_string();
            comp.quarters_since_action = 0;
            if rng.gen_bool(0.4) {
                messages.push_back(format!("[INTEL] {}: {}", comp.name, comp.recent_action));
            }
            continue;
        }

        if rng.gen_bool(0.25) {
            let action = if player_threat && comp.aggression > 0.7 {
                let pool = if rng.gen_bool(0.5) {
                    &aggressive_actions[..]
                } else {
                    &normal_actions[..]
                };
                pool[rng.gen_range(0..pool.len())]
            } else {
                normal_actions[rng.gen_range(0..normal_actions.len())]
            };
            comp.recent_action = action.to_string();
            comp.quarters_since_action = 0;
            if rng.gen_bool(0.3) {
                messages.push_back(format!("[INTEL] {}: {}", comp.name, comp.recent_action));
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
