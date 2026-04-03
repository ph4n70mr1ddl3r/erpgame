use rand::Rng;

use super::achievements::check_achievements;
use super::board::update_board;
use super::campaigns::{campaign_revenue_multiplier, process_campaigns};
use super::competitors::{
    average_competitor_strength, update_competitors_with_actions, PlayerActions,
};
use super::ecommerce::process_ecommerce;
use super::events::{generate_auto_events, generate_pending_events};
use super::loyalty::{loyalty_revenue_multiplier, update_loyalty};
use super::products::{
    total_product_margin_modifier, total_product_revenue_modifier, update_product_categories,
};
use super::state::*;
use super::supply_chain::process_supply_chain;
use super::upgrades::{
    get_store_cost_modifier, get_store_revenue_modifier, get_store_satisfaction_modifier,
};

fn find_exec_skill(state: &GameState, position: ExecutivePosition) -> Option<f64> {
    state
        .executives
        .iter()
        .find(|e| e.position == position)
        .map(|e| e.skill)
}

pub fn simulate_quarter(state: &mut GameState) {
    if state.game_over {
        return;
    }
    state.messages.clear();
    state.current_quarter += 1;
    if state.current_quarter > 4 {
        state.current_quarter = 1;
        state.current_year += 1;
    }

    let mut rng = rand::thread_rng();
    let quarter = state.current_quarter;

    update_economy(state, &mut rng);
    update_seasonality(state, quarter);
    process_store_construction(state);
    update_product_categories(
        &mut state.products,
        &mut rng,
        state.economy.construction_index,
        quarter,
    );

    let player_actions = PlayerActions {
        player_market_share: state.company.market_share,
        player_pricing: state.policies.pricing,
        player_expansion: state.policies.expansion,
        opened_new_store: state
            .stores
            .iter()
            .any(|s| s.status == StoreStatus::UnderConstruction),
    };
    update_competitors_with_actions(
        &mut state.competitors,
        &mut rng,
        &player_actions,
        &mut state.messages,
    );
    state.market.competitor_count = state.competitors.iter().map(|c| c.store_count).sum();
    state.market.competitor_strength = average_competitor_strength(&state.competitors);

    let operating_count = state.operating_store_count();
    let cfo_skill = find_exec_skill(state, ExecutivePosition::CFO);
    let coo_skill = find_exec_skill(state, ExecutivePosition::COO);
    let cto_skill = find_exec_skill(state, ExecutivePosition::CTO);

    let total_revenue = calculate_revenue(state, &mut rng, cfo_skill, coo_skill);
    let ecommerce_cost = process_ecommerce(state);
    let supply_chain_cost = process_supply_chain(state);
    let online_revenue = state.ecommerce.quarterly_online_revenue;
    let total_revenue = total_revenue + online_revenue;
    let total_expenses = calculate_expenses(state, operating_count, cto_skill);
    let loyalty_cost = update_loyalty(state);
    process_campaigns(state);
    let loan_payments = process_loans(state);
    let event_impacts = process_random_events(state, &mut rng, total_revenue);

    process_pending_event_generation(state, &mut rng);
    process_executive_decisions(state, &mut rng);
    let hiring_costs = update_employees(state, &mut rng);
    update_company_metrics(state, &mut rng);

    let operating_count = state.operating_store_count();
    let all_expenses =
        total_expenses + hiring_costs + loyalty_cost + ecommerce_cost + supply_chain_cost;
    let board_game_over = update_board(
        &mut state.board,
        total_revenue,
        all_expenses,
        state.company.market_share,
        operating_count,
        quarter,
        state.current_year,
        &mut state.messages,
    );

    let gross_profit = total_revenue - all_expenses;
    let net_profit = gross_profit - loan_payments + event_impacts.cash_impact;
    let tax = if net_profit > 0.0 {
        net_profit * state.economy.corporate_tax_rate / 100.0
    } else {
        0.0
    };
    let final_profit = net_profit - tax;

    state.company.cash += final_profit;
    state.company.total_revenue += total_revenue;
    state.company.total_expenses += all_expenses;
    state.company.total_profit += final_profit;

    let total_debt: f64 = state.company.loans.iter().map(|l| l.remaining).sum();
    state.company.company_value = state.company.cash
        + state
            .stores
            .iter()
            .filter(|s| s.status == StoreStatus::Operating)
            .map(|s| s.store_type.opening_cost() * 0.7)
            .sum::<f64>()
        + if total_revenue > 0.0 {
            total_revenue * 4.0
        } else {
            0.0
        }
        - total_debt;

    check_achievements(state, total_revenue);

    let report = QuarterlyReport {
        quarter,
        year: state.current_year,
        revenue: total_revenue,
        expenses: all_expenses,
        profit: final_profit,
        tax_paid: tax,
        cash_flow: final_profit,
        cash_on_hand: state.company.cash,
        event_impact: event_impacts.cash_impact,
        store_count: state.operating_store_count(),
        employee_count: state.employees.total_count,
        market_share: state.company.market_share,
        customer_satisfaction: state.company.customer_satisfaction,
        employee_satisfaction: state.company.employee_satisfaction,
        brand_reputation: state.company.brand_reputation,
    };
    state.financial_history.push(report);

    if state.company.cash < super::state::BANKRUPTCY_THRESHOLD {
        state.game_over = true;
        state.push_message(
            "GAME OVER: Your company has gone bankrupt! The board of directors has seized control."
                .into(),
        );
    }

    if board_game_over {
        state.game_over = true;
    }

    if state.company.company_value >= super::state::WINNING_VALUE {
        state.game_over = true;
        state.push_message(
            "CONGRATULATIONS: Bahay Depot has become a P10B+ company! You are a legendary CEO!"
                .into(),
        );
    }

    let summary = format!(
        "Q{} {} | Revenue: {} | Expenses: {} | Profit: {} | Cash: {} | Decisions: {} made, {} delegated",
        quarter, state.current_year,
        format_currency(total_revenue), format_currency(all_expenses),
        format_currency(final_profit), format_currency(state.company.cash),
        state.decisions_made, state.decisions_delegated,
    );
    state.push_message(summary);
}

