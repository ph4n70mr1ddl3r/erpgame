use serde::{Deserialize, Serialize};

use super::state::GameState;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SponsorshipType {
    SportsTeam,
    CommunityEvent,
    TradeShow,
    CharityGala,
    UniversityProgram,
    HomeImprovementShow,
    LocalFestival,
    YouthSports,
}

impl SponsorshipType {
    pub fn label(&self) -> &str {
        match self {
            SponsorshipType::SportsTeam => "Sports Team",
            SponsorshipType::CommunityEvent => "Community Event",
            SponsorshipType::TradeShow => "Trade Show",
            SponsorshipType::CharityGala => "Charity Gala",
            SponsorshipType::UniversityProgram => "University Program",
            SponsorshipType::HomeImprovementShow => "Home Improvement Show",
            SponsorshipType::LocalFestival => "Local Festival",
            SponsorshipType::YouthSports => "Youth Sports League",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            SponsorshipType::SportsTeam => "sports_team",
            SponsorshipType::CommunityEvent => "community_event",
            SponsorshipType::TradeShow => "trade_show",
            SponsorshipType::CharityGala => "charity_gala",
            SponsorshipType::UniversityProgram => "university_program",
            SponsorshipType::HomeImprovementShow => "home_improvement_show",
            SponsorshipType::LocalFestival => "local_festival",
            SponsorshipType::YouthSports => "youth_sports",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            SponsorshipType::SportsTeam => "Trophy",
            SponsorshipType::CommunityEvent => "Users",
            SponsorshipType::TradeShow => "Briefcase",
            SponsorshipType::CharityGala => "Heart",
            SponsorshipType::UniversityProgram => "GraduationCap",
            SponsorshipType::HomeImprovementShow => "Home",
            SponsorshipType::LocalFestival => "Star",
            SponsorshipType::YouthSports => "Medal",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            SponsorshipType::SportsTeam => "Sponsor a professional or semi-pro sports team. High visibility, especially during games and events.",
            SponsorshipType::CommunityEvent => "Support local community events like fiestas, cultural celebrations, and neighborhood gatherings.",
            SponsorshipType::TradeShow => "Sponsor construction and home improvement trade shows. Direct access to contractors and professionals.",
            SponsorshipType::CharityGala => "Support charitable causes while building brand reputation. Tax-deductible and socially responsible.",
            SponsorshipType::UniversityProgram => "Partner with universities for engineering and architecture programs. Builds future talent pipeline.",
            SponsorshipType::HomeImprovementShow => "Sponsor TV or live home improvement shows. Direct product placement and expert endorsement.",
            SponsorshipType::LocalFestival => "Sponsor regional festivals and town celebrations. Strong local brand recognition.",
            SponsorshipType::YouthSports => "Support youth sports leagues. Family-oriented branding and community goodwill.",
        }
    }

    pub fn base_cost(&self) -> f64 {
        match self {
            SponsorshipType::SportsTeam => 25_000_000.0,
            SponsorshipType::CommunityEvent => 3_000_000.0,
            SponsorshipType::TradeShow => 8_000_000.0,
            SponsorshipType::CharityGala => 5_000_000.0,
            SponsorshipType::UniversityProgram => 4_000_000.0,
            SponsorshipType::HomeImprovementShow => 12_000_000.0,
            SponsorshipType::LocalFestival => 2_000_000.0,
            SponsorshipType::YouthSports => 1_500_000.0,
        }
    }

    pub fn duration_years(&self) -> i32 {
        match self {
            SponsorshipType::SportsTeam => 2,
            SponsorshipType::CommunityEvent => 1,
            SponsorshipType::TradeShow => 1,
            SponsorshipType::CharityGala => 1,
            SponsorshipType::UniversityProgram => 3,
            SponsorshipType::HomeImprovementShow => 2,
            SponsorshipType::LocalFestival => 1,
            SponsorshipType::YouthSports => 2,
        }
    }

    pub fn annual_cost(&self) -> f64 {
        match self {
            SponsorshipType::SportsTeam => 10_000_000.0,
            SponsorshipType::CommunityEvent => 1_000_000.0,
            SponsorshipType::TradeShow => 3_000_000.0,
            SponsorshipType::CharityGala => 1_500_000.0,
            SponsorshipType::UniversityProgram => 1_200_000.0,
            SponsorshipType::HomeImprovementShow => 5_000_000.0,
            SponsorshipType::LocalFestival => 800_000.0,
            SponsorshipType::YouthSports => 600_000.0,
        }
    }

