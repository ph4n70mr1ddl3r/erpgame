use serde::{Deserialize, Serialize};

use super::state::GameState;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CampaignType {
    TvCommercial,
    SocialMediaBlitz,
    SeasonalSale,
    InfluencerPartnership,
    CommunityEvent,
    DigitalAds,
    RadioCampaign,
    BillboardCampaign,
}

impl CampaignType {
    pub fn label(&self) -> &str {
        match self {
            CampaignType::TvCommercial => "TV Commercial",
            CampaignType::SocialMediaBlitz => "Social Media Blitz",
            CampaignType::SeasonalSale => "Seasonal Sale",
            CampaignType::InfluencerPartnership => "Influencer Partnership",
            CampaignType::CommunityEvent => "Community Event",
            CampaignType::DigitalAds => "Digital Ads",
            CampaignType::RadioCampaign => "Radio Campaign",
            CampaignType::BillboardCampaign => "Billboard Campaign",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            CampaignType::TvCommercial => "tv_commercial",
            CampaignType::SocialMediaBlitz => "social_media",
            CampaignType::SeasonalSale => "seasonal_sale",
            CampaignType::InfluencerPartnership => "influencer",
            CampaignType::CommunityEvent => "community_event",
            CampaignType::DigitalAds => "digital_ads",
            CampaignType::RadioCampaign => "radio",
            CampaignType::BillboardCampaign => "billboard",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            CampaignType::TvCommercial => "Tv",
            CampaignType::SocialMediaBlitz => "Smartphone",
            CampaignType::SeasonalSale => "Tag",
            CampaignType::InfluencerPartnership => "User",
            CampaignType::CommunityEvent => "Home",
            CampaignType::DigitalAds => "Globe",
            CampaignType::RadioCampaign => "Radio",
            CampaignType::BillboardCampaign => "Image",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            CampaignType::TvCommercial => "Prime-time TV ads on major networks. High reach, high cost. Best for building broad brand awareness.",
            CampaignType::SocialMediaBlitz => "Intensive Facebook, Instagram, and TikTok ad campaign. Cost-effective reach to younger demographics.",
            CampaignType::SeasonalSale => "Limited-time promotional sale with special pricing. Immediate revenue boost, slight margin reduction.",
            CampaignType::InfluencerPartnership => "Partner with home improvement influencers and contractors. Builds credibility and drives targeted traffic.",
            CampaignType::CommunityEvent => "Sponsor local community events and DIY workshops. Strong local reputation and satisfaction gains.",
            CampaignType::DigitalAds => "Google Ads and online retargeting campaigns. Steady traffic increase with measurable ROI.",
            CampaignType::RadioCampaign => "Radio spots on popular FM/AM stations. Broad regional reach at moderate cost.",
            CampaignType::BillboardCampaign => "Strategic billboard placements near major highways and store locations. Persistent visibility boost.",
        }
    }

    pub fn base_cost(&self) -> f64 {
        match self {
            CampaignType::TvCommercial => 15_000_000.0,
            CampaignType::SocialMediaBlitz => 3_000_000.0,
            CampaignType::SeasonalSale => 5_000_000.0,
            CampaignType::InfluencerPartnership => 4_000_000.0,
            CampaignType::CommunityEvent => 2_000_000.0,
            CampaignType::DigitalAds => 2_500_000.0,
            CampaignType::RadioCampaign => 6_000_000.0,
            CampaignType::BillboardCampaign => 8_000_000.0,
        }
    }

    pub fn duration_quarters(&self) -> i32 {
        match self {
            CampaignType::TvCommercial => 2,
            CampaignType::SocialMediaBlitz => 1,
            CampaignType::SeasonalSale => 1,
            CampaignType::InfluencerPartnership => 2,
            CampaignType::CommunityEvent => 1,
            CampaignType::DigitalAds => 3,
            CampaignType::RadioCampaign => 2,
            CampaignType::BillboardCampaign => 3,
        }
    }

    pub fn revenue_boost(&self) -> f64 {
        match self {
            CampaignType::TvCommercial => 0.12,
            CampaignType::SocialMediaBlitz => 0.08,
            CampaignType::SeasonalSale => 0.18,
            CampaignType::InfluencerPartnership => 0.10,
            CampaignType::CommunityEvent => 0.05,
            CampaignType::DigitalAds => 0.07,
            CampaignType::RadioCampaign => 0.06,
            CampaignType::BillboardCampaign => 0.05,
        }
    }