fn process_pending_event_generation(state: &mut GameState, rng: &mut rand::rngs::ThreadRng) {
    let pending = generate_pending_events(state, rng);
    for event in pending {
        if state.delegation.is_delegated(event.category) {
            auto_resolve_event(state, &event, rng);
        } else {
            state.pending_events.push(event);
        }
    }
}

fn auto_resolve_event(
    state: &mut GameState,
    event: &PendingEvent,
    rng: &mut rand::rngs::ThreadRng,
) {
    let delegate_pos = event.category.delegate_position();
    let exec_info = state
        .executives
        .iter()
        .find(|e| e.position == delegate_pos)
        .map(|e| (e.position.short_title().to_string(), e.skill));

    let (choice_idx, description_prefix) = if let Some((ref exec_title, exec_skill)) = exec_info {
        if event.choices.is_empty() {
            (0, format!("[DELEGATED] {} chose", exec_title))
        } else {
            let best_idx = find_best_choice(&event.choices);
            let pick_best = rng.gen_bool(exec_skill / 100.0);
            let idx = if pick_best {
                best_idx
            } else {
                rng.gen_range(0..event.choices.len())
            };
            (idx, format!("[DELEGATED] {} chose", exec_title))
        }
    } else {
        let delegate_title = delegate_pos.short_title().to_string();
        if event.choices.is_empty() {
            (0, format!("[AUTO] No {} hired, random", delegate_title))
        } else {
            (
                rng.gen_range(0..event.choices.len()),
                format!("[AUTO] No {} hired, random", delegate_title),
            )
        }
    };

    if choice_idx < event.choices.len() {
        let choice = &event.choices[choice_idx];
        apply_event_effects(state, &choice.effects);
        state.decisions_delegated += 1;

        state.log_event(GameEvent {
            id: event.id.clone(),
            title: event.title.clone(),
            description: format!("{}: {}", description_prefix, choice.label),
            event_type: category_to_event_type(event.category),
            impact: EventImpact {
                cash_impact: choice.effects.cash,
                revenue_impact: choice.effects.revenue_modifier,
                expense_impact: choice.effects.expense_modifier,
                morale_impact: choice.effects.morale,
                reputation_impact: choice.effects.reputation,
                satisfaction_impact: choice.effects.satisfaction,
            },
            quarter: event.quarter,
            year: event.year,
        });

        state.push_message(format!(
            "{} decision on '{}': {}",
            description_prefix, event.title, choice.label
        ));
    }
}

