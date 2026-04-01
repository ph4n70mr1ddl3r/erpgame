use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use rand::Rng;

use crate::game::{
    CustomerServicePolicy, Executive, ExecutivePosition, ExpansionPolicy, GameState,
    HrPolicy, InventoryPolicy, Loan, MarketingPolicy, PricingPolicy, Region, Store,
    StoreStatus, StoreType, format_currency, format_currency_full, generate_executive_name,
    get_available_cities, pct, simulate_quarter,
};
use super::dto::*;

pub type AppState = Arc<Mutex<GameState>>;

pub async fn dashboard(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let last_report = s.financial_history.last();
    let operating_count = s.operating_store_count();

    let (q_rev, q_exp, q_prof) = if let Some(r) = last_report {
        (
            format_currency(r.revenue),
            format_currency(r.expenses),
            format_currency(r.profit),
        )
    } else {
        ("-".into(), "-".into(), "-".into())
    };

    let financial_history: Vec<FinancialRow> = s
        .financial_history
        .iter()
        .rev()
        .take(20)
        .map(report_to_row)
        .collect();

    let next_q = if s.current_quarter >= 4 {
        format!("Q1 {}", s.current_year + 1)
    } else {
        format!("Q{} {}", s.current_quarter + 1, s.current_year)
    };

    let data = crate::templates::DashboardTemplate {
        company_name: s.company.name.clone(),
        cash: format_currency(s.company.cash),
        cash_full: format_currency_full(s.company.cash),
        quarterly_revenue: q_rev,
        quarterly_expenses: q_exp,
        quarterly_profit: q_prof,
        company_value: format_currency(s.company.company_value),
        market_share: pct(s.company.market_share),
        customer_satisfaction: pct(s.company.customer_satisfaction),
        employee_satisfaction: pct(s.company.employee_satisfaction),
        brand_reputation: pct(s.company.brand_reputation),
        store_count: operating_count,
        employee_count: s.employees.total_count,
        executive_count: s.executives.len(),
        current_quarter: s.advance_quarter_label(),
        next_quarter: next_q,
        game_over: s.game_over,
        messages: s.messages.clone(),
        financial_history,
        economy_gdp: pct(s.economy.gdp_growth_rate),
        economy_inflation: pct(s.economy.inflation_rate),
        economy_interest: pct(s.economy.interest_rate),
        economy_description: s.economy.description(),
        competition_description: s.market.competition_description(),
        seasonal_multiplier: format!("{:.2}x", s.market.seasonal_multiplier),
        active_page: "dashboard".to_string(),
    };

    data.into_response()
}