    pub fn reputation_boost(&self) -> f64 {
        match self {
            CampaignType::TvCommercial => 3.0,
            CampaignType::SocialMediaBlitz => 1.5,
            CampaignType::SeasonalSale => 0.0,
            CampaignType::InfluencerPartnership => 2.0,
            CampaignType::CommunityEvent => 3.5,
            CampaignType::DigitalAds => 1.0,
            CampaignType::RadioCampaign => 1.5,
            CampaignType::BillboardCampaign => 2.0,
        }
    }

    pub fn satisfaction_boost(&self) -> f64 {
        match self {
            CampaignType::TvCommercial => 0.0,
            CampaignType::SocialMediaBlitz => 0.5,
            CampaignType::SeasonalSale => 2.0,
            CampaignType::InfluencerPartnership => 1.0,
            CampaignType::CommunityEvent => 3.0,
            CampaignType::DigitalAds => 0.0,
            CampaignType::RadioCampaign => 0.0,
            CampaignType::BillboardCampaign => 0.5,
        }
    }

    pub fn all_types() -> Vec<CampaignType> {
        vec![
            CampaignType::TvCommercial,
            CampaignType::SocialMediaBlitz,
            CampaignType::SeasonalSale,
            CampaignType::InfluencerPartnership,
            CampaignType::CommunityEvent,
            CampaignType::DigitalAds,
            CampaignType::RadioCampaign,
            CampaignType::BillboardCampaign,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "tv_commercial" => Some(CampaignType::TvCommercial),
            "social_media" => Some(CampaignType::SocialMediaBlitz),
            "seasonal_sale" => Some(CampaignType::SeasonalSale),
            "influencer" => Some(CampaignType::InfluencerPartnership),
            "community_event" => Some(CampaignType::CommunityEvent),
            "digital_ads" => Some(CampaignType::DigitalAds),
            "radio" => Some(CampaignType::RadioCampaign),
            "billboard" => Some(CampaignType::BillboardCampaign),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub campaign_type: CampaignType,
    pub quarters_remaining: i32,
    pub quarters_total: i32,
    pub start_quarter: i32,
    pub start_year: i32,
}

pub fn launch_campaign(
    state: &mut GameState,
    campaign_type: CampaignType,
) -> Result<f64, &'static str> {
    let active_count = state.campaigns.len();
    if active_count >= 5 {
        return Err("Maximum 5 concurrent campaigns allowed.");
    }

    let same_active = state
        .campaigns
        .iter()
        .any(|c| c.campaign_type == campaign_type);
    if same_active {
        return Err("This campaign type is already active.");
    }

    let cost = campaign_type.base_cost();
    if state.company.cash < cost {
        return Err("Not enough cash to launch this campaign.");
    }

    state.company.cash -= cost;
    let campaign = Campaign {
        id: uuid::Uuid::new_v4().to_string(),
        campaign_type,
        quarters_remaining: campaign_type.duration_quarters(),
        quarters_total: campaign_type.duration_quarters(),
        start_quarter: state.current_quarter,
        start_year: state.current_year,
    };
    state.campaigns.push(campaign);
    Ok(cost)
}

pub fn process_campaigns(state: &mut GameState) {
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == super::state::ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let skill_bonus = 1.0 + cmo_skill * 0.005;

    let mut total_reputation_boost = 0.0;
    let mut total_satisfaction_boost = 0.0;

    for campaign in &mut state.campaigns {
        let remaining_ratio = campaign.quarters_remaining as f64 / campaign.quarters_total as f64;
        let decay = 0.7 + 0.3 * remaining_ratio;
        let rep = campaign.campaign_type.reputation_boost() * skill_bonus * decay;
        let sat = campaign.campaign_type.satisfaction_boost() * skill_bonus * decay;

        total_reputation_boost += rep;
        total_satisfaction_boost += sat;

        campaign.quarters_remaining -= 1;
    }

    state.company.brand_reputation =
        (state.company.brand_reputation + total_reputation_boost).clamp(10.0, 100.0);
    state.company.customer_satisfaction =
        (state.company.customer_satisfaction + total_satisfaction_boost).clamp(10.0, 100.0);

    let expired: Vec<String> = state
        .campaigns
        .iter()
        .filter(|c| c.quarters_remaining <= 0)
        .map(|c| c.campaign_type.label().to_string())
        .collect();

    state.campaigns.retain(|c| c.quarters_remaining > 0);

    for name in &expired {
        state.push_message(format!("Marketing campaign '{}' has ended.", name));
    }
}

pub fn campaign_revenue_multiplier(state: &GameState) -> f64 {
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == super::state::ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let skill_bonus = 1.0 + cmo_skill * 0.005;

    let total: f64 = state
        .campaigns
        .iter()
        .map(|c| c.campaign_type.revenue_boost() * skill_bonus)
        .sum();

    1.0 + total
}