fn find_best_choice(choices: &[EventChoice]) -> usize {
    if choices.is_empty() {
        return 0;
    }
    let mut best_idx = 0;
    let mut best_score: f64 = f64::NEG_INFINITY;
    for (i, c) in choices.iter().enumerate() {
        let score = c.effects.reputation
            + c.effects.morale
            + c.effects.satisfaction
            + c.effects.revenue_modifier * 50.0
            - c.effects.expense_modifier / 1_000_000.0
            + c.effects.cash / 1_000_000.0;
        if score > best_score {
            best_score = score;
            best_idx = i;
        }
    }
    best_idx
}

fn update_economy(state: &mut GameState, rng: &mut rand::rngs::ThreadRng) {
    let e = &mut state.economy;

    let gdp_shock = rng.gen_range(-0.6..0.6);
    let inflation_shock = rng.gen_range(-0.4..0.4);
    let interest_shock = rng.gen_range(-0.25..0.25);

    e.gdp_growth_rate = e.gdp_growth_rate * 0.85 + gdp_shock;

    let inflation_pressure = (e.gdp_growth_rate - 5.0) * 0.15;
    e.inflation_rate =
        (e.inflation_rate * 0.85 + inflation_pressure + inflation_shock).clamp(1.0, 10.0);

    let target_rate = 2.0 + e.inflation_rate * 0.7 + (e.gdp_growth_rate - 5.0) * 0.2;
    e.interest_rate = (e.interest_rate * 0.8 + target_rate * 0.2 + interest_shock).clamp(2.0, 12.0);

    e.gdp_growth_rate = e.gdp_growth_rate.clamp(0.5, 10.0);

    let construction_momentum = (e.gdp_growth_rate - 4.0) * 2.0;
    e.construction_index =
        (e.construction_index * 0.9 + construction_momentum + rng.gen_range(-3.0..3.0))
            .clamp(20.0, 100.0);

    let confidence_from_gdp = (e.gdp_growth_rate - 4.0) * 3.0;
    let confidence_from_inflation = -(e.inflation_rate - 3.0) * 2.0;
    e.consumer_confidence = (e.consumer_confidence * 0.85
        + confidence_from_gdp * 0.1
        + confidence_from_inflation * 0.1
        + rng.gen_range(-3.0..3.0))
    .clamp(15.0, 95.0);

    let peso_pressure = -(e.interest_rate - 5.0) * 0.3 + (e.inflation_rate - 3.0) * 0.5;
    e.peso_usd_rate =
        (e.peso_usd_rate * 0.92 + (55.0 + peso_pressure) * 0.08 + rng.gen_range(-0.8..0.8))
            .clamp(40.0, 70.0);

    e.minimum_wage_daily =
        (e.minimum_wage_daily + e.inflation_rate * 0.3 + rng.gen_range(-2.0..5.0))
            .clamp(450.0, 900.0);

    if state.current_quarter == 1 {
        let wage_increase = rng.gen_range(10.0..30.0);
        e.minimum_wage_daily = (e.minimum_wage_daily + wage_increase).clamp(450.0, 900.0);
        let msg = format!(
            "Government raised minimum wage to P{:.0}/day",
            e.minimum_wage_daily
        );
        state.messages.push_back(msg);
        while state.messages.len() > super::state::MAX_MESSAGES {
            state.messages.pop_front();
        }
    }
}

fn update_seasonality(state: &mut GameState, quarter: i32) {
    state.market.seasonal_multiplier = match quarter {
        1 => 0.85,
        2 => 1.0,
        3 => 1.05,
        4 => 1.25,
        _ => 1.0,
    };
}

