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