    pub fn reputation_boost(&self) -> f64 {
        match self {
            SponsorshipType::SportsTeam => 4.0,
            SponsorshipType::CommunityEvent => 2.5,
            SponsorshipType::TradeShow => 1.5,
            SponsorshipType::CharityGala => 3.5,
            SponsorshipType::UniversityProgram => 2.0,
            SponsorshipType::HomeImprovementShow => 2.5,
            SponsorshipType::LocalFestival => 2.0,
            SponsorshipType::YouthSports => 2.5,
        }
    }

    pub fn satisfaction_boost(&self) -> f64 {
        match self {
            SponsorshipType::SportsTeam => 1.0,
            SponsorshipType::CommunityEvent => 2.0,
            SponsorshipType::TradeShow => 0.5,
            SponsorshipType::CharityGala => 1.5,
            SponsorshipType::UniversityProgram => 0.5,
            SponsorshipType::HomeImprovementShow => 1.5,
            SponsorshipType::LocalFestival => 2.0,
            SponsorshipType::YouthSports => 2.5,
        }
    }

    pub fn revenue_boost(&self) -> f64 {
        match self {
            SponsorshipType::SportsTeam => 0.04,
            SponsorshipType::CommunityEvent => 0.01,
            SponsorshipType::TradeShow => 0.03,
            SponsorshipType::CharityGala => 0.0,
            SponsorshipType::UniversityProgram => 0.02,
            SponsorshipType::HomeImprovementShow => 0.05,
            SponsorshipType::LocalFestival => 0.01,
            SponsorshipType::YouthSports => 0.01,
        }
    }

    pub fn morale_boost(&self) -> f64 {
        match self {
            SponsorshipType::SportsTeam => 1.0,
            SponsorshipType::CommunityEvent => 0.5,
            SponsorshipType::TradeShow => 0.0,
            SponsorshipType::CharityGala => 1.0,
            SponsorshipType::UniversityProgram => 0.5,
            SponsorshipType::HomeImprovementShow => 0.5,
            SponsorshipType::LocalFestival => 0.5,
            SponsorshipType::YouthSports => 1.0,
        }
    }

    pub fn min_stores(&self) -> u32 {
        match self {
            SponsorshipType::SportsTeam => 5,
            SponsorshipType::CommunityEvent => 1,
            SponsorshipType::TradeShow => 3,
            SponsorshipType::CharityGala => 2,
            SponsorshipType::UniversityProgram => 3,
            SponsorshipType::HomeImprovementShow => 4,
            SponsorshipType::LocalFestival => 1,
            SponsorshipType::YouthSports => 1,
        }
    }