pub async fn stores_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let can_close_any = s.stores.iter().filter(|x| x.status == StoreStatus::Operating).count() > 1;

    let store_rows: Vec<StoreRow> = s
        .stores
        .iter()
        .map(|st| {
            let profit = st.quarterly_revenue - st.quarterly_expenses;
            StoreRow {
                id: st.id.clone(),
                name: st.name.clone(),
                city: st.city.clone(),
                region: format!("{:?}", st.region),
                store_type: st.store_type.label().into(),
                size_sqm: st.size_sqm,
                status: st.status.label().into(),
                status_class: st.status.css_class().into(),
                quarterly_revenue: format_currency(st.quarterly_revenue),
                quarterly_expenses: format_currency(st.quarterly_expenses),
                quarterly_profit: format_currency(profit),
                employees: st.employee_count,
                satisfaction: pct(st.satisfaction),
                age: if st.age_quarters < 4 {
                    format!("{} Q", st.age_quarters)
                } else {
                    format!("{} Y", st.age_quarters / 4)
                },
                can_close: st.status == StoreStatus::Operating && can_close_any,
            }
        })
        .collect();

    let occupied_cities: std::collections::HashSet<String> = s
        .stores
        .iter()
        .filter(|st| st.status != StoreStatus::Closed)
        .map(|st| st.city.clone())
        .collect();

    let cities: Vec<CityOption> = get_available_cities()
        .iter()
        .filter(|c| !occupied_cities.contains(&c.name))
        .map(|c| CityOption {
            name: c.name.clone(),
            region: format!("{:?}", c.region),
            rent: format!("{}sqm/mo", c.rent_per_sqm),
            demand: format!("{:.1}x", c.demand_factor),
            population: format!("{}", c.population),
            competitor: if c.has_competitor {
                "Yes".to_string()
            } else {
                "No".to_string()
            },
        })
        .collect();

    let store_types: Vec<StoreTypeOption> = vec![
        StoreTypeOption {
            key: "express".into(),
            label: "Express".into(),
            size: 800,
            cost: format_currency(StoreType::Express.opening_cost()),
            construction: StoreType::Express.construction_quarters(),
        },
        StoreTypeOption {
            key: "standard".into(),
            label: "Standard".into(),
            size: 3500,
            cost: format_currency(StoreType::Standard.opening_cost()),
            construction: StoreType::Standard.construction_quarters(),
        },
        StoreTypeOption {
            key: "mega".into(),
            label: "Mega".into(),
            size: 12000,
            cost: format_currency(StoreType::Mega.opening_cost()),
            construction: StoreType::Mega.construction_quarters(),
        },
        StoreTypeOption {
            key: "depot".into(),
            label: "Depot".into(),
            size: 18000,
            cost: format_currency(StoreType::Depot.opening_cost()),
            construction: StoreType::Depot.construction_quarters(),
        },
    ];

    crate::templates::StoresTemplate {
        store_rows,
        cities,
        store_types,
        cash: format_currency_full(s.company.cash),
        messages: s.messages.clone(),
        current_quarter: s.advance_quarter_label(),
        active_page: "stores".to_string(),
    }
    .into_response()
}

pub async fn open_store(State(state): State<AppState>, Form(form): Form<NewStoreForm>) -> Response {
    let mut state = state.lock().await;
    let store_type = match form.store_type.as_str() {
        "express" => StoreType::Express,
        "standard" => StoreType::Standard,
        "mega" => StoreType::Mega,
        "depot" => StoreType::Depot,
        _ => StoreType::Standard,
    };

    let cost = store_type.opening_cost();
    let current_cash = state.company.cash;
    if current_cash < cost {
        state.messages.push(format!(
            "Cannot open store: need {} but only have {}",
            format_currency(cost),
            format_currency(current_cash)
        ));
        return Redirect::to("/stores").into_response();
    }

    let cities = get_available_cities();
    let region = cities
        .iter()
        .find(|c| c.name == form.city)
        .map(|c| c.region)
        .unwrap_or(Region::Luzon);

    state.company.cash -= cost;

    let store_name = if form.store_name.is_empty() {
        format!("Bahay Depot {}", form.city)
    } else {
        form.store_name.clone()
    };

    let store = Store {
        id: uuid::Uuid::new_v4().to_string(),
        name: store_name,
        city: form.city.clone(),
        region,
        store_type,
        size_sqm: store_type.default_size(),
        status: StoreStatus::UnderConstruction,
        quarterly_revenue: 0.0,
        quarterly_expenses: 0.0,
        customer_count: 0,
        employee_count: 0,
        satisfaction: 50.0,
        age_quarters: 0,
        construction_quarters_left: store_type.construction_quarters(),
        opened_quarter: 0,
        opened_year: 0,
    };

    state.messages.push(format!(
        "Breaking ground on {} in {} (Cost: {}, Opens in {} quarters)",
        store.name,
        store.city,
        format_currency(cost),
        store.construction_quarters_left
    ));
    state.stores.push(store);

    Redirect::to("/stores").into_response()
}

pub async fn close_store(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let mut state = state.lock().await;
    let store_idx = state.stores.iter().position(|s| s.id == id);
    if let Some(idx) = store_idx {
        let sell_value = state.stores[idx].store_type.opening_cost() * 0.4;
        let store_name = state.stores[idx].name.clone();
        let store_city = state.stores[idx].city.clone();
        state.stores[idx].status = StoreStatus::Closed;
        state.company.cash += sell_value;
        state.messages.push(format!(
            "Closed {} in {}. Received {} from asset sale.",
            store_name,
            store_city,
            format_currency(sell_value)
        ));
    }
    Redirect::to("/stores").into_response()
}