fn process_store_construction(state: &mut GameState) {
    let mut opened: Vec<String> = Vec::new();
    for store in &mut state.stores {
        if store.status == StoreStatus::UnderConstruction {
            store.construction_quarters_left -= 1;
            if store.construction_quarters_left <= 0 {
                store.status = StoreStatus::Operating;
                store.age_quarters = 0;
                store.opened_quarter = state.current_quarter;
                store.opened_year = state.current_year;
                opened.push(format!("{} in {} is now OPEN!", store.name, store.city));
                continue;
            }
        }
        if store.status == StoreStatus::Operating {
            store.age_quarters += 1;
        }
    }
    for msg in opened {
        state.push_message(msg);
    }
}

fn calculate_revenue(
    state: &mut GameState,
    rng: &mut rand::rngs::ThreadRng,
    cfo_skill: Option<f64>,
    coo_skill: Option<f64>,
) -> f64 {
    let economy_mult = 1.0 + (state.economy.gdp_growth_rate - 4.0) / 100.0;
    let construction_mult = 1.0 + (state.economy.construction_index - 60.0) / 200.0;
    let confidence_mult = 1.0 + (state.economy.consumer_confidence - 60.0) / 200.0;
    let season_mult = state.market.seasonal_multiplier;
    let demand_mult = state.market.demand_trend;

    let pricing_mult = match state.policies.pricing {
        PricingPolicy::Budget => 1.25,
        PricingPolicy::Competitive => 1.1,
        PricingPolicy::Premium => 0.85,
        PricingPolicy::Dynamic => 1.05 + rng.gen_range(-0.1..0.1),
    };

    let service_mult = match state.policies.customer_service {
        CustomerServicePolicy::Basic => 0.9,
        CustomerServicePolicy::Good => 1.0,
        CustomerServicePolicy::Excellent => 1.1,
        CustomerServicePolicy::WhiteGlove => 1.2,
    };

    let marketing_mult = match state.policies.marketing {
        MarketingPolicy::LowKey => 0.85,
        MarketingPolicy::Moderate => 1.0,
        MarketingPolicy::Heavy => 1.15,
        MarketingPolicy::Aggressive => 1.3,
    };

    let expansion_mult = match state.policies.expansion {
        ExpansionPolicy::Conservative => 0.95,
        ExpansionPolicy::Moderate => 1.0,
        ExpansionPolicy::Aggressive => 1.05,
        ExpansionPolicy::Blitz => 1.10,
    };

    let morale_mult = 0.9 + (state.company.employee_satisfaction / 100.0) * 0.3;
    let reputation_mult = 0.8 + (state.company.brand_reputation / 100.0) * 0.4;

    let cfo_bonus = 1.0 + cfo_skill.unwrap_or(0.0) * 0.002;
    let coo_bonus = 1.0 + coo_skill.unwrap_or(0.0) * 0.003;

    let product_rev_mult = total_product_revenue_modifier(&state.products);

    let loyalty_rev_mult = loyalty_revenue_multiplier(state);

    let campaign_mult = campaign_revenue_multiplier(state);

    let skill_mult = 0.85 + (state.employees.avg_skill / 100.0) * 0.3;

    let mut city_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let mut region_counts: std::collections::HashMap<Region, usize> =
        std::collections::HashMap::new();
    for store in &state.stores {
        if store.status == StoreStatus::Operating {
            *city_counts.entry(store.city.clone()).or_insert(0) += 1;
            *region_counts.entry(store.region).or_insert(0) += 1;
        }
    }

    let mut total_revenue = 0.0;

    for store in &mut state.stores {
        if store.status != StoreStatus::Operating {
            store.quarterly_revenue = 0.0;
            continue;
        }

        let revenue_per_sqm_base = store.region.rent_multiplier() * 3000.0;
        let base_revenue = store.size_sqm as f64 * revenue_per_sqm_base;

        let growth_mult = if store.age_quarters < 4 {
            0.7 + store.age_quarters as f64 * 0.1
        } else {
            1.0
        };
        let competitor_penalty = if store_has_competitor_nearby(store) {
            0.85
        } else {
            1.0
        };

        let same_city = *city_counts.get(&store.city).unwrap_or(&1) as f64;
        let cannibalization = if same_city > 1.0 {
            1.0 / (1.0 + (same_city - 1.0) * 0.25)
        } else {
            1.0
        };

        let same_region = *region_counts.get(&store.region).unwrap_or(&1) as f64;
        let saturation = if same_region > 3.0 {
            1.0 / (1.0 + (same_region - 3.0) * 0.08)
        } else {
            1.0
        };

        let upgrade_rev_mult = get_store_revenue_modifier(&state.upgrades, &store.id);
        let noise = rng.gen_range(0.95..1.05);

        let store_revenue = base_revenue
            * economy_mult
            * construction_mult
            * confidence_mult
            * season_mult
            * demand_mult
            * pricing_mult
            * service_mult
            * marketing_mult
            * morale_mult
            * reputation_mult
            * growth_mult
            * competitor_penalty
            * cannibalization
            * saturation
            * cfo_bonus
            * coo_bonus
            * product_rev_mult
            * upgrade_rev_mult
            * expansion_mult
            * loyalty_rev_mult
            * campaign_mult
            * skill_mult
            * noise;

        let store_revenue = store_revenue.max(0.0);
        store.quarterly_revenue = store_revenue;
        store.customer_count = (store_revenue / 500.0 * rng.gen_range(0.8..1.2)) as u32;
        total_revenue += store_revenue;
    }

    total_revenue
}

