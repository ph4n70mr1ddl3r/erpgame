use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use rand::Rng;

use crate::game::{
    CustomerServicePolicy, Executive, ExecutivePosition, ExpansionPolicy,
    EventCategory, GameState, HrPolicy, InventoryPolicy, Loan, MarketingPolicy, PricingPolicy,
    Store, StoreStatus, StoreType, UpgradeType, format_currency, format_currency_full,
    generate_executive_name, get_available_cities, pct, simulate_quarter,
    achievements::unlocked_count, competitors::total_competitor_market_share,
    campaigns::{CampaignType, campaign_revenue_multiplier, launch_campaign},
    csr::{CsrInitiative, launch_initiative, discontinue_initiative, csr_tax_deduction},
    ecommerce::{EcommerceLevel, upgrade_ecommerce},
    loyalty::{LoyaltyTier, loyalty_revenue_multiplier},
    private_label::{self, PrivateLabelStatus},
    products::invest_in_category, upgrades::purchase_upgrade as do_purchase_upgrade,
    seasonal::{
        SeasonalPromotionType, get_available_promotions_for_quarter,
        activate_promotion, seasonal_revenue_multiplier,
    },
    research::{ResearchTrack, start_research, cancel_research},
    supply_chain::{
        SupplierCategory, LogisticsLevel, WarehouseTier, DeliveryServiceLevel,
        available_supplier_categories, negotiate_supplier, terminate_supplier,
        upgrade_logistics, upgrade_warehouse, upgrade_delivery_service,
    },
};
use super::dto::*;

pub type AppState = Arc<Mutex<GameState>>;

pub async fn dashboard(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    if s.has_pending_events() {
        return Redirect::to("/decisions").into_response();
    }

    let last_report = s.financial_history.last();
    let operating_count = s.operating_store_count();

    let (q_rev, q_exp, q_prof) = if let Some(r) = last_report {
        (format_currency(r.revenue), format_currency(r.expenses), format_currency(r.profit))
    } else { ("-".into(), "-".into(), "-".into()) };

    let financial_history: Vec<FinancialRow> = s.financial_history.iter().rev().take(20).map(report_to_row).collect();

    let chart_data = build_chart_data(s);
    let chart_json = serde_json::to_string(&chart_data).unwrap_or_default();

    let next_q = if s.current_quarter >= 4 { format!("Q1 {}", s.current_year + 1) } else { format!("Q{} {}", s.current_quarter + 1, s.current_year) };

    crate::templates::DashboardTemplate {
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
        current_quarter: s.current_quarter_label(),
        next_quarter: next_q,
        game_over: s.game_over,
        messages: s.messages_vec(),
        financial_history,
        economy_gdp: pct(s.economy.gdp_growth_rate),
        economy_inflation: pct(s.economy.inflation_rate),
        economy_interest: pct(s.economy.interest_rate),
        economy_description: s.economy.description(),
        competition_description: s.market.competition_description(),
        seasonal_multiplier: format!("{:.2}x", s.market.seasonal_multiplier),
        board_patience: format!("{:.0}%", s.board.patience),
        board_patience_class: s.board.patience_class().to_string(),
        board_patience_color: patience_color(s.board.patience),
        active_page: "dashboard".to_string(),
        chart_json,
        achievements_unlocked: unlocked_count(&s.achievements),
        achievements_total: s.achievements.len(),
    }.into_response()
}

pub async fn stores_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let can_close_any = s.stores.iter().filter(|x| x.status == StoreStatus::Operating).count() > 1;

    let store_rows: Vec<StoreRow> = s.stores.iter().map(|st| {
        let profit = st.quarterly_revenue - st.quarterly_expenses;
        StoreRow {
            id: st.id.clone(), name: st.name.clone(), city: st.city.clone(), region: format!("{:?}", st.region),
            store_type: st.store_type.label().into(), size_sqm: st.size_sqm, status: st.status.label().into(),
            status_class: st.status.css_class().into(), quarterly_revenue: format_currency(st.quarterly_revenue),
            quarterly_expenses: format_currency(st.quarterly_expenses), quarterly_profit: format_currency(profit),
            employees: st.employee_count, satisfaction: pct(st.satisfaction),
            age: if st.age_quarters < 4 { format!("{} Q", st.age_quarters) } else { format!("{} Y", st.age_quarters / 4) },
            can_close: st.status == StoreStatus::Operating && can_close_any,
        }
    }).collect();

    let occupied_cities: std::collections::HashSet<String> = s.stores.iter().filter(|st| st.status != StoreStatus::Closed).map(|st| st.city.clone()).collect();
    let cities: Vec<CityOption> = get_available_cities().iter().filter(|c| !occupied_cities.contains(&c.name)).map(|c| CityOption {
        name: c.name.clone(), region: format!("{:?}", c.region), rent: format!("{}sqm/mo", c.rent_per_sqm),
        demand: format!("{:.1}x", c.demand_factor), population: format!("{}", c.population),
        competitor: if c.has_competitor { "Yes".to_string() } else { "No".to_string() },
    }).collect();

    let store_types: Vec<StoreTypeOption> = vec![
        StoreTypeOption { key: "express".into(), label: "Express".into(), size: 800, cost: format_currency(StoreType::Express.opening_cost()), construction: StoreType::Express.construction_quarters() },
        StoreTypeOption { key: "standard".into(), label: "Standard".into(), size: 3500, cost: format_currency(StoreType::Standard.opening_cost()), construction: StoreType::Standard.construction_quarters() },
        StoreTypeOption { key: "mega".into(), label: "Mega".into(), size: 12000, cost: format_currency(StoreType::Mega.opening_cost()), construction: StoreType::Mega.construction_quarters() },
        StoreTypeOption { key: "depot".into(), label: "Depot".into(), size: 18000, cost: format_currency(StoreType::Depot.opening_cost()), construction: StoreType::Depot.construction_quarters() },
    ];

    crate::templates::StoresTemplate { store_rows, cities, store_types, cash: format_currency_full(s.company.cash), messages: s.messages_vec(), current_quarter: s.current_quarter_label(), active_page: "stores".to_string() }.into_response()
}

pub async fn open_store(State(state): State<AppState>, Form(form): Form<NewStoreForm>) -> Response {
    let mut state = state.lock().await;
    let store_type = match form.store_type.as_str() { "express" => StoreType::Express, "standard" => StoreType::Standard, "mega" => StoreType::Mega, "depot" => StoreType::Depot, _ => StoreType::Standard };
    let cost = store_type.opening_cost();
    let current_cash = state.company.cash;
    if current_cash < cost {
            state.push_message(format!("Cannot open store: need {} but only have {}", format_currency(cost), format_currency(current_cash)));
        return Redirect::to("/stores").into_response();
    }
    let mut store_name_raw = form.store_name.clone();
    if store_name_raw.len() > 100 {
        store_name_raw.truncate(100);
    }
    let cities = get_available_cities();
    let (region, city_name) = match cities.iter().find(|c| c.name == form.city) {
        Some(c) => (c.region, c.name.clone()),
        None => {
            state.push_message("Invalid city selected.".into());
            return Redirect::to("/stores").into_response();
        }
    };
    state.company.cash -= cost;
    let store_name = if store_name_raw.is_empty() { format!("Bahay Depot {}", city_name) } else { store_name_raw };
    let store = Store {
        id: uuid::Uuid::new_v4().to_string(), name: store_name, city: city_name, region, store_type,
        size_sqm: store_type.default_size(), status: StoreStatus::UnderConstruction, quarterly_revenue: 0.0,
        quarterly_expenses: 0.0, customer_count: 0, employee_count: 0, satisfaction: 50.0, age_quarters: 0,
        construction_quarters_left: store_type.construction_quarters(), opened_quarter: 0, opened_year: 0,
    };
    state.push_message(format!("Breaking ground on {} in {} (Cost: {}, Opens in {} quarters)", store.name, store.city, format_currency(cost), store.construction_quarters_left));
    state.stores.push(store);
    Redirect::to("/stores").into_response()
}

pub async fn close_store(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let mut state = state.lock().await;
    let operating_count = state.stores.iter().filter(|s| s.status == StoreStatus::Operating).count();
    if let Some(idx) = state.stores.iter().position(|s| s.id == id) {
        if state.stores[idx].status != StoreStatus::Operating {
            state.push_message("Store is not operating.".into());
            return Redirect::to("/stores").into_response();
        }
        if operating_count <= 1 {
            state.push_message("Cannot close your last operating store.".into());
            return Redirect::to("/stores").into_response();
        }
        let sell_value = state.stores[idx].store_type.opening_cost() * 0.4;
        let store_name = state.stores[idx].name.clone();
        let store_city = state.stores[idx].city.clone();
        state.stores[idx].status = StoreStatus::Closed;
        state.company.cash += sell_value;
        state.push_message(format!("Closed {} in {}. Received {} from asset sale.", store_name, store_city, format_currency(sell_value)));
    } else {
        state.push_message("Store not found.".into());
    }
    Redirect::to("/stores").into_response()
}

