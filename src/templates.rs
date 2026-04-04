use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub company_name: String,
    pub cash: String,
    pub cash_full: String,
    pub quarterly_revenue: String,
    pub quarterly_expenses: String,
    pub quarterly_profit: String,
    pub company_value: String,
    pub market_share: String,
    pub customer_satisfaction: String,
    pub employee_satisfaction: String,
    pub brand_reputation: String,
    pub store_count: u32,
    pub employee_count: u32,
    pub executive_count: usize,
    pub current_quarter: String,
    pub next_quarter: String,
    pub game_over: bool,
    pub messages: Vec<String>,
    pub financial_history: Vec<crate::api::dto::FinancialRow>,
    pub economy_gdp: String,
    pub economy_inflation: String,
    pub economy_interest: String,
    pub economy_description: String,
    pub competition_description: String,
    pub seasonal_multiplier: String,
    pub board_patience: String,
    pub board_patience_class: String,
    pub board_patience_color: String,
    pub active_page: String,
    pub chart_json: String,
    pub achievements_unlocked: usize,
    pub achievements_total: usize,
}

#[derive(Template)]
#[template(path = "stores.html")]
pub struct StoresTemplate {
    pub store_rows: Vec<crate::api::dto::StoreRow>,
    pub cities: Vec<crate::api::dto::CityOption>,
    pub store_types: Vec<crate::api::dto::StoreTypeOption>,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "executives.html")]
pub struct ExecutivesTemplate {
    pub executives: Vec<crate::api::dto::ExecutiveRow>,
    pub open_positions: Vec<String>,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "policies.html")]
pub struct PoliciesTemplate {
    pub pricing: String,
    pub pricing_key: String,
    pub hr: String,
    pub hr_key: String,
    pub expansion: String,
    pub expansion_key: String,
    pub customer_service: String,
    pub customer_service_key: String,
    pub marketing: String,
    pub marketing_key: String,
    pub inventory: String,
    pub inventory_key: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "finances.html")]
pub struct FinancesTemplate {
    pub cash: String,
    pub company_value: String,
    pub total_revenue: String,
    pub total_expenses: String,
    pub total_profit: String,
    pub monthly_payroll: String,
    pub executive_payroll: String,
    pub total_loans: String,
    pub tax_rate: String,
    pub interest_rate: String,
    pub financial_history: Vec<crate::api::dto::FinancialRow>,
    pub loans: Vec<crate::api::dto::LoanInfo>,
    pub max_loan: String,
    pub suggested_rate: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "events.html")]
pub struct EventsTemplate {
    pub events: Vec<crate::api::dto::EventRow>,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "decisions.html")]
pub struct DecisionsTemplate {
    pub pending_events: Vec<crate::api::dto::PendingEventRow>,
    pub decisions_made: u32,
    pub decisions_delegated: u32,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "delegation.html")]
pub struct DelegationTemplate {
    pub rows: Vec<crate::api::dto::DelegationRow>,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "products.html")]
pub struct ProductsTemplate {
    pub rows: Vec<crate::api::dto::ProductRow>,
    pub cash: String,
    pub total_invested: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "upgrades.html")]
pub struct UpgradesTemplate {
    pub store_rows: Vec<crate::api::dto::UpgradeStoreRow>,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "board.html")]
pub struct BoardTemplate {
    pub board: crate::api::dto::BoardInfo,
    pub company_value: String,
    pub market_share: String,
    pub competitor_share: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "competitors.html")]
pub struct CompetitorsTemplate {
    pub rows: Vec<crate::api::dto::CompetitorRow>,
    pub player_share: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "loyalty.html")]
pub struct LoyaltyTemplate {
    pub current_tier: String,
    pub current_tier_class: String,
    pub members: String,
    pub member_penetration: String,
    pub effective_revenue_bonus: String,
    pub quarters_active: i32,
    pub quarterly_cost: String,
    pub satisfaction_bonus: String,
    pub points_multiplier: String,
    pub growth_rate: String,
    pub tiers: Vec<crate::api::dto::LoyaltyTierRow>,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "achievements.html")]