fn store_has_competitor_nearby(store: &Store) -> bool {
    static COMPETITOR_CITIES: std::sync::LazyLock<std::collections::HashSet<&'static str>> =
        std::sync::LazyLock::new(|| {
            get_available_cities()
                .iter()
                .filter(|c| c.has_competitor)
                .map(|c| c.name.as_str())
                .collect()
        });
    COMPETITOR_CITIES.contains(store.city.as_str())
}

fn calculate_expenses(state: &mut GameState, operating_count: u32, cto_skill: Option<f64>) -> f64 {
    let avg_salary = if state.employees.total_count > 0 {
        state.employees.monthly_payroll / state.employees.total_count as f64
    } else {
        15000.0
    };

    let marketing_total = operating_count as f64
        * match state.policies.marketing {
            MarketingPolicy::LowKey => 100_000.0,
            MarketingPolicy::Moderate => 300_000.0,
            MarketingPolicy::Heavy => 600_000.0,
            MarketingPolicy::Aggressive => 1_000_000.0,
        };
    let cto_cost_reduction = 1.0 - cto_skill.unwrap_or(0.0) * 0.001;
    let product_cost_factor = 1.0 + (1.0 - total_product_margin_modifier(&state.products)) * 0.3;
    let sc_cost_modifier = super::supply_chain::supply_chain_cost_modifier(state);
    let mut total_expenses = 0.0;
    let cities = get_available_cities();
    let non_closed_count = state
        .stores
        .iter()
        .filter(|s| s.status != StoreStatus::Closed)
        .count()
        .max(1) as f64;
    for store in &mut state.stores {
        if store.status == StoreStatus::Closed {
            store.quarterly_expenses = 0.0;
            continue;
        }
        let rent_rate = cities
            .iter()
            .find(|c| c.name == store.city)
            .map(|c| c.rent_per_sqm)
            .unwrap_or(500.0);
        let monthly_rent = store.size_sqm as f64 * rent_rate;
        let quarterly_rent = monthly_rent * 3.0;
        let employee_count = if store.status == StoreStatus::UnderConstruction {
            (store.size_sqm as f64 * 0.005) as u32
        } else {
            (store.size_sqm as f64 * store.store_type.employees_per_sqm()) as u32
        };
        store.employee_count = employee_count;
        let quarterly_payroll = employee_count as f64 * avg_salary * 3.0;
        let utilities = store.size_sqm as f64 * 150.0 * 3.0;
        let inventory_cost = if store.status == StoreStatus::Operating {
            let estimated_revenue = if store.quarterly_revenue > 0.0 {
                store.quarterly_revenue
            } else {
                store.size_sqm as f64
                    * cities
                        .iter()
                        .find(|c| c.name == store.city)
                        .map(|c| c.rent_per_sqm)
                        .unwrap_or(500.0)
                    * store.region.rent_multiplier()
                    * 3000.0
            };
            estimated_revenue
                * match state.policies.inventory {
                    InventoryPolicy::Lean => 0.35,
                    InventoryPolicy::Standard => 0.42,
                    InventoryPolicy::Buffered => 0.50,
                    InventoryPolicy::Abundant => 0.60,
                }
                * product_cost_factor
                * sc_cost_modifier
        } else {
            0.0
        };
        let maintenance = store.size_sqm as f64 * 80.0 * 3.0;
        let service_cost = match state.policies.customer_service {
            CustomerServicePolicy::Basic => 0.0,
            CustomerServicePolicy::Good => employee_count as f64 * 2000.0 * 3.0,
            CustomerServicePolicy::Excellent => employee_count as f64 * 4000.0 * 3.0,
            CustomerServicePolicy::WhiteGlove => employee_count as f64 * 7000.0 * 3.0,
        };
        let hr_cost = match state.policies.hr {
            HrPolicy::Minimal => 0.0,
            HrPolicy::Standard => employee_count as f64 * 1000.0 * 3.0,
            HrPolicy::Generous => employee_count as f64 * 2500.0 * 3.0,
            HrPolicy::Elite => employee_count as f64 * 5000.0 * 3.0,
        };
        let construction_cost = if store.status == StoreStatus::UnderConstruction {
            store.store_type.opening_cost() / store.store_type.construction_quarters() as f64
        } else {
            0.0
        };
        let upgrade_cost_mult = get_store_cost_modifier(&state.upgrades, &store.id);
        let mut store_expenses = quarterly_rent
            + quarterly_payroll
            + utilities
            + inventory_cost
            + maintenance
            + marketing_total / non_closed_count
            + service_cost
            + hr_cost
            + construction_cost;
        store_expenses *= cto_cost_reduction * upgrade_cost_mult;
        store.quarterly_expenses = store_expenses;
        total_expenses += store_expenses;
    }
    let executive_payroll: f64 = state
        .executives
        .iter()
        .map(|e| e.salary_monthly * 3.0)
        .sum();
    total_expenses += executive_payroll;

    let expansion_overhead = match state.policies.expansion {
        ExpansionPolicy::Conservative => 300_000.0,
        ExpansionPolicy::Moderate => 500_000.0,
        ExpansionPolicy::Aggressive => 900_000.0,
        ExpansionPolicy::Blitz => 1_500_000.0,
    };
    total_expenses += expansion_overhead * 3.0;

    let total_store_employees: u32 = state.stores.iter().map(|s| s.employee_count).sum();
    total_expenses += (total_store_employees as f64 * 0.1) * avg_salary * 3.0;
    total_expenses
}