pub async fn executives_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let exec_rows: Vec<ExecutiveRow> = s.executives.iter().map(|e| ExecutiveRow {
        id: e.id.clone(), name: e.name.clone(), position: e.position.title().into(), short_position: e.position.short_title().into(),
        skill: format!("{:.0}/100", e.skill), salary: format!("{}/mo", format_currency_full(e.salary_monthly)),
        morale: format!("{:.0}%", e.morale), loyalty: format!("{:.0}%", e.loyalty),
        performance: format!("{:.0}/100", e.performance_rating),
        tenure: if e.tenure_quarters < 4 { format!("{} Q", e.tenure_quarters) } else { format!("{} Y", e.tenure_quarters / 4) },
        recommendation: e.recommendation.clone().unwrap_or_default(),
        morale_class: rating_class(e.morale, 50.0), loyalty_class: rating_class(e.loyalty, 50.0), performance_class: rating_class(e.performance_rating, 60.0),
    }).collect();

    let mut open_positions = Vec::new();
    for pos in ExecutivePosition::all_positions() { if !s.is_executive_hired(pos) { open_positions.push(pos.title().to_string()); } }

    crate::templates::ExecutivesTemplate { executives: exec_rows, open_positions, cash: format_currency_full(s.company.cash), messages: s.messages_vec(), current_quarter: s.current_quarter_label(), active_page: "executives".to_string() }.into_response()
}

pub async fn hire_executive(State(state): State<AppState>, Form(form): Form<HireExecutiveForm>) -> Response {
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
        state.push_message(format!("{} position is already filled.", position.short_title()));
        return Redirect::to("/executives").into_response();
    }
    let mut rng = rand::thread_rng();
    let name = generate_executive_name(&mut rng);
    let skill = rng.gen_range(40.0..95.0);
    let (min_sal, max_sal) = position.salary_range();
    let salary = rng.gen_range(min_sal..max_sal);
    let hiring_bonus = salary * 3.0;
    if state.company.cash < hiring_bonus { state.push_message(format!("Cannot afford hiring bonus for {} ({})", position.short_title(), format_currency(hiring_bonus))); return Redirect::to("/executives").into_response(); }
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
    state.push_message(format!(
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
            let cur = format_currency(state.company.cash);
            state.push_message(format!(
                "Cannot afford {} severance for {} ({}).",
                format_currency(severance),
                exec_name,
                cur
            ));
            return Redirect::to("/executives").into_response();
        }
        let exec_position = state.executives[idx].position;
        state.executives.remove(idx);
        state.company.cash -= severance;
        for cat in EventCategory::all_categories() {
            if cat.delegate_position() == exec_position && state.delegation.is_delegated(cat) {
                state.delegation.set(cat, false);
            }
        }
        state.push_message(format!("Fired {} ({}). Paid {} severance.", exec_name, exec_pos, format_currency(severance)));
    } else {
        state.push_message("Executive not found.".into());
    }
    Redirect::to("/executives").into_response()
}

pub async fn policies_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let p = &s.policies;
    crate::templates::PoliciesTemplate {
        pricing: p.pricing.label().into(), pricing_key: p.pricing.key().into(),
        hr: p.hr.label().into(), hr_key: p.hr.key().into(),
        expansion: p.expansion.label().into(), expansion_key: p.expansion.key().into(),
        customer_service: p.customer_service.label().into(), customer_service_key: p.customer_service.key().into(),
        marketing: p.marketing.label().into(), marketing_key: p.marketing.key().into(),
        inventory: p.inventory.label().into(), inventory_key: p.inventory.key().into(),
        messages: s.messages_vec(), current_quarter: s.current_quarter_label(), active_page: "policies".to_string(),
    }.into_response()
}

pub async fn update_policies(State(state): State<AppState>, Form(form): Form<PolicyForm>) -> Response {
    let mut state = state.lock().await;
    state.policies.pricing = match form.pricing.as_str() { "budget" => PricingPolicy::Budget, "premium" => PricingPolicy::Premium, "dynamic" => PricingPolicy::Dynamic, _ => PricingPolicy::Competitive };
    state.policies.hr = match form.hr.as_str() { "minimal" => HrPolicy::Minimal, "generous" => HrPolicy::Generous, "elite" => HrPolicy::Elite, _ => HrPolicy::Standard };
    state.policies.expansion = match form.expansion.as_str() { "conservative" => ExpansionPolicy::Conservative, "aggressive" => ExpansionPolicy::Aggressive, "blitz" => ExpansionPolicy::Blitz, _ => ExpansionPolicy::Moderate };
    state.policies.customer_service = match form.customer_service.as_str() { "basic" => CustomerServicePolicy::Basic, "excellent" => CustomerServicePolicy::Excellent, "whiteglove" => CustomerServicePolicy::WhiteGlove, _ => CustomerServicePolicy::Good };
    state.policies.marketing = match form.marketing.as_str() { "lowkey" => MarketingPolicy::LowKey, "heavy" => MarketingPolicy::Heavy, "aggressive" => MarketingPolicy::Aggressive, _ => MarketingPolicy::Moderate };
    state.policies.inventory = match form.inventory.as_str() { "lean" => InventoryPolicy::Lean, "buffered" => InventoryPolicy::Buffered, "abundant" => InventoryPolicy::Abundant, _ => InventoryPolicy::Standard };
    state.push_message("Company policies updated.".into());
    Redirect::to("/policies").into_response()
}

pub async fn tick(State(state): State<AppState>) -> Response {
    let mut state = state.lock().await;
    if !state.game_over {
        simulate_quarter(&mut state);
    }
    if state.has_pending_events() {
        Redirect::to("/decisions").into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

pub async fn finances_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let financial_history: Vec<FinancialRow> = s.financial_history.iter().rev().take(50).map(report_to_row).collect();
    let loans: Vec<LoanInfo> = s.company.loans.iter().map(|l| LoanInfo {
        id: l.id.clone(),
        amount: format_currency_full(l.amount),
        remaining: format_currency_full(l.remaining),
        rate: pct(l.interest_rate),
        quarterly_payment: format_currency_full(l.quarterly_payment),
        quarters_left: l.quarters_remaining,
    }).collect();
    let total_loan_remaining: f64 = s.company.loans.iter().map(|l| l.remaining).sum();
    let max_loan = (s.company.company_value * 0.5).max(crate::game::state::MINIMUM_LOAN_AMOUNT);
    let suggested_rate = s.economy.interest_rate + 1.5;

    crate::templates::FinancesTemplate {
        cash: format_currency_full(s.company.cash), company_value: format_currency(s.company.company_value),
        total_revenue: format_currency(s.company.total_revenue), total_expenses: format_currency(s.company.total_expenses),
        total_profit: format_currency(s.company.total_profit), monthly_payroll: format_currency(s.employees.monthly_payroll),
        executive_payroll: format_currency(s.executives.iter().map(|e| e.salary_monthly).sum::<f64>()),
        total_loans: format_currency(total_loan_remaining), tax_rate: pct(s.economy.corporate_tax_rate),
        interest_rate: pct(s.economy.interest_rate), financial_history, loans,
        max_loan: format_currency_full(max_loan), suggested_rate: format!("{:.1}", suggested_rate),
        messages: s.messages_vec(), current_quarter: s.current_quarter_label(), active_page: "finances".to_string(),
    }.into_response()
}

pub async fn take_loan(State(state): State<AppState>, Form(form): Form<LoanForm>) -> Response {
    let mut state = state.lock().await;
    let amount: f64 = form.amount.parse().unwrap_or(0.0);
    let quarters: i32 = form.quarters.parse().unwrap_or(8);
    if !amount.is_finite() || amount < crate::game::state::MINIMUM_LOAN_AMOUNT || !quarters.is_positive() || quarters > 40 {
        state.push_message(format!("Invalid loan parameters. Minimum loan is {}.", format_currency(crate::game::state::MINIMUM_LOAN_AMOUNT)));
        return Redirect::to("/finances").into_response();
    }
    let total_outstanding: f64 = state.company.loans.iter().map(|l| l.remaining).sum();
    let max_loan = (state.company.company_value * 0.5).max(crate::game::state::MINIMUM_LOAN_AMOUNT);
    if total_outstanding + amount > max_loan {
        state.push_message(format!("Total debt ({} + new {}) would exceed maximum of {}.", format_currency(total_outstanding), format_currency(amount), format_currency(max_loan)));
        return Redirect::to("/finances").into_response();
    }
    let rate = state.economy.interest_rate + 1.5;
    let quarterly_rate = rate / 100.0 / 4.0;
    let n = quarters as f64;
    let quarterly_payment = if quarterly_rate > 0.0 {
        amount * quarterly_rate * (1.0 + quarterly_rate).powf(n) / ((1.0 + quarterly_rate).powf(n) - 1.0)
    } else {
        amount / n
    };
    let loan = Loan { id: uuid::Uuid::new_v4().to_string(), amount, interest_rate: rate, remaining: amount, quarterly_payment, quarters_remaining: quarters };
    state.company.cash += amount;
    state.company.has_ever_had_loan = true;
    state.company.loans.push(loan);
    state.push_message(format!("Took a loan of {} at {} APR over {} quarters. Quarterly payment: {}", format_currency(amount), pct(rate), quarters, format_currency(quarterly_payment)));
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
    let events: Vec<EventRow> = s.event_log.iter().take(30).map(|e| EventRow { title: e.title.clone(), icon: e.event_type.icon().into(), quarter: format!("Q{} {}", e.quarter, e.year) }).collect();
    crate::templates::EventsTemplate { events, messages: s.messages_vec(), current_quarter: s.current_quarter_label(), active_page: "events".to_string() }.into_response()
}

pub async fn decisions_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let pending_rows: Vec<PendingEventRow> = s.pending_events.iter().map(|pe| {
        let choices: Vec<ChoiceOption> = pe.choices.iter().map(|c| {
            let risk_class = match c.risk_level.as_str() { "Low" => "text-green-400", "Medium" => "text-yellow-400", _ => "text-red-400" };
            let risk_css = match c.risk_level.as_str() { "Low" => "low", "Medium" => "medium", _ => "high" };
            ChoiceOption {
                id: c.id.clone(), label: c.label.clone(), description: c.description.clone(),
                risk_level: c.risk_level.clone(), risk_class: risk_class.into(), risk_css: risk_css.into(),
                cash: if c.effects.cash == 0.0 { "-".into() } else { format_currency(c.effects.cash) },
                morale: if c.effects.morale == 0.0 { "-".into() } else { format!("{:+.0}", c.effects.morale) },
                reputation: if c.effects.reputation == 0.0 { "-".into() } else { format!("{:+.0}", c.effects.reputation) },
                satisfaction: if c.effects.satisfaction == 0.0 { "-".into() } else { format!("{:+.0}", c.effects.satisfaction) },
            }
        }).collect();
        PendingEventRow { id: pe.id.clone(), title: pe.title.clone(), description: pe.description.clone(), category: pe.category.label().into(), category_icon: pe.category.icon().into(), choices }
    }).collect();

    crate::templates::DecisionsTemplate {
        pending_events: pending_rows,
        decisions_made: s.decisions_made,
        decisions_delegated: s.decisions_delegated,
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "decisions".to_string(),
    }.into_response()
}

