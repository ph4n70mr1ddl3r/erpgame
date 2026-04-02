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