fn process_loans(state: &mut GameState) -> f64 {
    let mut total_payment = 0.0;
    let mut i = 0;
    while i < state.company.loans.len() {
        let loan = &mut state.company.loans[i];
        let quarterly_rate = loan.interest_rate / 100.0 / 4.0;
        let interest = loan.remaining * quarterly_rate;
        let principal = (loan.quarterly_payment - interest)
            .max(0.0)
            .min(loan.remaining);
        let actual_payment = interest + principal;
        loan.remaining -= principal;
        total_payment += actual_payment;
        loan.quarters_remaining -= 1;
        if loan.quarters_remaining <= 0 || loan.remaining <= 0.0 {
            state.company.loans.remove(i);
            state.push_message("A loan has been fully repaid.".into());
        } else {
            i += 1;
        }
    }
    total_payment
}

fn process_executive_decisions(state: &mut GameState, rng: &mut rand::rngs::ThreadRng) {
    let recent_profit = state
        .financial_history
        .last()
        .map(|r| r.profit)
        .unwrap_or(0.0);
    let company_profitable = recent_profit > 0.0;
    let performance_delta = if company_profitable { 1.0 } else { -3.0 };
    let morale_delta = if company_profitable { 1.5 } else { -2.0 };

    let mut pending_msgs: Vec<String> = Vec::new();

    for exec in &mut state.executives {
        exec.tenure_quarters += 1;
        exec.performance_rating =
            (exec.performance_rating + performance_delta + rng.gen_range(-3.0..3.0))
                .clamp(30.0, 100.0);
        exec.morale = (exec.morale + morale_delta + rng.gen_range(-2.0..2.0)).clamp(10.0, 100.0);
        exec.loyalty = (exec.loyalty + rng.gen_range(-2.0..2.0)).clamp(10.0, 100.0);

        if exec.tenure_quarters % 4 == 0 {
            let raise = exec.salary_monthly * rng.gen_range(0.02..0.08);
            exec.salary_monthly += raise;
            pending_msgs.push(format!(
                "{} got a raise to {}/month",
                exec.name,
                format_currency_full(exec.salary_monthly)
            ));
        }
    }

    let mut resigned: Vec<(String, ExecutivePosition)> = vec![];
    state.executives.retain(|exec| {
        let should_resign = (exec.morale < 25.0 && exec.loyalty < 30.0 && rng.gen_bool(0.5))
            || (exec.morale < 35.0 && exec.morale >= 25.0 && rng.gen_bool(0.2));
        if should_resign {
            resigned.push((exec.name.clone(), exec.position));
            false
        } else if exec.morale < 30.0 {
            pending_msgs.push(format!(
                "{} ({}) is considering leaving due to low morale!",
                exec.name,
                exec.position.short_title()
            ));
            true
        } else {
            true
        }
    });

    for msg in pending_msgs {
        state.push_message(msg);
    }

    for (name, position) in resigned {
        for cat in EventCategory::all_categories() {
            if cat.delegate_position() == position && state.delegation.is_delegated(cat) {
                state.delegation.set(cat, false);
            }
        }
        state.push_message(format!(
            "[RESIGNED] {} ({}) has resigned due to low morale! Position is now vacant.",
            name,
            position.short_title()
        ));
    }

    super::executive_ai::generate_recommendations(state);
}