pub async fn resolve_decision(State(state): State<AppState>, Form(form): Form<ResolveEventForm>) -> Response {
    let mut state = state.lock().await;
    let found = state.resolve_pending_event(&form.event_id, &form.choice_id);
    if found.is_none() {
        state.push_message("Event not found or already resolved.".into());
    }
    if state.has_pending_events() {
        Redirect::to("/decisions").into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

pub async fn delegation_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let rows: Vec<DelegationRow> = EventCategory::all_categories().iter().map(|cat| {
        let delegate_pos = cat.delegate_position();
        DelegationRow {
            category: cat.label().into(),
            key: cat.key().into(),
            icon: cat.icon().into(),
            delegate_to: delegate_pos.short_title().into(),
            is_delegated: s.delegation.is_delegated(*cat),
            has_executive: s.is_executive_hired(delegate_pos),
        }
    }).collect();

    crate::templates::DelegationTemplate {
        rows,
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "delegation".to_string(),
    }.into_response()
}

pub async fn update_delegation(State(state): State<AppState>, Form(form): Form<DelegationForm>) -> Response {
    let mut state = state.lock().await;
    let pairs: Vec<(EventCategory, Option<&String>)> = vec![
        (EventCategory::Crisis, form.crisis.as_ref()),
        (EventCategory::Financial, form.financial.as_ref()),
        (EventCategory::Marketing, form.marketing.as_ref()),
        (EventCategory::HR, form.hr.as_ref()),
        (EventCategory::SupplyChain, form.supply_chain.as_ref()),
        (EventCategory::Competition, form.competition.as_ref()),
        (EventCategory::Technology, form.technology.as_ref()),
        (EventCategory::Regulation, form.regulation.as_ref()),
    ];
    for (cat, val) in pairs {
        state.delegation.set(cat, val.is_some());
    }
    state.push_message("Delegation settings updated.".into());
    Redirect::to("/delegation").into_response()
}

fn report_to_row(r: &crate::game::QuarterlyReport) -> FinancialRow {
    FinancialRow {
        quarter: format!("Q{} {}", r.quarter, r.year), revenue: format_currency(r.revenue),
        expenses: format_currency(r.expenses), profit: format_currency(r.profit), tax: format_currency(r.tax_paid),
        stores: r.store_count, employees: r.employee_count, market_share: pct(r.market_share),
        satisfaction: pct(r.customer_satisfaction),
        profit_class: if r.profit >= 0.0 { "text-green-400".to_string() } else { "text-red-400".to_string() },
    }
}

fn rating_class(value: f64, mid: f64) -> String {
    if value >= mid + 10.0 { "text-green-400".to_string() } else if value >= mid { "text-yellow-400".to_string() } else { "text-red-400".to_string() }
}

fn patience_color(value: f64) -> String {
    if value > 70.0 { "#22c55e".to_string() }
    else if value > 50.0 { "#f59e0b".to_string() }
    else if value > 30.0 { "#f97316".to_string() }
    else { "#ef4444".to_string() }
}

fn pressure_color(value: f64) -> String {
    if value < 30.0 { "#22c55e".to_string() }
    else if value < 50.0 { "#f59e0b".to_string() }
    else if value < 70.0 { "#f97316".to_string() }
    else { "#ef4444".to_string() }
}

fn build_chart_data(s: &GameState) -> crate::api::dto::ChartData {
    let history = &s.financial_history;
    let labels: Vec<String> = history.iter().map(|r| format!("Q{} {}", r.quarter, r.year)).collect();
    let revenue: Vec<f64> = history.iter().map(|r| r.revenue / 1_000_000.0).collect();
    let expenses: Vec<f64> = history.iter().map(|r| r.expenses / 1_000_000.0).collect();
    let profit: Vec<f64> = history.iter().map(|r| r.profit / 1_000_000.0).collect();

    let cash: Vec<f64> = history.iter().map(|r| r.cash_on_hand / 1_000_000.0).collect();

    let market_share: Vec<f64> = history.iter().map(|r| r.market_share).collect();
    let customer_sat: Vec<f64> = history.iter().map(|r| r.customer_satisfaction).collect();
    let employee_sat: Vec<f64> = history.iter().map(|r| r.employee_satisfaction).collect();
    let brand_rep: Vec<f64> = history.iter().map(|r| r.brand_reputation).collect();

    crate::api::dto::ChartData {
        labels,
        revenue,
        expenses,
        profit,
        cash,
        market_share,
        customer_sat,
        employee_sat,
        brand_rep,
    }
}

pub async fn products_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let rows: Vec<crate::api::dto::ProductRow> = s.products.iter().map(|p| {
        crate::api::dto::ProductRow {
            id: p.id.clone(), name: p.name.clone(), icon: p.icon.clone(),
            margin: p.margin_display(), demand: p.demand_display(),
            investment: p.investment_display(), investment_pct: p.investment_level,
            trend: p.trend_display(), trend_class: p.trend_class().to_string(),
        }
    }).collect();
    let total_invested: f64 = s.products.iter().map(|p| p.investment_level * 50_000.0).sum();
    crate::templates::ProductsTemplate {
        rows, cash: format_currency_full(s.company.cash),
        total_invested: format_currency(total_invested),
        messages: s.messages_vec(), current_quarter: s.current_quarter_label(),
        active_page: "products".to_string(),
    }.into_response()
}

pub async fn invest_product(State(state): State<AppState>, Form(form): Form<super::dto::ProductInvestForm>) -> Response {
    let mut state = state.lock().await;
    let amount: f64 = form.amount.parse().unwrap_or(0.0);
    if !amount.is_finite() || amount < 500_000.0 {
        state.push_message("Minimum investment is P500K.".into());
        return Redirect::to("/products").into_response();
    }
    if state.company.cash < amount {
        state.push_message(format!("Cannot afford {} investment.", format_currency(amount)));
        return Redirect::to("/products").into_response();
    }
    let (increase, actual_cost) = invest_in_category(&mut state.products, &form.category_id, amount);
    if actual_cost > 0.0 {
        state.company.cash -= actual_cost;
        if increase < (amount / 500_000.0).min(5.0) {
            state.push_message(format!(
                "Invested {} in product category. Investment increased by {:.1}% (capped at 100%).",
                format_currency(actual_cost), increase
            ));
        } else {
            state.push_message(format!(
                "Invested {} in product category. Investment increased by {:.1}%.",
                format_currency(actual_cost), increase
            ));
        }
    } else {
        state.push_message("Category not found or already at maximum investment.".into());
    }
    Redirect::to("/products").into_response()
}

pub async fn upgrades_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let mut store_rows = Vec::new();

    for store in s.stores.iter().filter(|st| st.status == StoreStatus::Operating) {
        let mut upgrade_rows = Vec::new();
        for ut in UpgradeType::all_types() {
            let current = crate::game::upgrades::get_upgrade_level(&s.upgrades, &store.id, ut);
            let can = current < ut.max_level() && s.company.cash >= ut.cost_per_level();
            upgrade_rows.push(crate::api::dto::SingleUpgradeRow {
                upgrade_type: ut.label().into(), upgrade_key: ut.key().into(),
                icon: ut.icon().into(), description: ut.description().into(),
                current_level: current, max_level: ut.max_level(),
                cost: format_currency(ut.cost_per_level()),
                revenue_effect: if ut.revenue_bonus_per_level() > 0.0 { format!("+{:.0}%/lvl", ut.revenue_bonus_per_level() * 100.0) } else { "-".into() },
                cost_effect: if ut.cost_reduction_per_level() > 0.0 { format!("-{:.0}%/lvl", ut.cost_reduction_per_level() * 100.0) } else { "-".into() },
                sat_effect: if ut.satisfaction_bonus_per_level() > 0.0 { format!("+{:.0}/lvl", ut.satisfaction_bonus_per_level()) } else { "-".into() },
                can_upgrade: can,
            });
        }
        store_rows.push(crate::api::dto::UpgradeStoreRow {
            store_id: store.id.clone(), store_name: store.name.clone(),
            store_city: store.city.clone(), upgrade_rows,
        });
    }

    crate::templates::UpgradesTemplate {
        store_rows, cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(), current_quarter: s.current_quarter_label(),
        active_page: "upgrades".to_string(),
    }.into_response()
}

