use super::state::GameState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CsrInitiative {
    CommunityBuild,
    ScholarshipProgram,
    DisasterRelief,
    EnvironmentalProgram,
    EmployeeWelfare,
    LocalSourcing,
    YouthTraining,
    HabitatPartner,
}

impl CsrInitiative {
    pub fn label(&self) -> &str {
        match self {
            CsrInitiative::CommunityBuild => "Community Build",
            CsrInitiative::ScholarshipProgram => "Scholarship Program",
            CsrInitiative::DisasterRelief => "Disaster Relief Fund",
            CsrInitiative::EnvironmentalProgram => "Green Initiative",
            CsrInitiative::EmployeeWelfare => "Employee Welfare Fund",
            CsrInitiative::LocalSourcing => "Local Sourcing Program",
            CsrInitiative::YouthTraining => "Youth Skills Training",
            CsrInitiative::HabitatPartner => "Habitat Partnership",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            CsrInitiative::CommunityBuild => "community_build",
            CsrInitiative::ScholarshipProgram => "scholarship",
            CsrInitiative::DisasterRelief => "disaster_relief",
            CsrInitiative::EnvironmentalProgram => "green_initiative",
            CsrInitiative::EmployeeWelfare => "employee_welfare",
            CsrInitiative::LocalSourcing => "local_sourcing",
            CsrInitiative::YouthTraining => "youth_training",
            CsrInitiative::HabitatPartner => "habitat_partner",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            CsrInitiative::CommunityBuild => "Hammer",
            CsrInitiative::ScholarshipProgram => "GraduationCap",
            CsrInitiative::DisasterRelief => "Heart",
            CsrInitiative::EnvironmentalProgram => "Leaf",
            CsrInitiative::EmployeeWelfare => "Users",
            CsrInitiative::LocalSourcing => "Truck",
            CsrInitiative::YouthTraining => "BookOpen",
            CsrInitiative::HabitatPartner => "Home",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            CsrInitiative::CommunityBuild => "Build homes and community centers in underserved areas. Major reputation boost and community goodwill.",
            CsrInitiative::ScholarshipProgram => "Fund scholarships for engineering and architecture students. Long-term talent pipeline and brand prestige.",
            CsrInitiative::DisasterRelief => "Maintain a disaster relief fund for typhoon and earthquake victims. Provides insurance against natural disaster events.",
            CsrInitiative::EnvironmentalProgram => "Reduce environmental impact through sustainable sourcing, recycling programs, and energy efficiency.",
            CsrInitiative::EmployeeWelfare => "Enhanced benefits including healthcare, housing assistance, and education support for employees and families.",
            CsrInitiative::LocalSourcing => "Partner with local artisans and suppliers. Strengthens supply chain resilience and community ties.",
            CsrInitiative::YouthTraining => "Vocational training programs for out-of-school youth in construction and retail skills.",
            CsrInitiative::HabitatPartner => "Partner with Habitat for Humanity Philippines. Direct community impact and high visibility.",
        }
    }

    pub fn setup_cost(&self) -> f64 {
        match self {
            CsrInitiative::CommunityBuild => 8_000_000.0,
            CsrInitiative::ScholarshipProgram => 5_000_000.0,
            CsrInitiative::DisasterRelief => 10_000_000.0,
            CsrInitiative::EnvironmentalProgram => 6_000_000.0,
            CsrInitiative::EmployeeWelfare => 7_000_000.0,
            CsrInitiative::LocalSourcing => 4_000_000.0,
            CsrInitiative::YouthTraining => 3_000_000.0,
            CsrInitiative::HabitatPartner => 12_000_000.0,
        }
    }

