use serde::{Deserialize, Serialize};

use super::state::GameState;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoyaltyTier {
    None,
    Basic,
    Silver,
    Gold,
    Platinum,
}

impl LoyaltyTier {
    pub fn label(&self) -> &str {
        match self {
            LoyaltyTier::None => "No Program",
            LoyaltyTier::Basic => "Basic",
            LoyaltyTier::Silver => "Silver",
            LoyaltyTier::Gold => "Gold",
            LoyaltyTier::Platinum => "Platinum",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            LoyaltyTier::None => "none",
            LoyaltyTier::Basic => "basic",
            LoyaltyTier::Silver => "silver",
            LoyaltyTier::Gold => "gold",
            LoyaltyTier::Platinum => "platinum",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            LoyaltyTier::None => "X",
            LoyaltyTier::Basic => "Star",
            LoyaltyTier::Silver => "Award",
            LoyaltyTier::Gold => "Crown",
            LoyaltyTier::Platinum => "Gem",
        }
    }

    pub fn color_class(&self) -> &str {
        match self {
            LoyaltyTier::None => "text-gray-400",
            LoyaltyTier::Basic => "text-blue-400",
            LoyaltyTier::Silver => "text-slate-300",
            LoyaltyTier::Gold => "text-yellow-400",
            LoyaltyTier::Platinum => "text-purple-400",
        }
    }

    pub fn setup_cost(&self) -> f64 {
        match self {
            LoyaltyTier::None => 0.0,
            LoyaltyTier::Basic => 2_000_000.0,
            LoyaltyTier::Silver => 5_000_000.0,
            LoyaltyTier::Gold => 10_000_000.0,
            LoyaltyTier::Platinum => 20_000_000.0,
        }
    }

    pub fn quarterly_cost_per_store(&self) -> f64 {
        match self {
            LoyaltyTier::None => 0.0,
            LoyaltyTier::Basic => 150_000.0,
            LoyaltyTier::Silver => 350_000.0,
            LoyaltyTier::Gold => 700_000.0,
            LoyaltyTier::Platinum => 1_200_000.0,
        }
    }

    pub fn revenue_bonus(&self) -> f64 {
        match self {
            LoyaltyTier::None => 0.0,
            LoyaltyTier::Basic => 0.03,
            LoyaltyTier::Silver => 0.06,
            LoyaltyTier::Gold => 0.10,
            LoyaltyTier::Platinum => 0.15,
        }
    }

    pub fn satisfaction_bonus(&self) -> f64 {
        match self {
            LoyaltyTier::None => 0.0,
            LoyaltyTier::Basic => 1.5,
            LoyaltyTier::Silver => 3.0,
            LoyaltyTier::Gold => 5.0,
            LoyaltyTier::Platinum => 8.0,
        }
    }

    pub fn points_multiplier(&self) -> f64 {
        match self {
            LoyaltyTier::None => 0.0,
            LoyaltyTier::Basic => 1.0,
            LoyaltyTier::Silver => 1.5,
            LoyaltyTier::Gold => 2.0,
            LoyaltyTier::Platinum => 3.0,
        }
    }

    pub fn member_growth_rate(&self) -> f64 {
        match self {
            LoyaltyTier::None => 0.0,
            LoyaltyTier::Basic => 0.08,
            LoyaltyTier::Silver => 0.12,
            LoyaltyTier::Gold => 0.18,
            LoyaltyTier::Platinum => 0.25,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            LoyaltyTier::None => "No loyalty program. Customers shop without rewards.",
            LoyaltyTier::Basic => "Stamp card and birthday discounts. Low cost, modest returns.",
            LoyaltyTier::Silver => "Points-based rewards with exclusive member pricing and seasonal promos.",
            LoyaltyTier::Gold => "Tiered membership with VIP lanes, free delivery, and partner discounts.",
            LoyaltyTier::Platinum => "Premium experience: personal shoppers, extended warranties, contractor perks, and priority events.",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "none" => Some(LoyaltyTier::None),
            "basic" => Some(LoyaltyTier::Basic),
            "silver" => Some(LoyaltyTier::Silver),
            "gold" => Some(LoyaltyTier::Gold),
            "platinum" => Some(LoyaltyTier::Platinum),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgram {
    pub tier: LoyaltyTier,
    pub members: u32,
    pub quarters_active: i32,
}

impl LoyaltyProgram {
    pub fn new() -> Self {
        LoyaltyProgram {
            tier: LoyaltyTier::None,
            members: 0,
            quarters_active: 0,
        }
    }
}

impl Default for LoyaltyProgram {
    fn default() -> Self {
        Self::new()
    }
}

pub fn update_loyalty(state: &mut GameState) -> f64 {
    let operating_count = state.operating_store_count();
    let customer_sat = state.company.customer_satisfaction;
    let program = &mut state.loyalty;

    if program.tier == LoyaltyTier::None {
        program.members = 0;
        program.quarters_active = 0;
        return 0.0;
    }

    program.quarters_active += 1;

    let satisfaction_factor = customer_sat / 100.0;
    let growth = program.tier.member_growth_rate() * satisfaction_factor;
    let base_pool = operating_count as f64 * 5_000.0;
    let new_members = (base_pool * growth) as u32;
    let churn = (program.members as f64 * 0.03) as u32;
    program.members = program
        .members
        .saturating_sub(churn)
        .saturating_add(new_members);

    let member_cap = operating_count * 15_000;
    program.members = program.members.min(member_cap);

    let cost = program.tier.quarterly_cost_per_store() * operating_count as f64;

    let sat_bonus = program.tier.satisfaction_bonus() * 0.1;
    state.company.customer_satisfaction =
        (state.company.customer_satisfaction + sat_bonus).clamp(10.0, 100.0);

    state.company.brand_reputation = (state.company.brand_reputation
        + program.tier.satisfaction_bonus() * 0.05)
        .clamp(10.0, 100.0);

    cost
}

pub fn loyalty_revenue_multiplier(state: &GameState) -> f64 {
    if state.loyalty.tier == LoyaltyTier::None || state.loyalty.members == 0 {
        return 1.0;
    }
    let member_penetration =
        state.loyalty.members as f64 / (state.operating_store_count() as f64 * 15_000.0).max(1.0);
    let effective_bonus = state.loyalty.tier.revenue_bonus() * member_penetration;
    1.0 + effective_bonus
}
