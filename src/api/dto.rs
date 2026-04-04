use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewStoreForm {
    pub city: String,
    pub store_type: String,
    pub store_name: String,
}

#[derive(Debug, Deserialize)]
pub struct HireExecutiveForm {
    pub position: String,
}

#[derive(Debug, Deserialize)]
pub struct PolicyForm {
    pub pricing: String,
    pub hr: String,
    pub expansion: String,
    pub customer_service: String,
    pub marketing: String,
    pub inventory: String,
}

#[derive(Debug, Deserialize)]
pub struct LoanForm {
    pub amount: String,
    pub quarters: String,
}

#[derive(Debug, Deserialize)]
pub struct ResolveEventForm {
    pub event_id: String,
    pub choice_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DelegationForm {
    pub crisis: Option<String>,
    pub financial: Option<String>,
    pub marketing: Option<String>,
    pub hr: Option<String>,
    pub supply_chain: Option<String>,
    pub competition: Option<String>,
    pub technology: Option<String>,
    pub regulation: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FinancialRow {
    pub quarter: String,
    pub revenue: String,
    pub expenses: String,
    pub profit: String,
    pub tax: String,
    pub stores: u32,
    pub employees: u32,
    pub market_share: String,
    pub satisfaction: String,
    pub profit_class: String,
}

#[derive(Debug, Serialize)]
pub struct StoreRow {
    pub id: String,
    pub name: String,
    pub city: String,
    pub region: String,
    pub store_type: String,
    pub size_sqm: u32,
    pub status: String,
    pub status_class: String,
    pub quarterly_revenue: String,
    pub quarterly_expenses: String,
    pub quarterly_profit: String,
    pub employees: u32,
    pub satisfaction: String,
    pub age: String,
    pub can_close: bool,
}

#[derive(Debug, Serialize)]
pub struct ExecutiveRow {
    pub id: String,
    pub name: String,
    pub position: String,
    pub short_position: String,
    pub skill: String,
    pub salary: String,
    pub morale: String,
    pub loyalty: String,
    pub performance: String,
    pub tenure: String,
    pub recommendation: String,
    pub morale_class: String,
    pub loyalty_class: String,
    pub performance_class: String,
}

#[derive(Debug, Serialize)]
pub struct CityOption {
    pub name: String,
    pub region: String,
    pub rent: String,
    pub demand: String,
    pub population: String,
    pub competitor: String,
}

#[derive(Debug, Serialize)]
pub struct StoreTypeOption {
    pub key: String,
    pub label: String,
    pub size: u32,
    pub cost: String,
    pub construction: i32,
}

#[derive(Debug, Serialize)]
pub struct LoanInfo {
    pub id: String,
    pub amount: String,
    pub remaining: String,
    pub rate: String,
    pub quarterly_payment: String,
    pub quarters_left: i32,
}

#[derive(Debug, Serialize)]
pub struct EventRow {
    pub title: String,
    pub icon: String,
    pub quarter: String,
}

#[derive(Debug, Serialize)]
pub struct PendingEventRow {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub category_icon: String,
    pub choices: Vec<ChoiceOption>,
}

#[derive(Debug, Serialize)]
pub struct ChoiceOption {
    pub id: String,
    pub label: String,
    pub description: String,
    pub risk_level: String,
    pub risk_class: String,
    pub risk_css: String,
    pub cash: String,
    pub morale: String,
    pub reputation: String,
    pub satisfaction: String,
}

#[derive(Debug, Serialize)]
pub struct DelegationRow {
    pub category: String,
    pub key: String,
    pub icon: String,
    pub delegate_to: String,
    pub is_delegated: bool,
    pub has_executive: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProductInvestForm {
    pub category_id: String,
    pub amount: String,
}

#[derive(Debug, Serialize)]
pub struct ProductRow {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub margin: String,
    pub demand: String,
    pub investment: String,
    pub investment_pct: f64,
    pub trend: String,
    pub trend_class: String,
}

#[derive(Debug, Serialize)]
pub struct UpgradeStoreRow {
    pub store_id: String,
    pub store_name: String,
    pub store_city: String,
    pub upgrade_rows: Vec<SingleUpgradeRow>,
}

#[derive(Debug, Serialize)]
pub struct SingleUpgradeRow {
    pub upgrade_type: String,
    pub upgrade_key: String,
    pub icon: String,
    pub description: String,
    pub current_level: u32,
    pub max_level: u32,
    pub cost: String,
    pub revenue_effect: String,
    pub cost_effect: String,
    pub sat_effect: String,
    pub can_upgrade: bool,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseUpgradeForm {
    pub store_id: String,
    pub upgrade_type: String,
}

#[derive(Debug, Serialize)]
pub struct CompetitorRow {
    pub name: String,
    pub store_count: u32,
    pub strength: String,
    pub strategy: String,
    pub market_share: String,
    pub recent_action: String,
    pub quarters_since: i32,
}

#[derive(Debug, Serialize)]
pub struct AchievementRow {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
    pub unlocked_quarter: String,
}

#[derive(Debug, Serialize)]
pub struct BoardInfo {
    pub patience: String,
    pub patience_class: String,
    pub patience_color: String,
    pub pressure: String,
    pub pressure_class: String,
    pub pressure_color: String,
    pub warnings: u32,
    pub description: String,
    pub last_review: String,
    pub quarters_until_review: i32,
}

#[derive(Debug, Serialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub revenue: Vec<f64>,
    pub expenses: Vec<f64>,
    pub profit: Vec<f64>,
    pub cash: Vec<f64>,
    pub market_share: Vec<f64>,
    pub customer_sat: Vec<f64>,
    pub employee_sat: Vec<f64>,
    pub brand_rep: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct LoyaltyTierRow {
    pub key: String,
    pub name: String,
    pub icon_char: String,
    pub color_class: String,
    pub description: String,
    pub setup_cost: String,
    pub quarterly_cost: String,
    pub revenue_bonus: String,
    pub sat_bonus: String,
    pub is_current: bool,
    pub can_select: bool,
    pub show_reason: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct LoyaltyTierForm {
    pub tier: String,
}

#[derive(Debug, Deserialize)]
pub struct CampaignForm {
    pub campaign_type: String,
}

#[derive(Debug, Serialize)]
pub struct CampaignRow {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub quarters_remaining: i32,
    pub quarters_total: i32,
    pub revenue_boost: String,
    pub reputation_boost: String,
    pub satisfaction_boost: String,
    pub started: String,
}

#[derive(Debug, Serialize)]
pub struct CampaignOption {
    pub key: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub cost: String,
    pub duration: i32,
    pub revenue_boost: String,
    pub reputation_boost: String,
    pub satisfaction_boost: String,
    pub can_launch: bool,
    pub reason: String,
    pub show_reason: bool,
}

#[derive(Debug, Deserialize)]
pub struct EcommerceForm {
    pub level: String,
}

#[derive(Debug, Serialize)]
pub struct EcommerceLevelRow {
    pub key: String,
    pub name: String,
    pub icon: String,
    pub color_class: String,
    pub description: String,
    pub setup_cost: String,
    pub quarterly_cost: String,
    pub revenue_bonus: String,
    pub satisfaction_bonus: String,
    pub reputation_bonus: String,
    pub min_stores: u32,
    pub is_current: bool,
    pub can_select: bool,
    pub show_reason: bool,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct SupplierRow {
    pub id: String,
    pub name: String,
    pub category: String,
    pub region: String,
    pub reliability: String,
    pub cost_modifier: String,
    pub lead_time: i32,
    pub quarters_remaining: i32,
    pub relationship: String,
    pub relationship_class: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct SupplierCategoryOption {
    pub key: String,
    pub name: String,
    pub can_negotiate: bool,
}

#[derive(Debug, Deserialize)]
pub struct NegotiateSupplierForm {
    pub category: String,
}

#[derive(Debug, Serialize)]
pub struct LogisticsLevelRow {
    pub key: String,
    pub name: String,
    pub description: String,
    pub quarterly_cost: String,
    pub cost_reduction: String,
    pub reliability: String,
    pub stockout_reduction: String,
    pub min_stores: u32,
    pub is_current: bool,
    pub can_select: bool,
    pub show_reason: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct LogisticsForm {
    pub level: String,
}

#[derive(Debug, Serialize)]
pub struct WarehouseTierRow {
    pub key: String,
    pub name: String,
    pub description: String,
    pub setup_cost: String,
    pub quarterly_cost: String,
    pub bulk_discount: String,
    pub stockout_reduction: String,
    pub min_stores: u32,
    pub is_current: bool,
    pub can_select: bool,
    pub show_reason: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct WarehouseForm {
    pub tier: String,
}

#[derive(Debug, Serialize)]
pub struct DeliveryServiceLevelRow {
    pub key: String,
    pub name: String,
    pub description: String,
    pub setup_cost: String,
    pub quarterly_cost: String,
    pub revenue_bonus: String,
    pub satisfaction_bonus: String,
    pub min_stores: u32,
    pub is_current: bool,
    pub can_select: bool,
    pub show_reason: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryForm {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct PrivateLabelForm {
    pub category_id: String,
}

#[derive(Debug, Serialize)]
pub struct PrivateLabelRow {
    pub id: String,
    pub brand_name: String,
    pub category: String,
    pub status: String,
    pub status_class: String,
    pub development_progress: String,
    pub quarters_remaining: i32,
    pub brand_power: String,
    pub brand_power_class: String,
    pub margin_rate: String,
    pub quarterly_revenue: String,
    pub total_revenue: String,
    pub quarterly_cost: String,
}

#[derive(Debug, Serialize)]
pub struct PrivateLabelCategoryOption {
    pub category_id: String,
    pub category_name: String,
    pub development_cost: String,
    pub development_quarters: i32,
    pub base_margin: String,
    pub quarterly_cost: String,
    pub can_start: bool,
    pub reason: String,
    pub show_reason: bool,
}

#[derive(Debug, Serialize)]
pub struct SeasonalPromoRow {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub revenue_boost: String,
    pub reputation_boost: String,
    pub satisfaction_boost: String,
    pub started: String,
    pub color_class: String,
}

#[derive(Debug, Serialize)]
pub struct SeasonalPromoOption {
    pub key: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub cost: String,
    pub quarter_label: String,
    pub revenue_boost: String,
    pub reputation_boost: String,
    pub satisfaction_boost: String,
    pub color_class: String,
    pub can_activate: bool,
    pub reason: String,
    pub show_reason: bool,
}

#[derive(Debug, Deserialize)]
pub struct SeasonalPromoForm {
    pub promotion_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ResearchForm {
    pub track: String,
}

#[derive(Debug, Serialize)]
pub struct ResearchTrackRow {
    pub key: String,
    pub name: String,
    pub icon: String,
    pub color_class: String,
    pub description: String,
    pub current_level: u32,
    pub max_level: u32,
    pub progress: f64,
    pub progress_pct: String,
    pub quarters_remaining: i32,
    pub is_researching: bool,
    pub effect_description: String,
    pub next_cost: String,
    pub next_quarters: i32,
    pub can_start: bool,
    pub reason: String,
    pub show_reason: bool,
    pub min_stores: u32,
}

#[derive(Debug, Deserialize)]
pub struct CsrForm {
    pub initiative: String,
}

#[derive(Debug, Deserialize)]
pub struct CsrDiscontinueForm {
    pub initiative_id: String,
}

#[derive(Debug, Serialize)]
pub struct CsrInitiativeRow {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub quarterly_cost: String,
    pub quarters_active: i32,
    pub total_invested: String,
    pub reputation_bonus: String,
    pub satisfaction_bonus: String,
    pub morale_bonus: String,
}

#[derive(Debug, Serialize)]
pub struct CsrOptionRow {
    pub key: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub setup_cost: String,
    pub quarterly_cost: String,
    pub reputation_bonus: String,
    pub satisfaction_bonus: String,
    pub morale_bonus: String,
    pub tax_deduction: String,
    pub min_stores: u32,
    pub can_launch: bool,
    pub reason: String,
    pub show_reason: bool,
}

#[derive(Debug, Deserialize)]
pub struct HireEmployeeForm {
    pub role: String,
    pub store_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FireEmployeeForm {
    pub employee_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TrainEmployeeForm {
    pub employee_id: String,
    pub training_type: String,
}

#[derive(Debug, Deserialize)]
pub struct RaiseEmployeeForm {
    pub employee_id: String,
    pub raise_percentage: String,
}

#[derive(Debug, Deserialize)]
pub struct TrainRoleForm {
    pub role: String,
    pub training_type: String,
}

#[derive(Debug, Serialize)]
pub struct EmployeeRow {
    pub id: String,
    pub name: String,
    pub role_key: String,
    pub role_name: String,
    pub category: String,
    pub store_name: String,
    pub salary_monthly: String,
    pub skill: String,
    pub morale: String,
    pub morale_class: String,
    pub performance: String,
    pub performance_class: String,
    pub tenure: String,
    pub training_count: u32,
}

#[derive(Debug, Serialize)]
pub struct RoleOption {
    pub key: String,
    pub name: String,
    pub category: String,
    pub base_salary: String,
}

#[derive(Debug, Serialize)]
pub struct TrainingOptionRow {
    pub key: String,
    pub name: String,
    pub cost_per_employee: String,
    pub skill_bonus: String,
    pub morale_bonus: String,
}

#[derive(Debug, Serialize)]
pub struct EmployeeStats {
    pub total_count: u32,
    pub monthly_payroll: String,
    pub avg_morale: String,
    pub avg_skill: String,
    pub turnover_rate: String,
}

#[derive(Debug, Deserialize)]
pub struct AdCampaignForm {
    pub budget: String,
    pub target_audience: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelAdCampaignForm {
    pub campaign_id: String,
}

#[derive(Debug, Serialize)]
pub struct AdCampaignRow {
    pub id: String,
    pub name: String,
    pub budget: String,
    pub budget_key: String,
    pub target_audience: String,
    pub audience_key: String,
    pub quarters_active: i32,
    pub total_spent: String,
    pub total_revenue: String,
    pub roi: String,
    pub roi_class: String,
    pub impressions: String,
    pub clicks: String,
    pub conversions: String,
    pub ctr: String,
    pub conversion_rate: String,
}

#[derive(Debug, Serialize)]
pub struct AdCampaignOption {
    pub budget_key: String,
    pub budget_name: String,
    pub audience_key: String,
    pub audience_name: String,
    pub audience_description: String,
    pub quarterly_cost: String,
    pub base_roi: String,
    pub base_conversion: String,
    pub avg_purchase: String,
    pub can_launch: bool,
    pub reason: String,
    pub show_reason: bool,
}