    pub fn quarterly_cost(&self) -> f64 {
        match self {
            CsrInitiative::CommunityBuild => 2_000_000.0,
            CsrInitiative::ScholarshipProgram => 1_500_000.0,
            CsrInitiative::DisasterRelief => 1_000_000.0,
            CsrInitiative::EnvironmentalProgram => 1_800_000.0,
            CsrInitiative::EmployeeWelfare => 2_500_000.0,
            CsrInitiative::LocalSourcing => 800_000.0,
            CsrInitiative::YouthTraining => 1_200_000.0,
            CsrInitiative::HabitatPartner => 3_000_000.0,
        }
    }

    pub fn reputation_bonus(&self) -> f64 {
        match self {
            CsrInitiative::CommunityBuild => 2.5,
            CsrInitiative::ScholarshipProgram => 2.0,
            CsrInitiative::DisasterRelief => 1.5,
            CsrInitiative::EnvironmentalProgram => 1.8,
            CsrInitiative::EmployeeWelfare => 1.0,
            CsrInitiative::LocalSourcing => 1.2,
            CsrInitiative::YouthTraining => 1.5,
            CsrInitiative::HabitatPartner => 3.0,
        }
    }

    pub fn satisfaction_bonus(&self) -> f64 {
        match self {
            CsrInitiative::CommunityBuild => 1.0,
            CsrInitiative::ScholarshipProgram => 0.5,
            CsrInitiative::DisasterRelief => 0.8,
            CsrInitiative::EnvironmentalProgram => 1.2,
            CsrInitiative::EmployeeWelfare => 2.5,
            CsrInitiative::LocalSourcing => 0.3,
            CsrInitiative::YouthTraining => 0.5,
            CsrInitiative::HabitatPartner => 1.0,
        }
    }

    pub fn employee_morale_bonus(&self) -> f64 {
        match self {
            CsrInitiative::CommunityBuild => 1.0,
            CsrInitiative::ScholarshipProgram => 0.5,
            CsrInitiative::DisasterRelief => 0.8,
            CsrInitiative::EnvironmentalProgram => 0.3,
            CsrInitiative::EmployeeWelfare => 3.0,
            CsrInitiative::LocalSourcing => 0.5,
            CsrInitiative::YouthTraining => 1.0,
            CsrInitiative::HabitatPartner => 1.5,
        }
    }

    pub fn tax_deduction_rate(&self) -> f64 {
        match self {
            CsrInitiative::CommunityBuild => 0.10,
            CsrInitiative::ScholarshipProgram => 0.08,
            CsrInitiative::DisasterRelief => 0.12,
            CsrInitiative::EnvironmentalProgram => 0.10,
            CsrInitiative::EmployeeWelfare => 0.05,
            CsrInitiative::LocalSourcing => 0.06,
            CsrInitiative::YouthTraining => 0.08,
            CsrInitiative::HabitatPartner => 0.15,
        }
    }

    pub fn min_stores(&self) -> u32 {
        match self {
            CsrInitiative::CommunityBuild => 3,
            CsrInitiative::ScholarshipProgram => 2,
            CsrInitiative::DisasterRelief => 1,
            CsrInitiative::EnvironmentalProgram => 2,
            CsrInitiative::EmployeeWelfare => 1,
            CsrInitiative::LocalSourcing => 3,
            CsrInitiative::YouthTraining => 2,
            CsrInitiative::HabitatPartner => 5,
        }
    }

