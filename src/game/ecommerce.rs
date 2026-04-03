use serde::{Deserialize, Serialize};

use super::state::GameState;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EcommerceLevel {
    None,
    BasicWebsite,
    OnlineStore,
    FullEcommerce,
    Omnichannel,
}

impl EcommerceLevel {
    pub fn label(&self) -> &str {
        match self {
            EcommerceLevel::None => "No Online Presence",
            EcommerceLevel::BasicWebsite => "Basic Website",
            EcommerceLevel::OnlineStore => "Online Store",
            EcommerceLevel::FullEcommerce => "Full E-commerce",
            EcommerceLevel::Omnichannel => "Omnichannel",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            EcommerceLevel::None => "none",
            EcommerceLevel::BasicWebsite => "basic_website",
            EcommerceLevel::OnlineStore => "online_store",
            EcommerceLevel::FullEcommerce => "full_ecommerce",
            EcommerceLevel::Omnichannel => "omnichannel",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            EcommerceLevel::None => "X",
            EcommerceLevel::BasicWebsite => "Globe",
            EcommerceLevel::OnlineStore => "ShoppingCart",
            EcommerceLevel::FullEcommerce => "Package",
            EcommerceLevel::Omnichannel => "Smartphone",
        }
    }

    pub fn color_class(&self) -> &str {
        match self {
            EcommerceLevel::None => "text-gray-400",
            EcommerceLevel::BasicWebsite => "text-blue-400",
            EcommerceLevel::OnlineStore => "text-green-400",
            EcommerceLevel::FullEcommerce => "text-yellow-400",
            EcommerceLevel::Omnichannel => "text-purple-400",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            EcommerceLevel::None => "No online presence. All sales are in-store only.",
            EcommerceLevel::BasicWebsite => {
                "A simple website with product catalog, store locations, and contact info. Builds online visibility."
            }
            EcommerceLevel::OnlineStore => {
                "Full online store with product listings, cart, checkout, and delivery. Tap into the growing digital market."
            }
            EcommerceLevel::FullEcommerce => {
                "Comprehensive e-commerce platform with wide product range, real-time inventory, multiple payment options, and nationwide delivery."
            }
            EcommerceLevel::Omnichannel => {
                "Seamless online + offline integration: buy online pick up in-store, same-day delivery, app, AR room planner, and contractor portal."
            },
        }
    }

    pub fn setup_cost(&self) -> f64 {
        match self {
            EcommerceLevel::None => 0.0,
            EcommerceLevel::BasicWebsite => 3_000_000.0,
            EcommerceLevel::OnlineStore => 12_000_000.0,
            EcommerceLevel::FullEcommerce => 30_000_000.0,
            EcommerceLevel::Omnichannel => 60_000_000.0,
        }
    }

    pub fn quarterly_cost(&self) -> f64 {
        match self {
            EcommerceLevel::None => 0.0,
            EcommerceLevel::BasicWebsite => 500_000.0,
            EcommerceLevel::OnlineStore => 2_000_000.0,
            EcommerceLevel::FullEcommerce => 5_000_000.0,
            EcommerceLevel::Omnichannel => 12_000_000.0,
        }
    }

    pub fn revenue_bonus(&self) -> f64 {
        match self {
            EcommerceLevel::None => 0.0,
            EcommerceLevel::BasicWebsite => 0.02,
            EcommerceLevel::OnlineStore => 0.07,
            EcommerceLevel::FullEcommerce => 0.14,
            EcommerceLevel::Omnichannel => 0.22,
        }
    }

    pub fn satisfaction_bonus(&self) -> f64 {
        match self {
            EcommerceLevel::None => 0.0,
            EcommerceLevel::BasicWebsite => 0.5,
            EcommerceLevel::OnlineStore => 1.5,
            EcommerceLevel::FullEcommerce => 2.5,
            EcommerceLevel::Omnichannel => 4.0,
        }
    }

    pub fn reputation_bonus(&self) -> f64 {
        match self {
            EcommerceLevel::None => 0.0,
            EcommerceLevel::BasicWebsite => 0.5,
            EcommerceLevel::OnlineStore => 1.0,
            EcommerceLevel::FullEcommerce => 2.0,
            EcommerceLevel::Omnichannel => 3.0,
        }
    }

    pub fn min_stores(&self) -> u32 {
        match self {
            EcommerceLevel::None => 0,
            EcommerceLevel::BasicWebsite => 1,
            EcommerceLevel::OnlineStore => 2,
            EcommerceLevel::FullEcommerce => 4,
            EcommerceLevel::Omnichannel => 8,
        }
    }