pub async fn executives_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let exec_rows: Vec<ExecutiveRow> = s
        .executives
        .iter()
        .map(|e| ExecutiveRow {
            id: e.id.clone(),
            name: e.name.clone(),
            position: e.position.title().into(),
            short_position: e.position.short_title().into(),
            skill: format!("{:.0}/100", e.skill),
            salary: format!("{}/mo", format_currency_full(e.salary_monthly)),
            morale: format!("{:.0}%", e.morale),
            loyalty: format!("{:.0}%", e.loyalty),
            performance: format!("{:.0}/100", e.performance_rating),
            tenure: if e.tenure_quarters < 4 {
                format!("{} Q", e.tenure_quarters)
            } else {
                format!("{} Y", e.tenure_quarters / 4)
            },
            recommendation: e.recommendation.clone().unwrap_or_default(),
            morale_class: rating_class(e.morale, 50.0),
            loyalty_class: rating_class(e.loyalty, 50.0),
            performance_class: rating_class(e.performance_rating, 60.0),
        })
        .collect();

    let mut open_positions = Vec::new();
    for pos in ExecutivePosition::all_positions() {
        if !s.is_executive_hired(pos) {
            open_positions.push(pos.title().to_string());
        }
    }

    crate::templates::ExecutivesTemplate {
        executives: exec_rows,
        open_positions,
        cash: format_currency_full(s.company.cash),
        messages: s.messages.clone(),
        current_quarter: s.advance_quarter_label(),
        active_page: "executives".to_string(),
    }
    .into_response()
}

pub async fn hire_executive(
    State(state): State<AppState>,
    Form(form): Form<HireExecutiveForm>,
) -> Response {
    let mut state = state.lock().await;
    let position = match form.position.as_str() {
        "cfo" => ExecutivePosition::CFO,
        "coo" => ExecutivePosition::COO,
        "cmo" => ExecutivePosition::CMO,
        "cto" => ExecutivePosition::CTO,
        "chro" => ExecutivePosition::CHRO,
        "csco" => ExecutivePosition::CSCO,
        _ => return Redirect::to("/executives").into_response(),
    };

    if state.is_executive_hired(position) {
        state.messages.push(format!(
            "{} position is already filled.",
            position.short_title()
        ));
        return Redirect::to("/executives").into_response();
    }

    let mut rng = rand::thread_rng();
    let name = generate_executive_name(&mut rng);
    let skill = rng.gen_range(40.0..95.0);
    let (min_sal, max_sal) = position.salary_range();
    let salary = rng.gen_range(min_sal..max_sal);
    let hiring_bonus = salary * 3.0;

    if state.company.cash < hiring_bonus {
        state.messages.push(format!(
            "Cannot afford hiring bonus for {} ({})",
            position.short_title(),
            format_currency(hiring_bonus)
        ));
        return Redirect::to("/executives").into_response();
    }

    state.company.cash -= hiring_bonus;

    let exec = Executive {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        position,
        skill,
        salary_monthly: salary,
        morale: rng.gen_range(60.0..85.0),
        loyalty: rng.gen_range(50.0..80.0),
        tenure_quarters: 0,
        performance_rating: rng.gen_range(50.0..80.0),
        recommendation: None,
    };

    state.messages.push(format!(
        "Hired {} as {} (Skill: {:.0}/100, Salary: {}/mo, Hiring Bonus: {})",
        exec.name,
        exec.position.short_title(),
        exec.skill,
        format_currency_full(exec.salary_monthly),
        format_currency(hiring_bonus)
    ));
    state.executives.push(exec);

    Redirect::to("/executives").into_response()
}

pub async fn fire_executive(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let mut state = state.lock().await;
    if let Some(idx) = state.executives.iter().position(|e| e.id == id) {
        let exec_name = state.executives[idx].name.clone();
        let exec_pos = state.executives[idx].position.short_title().to_string();
        let severance = state.executives[idx].salary_monthly * 6.0;

        if state.company.cash < severance {
            let current_cash = format_currency(state.company.cash);
            state.messages.push(format!(
                "Cannot afford {} severance for {} ({}).",
                format_currency(severance),
                exec_name,
                current_cash
            ));
            return Redirect::to("/executives").into_response();
        }

        state.executives.remove(idx);
        state.company.cash -= severance;
        state.messages.push(format!(
            "Fired {} ({}). Paid {} severance.",
            exec_name,
            exec_pos,
            format_currency(severance)
        ));
    }
    Redirect::to("/executives").into_response()
}

