use rand::Rng;
use serde::{Deserialize, Serialize};

use super::state::{ExecutivePosition, GameState};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AdBudget {
    Low,
    Medium,
    High,
}

impl AdBudget {
    pub fn label(&self) -> &str {
        match self {
            AdBudget::Low => "Low Budget",
            AdBudget::Medium => "Medium Budget",
            AdBudget::High => "High Budget",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            AdBudget::Low => "low",
            AdBudget::Medium => "medium",
            AdBudget::High => "high",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "low" => Some(AdBudget::Low),
            "medium" => Some(AdBudget::Medium),
            "high" => Some(AdBudget::High),
            _ => None,
        }
    }

    pub fn quarterly_cost(&self) -> f64 {
        match self {
            AdBudget::Low => 500_000.0,
            AdBudget::Medium => 2_000_000.0,
            AdBudget::High => 5_000_000.0,
        }
    }

    pub fn base_roi(&self) -> f64 {
        match self {
            AdBudget::Low => 1.5,
            AdBudget::Medium => 2.2,
            AdBudget::High => 3.5,
        }
    }

    pub fn all_budgets() -> Vec<Self> {
        vec![AdBudget::Low, AdBudget::Medium, AdBudget::High]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TargetAudience {
    YoungProfessionals,
    Homeowners,
    Contractors,
    DIYEnthusiasts,
    Seniors,
}

impl TargetAudience {
    pub fn label(&self) -> &str {
        match self {
            TargetAudience::YoungProfessionals => "Young Professionals",
            TargetAudience::Homeowners => "Homeowners",
            TargetAudience::Contractors => "Contractors",
            TargetAudience::DIYEnthusiasts => "DIY Enthusiasts",
            TargetAudience::Seniors => "Seniors",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            TargetAudience::YoungProfessionals => "young_professionals",
            TargetAudience::Homeowners => "homeowners",
            TargetAudience::Contractors => "contractors",
            TargetAudience::DIYEnthusiasts => "diy_enthusiasts",
            TargetAudience::Seniors => "seniors",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TargetAudience::YoungProfessionals => "Urban professionals aged 25-35. High online engagement, responsive to social media ads.",
            TargetAudience::Homeowners => "Property owners seeking home improvement. High average purchase value, loyal customers.",
            TargetAudience::Contractors => "Professional builders and renovators. Bulk purchases, B2B relationships.",
            TargetAudience::DIYEnthusiasts => "Weekend warriors and hobbyists. Regular small purchases, respond to tutorials.",
            TargetAudience::Seniors => "Older customers aged 55+. Prefer in-store experience, traditional media channels.",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            TargetAudience::YoungProfessionals => "Briefcase",
            TargetAudience::Homeowners => "Home",
            TargetAudience::Contractors => "HardHat",
            TargetAudience::DIYEnthusiasts => "Hammer",
            TargetAudience::Seniors => "Heart",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "young_professionals" => Some(TargetAudience::YoungProfessionals),
            "homeowners" => Some(TargetAudience::Homeowners),
            "contractors" => Some(TargetAudience::Contractors),
            "diy_enthusiasts" => Some(TargetAudience::DIYEnthusiasts),
            "seniors" => Some(TargetAudience::Seniors),
            _ => None,
        }
    }

    pub fn base_conversion(&self) -> f64 {
        match self {
            TargetAudience::YoungProfessionals => 3.5,
            TargetAudience::Homeowners => 4.5,
            TargetAudience::Contractors => 6.0,
            TargetAudience::DIYEnthusiasts => 5.0,
            TargetAudience::Seniors => 2.5,
        }
    }

    pub fn avg_purchase_value(&self) -> f64 {
        match self {
            TargetAudience::YoungProfessionals => 3_500.0,
            TargetAudience::Homeowners => 12_000.0,
            TargetAudience::Contractors => 45_000.0,
            TargetAudience::DIYEnthusiasts => 2_800.0,
            TargetAudience::Seniors => 5_500.0,
        }
    }

    pub fn all_audiences() -> Vec<Self> {
        vec![
            TargetAudience::YoungProfessionals,
            TargetAudience::Homeowners,
            TargetAudience::Contractors,
            TargetAudience::DIYEnthusiasts,
            TargetAudience::Seniors,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdCampaign {
    pub id: String,
    pub name: String,
    pub budget: AdBudget,
    pub target_audience: TargetAudience,
    pub quarters_active: i32,
    pub total_spent: f64,
    pub total_revenue_generated: f64,
    pub impressions: u64,
    pub clicks: u64,
    pub conversions: u64,
}

impl AdCampaign {
    pub fn new(budget: AdBudget, target_audience: TargetAudience) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("{} - {}", budget.label(), target_audience.label()),
            budget,
            target_audience,
            quarters_active: 0,
            total_spent: 0.0,
            total_revenue_generated: 0.0,
            impressions: 0,
            clicks: 0,
            conversions: 0,
        }
    }

    pub fn roi(&self) -> f64 {
        if self.total_spent > 0.0 {
            (self.total_revenue_generated - self.total_spent) / self.total_spent * 100.0
        } else {
            0.0
        }
    }

    pub fn conversion_rate(&self) -> f64 {
        if self.clicks > 0 {
            self.conversions as f64 / self.clicks as f64 * 100.0
        } else {
            0.0
        }
    }

    pub fn ctr(&self) -> f64 {
        if self.impressions > 0 {
            self.clicks as f64 / self.impressions as f64 * 100.0
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdCampaignState {
    pub campaigns: Vec<AdCampaign>,
}

impl AdCampaignState {
    pub fn new() -> Self {
        Self { campaigns: vec![] }
    }

    pub fn active_campaign_count(&self) -> usize {
        self.campaigns.len()
    }

    pub fn can_launch(&self, budget: AdBudget, audience: TargetAudience) -> bool {
        if self.campaigns.len() >= 10 {
            return false;
        }
        !self
            .campaigns
            .iter()
            .any(|c| c.budget == budget && c.target_audience == audience)
    }

    pub fn launch(
        &mut self,
        budget: AdBudget,
        audience: TargetAudience,
    ) -> Result<&AdCampaign, &'static str> {
        if self.campaigns.len() >= 10 {
            return Err("Maximum 10 ad campaigns allowed.");
        }
        if !self.can_launch(budget, audience) {
            return Err("This exact campaign combination already exists.");
        }
        let campaign = AdCampaign::new(budget, audience);
        self.campaigns.push(campaign);
        Ok(self.campaigns.last().unwrap())
    }

    pub fn cancel(&mut self, campaign_id: &str) -> Option<AdCampaign> {
        let idx = self.campaigns.iter().position(|c| c.id == campaign_id)?;
        Some(self.campaigns.remove(idx))
    }
}

pub fn launch_ad_campaign(
    state: &mut GameState,
    budget: AdBudget,
    audience: TargetAudience,
) -> Result<f64, &'static str> {
    let cost = budget.quarterly_cost();

    if state.company.cash < cost {
        return Err("Not enough cash to launch this ad campaign.");
    }

    let campaign = state.ad_campaigns.launch(budget, audience)?;
    state.company.cash -= cost;
    state.ad_campaigns.campaigns.last_mut().unwrap().total_spent = cost;

    Ok(cost)
}

pub fn cancel_ad_campaign(state: &mut GameState, campaign_id: &str) -> Option<AdCampaign> {
    state.ad_campaigns.cancel(campaign_id)
}

pub fn process_ad_campaigns(state: &mut GameState, rng: &mut rand::rngs::ThreadRng) -> f64 {
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let cmo_multiplier = 1.0 + cmo_skill * 0.008;
    let store_count = state.operating_store_count() as f64;
    let reputation_factor = state.company.brand_reputation / 100.0;
    let economy_factor = 1.0 + (state.economy.consumer_confidence - 60.0) / 200.0;

    let mut total_revenue = 0.0;
    let mut total_cost = 0.0;

    for campaign in &mut state.ad_campaigns.campaigns {
        let cost = campaign.budget.quarterly_cost();

        if state.company.cash < cost {
            continue;
        }

        state.company.cash -= cost;
        campaign.total_spent += cost;
        campaign.quarters_active += 1;
        total_cost += cost;

        let base_impressions = match campaign.budget {
            AdBudget::Low => 50_000,
            AdBudget::Medium => 250_000,
            AdBudget::High => 800_000,
        };

        let audience_reach = match campaign.target_audience {
            TargetAudience::YoungProfessionals => 1.2,
            TargetAudience::Homeowners => 1.0,
            TargetAudience::Contractors => 0.6,
            TargetAudience::DIYEnthusiasts => 0.9,
            TargetAudience::Seniors => 0.7,
        };

        let impressions = (base_impressions as f64
            * audience_reach
            * store_count.sqrt()
            * economy_factor
            * (0.9 + rng.gen_range(0.0..0.2))) as u64;
        campaign.impressions += impressions;

        let base_ctr = match campaign.target_audience {
            TargetAudience::YoungProfessionals => 4.5,
            TargetAudience::Homeowners => 3.2,
            TargetAudience::Contractors => 5.5,
            TargetAudience::DIYEnthusiasts => 4.0,
            TargetAudience::Seniors => 2.0,
        };

        let ctr = base_ctr * cmo_multiplier * reputation_factor * (0.8 + rng.gen_range(0.0..0.4));
        let clicks = (impressions as f64 * ctr / 100.0) as u64;
        campaign.clicks += clicks;

        let base_conversion = campaign.target_audience.base_conversion();
        let conversion_rate =
            base_conversion * cmo_multiplier * reputation_factor * (0.7 + rng.gen_range(0.0..0.6));
        let conversions = (clicks as f64 * conversion_rate / 100.0) as u64;
        campaign.conversions += conversions;

        let avg_purchase = campaign.target_audience.avg_purchase_value();
        let purchase_variance = rng.gen_range(0.7..1.3);
        let revenue = conversions as f64 * avg_purchase * purchase_variance * cmo_multiplier;

        campaign.total_revenue_generated += revenue;
        total_revenue += revenue;
    }

    total_cost
}

pub fn total_ad_revenue(state: &GameState) -> f64 {
    state
        .ad_campaigns
        .campaigns
        .iter()
        .map(|c| c.total_revenue_generated)
        .sum()
}

pub fn total_ad_spend(state: &GameState) -> f64 {
    state
        .ad_campaigns
        .campaigns
        .iter()
        .map(|c| c.total_spent)
        .sum()
}

pub fn average_ad_roi(state: &GameState) -> f64 {
    let campaigns = &state.ad_campaigns.campaigns;
    if campaigns.is_empty() {
        return 0.0;
    }

    let total_roi: f64 = campaigns.iter().map(|c| c.roi()).sum();
    total_roi / campaigns.len() as f64
}
