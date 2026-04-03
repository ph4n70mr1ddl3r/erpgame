use serde::{Deserialize, Serialize};

use super::state::{ExecutivePosition, GameState};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SeasonalPromotionType {
    ChineseNewYearSale,
    SummerHomeMakeover,
    RainySeasonPrep,
    BarangayFiestaDeals,
    SchoolSeasonSale,
    PaydayWeekendBonanza,
    HalloweenDecorSale,
    ChristmasMegaSale,
    YearEndClearance,
}

impl SeasonalPromotionType {
    pub fn label(&self) -> &str {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => "Chinese New Year Sale",
            SeasonalPromotionType::SummerHomeMakeover => "Summer Home Makeover",
            SeasonalPromotionType::RainySeasonPrep => "Rainy Season Prep",
            SeasonalPromotionType::BarangayFiestaDeals => "Barangay Fiesta Deals",
            SeasonalPromotionType::SchoolSeasonSale => "School Season Sale",
            SeasonalPromotionType::PaydayWeekendBonanza => "Payday Weekend Bonanza",
            SeasonalPromotionType::HalloweenDecorSale => "Halloween Decor Sale",
            SeasonalPromotionType::ChristmasMegaSale => "Christmas Mega Sale",
            SeasonalPromotionType::YearEndClearance => "Year-End Clearance",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => "cny_sale",
            SeasonalPromotionType::SummerHomeMakeover => "summer_makeover",
            SeasonalPromotionType::RainySeasonPrep => "rainy_prep",
            SeasonalPromotionType::BarangayFiestaDeals => "fiesta_deals",
            SeasonalPromotionType::SchoolSeasonSale => "school_sale",
            SeasonalPromotionType::PaydayWeekendBonanza => "payday_bonanza",
            SeasonalPromotionType::HalloweenDecorSale => "halloween_decor",
            SeasonalPromotionType::ChristmasMegaSale => "christmas_sale",
            SeasonalPromotionType::YearEndClearance => "year_end_clearance",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => "Sparkles",
            SeasonalPromotionType::SummerHomeMakeover => "Sun",
            SeasonalPromotionType::RainySeasonPrep => "CloudRain",
            SeasonalPromotionType::BarangayFiestaDeals => "Music",
            SeasonalPromotionType::SchoolSeasonSale => "GraduationCap",
            SeasonalPromotionType::PaydayWeekendBonanza => "Wallet",
            SeasonalPromotionType::HalloweenDecorSale => "Ghost",
            SeasonalPromotionType::ChristmasMegaSale => "Gift",
            SeasonalPromotionType::YearEndClearance => "Tags",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => "Capitalize on the Lunar New Year home renovation rush. Filipinos traditionally spruce up homes for good luck. Strong revenue boost with moderate cost.",
            SeasonalPromotionType::SummerHomeMakeover => "Summer is peak home improvement season. Promote outdoor living, garden supplies, and renovation projects with special financing deals.",
            SeasonalPromotionType::RainySeasonPrep => "Position your stores as the go-to for rainy season essentials: waterproofing, roofing materials, drainage solutions, and emergency repairs.",
            SeasonalPromotionType::BarangayFiestaDeals => "Community-focused promotions tied to local fiestas across the country. Builds grassroots goodwill and drives foot traffic in provincial stores.",
            SeasonalPromotionType::SchoolSeasonSale => "Back-to-school means home organization. Promote storage solutions, study room furniture, and budget renovation packages for families.",
            SeasonalPromotionType::PaydayWeekendBonanza => "Strategic promotions aligned with the Philippine payday cycle (15th and 30th). Capitalize on consumer spending patterns for maximum revenue.",
            SeasonalPromotionType::HalloweenDecorSale => "Halloween is growing in the Philippines. Promote decorations, lighting, and home accessories for the spooky season.",
            SeasonalPromotionType::ChristmasMegaSale => "The biggest retail season in the Philippines. Massive revenue potential but requires significant investment in inventory, marketing, and staffing.",
            SeasonalPromotionType::YearEndClearance => "Clear out old inventory before the new year. Strong revenue boost but may slightly hurt brand perception as a 'discount' retailer.",
        }
    }

    pub fn available_quarter(&self) -> i32 {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => 1,
            SeasonalPromotionType::SummerHomeMakeover => 2,
            SeasonalPromotionType::RainySeasonPrep => 2,
            SeasonalPromotionType::BarangayFiestaDeals => 3,
            SeasonalPromotionType::SchoolSeasonSale => 3,
            SeasonalPromotionType::PaydayWeekendBonanza => 3,
            SeasonalPromotionType::HalloweenDecorSale => 4,
            SeasonalPromotionType::ChristmasMegaSale => 4,
            SeasonalPromotionType::YearEndClearance => 4,
        }
    }

    pub fn quarter_label(&self) -> &str {
        match self.available_quarter() {
            1 => "Q1 (Jan-Mar)",
            2 => "Q2 (Apr-Jun)",
            3 => "Q3 (Jul-Sep)",
            4 => "Q4 (Oct-Dec)",
            _ => "?",
        }
    }

    pub fn base_cost(&self) -> f64 {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => 5_000_000.0,
            SeasonalPromotionType::SummerHomeMakeover => 4_000_000.0,
            SeasonalPromotionType::RainySeasonPrep => 6_000_000.0,
            SeasonalPromotionType::BarangayFiestaDeals => 3_000_000.0,
            SeasonalPromotionType::SchoolSeasonSale => 4_000_000.0,
            SeasonalPromotionType::PaydayWeekendBonanza => 5_000_000.0,
            SeasonalPromotionType::HalloweenDecorSale => 3_000_000.0,
            SeasonalPromotionType::ChristmasMegaSale => 10_000_000.0,
            SeasonalPromotionType::YearEndClearance => 2_000_000.0,
        }
    }

    pub fn revenue_boost(&self) -> f64 {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => 0.15,
            SeasonalPromotionType::SummerHomeMakeover => 0.12,
            SeasonalPromotionType::RainySeasonPrep => 0.18,
            SeasonalPromotionType::BarangayFiestaDeals => 0.10,
            SeasonalPromotionType::SchoolSeasonSale => 0.08,
            SeasonalPromotionType::PaydayWeekendBonanza => 0.14,
            SeasonalPromotionType::HalloweenDecorSale => 0.08,
            SeasonalPromotionType::ChristmasMegaSale => 0.25,
            SeasonalPromotionType::YearEndClearance => 0.15,
        }
    }

    pub fn reputation_boost(&self) -> f64 {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => 1.5,
            SeasonalPromotionType::SummerHomeMakeover => 1.0,
            SeasonalPromotionType::RainySeasonPrep => 2.0,
            SeasonalPromotionType::BarangayFiestaDeals => 2.5,
            SeasonalPromotionType::SchoolSeasonSale => 0.5,
            SeasonalPromotionType::PaydayWeekendBonanza => 1.0,
            SeasonalPromotionType::HalloweenDecorSale => 0.5,
            SeasonalPromotionType::ChristmasMegaSale => 3.0,
            SeasonalPromotionType::YearEndClearance => -1.0,
        }
    }

    pub fn satisfaction_boost(&self) -> f64 {
        match self {
            SeasonalPromotionType::ChineseNewYearSale => 2.0,
            SeasonalPromotionType::SummerHomeMakeover => 1.5,
            SeasonalPromotionType::RainySeasonPrep => 1.0,
            SeasonalPromotionType::BarangayFiestaDeals => 3.0,
            SeasonalPromotionType::SchoolSeasonSale => 1.0,
            SeasonalPromotionType::PaydayWeekendBonanza => 2.0,
            SeasonalPromotionType::HalloweenDecorSale => 1.0,
            SeasonalPromotionType::ChristmasMegaSale => 3.0,
            SeasonalPromotionType::YearEndClearance => 1.0,
        }
    }

    pub fn all_types() -> Vec<SeasonalPromotionType> {
        vec![
            SeasonalPromotionType::ChineseNewYearSale,
            SeasonalPromotionType::SummerHomeMakeover,
            SeasonalPromotionType::RainySeasonPrep,
            SeasonalPromotionType::BarangayFiestaDeals,
            SeasonalPromotionType::SchoolSeasonSale,
            SeasonalPromotionType::PaydayWeekendBonanza,
            SeasonalPromotionType::HalloweenDecorSale,
            SeasonalPromotionType::ChristmasMegaSale,
            SeasonalPromotionType::YearEndClearance,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "cny_sale" => Some(SeasonalPromotionType::ChineseNewYearSale),
            "summer_makeover" => Some(SeasonalPromotionType::SummerHomeMakeover),
            "rainy_prep" => Some(SeasonalPromotionType::RainySeasonPrep),
            "fiesta_deals" => Some(SeasonalPromotionType::BarangayFiestaDeals),
            "school_sale" => Some(SeasonalPromotionType::SchoolSeasonSale),
            "payday_bonanza" => Some(SeasonalPromotionType::PaydayWeekendBonanza),
            "halloween_decor" => Some(SeasonalPromotionType::HalloweenDecorSale),
            "christmas_sale" => Some(SeasonalPromotionType::ChristmasMegaSale),
            "year_end_clearance" => Some(SeasonalPromotionType::YearEndClearance),
            _ => None,
        }
    }

    pub fn color_class(&self) -> &str {
        match self.available_quarter() {
            1 => "text-red-400",
            2 => "text-yellow-400",
            3 => "text-green-400",
            4 => "text-blue-400",
            _ => "text-gray-400",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSeasonalPromotion {
    pub id: String,
    pub promotion_type: SeasonalPromotionType,
    pub start_quarter: i32,
    pub start_year: i32,
}

pub fn activate_promotion(
    state: &mut GameState,
    promo_type: SeasonalPromotionType,
) -> Result<f64, &'static str> {
    let current_active_count = state
        .seasonal_promotions
        .iter()
        .filter(|p| p.start_quarter == state.current_quarter && p.start_year == state.current_year)
        .count();

    if current_active_count >= 2 {
        return Err("Maximum 2 seasonal promotions per quarter.");
    }

    if promo_type.available_quarter() != state.current_quarter {
        return Err("This promotion is not available in the current quarter.");
    }

    let already_active = state.seasonal_promotions.iter().any(|p| {
        p.promotion_type == promo_type
            && p.start_quarter == state.current_quarter
            && p.start_year == state.current_year
    });

    if already_active {
        return Err("This promotion is already active this quarter.");
    }

    let cost = promo_type.base_cost();
    if state.company.cash < cost {
        return Err("Not enough cash to activate this promotion.");
    }

    state.company.cash -= cost;

    let promotion = ActiveSeasonalPromotion {
        id: uuid::Uuid::new_v4().to_string(),
        promotion_type: promo_type,
        start_quarter: state.current_quarter,
        start_year: state.current_year,
    };
    state.seasonal_promotions.push(promotion);
    Ok(cost)
}

pub fn process_seasonal_promotions(state: &mut GameState) {
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let skill_bonus = 1.0 + cmo_skill * 0.005;

    let mut total_reputation = 0.0;
    let mut total_satisfaction = 0.0;

    for promo in &state.seasonal_promotions {
        let rep = promo.promotion_type.reputation_boost() * skill_bonus;
        let sat = promo.promotion_type.satisfaction_boost() * skill_bonus;
        total_reputation += rep;
        total_satisfaction += sat;
    }

    state.company.brand_reputation =
        (state.company.brand_reputation + total_reputation).clamp(10.0, 100.0);
    state.company.customer_satisfaction =
        (state.company.customer_satisfaction + total_satisfaction).clamp(10.0, 100.0);

    let expired: Vec<String> = state
        .seasonal_promotions
        .iter()
        .filter(|p| p.start_quarter != state.current_quarter || p.start_year != state.current_year)
        .map(|p| p.promotion_type.label().to_string())
        .collect();

    state
        .seasonal_promotions
        .retain(|p| p.start_quarter == state.current_quarter && p.start_year == state.current_year);

    for name in &expired {
        state.push_message(format!("Seasonal promotion '{}' has ended.", name));
    }
}

pub fn seasonal_revenue_multiplier(state: &GameState) -> f64 {
    let cmo_skill = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CMO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let skill_bonus = 1.0 + cmo_skill * 0.005;

    let current_quarter_active: f64 = state
        .seasonal_promotions
        .iter()
        .filter(|p| p.start_quarter == state.current_quarter && p.start_year == state.current_year)
        .map(|p| p.promotion_type.revenue_boost() * skill_bonus)
        .sum();

    1.0 + current_quarter_active
}

pub fn get_available_promotions_for_quarter(quarter: i32) -> Vec<SeasonalPromotionType> {
    SeasonalPromotionType::all_types()
        .into_iter()
        .filter(|t| t.available_quarter() == quarter)
        .collect()
}