    pub fn all_levels() -> Vec<EcommerceLevel> {
        vec![
            EcommerceLevel::None,
            EcommerceLevel::BasicWebsite,
            EcommerceLevel::OnlineStore,
            EcommerceLevel::FullEcommerce,
            EcommerceLevel::Omnichannel,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "none" => Some(EcommerceLevel::None),
            "basic_website" => Some(EcommerceLevel::BasicWebsite),
            "online_store" => Some(EcommerceLevel::OnlineStore),
            "full_ecommerce" => Some(EcommerceLevel::FullEcommerce),
            "omnichannel" => Some(EcommerceLevel::Omnichannel),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommerceChannel {
    pub level: EcommerceLevel,
    pub quarters_active: i32,
    pub quarterly_online_revenue: f64,
    pub total_online_revenue: f64,
    pub conversion_rate: f64,
}

impl EcommerceChannel {
    pub fn new() -> Self {
        EcommerceChannel {
            level: EcommerceLevel::None,
            quarters_active: 0,
            quarterly_online_revenue: 0.0,
            total_online_revenue: 0.0,
            conversion_rate: 0.0,
        }
    }
}

impl Default for EcommerceChannel {
    fn default() -> Self {
        Self::new()
    }
}

pub fn upgrade_ecommerce(
    state: &mut GameState,
    new_level: EcommerceLevel,
) -> Result<f64, &'static str> {
    if new_level == state.ecommerce.level {
        return Err("Already at this level.");
    }

    if new_level == EcommerceLevel::None {
        return downgrade_ecommerce(state);
    }

    let current_ord = state.ecommerce.level as u8;
    let new_ord = new_level as u8;
    if new_ord != current_ord + 1 {
        return Err("You can only upgrade one level at a time.");
    }

    let operating = state.operating_store_count();
    if operating < new_level.min_stores() {
        return Err("Not enough operating stores for this level.");
    }

    let cost = new_level.setup_cost();
    if state.company.cash < cost {
        return Err("Not enough cash.");
    }

    state.company.cash -= cost;
    state.ecommerce.level = new_level;
    state.ecommerce.quarters_active = 0;

    if new_level == EcommerceLevel::None {
        state.ecommerce.quarterly_online_revenue = 0.0;
        state.ecommerce.conversion_rate = 0.0;
        state.push_message("E-commerce channel shut down. Online presence discontinued.".into());
    } else {
        state.push_message(format!(
            "Launched {} e-commerce channel for {}! Online sales will begin next quarter.",
            new_level.label(),
            super::state::format_currency(cost)
        ));
    }

    Ok(cost)
}

fn downgrade_ecommerce(state: &mut GameState) -> Result<f64, &'static str> {
    state.ecommerce.level = EcommerceLevel::None;
    state.ecommerce.quarters_active = 0;
    state.ecommerce.quarterly_online_revenue = 0.0;
    state.ecommerce.conversion_rate = 0.0;
    state.push_message("E-commerce channel shut down. Online presence discontinued.".into());
    Ok(0.0)
}

pub fn process_ecommerce(state: &mut GameState) -> f64 {
    let channel = &mut state.ecommerce;

    if channel.level == EcommerceLevel::None {
        channel.quarterly_online_revenue = 0.0;
        return 0.0;
    }

    channel.quarters_active += 1;

    let cto_skill = state
        .executives
        .iter()
        .find(|e| e.position == super::state::ExecutivePosition::CTO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let cto_factor = 1.0 + cto_skill * 0.005;

    let sat_factor = state.company.customer_satisfaction / 100.0;

    let store_revenue: f64 = state
        .stores
        .iter()
        .filter(|s| s.status == super::state::StoreStatus::Operating)
        .map(|s| s.quarterly_revenue)
        .sum();

    let base_rev = store_revenue * channel.level.revenue_bonus();
    let ramp_up = if channel.quarters_active <= 4 {
        0.5 + channel.quarters_active as f64 * 0.125
    } else {
        1.0
    };

    let marketing_factor = match state.policies.marketing {
        super::state::MarketingPolicy::LowKey => 0.8,
        super::state::MarketingPolicy::Moderate => 1.0,
        super::state::MarketingPolicy::Heavy => 1.15,
        super::state::MarketingPolicy::Aggressive => 1.3,
    };

    let online_revenue = (base_rev * cto_factor * sat_factor * ramp_up * marketing_factor).max(0.0);
    channel.quarterly_online_revenue = online_revenue;
    channel.total_online_revenue += online_revenue;

    channel.conversion_rate = if online_revenue > 0.0 {
        (2.0 + channel.level.revenue_bonus() * 30.0 + cto_skill * 0.03) * sat_factor * ramp_up
    } else {
        0.0
    };

    let maintenance_cost = channel.level.quarterly_cost();

    state.company.customer_satisfaction = (state.company.customer_satisfaction
        + channel.level.satisfaction_bonus() * 0.1)
        .clamp(10.0, 100.0);
    state.company.brand_reputation = (state.company.brand_reputation
        + channel.level.reputation_bonus() * 0.05)
        .clamp(10.0, 100.0);

    maintenance_cost
}