pub async fn purchase_upgrade(State(state): State<AppState>, Form(form): Form<super::dto::PurchaseUpgradeForm>) -> Response {
    let mut state = state.lock().await;
    let upgrade_type = match UpgradeType::from_key(&form.upgrade_type) {
        Some(ut) => ut,
        None => { state.push_message("Invalid upgrade type.".into()); return Redirect::to("/upgrades").into_response(); }
    };
    let cost = upgrade_type.cost_per_level();
    if state.company.cash < cost {
        state.push_message(format!("Cannot afford upgrade (need {}).", format_currency(cost)));
        return Redirect::to("/upgrades").into_response();
    }
    if !state.stores.iter().any(|s| s.id == form.store_id) {
        state.push_message("Invalid store selected.".into());
        return Redirect::to("/upgrades").into_response();
    }
    match do_purchase_upgrade(&mut state.upgrades, &form.store_id, upgrade_type) {
        Ok(cost) => {
            state.company.cash -= cost;
            let store_name = state.stores.iter().find(|s| s.id == form.store_id).map(|s| s.name.clone()).unwrap_or_else(|| "Unknown".into());
            state.push_message(format!("Upgraded {} at {} for {}.", upgrade_type.label(), store_name, format_currency(cost)));
        }
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/upgrades").into_response()
}

pub async fn board_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let b = &s.board;
    let quarters_until = (4 - b.quarters_since_review).max(0);
    crate::templates::BoardTemplate {
        board: crate::api::dto::BoardInfo {
            patience: format!("{:.0}%", b.patience), patience_class: b.patience_class().to_string(),
            patience_color: patience_color(b.patience),
            pressure: format!("{:.0}%", b.pressure_level), pressure_class: b.pressure_class().to_string(),
            pressure_color: pressure_color(b.pressure_level),
            warnings: b.warnings, description: b.description(),
            last_review: if b.last_review_year > 0 { format!("Q{} {}", b.last_review_quarter, b.last_review_year) } else { "None yet".into() },
            quarters_until_review: quarters_until,
        },
        company_value: format_currency(s.company.company_value),
        market_share: pct(s.company.market_share),
        competitor_share: pct(total_competitor_market_share(&s.competitors)),
        messages: s.messages_vec(), current_quarter: s.current_quarter_label(),
        active_page: "board".to_string(),
    }.into_response()
}

pub async fn competitors_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let rows: Vec<crate::api::dto::CompetitorRow> = s.competitors.iter().map(|c| {
        crate::api::dto::CompetitorRow {
            name: c.name.clone(), store_count: c.store_count,
            strength: format!("{:.0}/100", c.strength),
            strategy: c.strategy.clone(), market_share: pct(c.market_share),
            recent_action: c.recent_action.clone(),
            quarters_since: c.quarters_since_action,
        }
    }).collect();
    crate::templates::CompetitorsTemplate {
        rows, player_share: pct(s.company.market_share),
        messages: s.messages_vec(), current_quarter: s.current_quarter_label(),
        active_page: "competitors".to_string(),
    }.into_response()
}

pub async fn achievements_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let rows: Vec<crate::api::dto::AchievementRow> = s.achievements.iter().map(|a| {
        crate::api::dto::AchievementRow {
            id: a.id.clone(), title: a.title.clone(), description: a.description.clone(),
            icon: a.icon.clone(),             unlocked: a.unlocked, unlocked_quarter: a.unlocked_quarter.clone().unwrap_or_default(),
        }
    }).collect();
    let total = s.achievements.len();
    let unlocked = unlocked_count(&s.achievements);
    crate::templates::AchievementsTemplate {
        rows, total, unlocked,
        messages: s.messages_vec(), current_quarter: s.current_quarter_label(),
        active_page: "achievements".to_string(),
    }.into_response()
}

pub async fn loyalty_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let program = &s.loyalty;
    let current = program.tier;
    let operating = s.operating_store_count() as f64;
    let max_members = operating * 15_000.0;
    let penetration = if max_members > 0.0 {
        (program.members as f64 / max_members * 100.0).min(100.0)
    } else {
        0.0
    };

    let effective_bonus = loyalty_revenue_multiplier(s);
    let effective_pct = format!("{:.1}%", (effective_bonus - 1.0) * 100.0);

    let all_tiers = [
        LoyaltyTier::None,
        LoyaltyTier::Basic,
        LoyaltyTier::Silver,
        LoyaltyTier::Gold,
        LoyaltyTier::Platinum,
    ];

    let tiers: Vec<crate::api::dto::LoyaltyTierRow> = all_tiers.iter().map(|t| {
        let is_current = *t == current;
        let can_afford = s.company.cash >= t.setup_cost();
        let (can_select, reason, show_reason) = if is_current {
            (true, String::new(), false)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(t.setup_cost())), true)
        } else {
            (true, String::new(), false)
        };
        crate::api::dto::LoyaltyTierRow {
            key: t.key().to_string(),
            name: t.label().to_string(),
            icon_char: t.icon().into(),
            color_class: t.color_class().to_string(),
            description: t.description().to_string(),
            setup_cost: if t.setup_cost() > 0.0 { format_currency(t.setup_cost()) } else { "Free".into() },
            quarterly_cost: if t.quarterly_cost_per_store() > 0.0 {
                format!("{}/store", format_currency(t.quarterly_cost_per_store()))
            } else {
                "Free".into()
            },
            revenue_bonus: if t.revenue_bonus() > 0.0 { format!("+{:.0}%", t.revenue_bonus() * 100.0) } else { "-".into() },
            sat_bonus: if t.satisfaction_bonus() > 0.0 { format!("+{:.1}", t.satisfaction_bonus()) } else { "-".into() },
            is_current,
            can_select,
            show_reason,
            reason,
        }
    }).collect();

    let quarterly_cost_val = current.quarterly_cost_per_store() * operating;

    crate::templates::LoyaltyTemplate {
        current_tier: current.label().to_string(),
        current_tier_class: current.color_class().to_string(),
        members: program.members.to_string(),
        member_penetration: format!("{:.1}", penetration),
        effective_revenue_bonus: effective_pct,
        quarters_active: program.quarters_active,
        quarterly_cost: if quarterly_cost_val > 0.0 { format_currency(quarterly_cost_val) } else { "-".into() },
        satisfaction_bonus: if current.satisfaction_bonus() > 0.0 { format!("+{:.1}", current.satisfaction_bonus()) } else { "-".into() },
        points_multiplier: if current.points_multiplier() > 0.0 { format!("{}x", current.points_multiplier()) } else { "-".into() },
        growth_rate: if current.member_growth_rate() > 0.0 { format!("{:.0}%/quarter", current.member_growth_rate() * 100.0) } else { "-".into() },
        tiers,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "loyalty".to_string(),
    }.into_response()
}

pub async fn update_loyalty(State(state): State<AppState>, Form(form): Form<super::dto::LoyaltyTierForm>) -> Response {
    let mut state = state.lock().await;
    let new_tier = match LoyaltyTier::from_key(&form.tier) {
        Some(t) => t,
        None => {
            state.push_message("Invalid loyalty tier.".into());
            return Redirect::to("/loyalty").into_response();
        }
    };

    if new_tier == state.loyalty.tier {
        return Redirect::to("/loyalty").into_response();
    }

    let allowed_tier = if new_tier == LoyaltyTier::None {
        true
    } else {
        let next_tier = match state.loyalty.tier {
            LoyaltyTier::None => Some(LoyaltyTier::Basic),
            LoyaltyTier::Basic => Some(LoyaltyTier::Silver),
            LoyaltyTier::Silver => Some(LoyaltyTier::Gold),
            LoyaltyTier::Gold => Some(LoyaltyTier::Platinum),
            LoyaltyTier::Platinum => None,
        };
        next_tier == Some(new_tier)
    };
    if !allowed_tier {
        state.push_message("You can only upgrade one loyalty tier at a time.".into());
        return Redirect::to("/loyalty").into_response();
    }

    let setup_cost = new_tier.setup_cost();
    if state.company.cash < setup_cost {
        state.push_message(format!(
            "Cannot afford {} loyalty program setup (need {})",
            new_tier.label(),
            format_currency(setup_cost)
        ));
        return Redirect::to("/loyalty").into_response();
    }

    state.company.cash -= setup_cost;
    state.loyalty.tier = new_tier;
    state.loyalty.members = 0;
    state.loyalty.quarters_active = 0;

    if new_tier == LoyaltyTier::None {
        state.push_message("Loyalty program discontinued. Members will be notified.".into());
    } else {
        state.push_message(format!(
            "Launched {} Loyalty Program! Setup cost: {}. Members will start enrolling next quarter.",
            new_tier.label(),
            format_currency(setup_cost)
        ));
    }

    Redirect::to("/loyalty").into_response()
}