    pub fn all_initiatives() -> Vec<CsrInitiative> {
        vec![
            CsrInitiative::CommunityBuild,
            CsrInitiative::ScholarshipProgram,
            CsrInitiative::DisasterRelief,
            CsrInitiative::EnvironmentalProgram,
            CsrInitiative::EmployeeWelfare,
            CsrInitiative::LocalSourcing,
            CsrInitiative::YouthTraining,
            CsrInitiative::HabitatPartner,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "community_build" => Some(CsrInitiative::CommunityBuild),
            "scholarship" => Some(CsrInitiative::ScholarshipProgram),
            "disaster_relief" => Some(CsrInitiative::DisasterRelief),
            "green_initiative" => Some(CsrInitiative::EnvironmentalProgram),
            "employee_welfare" => Some(CsrInitiative::EmployeeWelfare),
            "local_sourcing" => Some(CsrInitiative::LocalSourcing),
            "youth_training" => Some(CsrInitiative::YouthTraining),
            "habitat_partner" => Some(CsrInitiative::HabitatPartner),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveCsrInitiative {
    pub id: String,
    pub initiative: CsrInitiative,
    pub quarters_active: i32,
    pub total_invested: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsrState {
    pub active_initiatives: Vec<ActiveCsrInitiative>,
    pub csr_score: f64,
    pub total_donated: f64,
}

impl CsrState {
    pub fn new() -> Self {
        CsrState {
            active_initiatives: vec![],
            csr_score: 0.0,
            total_donated: 0.0,
        }
    }
}

impl Default for CsrState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn launch_initiative(
    state: &mut GameState,
    initiative: CsrInitiative,
) -> Result<f64, &'static str> {
    if state
        .csr
        .active_initiatives
        .iter()
        .any(|i| i.initiative == initiative)
    {
        return Err("This CSR initiative is already active.");
    }

    let setup_cost = initiative.setup_cost();
    if state.company.cash < setup_cost {
        return Err("Not enough cash to launch this initiative.");
    }

    let min = initiative.min_stores();
    if state.operating_store_count() < min {
        return Err("Not enough operating stores for this initiative.");
    }

    state.company.cash -= setup_cost;
    state.csr.total_donated += setup_cost;

    let active = ActiveCsrInitiative {
        id: uuid::Uuid::new_v4().to_string(),
        initiative,
        quarters_active: 0,
        total_invested: setup_cost,
    };
    state.csr.active_initiatives.push(active);
    Ok(setup_cost)
}

pub fn discontinue_initiative(
    state: &mut GameState,
    initiative_id: &str,
) -> Result<String, &'static str> {
    let idx = state
        .csr
        .active_initiatives
        .iter()
        .position(|i| i.id == initiative_id)
        .ok_or("Initiative not found.")?;
    let name = state.csr.active_initiatives[idx]
        .initiative
        .label()
        .to_string();
    state.csr.active_initiatives.remove(idx);
    Ok(name)
}

pub fn process_csr(state: &mut GameState) -> f64 {
    let mut total_cost = 0.0;
    let mut total_rep = 0.0;
    let mut total_sat = 0.0;
    let mut total_morale = 0.0;

    for initiative in &mut state.csr.active_initiatives {
        let q_cost = initiative.initiative.quarterly_cost();
        total_cost += q_cost;
        initiative.quarters_active += 1;
        initiative.total_invested += q_cost;
        state.csr.total_donated += q_cost;

        total_rep += initiative.initiative.reputation_bonus();
        total_sat += initiative.initiative.satisfaction_bonus();
        total_morale += initiative.initiative.employee_morale_bonus();
    }

    state.company.brand_reputation =
        (state.company.brand_reputation + total_rep).clamp(10.0, 100.0);
    state.company.customer_satisfaction =
        (state.company.customer_satisfaction + total_sat).clamp(10.0, 100.0);
    state.company.employee_satisfaction =
        (state.company.employee_satisfaction + total_morale).clamp(10.0, 100.0);

    let initiative_count = state.csr.active_initiatives.len() as f64;
    let target_score = (initiative_count * 12.5).min(100.0);
    state.csr.csr_score += (target_score - state.csr.csr_score) * 0.3;
    state.csr.csr_score = state.csr.csr_score.clamp(0.0, 100.0);

    total_cost
}

pub fn csr_tax_deduction(state: &GameState) -> f64 {
    state
        .csr
        .active_initiatives
        .iter()
        .map(|i| i.initiative.tax_deduction_rate())
        .sum()
}

pub fn csr_reputation_multiplier(state: &GameState) -> f64 {
    1.0 + state.csr.csr_score / 200.0
}