pub async fn policies_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let p = &s.policies;

    crate::templates::PoliciesTemplate {
        pricing: p.pricing.label().into(),
        pricing_key: p.pricing.key().into(),
        hr: p.hr.label().into(),
        hr_key: p.hr.key().into(),
        expansion: p.expansion.label().into(),
        expansion_key: p.expansion.key().into(),
        customer_service: p.customer_service.label().into(),
        customer_service_key: p.customer_service.key().into(),
        marketing: p.marketing.label().into(),
        marketing_key: p.marketing.key().into(),
        inventory: p.inventory.label().into(),
        inventory_key: p.inventory.key().into(),
        messages: s.messages.clone(),
        current_quarter: s.advance_quarter_label(),
        active_page: "policies".to_string(),
    }
    .into_response()
}

pub async fn update_policies(State(state): State<AppState>, Form(form): Form<PolicyForm>) -> Response {
    let mut state = state.lock().await;

    state.policies.pricing = match form.pricing.as_str() {
        "budget" => PricingPolicy::Budget,
        "premium" => PricingPolicy::Premium,
        "dynamic" => PricingPolicy::Dynamic,
        _ => PricingPolicy::Competitive,
    };
    state.policies.hr = match form.hr.as_str() {
        "minimal" => HrPolicy::Minimal,
        "generous" => HrPolicy::Generous,
        "elite" => HrPolicy::Elite,
        _ => HrPolicy::Standard,
    };
    state.policies.expansion = match form.expansion.as_str() {
        "conservative" => ExpansionPolicy::Conservative,
        "aggressive" => ExpansionPolicy::Aggressive,
        "blitz" => ExpansionPolicy::Blitz,
        _ => ExpansionPolicy::Moderate,
    };
    state.policies.customer_service = match form.customer_service.as_str() {
        "basic" => CustomerServicePolicy::Basic,
        "excellent" => CustomerServicePolicy::Excellent,
        "whiteglove" => CustomerServicePolicy::WhiteGlove,
        _ => CustomerServicePolicy::Good,
    };
    state.policies.marketing = match form.marketing.as_str() {
        "lowkey" => MarketingPolicy::LowKey,
        "heavy" => MarketingPolicy::Heavy,
        "aggressive" => MarketingPolicy::Aggressive,
        _ => MarketingPolicy::Moderate,
    };
    state.policies.inventory = match form.inventory.as_str() {
        "lean" => InventoryPolicy::Lean,
        "buffered" => InventoryPolicy::Buffered,
        "abundant" => InventoryPolicy::Abundant,
        _ => InventoryPolicy::Standard,
    };

    state.messages.push("Company policies updated.".into());
    Redirect::to("/policies").into_response()
}

pub async fn tick(State(state): State<AppState>) -> Response {
    let mut state = state.lock().await;
    if !state.game_over {
        simulate_quarter(&mut state);
    }
    Redirect::to("/").into_response()
}