pub async fn campaigns_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let active_rows: Vec<crate::api::dto::CampaignRow> = s.campaigns.iter().map(|c| {
        let ct = c.campaign_type;
        crate::api::dto::CampaignRow {
            id: c.id.clone(),
            name: ct.label().to_string(),
            icon: ct.icon().to_string(),
            description: ct.description().to_string(),
            quarters_remaining: c.quarters_remaining,
            quarters_total: c.quarters_total,
            revenue_boost: format!("+{:.0}%", ct.revenue_boost() * 100.0),
            reputation_boost: if ct.reputation_boost() > 0.0 { format!("+{:.1}", ct.reputation_boost()) } else { "-".into() },
            satisfaction_boost: if ct.satisfaction_boost() > 0.0 { format!("+{:.1}", ct.satisfaction_boost()) } else { "-".into() },
            started: format!("Q{} {}", c.start_quarter, c.start_year),
        }
    }).collect();

    let active_type_keys: std::collections::HashSet<String> = s.campaigns.iter().map(|c| c.campaign_type.key().to_string()).collect();
    let can_launch_more = s.campaigns.len() < 5;

    let options: Vec<crate::api::dto::CampaignOption> = CampaignType::all_types().iter().map(|ct| {
        let already_active = active_type_keys.contains(ct.key());
        let can_afford = s.company.cash >= ct.base_cost();
        let (can_launch, reason, show_reason) = if already_active {
            (false, "Already active".into(), true)
        } else if !can_launch_more {
            (false, "Max 5 concurrent campaigns".into(), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(ct.base_cost())), true)
        } else {
            (true, String::new(), false)
        };
        crate::api::dto::CampaignOption {
            key: ct.key().to_string(),
            name: ct.label().to_string(),
            icon: ct.icon().to_string(),
            description: ct.description().to_string(),
            cost: format_currency(ct.base_cost()),
            duration: ct.duration_quarters(),
            revenue_boost: format!("+{:.0}%", ct.revenue_boost() * 100.0),
            reputation_boost: if ct.reputation_boost() > 0.0 { format!("+{:.1}", ct.reputation_boost()) } else { "-".into() },
            satisfaction_boost: if ct.satisfaction_boost() > 0.0 { format!("+{:.1}", ct.satisfaction_boost()) } else { "-".into() },
            can_launch,
            reason,
            show_reason,
        }
    }).collect();

    let effective_mult = campaign_revenue_multiplier(s);
    let effective_pct = format!("{:.1}%", (effective_mult - 1.0) * 100.0);

    let cmo_skill = s.executives.iter().find(|e| e.position == ExecutivePosition::CMO).map(|e| format!("{:.0}/100", e.skill)).unwrap_or_else(|| "Not hired".into());

    crate::templates::CampaignsTemplate {
        active_campaigns: active_rows,
        options,
        effective_revenue_bonus: effective_pct,
        cmo_skill,
        active_count: s.campaigns.len(),
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "campaigns".to_string(),
    }.into_response()
}

pub async fn launch_campaign_route(State(state): State<AppState>, Form(form): Form<super::dto::CampaignForm>) -> Response {
    let mut state = state.lock().await;
    let campaign_type = match CampaignType::from_key(&form.campaign_type) {
        Some(ct) => ct,
        None => {
            state.push_message("Invalid campaign type.".into());
            return Redirect::to("/campaigns").into_response();
        }
    };
    match launch_campaign(&mut state, campaign_type) {
        Ok(cost) => {
            state.push_message(format!(
                "Launched '{}' campaign for {}. It will run for {} quarter(s).",
                campaign_type.label(),
                format_currency(cost),
                campaign_type.duration_quarters()
            ));
        }
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/campaigns").into_response()
}

pub async fn ecommerce_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let current = s.ecommerce.level;

    let effective_pct = if current != EcommerceLevel::None {
        let cto_skill = s.executives.iter().find(|e| e.position == ExecutivePosition::CTO).map(|e| e.skill).unwrap_or(0.0);
        let cto_factor = 1.0 + cto_skill * 0.005;
        let sat_factor = s.company.customer_satisfaction / 100.0;
        let base_bonus = current.revenue_bonus();
        let effective = base_bonus * cto_factor * sat_factor;
        format!("{:.1}%", effective * 100.0)
    } else {
        "-".into()
    };

    let cto_skill = s.executives.iter().find(|e| e.position == ExecutivePosition::CTO).map(|e| format!("{:.0}/100", e.skill)).unwrap_or_else(|| "Not hired".into());

    let all_levels = EcommerceLevel::all_levels();

    let levels: Vec<crate::api::dto::EcommerceLevelRow> = all_levels.iter().map(|l| {
        let is_current = *l == current;
        let can_afford = s.company.cash >= l.setup_cost();
        let has_stores = s.operating_store_count() >= l.min_stores();
        let (can_select, reason, show_reason) = if is_current {
            (true, String::new(), false)
        } else if !has_stores {
            (false, format!("Need {} operating stores", l.min_stores()), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(l.setup_cost())), true)
        } else {
            (true, String::new(), false)
        };
        crate::api::dto::EcommerceLevelRow {
            key: l.key().to_string(),
            name: l.label().to_string(),
            icon: l.icon().to_string(),
            color_class: l.color_class().to_string(),
            description: l.description().to_string(),
            setup_cost: if l.setup_cost() > 0.0 { format_currency(l.setup_cost()) } else { "Free".into() },
            quarterly_cost: if l.quarterly_cost() > 0.0 { format_currency(l.quarterly_cost()) } else { "-".into() },
            revenue_bonus: if l.revenue_bonus() > 0.0 { format!("+{:.0}%", l.revenue_bonus() * 100.0) } else { "-".into() },
            satisfaction_bonus: if l.satisfaction_bonus() > 0.0 { format!("+{:.1}", l.satisfaction_bonus()) } else { "-".into() },
            reputation_bonus: if l.reputation_bonus() > 0.0 { format!("+{:.1}", l.reputation_bonus()) } else { "-".into() },
            min_stores: l.min_stores(),
            is_current,
            can_select,
            show_reason,
            reason,
        }
    }).collect();

    crate::templates::EcommerceTemplate {
        current_level: current.label().to_string(),
        current_level_class: current.color_class().to_string(),
        quarters_active: s.ecommerce.quarters_active,
        quarterly_online_revenue: if s.ecommerce.quarterly_online_revenue > 0.0 { format_currency(s.ecommerce.quarterly_online_revenue) } else { "-".into() },
        total_online_revenue: if s.ecommerce.total_online_revenue > 0.0 { format_currency(s.ecommerce.total_online_revenue) } else { "-".into() },
        conversion_rate: if s.ecommerce.conversion_rate > 0.0 { format!("{:.1}%", s.ecommerce.conversion_rate) } else { "-".into() },
        quarterly_cost: if current.quarterly_cost() > 0.0 { format_currency(current.quarterly_cost()) } else { "-".into() },
        effective_revenue_bonus: effective_pct,
        cto_skill,
        levels,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "ecommerce".to_string(),
    }.into_response()
}