    pub fn all_types() -> Vec<SponsorshipType> {
        vec![
            SponsorshipType::SportsTeam,
            SponsorshipType::CommunityEvent,
            SponsorshipType::TradeShow,
            SponsorshipType::CharityGala,
            SponsorshipType::UniversityProgram,
            SponsorshipType::HomeImprovementShow,
            SponsorshipType::LocalFestival,
            SponsorshipType::YouthSports,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "sports_team" => Some(SponsorshipType::SportsTeam),
            "community_event" => Some(SponsorshipType::CommunityEvent),
            "trade_show" => Some(SponsorshipType::TradeShow),
            "charity_gala" => Some(SponsorshipType::CharityGala),
            "university_program" => Some(SponsorshipType::UniversityProgram),
            "home_improvement_show" => Some(SponsorshipType::HomeImprovementShow),
            "local_festival" => Some(SponsorshipType::LocalFestival),
            "youth_sports" => Some(SponsorshipType::YouthSports),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sponsorship {
    pub id: String,
    pub sponsorship_type: SponsorshipType,
    pub quarters_remaining: i32,
    pub quarters_total: i32,
    pub start_quarter: i32,
    pub start_year: i32,
    pub name: String,
}

pub fn launch_sponsorship(
    state: &mut GameState,
    sponsorship_type: SponsorshipType,
) -> Result<f64, &'static str> {
    let active_count = state.sponsorships.len();
    if active_count >= 5 {
        return Err("Maximum 5 concurrent sponsorships allowed.");
    }

    let same_active = state
        .sponsorships
        .iter()
        .any(|s| s.sponsorship_type == sponsorship_type);
    if same_active {
        return Err("This sponsorship type is already active.");
    }

    let operating = state.operating_store_count();
    if operating < sponsorship_type.min_stores() {
        return Err("Not enough operating stores for this sponsorship.");
    }

    let cost = sponsorship_type.base_cost();
    if state.company.cash < cost {
        return Err("Not enough cash to launch this sponsorship.");
    }

    state.company.cash -= cost;

    let name = match sponsorship_type {
        SponsorshipType::SportsTeam => "Metro Manila Builders FC".to_string(),
        SponsorshipType::CommunityEvent => "Bahay Depot Community Day".to_string(),
        SponsorshipType::TradeShow => "Philippine Construction Expo".to_string(),
        SponsorshipType::CharityGala => "Homes for Hope Gala".to_string(),
        SponsorshipType::UniversityProgram => "Future Builders Program".to_string(),
        SponsorshipType::HomeImprovementShow => "DIY Philippines TV Show".to_string(),
        SponsorshipType::LocalFestival => "Regional Fiesta Series".to_string(),
        SponsorshipType::YouthSports => "Bahay Depot Youth League".to_string(),
    };

    let sponsorship = Sponsorship {
        id: uuid::Uuid::new_v4().to_string(),
        sponsorship_type,
        quarters_remaining: sponsorship_type.duration_years() * 4,
        quarters_total: sponsorship_type.duration_years() * 4,
        start_quarter: state.current_quarter,
        start_year: state.current_year,
        name,
    };
    state.sponsorships.push(sponsorship);
    Ok(cost)
}

pub fn cancel_sponsorship(state: &mut GameState, sponsorship_id: &str) -> Result<(), &'static str> {
    let idx = state
        .sponsorships
        .iter()
        .position(|s| s.id == sponsorship_id)
        .ok_or("Sponsorship not found.")?;

    let sponsorship = &state.sponsorships[idx];
    let penalty = sponsorship.sponsorship_type.annual_cost() * 0.5;

    if state.company.cash < penalty {
        return Err("Not enough cash to pay cancellation penalty.");
    }

    state.company.cash -= penalty;
    let name = sponsorship.name.clone();
    state.sponsorships.remove(idx);
    state.push_message(format!(
        "Cancelled '{}' sponsorship. Penalty paid: {}.",
        name,
        super::state::format_currency(penalty)
    ));

    Ok(())
}

pub fn process_sponsorships(state: &mut GameState) -> f64 {
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == super::state::ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let skill_bonus = 1.0 + cmo_skill * 0.003;

    let mut total_reputation_boost = 0.0;
    let mut total_satisfaction_boost = 0.0;
    let mut total_morale_boost = 0.0;
    let mut total_annual_cost = 0.0;

    for sponsorship in &state.campaigns {
        let sp_type = sponsorship.sponsorship_type;

        total_reputation_boost += sp_type.reputation_boost() * skill_bonus * 0.25;
        total_satisfaction_boost += sp_type.satisfaction_boost() * skill_bonus * 0.25;
        total_morale_boost += sp_type.morale_boost() * skill_bonus * 0.25;
        total_annual_cost += sp_type.annual_cost() / 4.0;
    }

    for sponsorship in &mut state.sponsorships {
        let sp_type = sponsorship.sponsorship_type;

        total_reputation_boost += sp_type.reputation_boost() * skill_bonus * 0.25;
        total_satisfaction_boost += sp_type.satisfaction_boost() * skill_bonus * 0.25;
        total_morale_boost += sp_type.morale_boost() * skill_bonus * 0.25;
        total_annual_cost += sp_type.annual_cost() / 4.0;

        sponsorship.quarters_remaining -= 1;
    }

    state.company.brand_reputation =
        (state.company.brand_reputation + total_reputation_boost).clamp(10.0, 100.0);
    state.company.customer_satisfaction =
        (state.company.customer_satisfaction + total_satisfaction_boost).clamp(10.0, 100.0);
    state.company.employee_satisfaction =
        (state.company.employee_satisfaction + total_morale_boost).clamp(10.0, 100.0);

    let expired: Vec<String> = state
        .sponsorships
        .iter()
        .filter(|s| s.quarters_remaining <= 0)
        .map(|s| s.name.clone())
        .collect();

    state.sponsorships.retain(|s| s.quarters_remaining > 0);

    for name in &expired {
        state.push_message(format!("Sponsorship '{}' has ended.", name));
    }

    total_annual_cost
}

pub fn sponsorship_revenue_multiplier(state: &GameState) -> f64 {
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == super::state::ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let skill_bonus = 1.0 + cmo_skill * 0.003;

    let total: f64 = state
        .sponsorships
        .iter()
        .map(|s| s.sponsorship_type.revenue_boost() * skill_bonus)
        .sum();

    1.0 + total
}