fn update_employees(state: &mut GameState, rng: &mut rand::rngs::ThreadRng) -> f64 {
    let store_employees: u32 = state.stores.iter().map(|s| s.employee_count).sum();
    let overhead_staff = (store_employees as f64 * 0.1) as u32;
    state.employees.total_count = store_employees + overhead_staff + state.executives.len() as u32;

    let base_salary = state.economy.minimum_wage_daily * 22.0;
    let salary_multiplier = match state.policies.hr {
        HrPolicy::Minimal => 1.1,
        HrPolicy::Standard => 1.3,
        HrPolicy::Generous => 1.5,
        HrPolicy::Elite => 1.8,
    };
    let avg_salary = base_salary * salary_multiplier;
    state.employees.monthly_payroll = state.employees.total_count as f64 * avg_salary;

    let hr_morale_base = match state.policies.hr {
        HrPolicy::Minimal => 35.0,
        HrPolicy::Standard => 55.0,
        HrPolicy::Generous => 72.0,
        HrPolicy::Elite => 88.0,
    };
    let recent_profit = state
        .financial_history
        .last()
        .map(|r| r.profit)
        .unwrap_or(0.0);
    let performance_factor = if recent_profit > 0.0 { 5.0 } else { -5.0 };
    let exec_bonus = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CHRO)
        .map(|e| e.skill * 0.1)
        .unwrap_or(0.0);
    let target_morale = (hr_morale_base + performance_factor + exec_bonus).clamp(20.0, 95.0);
    state.employees.avg_morale += (target_morale - state.employees.avg_morale) * 0.2;
    state.employees.avg_morale =
        (state.employees.avg_morale + rng.gen_range(-3.0..3.0)).clamp(15.0, 98.0);
    state.company.employee_satisfaction = state.employees.avg_morale;

    let training_bonus = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CHRO)
        .map(|e| e.skill * 0.02)
        .unwrap_or(0.0);
    state.employees.avg_skill =
        (state.employees.avg_skill + training_bonus + rng.gen_range(-1.0..2.0)).clamp(30.0, 95.0);

    let base_turnover = match state.policies.hr {
        HrPolicy::Minimal => 12.0,
        HrPolicy::Standard => 7.0,
        HrPolicy::Generous => 4.0,
        HrPolicy::Elite => 2.0,
    };
    let morale_turnover = if state.employees.avg_morale < 40.0 {
        8.0
    } else {
        0.0
    };
    let turnover: f64 = base_turnover + morale_turnover + rng.gen_range(-2.0..2.0);
    state.employees.turnover_rate = turnover.clamp(1.0, 25.0);

    turnover / 100.0 * state.employees.total_count as f64 * avg_salary * 2.0
}