pub async fn upgrade_ecommerce_route(State(state): State<AppState>, Form(form): Form<super::dto::EcommerceForm>) -> Response {
    let mut state = state.lock().await;
    let new_level = match EcommerceLevel::from_key(&form.level) {
        Some(l) => l,
        None => {
            state.push_message("Invalid e-commerce level.".into());
            return Redirect::to("/ecommerce").into_response();
        }
    };
    match upgrade_ecommerce(&mut state, new_level) {
        Ok(_cost) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/ecommerce").into_response()
}

pub async fn supply_chain_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let sc = &s.supply_chain;

    let supplier_rows: Vec<SupplierRow> = sc.suppliers.iter().map(|sup| {
        let rel = sup.relationship_score;
        let rel_class = if rel >= 75.0 { "text-green-400".to_string() }
            else if rel >= 50.0 { "text-yellow-400".to_string() }
            else { "text-red-400".to_string() };
        SupplierRow {
            id: sup.id.clone(),
            name: sup.name.clone(),
            category: sup.category.label().to_string(),
            region: format!("{:?}", sup.region),
            reliability: format!("{:.0}%", sup.reliability),
            cost_modifier: format!("{:.0}%", sup.effective_cost_modifier() * 100.0),
            lead_time: sup.lead_time_quarters,
            quarters_remaining: sup.quarters_remaining,
            relationship: format!("{:.0}%", rel),
            relationship_class: rel_class,
            is_active: sup.is_active,
        }
    }).collect();

    let active_count = sc.suppliers.iter().filter(|s| s.is_active).count();
    let available_cats = available_supplier_categories(s);
    let category_options: Vec<SupplierCategoryOption> = SupplierCategory::all_categories().iter().map(|c| {
        let available = available_cats.contains(c);
        let can = available && active_count < 8 && s.company.cash >= 500_000.0;
        SupplierCategoryOption {
            key: c.key().to_string(),
            name: c.label().to_string(),
            can_negotiate: can,
        }
    }).collect();

    let csco_skill = s.executives.iter().find(|e| e.position == ExecutivePosition::CSCO).map(|e| format!("{:.0}/100", e.skill)).unwrap_or_else(|| "Not hired".into());

    let logistics_levels: Vec<LogisticsLevelRow> = LogisticsLevel::all_levels().iter().map(|l| {
        let is_current = *l == sc.logistics;
        let can_afford = s.company.cash >= match *l {
            LogisticsLevel::Basic => 0.0,
            LogisticsLevel::Regional => 8_000_000.0,
            LogisticsLevel::National => 20_000_000.0,
            LogisticsLevel::Advanced => 50_000_000.0,
        };
        let has_stores = s.operating_store_count() >= l.min_stores();
        let current_ord = sc.logistics as u8;
        let new_ord = *l as u8;
        let is_next = new_ord == current_ord + 1;
        let (can_select, reason, show_reason) = if is_current {
            (false, String::new(), false)
        } else if *l == LogisticsLevel::Basic && sc.logistics != LogisticsLevel::Basic {
            (true, String::new(), false)
        } else if !has_stores {
            (false, format!("Need {} stores", l.min_stores()), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(match *l {
                LogisticsLevel::Basic => 0.0,
                LogisticsLevel::Regional => 8_000_000.0,
                LogisticsLevel::National => 20_000_000.0,
                LogisticsLevel::Advanced => 50_000_000.0,
            })), true)
        } else if !is_next && *l != LogisticsLevel::Basic {
            (false, "Upgrade one level at a time".into(), true)
        } else {
            (true, String::new(), false)
        };
        LogisticsLevelRow {
            key: l.key().to_string(),
            name: l.label().to_string(),
            description: l.description().to_string(),
            quarterly_cost: if l.quarterly_cost() > 0.0 { format_currency(l.quarterly_cost()) } else { "-".into() },
            cost_reduction: format!("-{:.0}%", l.cost_reduction_pct() * 100.0),
            reliability: format!("{:.0}%", l.delivery_reliability() * 100.0),
            stockout_reduction: format!("-{:.0}%", l.stockout_reduction() * 100.0),
            min_stores: l.min_stores(),
            is_current,
            can_select,
            show_reason,
            reason,
        }
    }).collect();

    let warehouse_tiers: Vec<WarehouseTierRow> = WarehouseTier::all_tiers().iter().map(|t| {
        let is_current = *t == sc.warehouse;
        let can_afford = s.company.cash >= t.setup_cost();
        let has_stores = s.operating_store_count() >= t.min_stores();
        let current_ord = sc.warehouse as u8;
        let new_ord = *t as u8;
        let is_next = new_ord == current_ord + 1;
        let (can_select, reason, show_reason) = if is_current {
            (false, String::new(), false)
        } else if *t == WarehouseTier::None && sc.warehouse != WarehouseTier::None {
            (true, String::new(), false)
        } else if !has_stores {
            (false, format!("Need {} stores", t.min_stores()), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(t.setup_cost())), true)
        } else if !is_next && *t != WarehouseTier::None {
            (false, "Upgrade one tier at a time".into(), true)
        } else {
            (true, String::new(), false)
        };
        WarehouseTierRow {
            key: t.key().to_string(),
            name: t.label().to_string(),
            description: t.description().to_string(),
            setup_cost: if t.setup_cost() > 0.0 { format_currency(t.setup_cost()) } else { "Free".into() },
            quarterly_cost: if t.quarterly_cost() > 0.0 { format_currency(t.quarterly_cost()) } else { "-".into() },
            bulk_discount: format!("-{:.0}%", t.bulk_discount_pct() * 100.0),
            stockout_reduction: format!("-{:.0}%", t.stockout_reduction() * 100.0),
            min_stores: t.min_stores(),
            is_current,
            can_select,
            show_reason,
            reason,
        }
    }).collect();

    let delivery_service_levels: Vec<DeliveryServiceLevelRow> = DeliveryServiceLevel::all_levels().iter().map(|l| {
        let is_current = *l == sc.delivery_service;
        let can_afford = s.company.cash >= l.setup_cost();
        let has_stores = s.operating_store_count() >= l.min_stores();
        let current_ord = sc.delivery_service as u8;
        let new_ord = *l as u8;
        let is_next = new_ord == current_ord + 1;
        let (can_select, reason, show_reason) = if is_current {
            (false, String::new(), false)
        } else if *l == DeliveryServiceLevel::None && sc.delivery_service != DeliveryServiceLevel::None {
            (true, String::new(), false)
        } else if !has_stores {
            (false, format!("Need {} stores", l.min_stores()), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(l.setup_cost())), true)
        } else if !is_next && *l != DeliveryServiceLevel::None {
            (false, "Upgrade one level at a time".into(), true)
        } else {
            (true, String::new(), false)
        };
        DeliveryServiceLevelRow {
            key: l.key().to_string(),
            name: l.label().to_string(),
            description: l.description().to_string(),
            setup_cost: if l.setup_cost() > 0.0 { format_currency(l.setup_cost()) } else { "Free".into() },
            quarterly_cost: if l.quarterly_cost() > 0.0 { format_currency(l.quarterly_cost()) } else { "-".into() },
            revenue_bonus: format!("+{:.0}%", l.revenue_bonus() * 100.0),
            satisfaction_bonus: format!("+{:.0}", l.satisfaction_bonus()),
            min_stores: l.min_stores(),
            is_current,
            can_select,
            show_reason,
            reason,
        }
    }).collect();

    let total_quarterly_cost = sc.logistics.quarterly_cost() + sc.warehouse.quarterly_cost() + sc.delivery_service.quarterly_cost();

    crate::templates::SupplyChainTemplate {
        supplier_rows,
        category_options,
        active_supplier_count: active_count,
        max_suppliers: 8,
        logistics_levels,
        warehouse_tiers,
        delivery_service_levels,
        stockout_rate: format!("{:.1}%", sc.stockout_rate),
        avg_delivery_time: format!("{:.1} Q", sc.avg_delivery_time),
        quarterly_logistics_cost: format_currency(sc.quarterly_logistics_cost),
        total_quarterly_cost: format_currency(total_quarterly_cost),
        total_supply_savings: format_currency(sc.total_supply_savings),
        last_stockout_penalty: if sc.last_stockout_penalty > 0.0 { format_currency(sc.last_stockout_penalty) } else { "-".into() },
        quarters_since_disruption: sc.quarters_since_disruption,
        csco_skill,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "supply_chain".to_string(),
    }.into_response()
}

