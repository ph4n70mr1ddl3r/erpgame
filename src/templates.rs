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
    pub active_page: String,
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
