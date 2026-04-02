use super::state::*;

fn fmt_months(v: f64) -> String {
    format!("{:.1}", v)
}

pub fn generate_recommendations(state: &mut GameState) {
    for i in 0..state.executives.len() {
        let rec = match state.executives[i].position {
            ExecutivePosition::CFO => cfo_recommendation(state, &state.executives[i]),
            ExecutivePosition::COO => coo_recommendation(state, &state.executives[i]),
            ExecutivePosition::CMO => cmo_recommendation(state, &state.executives[i]),
            ExecutivePosition::CTO => cto_recommendation(state, &state.executives[i]),
            ExecutivePosition::CHRO => chro_recommendation(state, &state.executives[i]),
            ExecutivePosition::CSCO => csco_recommendation(state, &state.executives[i]),
        };
        state.executives[i].recommendation = Some(rec);
    }
}

fn cfo_recommendation(state: &GameState, _exec: &Executive) -> String {
    let cash_months = state.company.cash / (state.company.total_expenses / 3.0).max(1.0);
    if cash_months < 3.0 {
        format!("URGENT: Cash reserves critically low ({} months runway). Consider reducing expenses or securing a loan.", fmt_months(cash_months))
    } else if state.company.cash > 100_000_000.0
        && state.policies.expansion == ExpansionPolicy::Conservative
    {
        format!("We have excess cash ({}). I recommend increasing our expansion pace or investing in store upgrades.", format_currency(state.company.cash))
    } else if !state.company.loans.is_empty() {
        format!("We have {} active loans. Total quarterly payments: {}. Consider early repayment if cash allows.", state.company.loans.len(), format_currency(state.company.loans.iter().map(|l| l.quarterly_payment).sum::<f64>()))
    } else if state.delegation.is_delegated(EventCategory::Financial) {
        "I'm handling financial decisions on your behalf. Current delegation is active for financial events.".into()
    } else {
        "Financials are stable. I recommend maintaining current spending levels and building cash reserves for expansion.".into()
    }
}

fn coo_recommendation(state: &GameState, _exec: &Executive) -> String {
    let operating = state.operating_store_count();
    let avg_satisfaction = if operating > 0 {
        state
            .stores
            .iter()
            .filter(|s| s.status == StoreStatus::Operating)
            .map(|s| s.satisfaction)
            .sum::<f64>()
            / operating as f64
    } else {
        50.0
    };

    if state.has_pending_events() && !state.delegation.is_delegated(EventCategory::Crisis) {
        format!("We have {} pending decisions. Consider delegating crisis/competition events to me for faster resolution.", state.pending_event_count())
    } else if avg_satisfaction < 50.0 {
        "Several stores are underperforming. I recommend reviewing store management, improving training programs, and upgrading customer service.".into()
    } else if operating < 3 {
        "We need more stores to capture market share. Based on my analysis, I recommend expanding to Cebu City or BGC Taguig next.".into()
    } else if state.delegation.is_delegated(EventCategory::Crisis)
        && state.delegation.is_delegated(EventCategory::Competition)
    {
        "I'm managing crisis and competition events. Operations are running smoothly.".into()
    } else {
        "Operations are running smoothly. I'll continue optimizing store performance and supply chain logistics.".into()
    }
}

fn cmo_recommendation(state: &GameState, _exec: &Executive) -> String {
    if state.company.brand_reputation < 40.0 {
        "Our brand reputation is suffering. I strongly recommend increasing our marketing budget and launching a customer satisfaction campaign.".into()
    } else if state.company.market_share < 3.0 {
        "Market share is low. I recommend aggressive marketing campaigns in our existing markets and a loyalty program to retain customers.".into()
    } else if state.delegation.is_delegated(EventCategory::Marketing) {
        "I'm handling marketing decisions on your behalf. Delegation is active for marketing events.".into()
    } else {
        "Marketing efforts are on track. Our brand awareness is growing steadily in our target markets.".into()
    }
}

fn cto_recommendation(state: &GameState, _exec: &Executive) -> String {
    if state.delegation.is_delegated(EventCategory::Technology) {
        "I'm handling technology decisions on your behalf. Current delegation is active for tech events.".into()
    } else {
        "Current IT systems are adequate. I recommend gradual upgrades to our point-of-sale and inventory tracking systems.".into()
    }
}

fn chro_recommendation(state: &GameState, _exec: &Executive) -> String {
    if state.employees.avg_morale < 40.0 {
        "Employee morale is dangerously low. I recommend immediate action: review compensation, improve working conditions, and address employee concerns.".into()
    } else if state.employees.turnover_rate > 10.0 {
        format!("Turnover rate is high at {}%. I recommend improving our HR policies and implementing a retention bonus program.", state.employees.turnover_rate)
    } else if state.delegation.is_delegated(EventCategory::HR) {
        "I'm handling HR decisions on your behalf. Delegation is active for HR events.".into()
    } else {
        "HR metrics are healthy. Employee satisfaction is stable and turnover is within acceptable range.".into()
    }
}

fn csco_recommendation(state: &GameState, _exec: &Executive) -> String {
    if state.delegation.is_delegated(EventCategory::SupplyChain) {
        "I'm handling supply chain decisions on your behalf. Delegation is active for supply chain events.".into()
    } else {
        match state.policies.inventory {
            InventoryPolicy::Lean => "Our lean inventory strategy is saving costs but risking stockouts. I recommend monitoring fill rates carefully.".into(),
            InventoryPolicy::Abundant => "Our inventory levels are very high, tying up capital. I recommend transitioning to a more balanced approach.".into(),
            _ => "Supply chain operations are running smoothly. I'm working on optimizing delivery routes between stores and warehouses.".into(),
        }
    }
}