pub async fn negotiate_supplier_route(State(state): State<AppState>, Form(form): Form<NegotiateSupplierForm>) -> Response {
    let mut state = state.lock().await;
    let category = match SupplierCategory::from_key(&form.category) {
        Some(c) => c,
        None => {
            state.push_message("Invalid supplier category.".into());
            return Redirect::to("/supply-chain").into_response();
        }
    };
    match negotiate_supplier(&mut state, category) {
        Ok(_) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/supply-chain").into_response()
}

pub async fn terminate_supplier_route(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let mut state = state.lock().await;
    match terminate_supplier(&mut state, &id) {
        Ok(_) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/supply-chain").into_response()
}

pub async fn upgrade_logistics_route(State(state): State<AppState>, Form(form): Form<LogisticsForm>) -> Response {
    let mut state = state.lock().await;
    let new_level = match LogisticsLevel::from_key(&form.level) {
        Some(l) => l,
        None => {
            state.push_message("Invalid logistics level.".into());
            return Redirect::to("/supply-chain").into_response();
        }
    };
    match upgrade_logistics(&mut state, new_level) {
        Ok(_) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/supply-chain").into_response()
}

pub async fn upgrade_warehouse_route(State(state): State<AppState>, Form(form): Form<WarehouseForm>) -> Response {
    let mut state = state.lock().await;
    let new_tier = match WarehouseTier::from_key(&form.tier) {
        Some(t) => t,
        None => {
            state.push_message("Invalid warehouse tier.".into());
            return Redirect::to("/supply-chain").into_response();
        }
    };
    match upgrade_warehouse(&mut state, new_tier) {
        Ok(_) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/supply-chain").into_response()
}

pub async fn upgrade_delivery_service_route(State(state): State<AppState>, Form(form): Form<DeliveryForm>) -> Response {
    let mut state = state.lock().await;
    let new_level = match DeliveryServiceLevel::from_key(&form.level) {
        Some(l) => l,
        None => {
            state.push_message("Invalid delivery service level.".into());
            return Redirect::to("/supply-chain").into_response();
        }
    };
    match upgrade_delivery_service(&mut state, new_level) {
        Ok(_) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/supply-chain").into_response()
}

pub async fn private_label_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let brand_rows: Vec<crate::api::dto::PrivateLabelRow> = s.private_labels.iter().map(|pl| {
        let status_str = match pl.status {
            PrivateLabelStatus::Developing => "Developing".to_string(),
            PrivateLabelStatus::Active => "Active".to_string(),
            PrivateLabelStatus::Discontinued => "Discontinued".to_string(),
        };
        let status_class = match pl.status {
            PrivateLabelStatus::Developing => "text-yellow-400".to_string(),
            PrivateLabelStatus::Active => "text-green-400".to_string(),
            PrivateLabelStatus::Discontinued => "text-red-400".to_string(),
        };
        let bp_class = if pl.brand_power >= 60.0 { "text-green-400".to_string() }
            else if pl.brand_power >= 35.0 { "text-yellow-400".to_string() }
            else { "text-red-400".to_string() };
        let cat_name = private_label::find_config(&pl.category_id).map(|c| c.category_name).unwrap_or_else(|| pl.category_id.clone());
        crate::api::dto::PrivateLabelRow {
            id: pl.id.clone(),
            brand_name: pl.brand_name.clone(),
            category: cat_name,
            status: status_str,
            status_class,
            development_progress: format!("{:.0}%", pl.development_progress),
            quarters_remaining: pl.quarters_remaining,
            brand_power: format!("{:.0}/100", pl.brand_power),
            brand_power_class: bp_class,
            margin_rate: format!("{:.0}%", pl.margin_rate * 100.0),
            quarterly_revenue: if pl.quarterly_revenue > 0.0 { format_currency(pl.quarterly_revenue) } else { "-".into() },
            total_revenue: format_currency(pl.total_revenue),
            quarterly_cost: format_currency(pl.quarterly_marketing_cost),
        }
    }).collect();

    let active_cats: std::collections::HashSet<String> = s.private_labels.iter()
        .filter(|pl| pl.status != PrivateLabelStatus::Discontinued)
        .map(|pl| pl.category_id.clone())
        .collect();

    let category_options: Vec<crate::api::dto::PrivateLabelCategoryOption> = private_label::get_category_configs().iter().map(|cfg| {
        let taken = active_cats.contains(&cfg.category_id);
        let can_afford = s.company.cash >= cfg.development_cost;
        let (can_start, reason, show_reason) = if taken {
            (false, "Already developed".into(), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(cfg.development_cost)), true)
        } else {
            (true, String::new(), false)
        };
        crate::api::dto::PrivateLabelCategoryOption {
            category_id: cfg.category_id.clone(),
            category_name: cfg.category_name.clone(),
            development_cost: format_currency(cfg.development_cost),
            development_quarters: cfg.development_quarters,
            base_margin: format!("{:.0}%", cfg.base_margin * 100.0),
            quarterly_cost: format_currency(cfg.quarterly_marketing_cost),
            can_start,
            reason,
            show_reason,
        }
    }).collect();

    let active_count = s.private_labels.iter().filter(|pl| pl.status == PrivateLabelStatus::Active).count();
    let developing_count = s.private_labels.iter().filter(|pl| pl.status == PrivateLabelStatus::Developing).count();
    let total_qr: f64 = s.private_labels.iter().filter(|pl| pl.status == PrivateLabelStatus::Active).map(|pl| pl.quarterly_revenue).sum();
    let total_tr: f64 = s.private_labels.iter().map(|pl| pl.total_revenue).sum();
    let total_qc: f64 = s.private_labels.iter().filter(|pl| pl.status == PrivateLabelStatus::Active).map(|pl| pl.quarterly_marketing_cost).sum();

    let cmo_skill = s.executives.iter().find(|e| e.position == ExecutivePosition::CMO).map(|e| format!("{:.0}/100", e.skill)).unwrap_or_else(|| "Not hired".into());

    crate::templates::PrivateLabelTemplate {
        brand_rows,
        category_options,
        active_count,
        developing_count,
        total_quarterly_revenue: format_currency(total_qr),
        total_pl_revenue: format_currency(total_tr),
        total_quarterly_cost: format_currency(total_qc),
        cmo_skill,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "private_label".to_string(),
    }.into_response()
}

pub async fn start_private_label(State(state): State<AppState>, Form(form): Form<super::dto::PrivateLabelForm>) -> Response {
    let mut state = state.lock().await;
    match private_label::start_development(&mut state, &form.category_id) {
        Ok(_cost) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/private-label").into_response()
}

pub async fn seasonal_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let active_rows: Vec<super::dto::SeasonalPromoRow> = s
        .seasonal_promotions
        .iter()
        .filter(|p| p.start_quarter == s.current_quarter && p.start_year == s.current_year)
        .map(|p| {
            let pt = p.promotion_type;
            super::dto::SeasonalPromoRow {
                id: p.id.clone(),
                name: pt.label().to_string(),
                icon: pt.icon().to_string(),
                description: pt.description().to_string(),
                revenue_boost: format!("+{:.0}%", pt.revenue_boost() * 100.0),
                reputation_boost: if pt.reputation_boost() >= 0.0 {
                    format!("+{:.1}", pt.reputation_boost())
                } else {
                    format!("{:.1}", pt.reputation_boost())
                },
                satisfaction_boost: format!("+{:.1}", pt.satisfaction_boost()),
                started: format!("Q{} {}", p.start_quarter, p.start_year),
                color_class: pt.color_class().to_string(),
            }
        })
        .collect();

    let current_quarter_active_count = s
        .seasonal_promotions
        .iter()
        .filter(|p| p.start_quarter == s.current_quarter && p.start_year == s.current_year)
        .count();

    let active_type_keys: std::collections::HashSet<String> = s
        .seasonal_promotions
        .iter()
        .filter(|p| p.start_quarter == s.current_quarter && p.start_year == s.current_year)
        .map(|p| p.promotion_type.key().to_string())
        .collect();

    let available = get_available_promotions_for_quarter(s.current_quarter);
    let can_activate_more = current_quarter_active_count < 2;

    let options: Vec<super::dto::SeasonalPromoOption> = available
        .iter()
        .map(|pt| {
            let already_active = active_type_keys.contains(pt.key());
            let can_afford = s.company.cash >= pt.base_cost();
            let (can_activate, reason, show_reason) = if already_active {
                (false, "Already active".into(), true)
            } else if !can_activate_more {
                (false, "Max 2 promotions per quarter".into(), true)
            } else if !can_afford {
                (false, format!("Need {}", format_currency(pt.base_cost())), true)
            } else {
                (true, String::new(), false)
            };
            super::dto::SeasonalPromoOption {
                key: pt.key().to_string(),
                name: pt.label().to_string(),
                icon: pt.icon().to_string(),
                description: pt.description().to_string(),
                cost: format_currency(pt.base_cost()),
                quarter_label: pt.quarter_label().to_string(),
                revenue_boost: format!("+{:.0}%", pt.revenue_boost() * 100.0),
                reputation_boost: if pt.reputation_boost() >= 0.0 {
                    format!("+{:.1}", pt.reputation_boost())
                } else {
                    format!("{:.1}", pt.reputation_boost())
                },
                satisfaction_boost: format!("+{:.1}", pt.satisfaction_boost()),
                color_class: pt.color_class().to_string(),
                can_activate,
                reason,
                show_reason,
            }
        })
        .collect();

    let effective_mult = seasonal_revenue_multiplier(s);
    let effective_pct = format!("{:.1}%", (effective_mult - 1.0) * 100.0);

    let cmo_skill = s
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CMO)
        .map(|e| format!("{:.0}/100", e.skill))
        .unwrap_or_else(|| "Not hired".into());

    let quarter_name = match s.current_quarter {
        1 => "Q1 - First Quarter",
        2 => "Q2 - Second Quarter",
        3 => "Q3 - Third Quarter",
        4 => "Q4 - Fourth Quarter",
        _ => "Unknown",
    };

    crate::templates::SeasonalTemplate {
        active_promotions: active_rows,
        available_options: options,
        current_quarter_label: quarter_name.to_string(),
        active_count: current_quarter_active_count,
        effective_revenue_bonus: effective_pct,
        cmo_skill,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "seasonal".to_string(),
    }
    .into_response()
}

pub async fn activate_seasonal_promo(
    State(state): State<AppState>,
    Form(form): Form<super::dto::SeasonalPromoForm>,
) -> Response {
    let mut state = state.lock().await;
    let promo_type = match SeasonalPromotionType::from_key(&form.promotion_type) {
        Some(pt) => pt,
        None => {
            state.push_message("Invalid seasonal promotion type.".into());
            return Redirect::to("/seasonal").into_response();
        }
    };
    match activate_promotion(&mut state, promo_type) {
        Ok(cost) => {
            state.push_message(format!(
                "Activated '{}' seasonal promotion for {}. It will boost revenue for this quarter.",
                promo_type.label(),
                format_currency(cost),
            ));
        }
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/seasonal").into_response()
}

pub async fn research_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;
    let lab = &s.research_lab;

    let active = lab.active_research();
    let has_active = active.is_some();
    let active_track_name = active.map(|p| p.track.label().to_string()).unwrap_or_default();
    let active_progress = active.map(|p| p.progress).unwrap_or(0.0);
    let active_quarters_remaining = active.map(|p| p.quarters_remaining).unwrap_or(0);

    let cto_skill = s.executives.iter().find(|e| e.position == ExecutivePosition::CTO).map(|e| format!("{:.0}/100", e.skill)).unwrap_or_else(|| "Not hired".into());

    let tracks: Vec<super::dto::ResearchTrackRow> = ResearchTrack::all_tracks().iter().map(|track| {
        let project = lab.get_project(*track);
        let current_level = project.map(|p| p.current_level).unwrap_or(0);
        let progress = project.map(|p| p.progress).unwrap_or(0.0);
        let quarters_remaining = project.map(|p| p.quarters_remaining).unwrap_or(0);
        let is_researching = project.map(|p| p.is_researching).unwrap_or(false);
        let max_level = track.max_level();
        let next_level = current_level + 1;
        let next_cost = if next_level <= max_level { format_currency(track.cost_per_level(current_level)) } else { "-".into() };
        let next_quarters = if next_level <= max_level { track.quarters_per_level(current_level) } else { 0 };
        let can_afford = s.company.cash >= track.cost_per_level(current_level);
        let has_stores = s.operating_store_count() >= track.min_stores();
        let (can_start, reason, show_reason) = if is_researching {
            (false, "Currently researching".into(), true)
        } else if has_active {
            (false, "Another project in progress".into(), true)
        } else if current_level >= max_level {
            (false, "Fully researched".into(), false)
        } else if !has_stores {
            (false, format!("Need {} stores", track.min_stores()), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(track.cost_per_level(current_level))), true)
        } else {
            (true, String::new(), false)
        };
        super::dto::ResearchTrackRow {
            key: track.key().to_string(),
            name: track.label().to_string(),
            icon: track.icon().to_string(),
            color_class: track.color_class().to_string(),
            description: track.description().to_string(),
            current_level,
            max_level,
            progress,
            progress_pct: format!("{:.1}", progress * 100.0),
            quarters_remaining,
            is_researching,
            effect_description: track.effect_description(current_level),
            next_cost,
            next_quarters,
            can_start,
            reason,
            show_reason,
            min_stores: track.min_stores(),
        }
    }).collect();

    crate::templates::ResearchTemplate {
        tracks,
        has_active,
        active_track_name,
        active_progress,
        active_progress_pct: format!("{:.1}", active_progress * 100.0),
        active_quarters_remaining,
        total_invested: format_currency(lab.total_invested),
        completed_count: lab.completed_count,
        cto_skill,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "research".to_string(),
    }.into_response()
}

pub async fn start_research_route(State(state): State<AppState>, Form(form): Form<super::dto::ResearchForm>) -> Response {
    let mut state = state.lock().await;
    let track = match ResearchTrack::from_key(&form.track) {
        Some(t) => t,
        None => {
            state.push_message("Invalid research track.".into());
            return Redirect::to("/research").into_response();
        }
    };
    match start_research(&mut state, track) {
        Ok(_cost) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/research").into_response()
}

pub async fn cancel_research_route(State(state): State<AppState>) -> Response {
    let mut state = state.lock().await;
    match cancel_research(&mut state) {
        Ok(_) => {}
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/research").into_response()
}

pub async fn csr_page(State(state): State<AppState>) -> Response {
    let state = state.lock().await;
    let s = &*state;

    let active_rows: Vec<crate::api::dto::CsrInitiativeRow> = s.csr.active_initiatives.iter().map(|i| {
        let init = i.initiative;
        crate::api::dto::CsrInitiativeRow {
            id: i.id.clone(),
            name: init.label().to_string(),
            icon: init.icon().to_string(),
            quarterly_cost: format_currency(init.quarterly_cost()),
            quarters_active: i.quarters_active,
            total_invested: format_currency(i.total_invested),
            reputation_bonus: format!("+{:.1}/Q", init.reputation_bonus()),
            satisfaction_bonus: format!("+{:.1}/Q", init.satisfaction_bonus()),
            morale_bonus: format!("+{:.1}/Q", init.employee_morale_bonus()),
        }
    }).collect();

    let active_keys: std::collections::HashSet<CsrInitiative> = s.csr.active_initiatives.iter().map(|i| i.initiative).collect();

    let options: Vec<crate::api::dto::CsrOptionRow> = CsrInitiative::all_initiatives().iter().map(|init| {
        let already_active = active_keys.contains(init);
        let can_afford = s.company.cash >= init.setup_cost();
        let has_stores = s.operating_store_count() >= init.min_stores();
        let (can_launch, reason, show_reason) = if already_active {
            (false, "Already active".into(), true)
        } else if !has_stores {
            (false, format!("Need {} stores", init.min_stores()), true)
        } else if !can_afford {
            (false, format!("Need {}", format_currency(init.setup_cost())), true)
        } else {
            (true, String::new(), false)
        };
        crate::api::dto::CsrOptionRow {
            key: init.key().to_string(),
            name: init.label().to_string(),
            icon: init.icon().to_string(),
            description: init.description().to_string(),
            setup_cost: format_currency(init.setup_cost()),
            quarterly_cost: format_currency(init.quarterly_cost()),
            reputation_bonus: format!("+{:.1}/Q", init.reputation_bonus()),
            satisfaction_bonus: format!("+{:.1}/Q", init.satisfaction_bonus()),
            morale_bonus: format!("+{:.1}/Q", init.employee_morale_bonus()),
            tax_deduction: format!("{:.0}%", init.tax_deduction_rate() * 100.0),
            min_stores: init.min_stores(),
            can_launch,
            reason,
            show_reason,
        }
    }).collect();

    let total_q_cost: f64 = s.csr.active_initiatives.iter().map(|i| i.initiative.quarterly_cost()).sum();
    let tax_ded = csr_tax_deduction(s);
    let chro_skill = s.executives.iter().find(|e| e.position == ExecutivePosition::CHRO).map(|e| format!("{:.0}/100", e.skill)).unwrap_or_else(|| "Not hired".into());

    crate::templates::CsrTemplate {
        active_initiatives: active_rows,
        options,
        csr_score: format!("{:.0}/100", s.csr.csr_score),
        active_count: s.csr.active_initiatives.len(),
        total_donated: format_currency(s.csr.total_donated),
        quarterly_cost: format_currency(total_q_cost),
        tax_deduction: format!("{:.1}%", tax_ded * 100.0),
        chro_skill,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "csr".to_string(),
    }.into_response()
}

pub async fn launch_csr_route(State(state): State<AppState>, Form(form): Form<super::dto::CsrForm>) -> Response {
    let mut state = state.lock().await;
    let initiative = match CsrInitiative::from_key(&form.initiative) {
        Some(i) => i,
        None => {
            state.push_message("Invalid CSR initiative.".into());
            return Redirect::to("/csr").into_response();
        }
    };
    match launch_initiative(&mut state, initiative) {
        Ok(cost) => {
            state.push_message(format!(
                "Launched '{}' CSR initiative for {}. It will provide ongoing benefits each quarter.",
                initiative.label(),
                format_currency(cost),
            ));
        }
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/csr").into_response()
}

pub async fn discontinue_csr_route(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let mut state = state.lock().await;
    match discontinue_initiative(&mut state, &id) {
        Ok(name) => {
            state.push_message(format!("Discontinued '{}' CSR initiative.", name));
        }
        Err(msg) => {
            state.push_message(msg.to_string());
        }
    }
    Redirect::to("/csr").into_response()
}

pub async fn employees_page(State(state): State<AppState>) -> Response {
    let s = state.lock().await;
    let s = &*s;

    let mut employee_rows: Vec<EmployeeRow> = Vec::new();
    for emp in &s.employee_system.employees {
        let store_name = if let Some(ref store_id) = emp.store_id {
            s.stores.iter()
                .find(|st| &st.id == store_id)
                .map(|st| format!("{} - {}", st.name, st.city)
                .unwrap_or_else(|| "Unknown Store".to_string())
        } else {
            "Corporate HQ".to_string()
        };

        let morale_class = if emp.morale >= 70.0 { "text-green-400" } else if emp.morale >= 50.0 { "text-yellow-400" } else { "text-red-400" });
            } else {
                "text-green-400".to_string()
            }
        });
    }

    let stats = EmployeeStats {
        total_count: s.employee_system.total_count
        monthly_payroll: format_currency_full(s.employee_system.monthly_payroll)
        avg_morale: format!("{:.1}%", s.employee_system.avg_morale)
        avg_skill: format!("{:.1}%", s.employee_system.avg_skill)
        turnover_rate: format!("{:.1}%", s.employee_system.turnover_rate)
    };
}

    let mut role_options: Vec<RoleOption> = Vec::new();
    for role in super::employees::EmployeeRole::all_roles() {
        role_options.push(RoleOption {
            key: role.key().to_string(),
            name: role.label().to_string(),
            category: role.category().label().to_string(),
            base_salary: format_currency_full(role.base_salary()),
        });
    }

    let stats = EmployeeStats {
        total_count: s.employee_system.total_count,
        monthly_payroll: format_currency_full(s.employee_system.monthly_payroll),
        avg_morale: format!("{:.1}%", s.employee_system.avg_morale),
        avg_skill: format!("{:.1}%", s.employee_system.avg_skill),
        turnover_rate: format!("{:.1}%", s.employee_system.turnover_rate),
    };

    let store_rows: Vec<StoreRow> = s.stores.iter().filter(|st| st.status == StoreStatus::Operating).map(|st| StoreRow {
        id: st.id.clone(),
        name: st.name.clone(),
        city: st.city.clone(),
        region: format!("{:?}", st.region),
        store_type: st.store_type.label().to_string(),
        size_sqm: st.size_sqm,
        status: st.status.label().to_string(),
        status_class: st.status.css_class().to_string(),
        quarterly_revenue: format_currency(st.quarterly_revenue),
        quarterly_expenses: format_currency(st.quarterly_expenses),
        quarterly_profit: format_currency(st.quarterly_revenue - st.quarterly_expenses),
        employees: st.employee_count,
        satisfaction: pct(st.satisfaction),
        age: if st.age_quarters < 4 { format!("{} Q", st.age_quarters) } else { format!("{} Y", st.age_quarters / 4) },
        can_close: false,
    }).collect();

    crate::templates::EmployeesTemplate {
        employees: employee_rows,
        role_options,
        stats,
        stores: store_rows,
        cash: format_currency_full(s.company.cash),
        messages: s.messages_vec(),
        current_quarter: s.current_quarter_label(),
        active_page: "employees".to_string(),
    }.into_response()
}