fn update_company_metrics(state: &mut GameState, rng: &mut rand::rngs::ThreadRng) {
    let operating = state.operating_store_count();

    if operating > 0 && !state.financial_history.is_empty() {
        let avg_satisfaction: f64 = state
            .stores
            .iter()
            .filter(|s| s.status == StoreStatus::Operating)
            .map(|s| s.satisfaction)
            .sum::<f64>()
            / operating as f64;
        state.company.customer_satisfaction =
            (state.company.customer_satisfaction * 0.7 + avg_satisfaction * 0.3).clamp(10.0, 100.0);
    }

    let market_base = operating as f64 * 1.5;
    let brand_bonus = state.company.brand_reputation / 100.0 * 2.0;
    let marketing_bonus = match state.policies.marketing {
        MarketingPolicy::LowKey => 0.5,
        MarketingPolicy::Moderate => 1.0,
        MarketingPolicy::Heavy => 1.8,
        MarketingPolicy::Aggressive => 2.5,
    };
    let competitor_pressure = state.market.competitor_strength / 100.0;
    let target_share =
        (market_base + brand_bonus + marketing_bonus - competitor_pressure).clamp(0.5, 35.0);
    state.company.market_share += (target_share - state.company.market_share) * 0.15;
    state.company.market_share =
        (state.company.market_share + rng.gen_range(-0.2..0.2)).clamp(0.1, 40.0);

    let rep_change = match state.policies.customer_service {
        CustomerServicePolicy::Basic => -0.5,
        CustomerServicePolicy::Good => 0.5,
        CustomerServicePolicy::Excellent => 1.5,
        CustomerServicePolicy::WhiteGlove => 2.5,
    };
    state.company.brand_reputation =
        (state.company.brand_reputation + rep_change + rng.gen_range(-1.0..1.5)).clamp(10.0, 100.0);

    state.market.demand_trend =
        (state.market.demand_trend + rng.gen_range(-0.03..0.03)).clamp(0.8, 1.3);
    state.market.competitor_strength =
        (state.market.competitor_strength + rng.gen_range(-3.0..3.0)).clamp(50.0, 100.0);

    for store in &mut state.stores {
        if store.status == StoreStatus::Operating {
            let upgrade_sat = get_store_satisfaction_modifier(&state.upgrades, &store.id);
            store.satisfaction =
                (state.company.customer_satisfaction + rng.gen_range(-5.0..5.0) + upgrade_sat)
                    .clamp(20.0, 100.0);
        }
    }
}

fn process_random_events(
    state: &mut GameState,
    rng: &mut rand::rngs::ThreadRng,
    total_revenue: f64,
) -> EventImpact {
    let events = generate_auto_events(state, rng);
    let mut total_impact = EventImpact {
        cash_impact: 0.0,
        revenue_impact: 0.0,
        expense_impact: 0.0,
        morale_impact: 0.0,
        reputation_impact: 0.0,
        satisfaction_impact: 0.0,
    };

    for event in events {
        state.log_event(event.clone());
        state.push_message(format!("[EVENT] {}", event.title));
        total_impact.cash_impact += event.impact.cash_impact;
        total_impact.cash_impact += event.impact.revenue_impact * total_revenue;
        total_impact.cash_impact -= event.impact.expense_impact;
        state.company.brand_reputation =
            (state.company.brand_reputation + event.impact.reputation_impact).clamp(5.0, 100.0);
        state.company.employee_satisfaction =
            (state.company.employee_satisfaction + event.impact.morale_impact).clamp(5.0, 100.0);
        state.company.customer_satisfaction = (state.company.customer_satisfaction
            + event.impact.satisfaction_impact)
            .clamp(5.0, 100.0);
    }

    total_impact
}