pub struct AchievementsTemplate {
    pub rows: Vec<crate::api::dto::AchievementRow>,
    pub total: usize,
    pub unlocked: usize,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "campaigns.html")]
pub struct CampaignsTemplate {
    pub active_campaigns: Vec<crate::api::dto::CampaignRow>,
    pub options: Vec<crate::api::dto::CampaignOption>,
    pub effective_revenue_bonus: String,
    pub cmo_skill: String,
    pub active_count: usize,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "ecommerce.html")]
pub struct EcommerceTemplate {
    pub current_level: String,
    pub current_level_class: String,
    pub quarters_active: i32,
    pub quarterly_online_revenue: String,
    pub total_online_revenue: String,
    pub conversion_rate: String,
    pub quarterly_cost: String,
    pub effective_revenue_bonus: String,
    pub cto_skill: String,
    pub levels: Vec<crate::api::dto::EcommerceLevelRow>,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "supply_chain.html")]
pub struct SupplyChainTemplate {
    pub supplier_rows: Vec<crate::api::dto::SupplierRow>,
    pub category_options: Vec<crate::api::dto::SupplierCategoryOption>,
    pub active_supplier_count: usize,
    pub max_suppliers: usize,
    pub logistics_levels: Vec<crate::api::dto::LogisticsLevelRow>,
    pub warehouse_tiers: Vec<crate::api::dto::WarehouseTierRow>,
    pub delivery_service_levels: Vec<crate::api::dto::DeliveryServiceLevelRow>,
    pub stockout_rate: String,
    pub avg_delivery_time: String,
    #[allow(dead_code)]
    pub quarterly_logistics_cost: String,
    #[allow(dead_code)]
    pub total_quarterly_cost: String,
    pub total_supply_savings: String,
    pub last_stockout_penalty: String,
    pub quarters_since_disruption: i32,
    pub csco_skill: String,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "private_label.html")]
pub struct PrivateLabelTemplate {
    pub brand_rows: Vec<crate::api::dto::PrivateLabelRow>,
    pub category_options: Vec<crate::api::dto::PrivateLabelCategoryOption>,
    pub active_count: usize,
    pub developing_count: usize,
    pub total_quarterly_revenue: String,
    pub total_pl_revenue: String,
    pub total_quarterly_cost: String,
    pub cmo_skill: String,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "seasonal.html")]
pub struct SeasonalTemplate {
    pub active_promotions: Vec<crate::api::dto::SeasonalPromoRow>,
    pub available_options: Vec<crate::api::dto::SeasonalPromoOption>,
    pub current_quarter_label: String,
    pub active_count: usize,
    pub effective_revenue_bonus: String,
    pub cmo_skill: String,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "research.html")]
pub struct ResearchTemplate {
    pub tracks: Vec<crate::api::dto::ResearchTrackRow>,
    pub has_active: bool,
    pub active_track_name: String,
    pub active_progress: f64,
    pub active_progress_pct: String,
    pub active_quarters_remaining: i32,
    pub total_invested: String,
    pub completed_count: u32,
    pub cto_skill: String,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "csr.html")]
pub struct CsrTemplate {
    pub active_initiatives: Vec<crate::api::dto::CsrInitiativeRow>,
    pub options: Vec<crate::api::dto::CsrOptionRow>,
    pub csr_score: String,
    pub active_count: usize,
    pub total_donated: String,
    pub quarterly_cost: String,
    pub tax_deduction: String,
    pub chro_skill: String,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "employees.html")]
pub struct EmployeesTemplate {
    pub employees: Vec<crate::api::dto::EmployeeRow>,
    pub role_options: Vec<crate::api::dto::RoleOption>,
    pub stats: crate::api::dto::EmployeeStats,
    pub stores: Vec<crate::api::dto::StoreRow>,
    pub cash: String,
    pub messages: Vec<String>,
    pub current_quarter: String,
    pub active_page: String,
}