pub async fn hire_employee_route(State(state): State<AppState>, Form(form): Form<super::dto::HireEmployeeForm>) -> Response {
    let mut state = state.lock().await;
    let role = match super::employees::EmployeeRole::from_key(&form.role) {
        Some(r) => r,
        None => {
            state.push_message("Invalid employee role.".into());
            return Redirect::to("/employees").into_response();
        }
    };

    let store_id = form.store_id.as_ref().and(|s| s.as_str());
    let hr_multiplier = match state.policies.hr {
        HrPolicy::Minimal => 1.0,
        HrPolicy::Standard => 1.1,
        HrPolicy::Generous => 1.2,
        HrPolicy::Elite => 1.3,
    };

    let mut rng = rand::thread_rng();
    let hiring_cost = state.employee_system.hire(&mut rng, role, store_id.map(|s| s.to_string()), hr_multiplier);

    if state.company.cash < hiring_cost {
        state.push_message(format!("Cannot afford hiring cost of {}", format_currency(hiring_cost)));
        return Redirect::to("/employees").into_response();
    }

    state.company.cash -= hiring_cost;
    state.push_message(format!(
        "Hired new {} (Cost: {})",
        role.label(),
        format_currency(hiring_cost)
    ));

    Redirect::to("/employees").into_response()
}

pub async fn fire_employee_route(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let mut state = state.lock().await;
    match state.employee_system.fire(&id) {
        Some((name, severance)) => {
            if state.company.cash < severance {
                state.push_message(format!("Cannot afford severance of {}", format_currency(severance)));
                return Redirect::to("/employees").into_response();
            }
            state.company.cash -= severance;
            state.push_message(format!("Fired {}. Severance: {}", name, format_currency(severance)));
        }
        None => {
            state.push_message("Employee not found.".into());
        }
    }
    Redirect::to("/employees").into_response()
}

pub async fn train_employee_route(State(state): State<AppState>, Form(form): Form<super::dto::TrainEmployeeForm>) -> Response {
    let mut state = state.lock().await;
    let training_type = match super::employees::TrainingType::from_key(&form.training_type) {
        Some(t) => t,
        None => {
            state.push_message("Invalid training type.".into());
            return Redirect::to("/employees").into_response();
        }
    };

    match state.employee_system.train_employee(&form.employee_id, training_type) {
        Some(cost) => {
            if state.company.cash < cost {
                state.push_message(format!("Cannot afford training cost of {}", format_currency(cost)));
                return Redirect::to("/employees").into_response();
            }
            state.company.cash -= cost;
            state.push_message(format!("Training completed. Cost: {}", format_currency(cost)));
        }
        None => {
            state.push_message("Employee not found.".into());
        }
    }
    Redirect::to("/employees").into_response()
}