pub async fn finances_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let financial_history: Vec<FinancialRow> = s
        .financial_history
        .iter()
        .rev()
        .take(50)
        .map(report_to_row)
        .collect();

    let loans: Vec<LoanInfo> = s
        .company
        .loans
        .iter()
        .map(|l| LoanInfo {
            id: l.id.clone(),
            amount: format_currency_full(l.amount),
            remaining: format_currency_full(l.remaining),
            rate: pct(l.interest_rate),
            quarterly_payment: format_currency_full(l.quarterly_payment),
            quarters_left: l.quarters_remaining,
        })
        .collect();

    let total_loan_remaining: f64 = s.company.loans.iter().map(|l| l.remaining).sum();

    let max_loan = (s.company.company_value * 0.5).max(10_000_000.0);
    let suggested_rate = s.economy.interest_rate + 1.5;

    crate::templates::FinancesTemplate {
        cash: format_currency_full(s.company.cash),
        company_value: format_currency(s.company.company_value),
        total_revenue: format_currency(s.company.total_revenue),
        total_expenses: format_currency(s.company.total_expenses),
        total_profit: format_currency(s.company.total_profit),
        monthly_payroll: format_currency(s.employees.monthly_payroll),
        executive_payroll: format_currency(s.executives.iter().map(|e| e.salary_monthly).sum::<f64>()),
        total_loans: format_currency(total_loan_remaining),
        tax_rate: pct(s.economy.corporate_tax_rate),
        interest_rate: pct(s.economy.interest_rate),
        financial_history,
        loans,
        max_loan: format_currency_full(max_loan),
        suggested_rate: format!("{:.1}", suggested_rate),
        messages: s.messages.clone(),
        current_quarter: s.advance_quarter_label(),
        active_page: "finances".to_string(),
    }
    .into_response()
}

pub async fn take_loan(State(state): State<AppState>, Form(form): Form<LoanForm>) -> Response {
    let mut state = state.lock().await;
    let amount: f64 = form.amount.parse().unwrap_or(0.0);
    let quarters: i32 = form.quarters.parse().unwrap_or(8);

    if amount <= 0.0 || quarters <= 0 {
        state.messages.push("Invalid loan parameters.".into());
        return Redirect::to("/finances").into_response();
    }

    let max_loan = (state.company.company_value * 0.5).max(10_000_000.0);
    if amount > max_loan {
        state.messages.push(format!(
            "Loan amount {} exceeds maximum of {}. Banks won't lend more than 50% of company value.",
            format_currency(amount),
            format_currency(max_loan)
        ));
        return Redirect::to("/finances").into_response();
    }

    let rate = state.economy.interest_rate + 1.5;
    let total_with_interest = amount * (1.0 + rate / 100.0 * quarters as f64 / 4.0);
    let quarterly_payment = total_with_interest / quarters as f64;

    let loan = Loan {
        id: uuid::Uuid::new_v4().to_string(),
        amount,
        interest_rate: rate,
        remaining: total_with_interest,
        quarterly_payment,
        quarters_remaining: quarters,
    };

    state.company.cash += amount;
    state.company.loans.push(loan);
    state.messages.push(format!(
        "Took a loan of {} at {} APR over {} quarters. Quarterly payment: {}",
        format_currency(amount),
        pct(rate),
        quarters,
        format_currency(quarterly_payment)
    ));

    Redirect::to("/finances").into_response()
}

pub async fn new_game(State(state): State<AppState>) -> Response {
    let mut state = state.lock().await;
    *state = GameState::new();
    Redirect::to("/").into_response()
}

pub async fn events_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let events: Vec<EventRow> = s
        .event_log
        .iter()
        .take(30)
        .map(|e| EventRow {
            title: e.title.clone(),
            icon: e.event_type.icon().into(),
            quarter: format!("Q{} {}", e.quarter, e.year),
        })
        .collect();

    crate::templates::EventsTemplate {
        events,
        messages: s.messages.clone(),
        current_quarter: s.advance_quarter_label(),
        active_page: "events".to_string(),
    }
    .into_response()
}

fn report_to_row(r: &crate::game::QuarterlyReport) -> FinancialRow {
    FinancialRow {
        quarter: format!("Q{} {}", r.quarter, r.year),
        revenue: format_currency(r.revenue),
        expenses: format_currency(r.expenses),
        profit: format_currency(r.profit),
        tax: format_currency(r.tax_paid),
        stores: r.store_count,
        employees: r.employee_count,
        market_share: pct(r.market_share),
        satisfaction: pct(r.customer_satisfaction),
        profit_class: if r.profit >= 0.0 {
            "text-green-400".to_string()
        } else {
            "text-red-400".to_string()
        },
    }
}

fn rating_class(value: f64, mid: f64) -> String {
    if value >= mid + 10.0 {
        "text-green-400".to_string()
    } else if value >= mid {
        "text-yellow-400".to_string()
    } else {
        "text-red-400".to_string()
    }
}
