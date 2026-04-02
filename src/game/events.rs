use rand::seq::SliceRandom;
use rand::Rng;

use super::state::*;

type EventGenerator = Box<dyn Fn(&GameState, &mut rand::rngs::ThreadRng) -> Option<PendingEvent>>;

pub fn generate_auto_events(state: &GameState, rng: &mut rand::rngs::ThreadRng) -> Vec<GameEvent> {
    let mut events = Vec::new();

    let typhoon_chance = match state.current_quarter {
        3 => 0.25,
        4 => 0.35,
        _ => 0.05,
    };

    if rng.gen_bool(typhoon_chance) {
        let severity = rng.gen_range(0.5..1.0);
        let damage = severity * 5_000_000.0 * state.operating_store_count() as f64;
        events.push(GameEvent {
            id: uuid::Uuid::new_v4().to_string(),
            title: format!("Typhoon hits the Philippines! (Severity: {:.0}%)", severity * 100.0),
            description: "A powerful typhoon has caused damage to stores and disrupted supply chains.".into(),
            event_type: EventType::NaturalDisaster,
            impact: EventImpact { cash_impact: -damage, revenue_impact: -0.1 * severity, expense_impact: damage * 0.5, morale_impact: -5.0 * severity, reputation_impact: -2.0, satisfaction_impact: -5.0 * severity },
            quarter: state.current_quarter,
            year: state.current_year,
        });
    }

    if rng.gen_bool(0.1) {
        let positive = rng.gen_bool(0.5);
        if positive {
            let boost = rng.gen_range(0.5..2.0) * 1_000_000.0;
            events.push(GameEvent {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Viral marketing success!".into(),
                description:
                    "A social media post about Bahay Depot went viral, boosting brand awareness."
                        .into(),
                event_type: EventType::Marketing,
                impact: EventImpact {
                    cash_impact: boost,
                    revenue_impact: 0.05,
                    expense_impact: 0.0,
                    morale_impact: 3.0,
                    reputation_impact: 5.0,
                    satisfaction_impact: 2.0,
                },
                quarter: state.current_quarter,
                year: state.current_year,
            });
        } else {
            events.push(GameEvent { id: uuid::Uuid::new_v4().to_string(), title: "Negative press coverage".into(), description: "A news outlet published a negative story about Bahay Depot's pricing practices.".into(), event_type: EventType::Marketing, impact: EventImpact { cash_impact: -500_000.0, revenue_impact: -0.03, expense_impact: 200_000.0, morale_impact: -3.0, reputation_impact: -5.0, satisfaction_impact: -3.0 }, quarter: state.current_quarter, year: state.current_year });
        }
    }

    if rng.gen_bool(0.08) {
        let cost = rng.gen_range(2_000_000.0..8_000_000.0);
        events.push(GameEvent {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Supply chain disruption".into(),
            description: "Shipping delays from overseas suppliers have caused inventory shortages."
                .into(),
            event_type: EventType::SupplyChain,
            impact: EventImpact {
                cash_impact: -cost,
                revenue_impact: -0.08,
                expense_impact: cost * 0.3,
                morale_impact: -2.0,
                reputation_impact: -2.0,
                satisfaction_impact: -4.0,
            },
            quarter: state.current_quarter,
            year: state.current_year,
        });
    }

    if rng.gen_bool(0.06) {
        if state.economy.gdp_growth_rate > 6.0 {
            events.push(GameEvent { id: uuid::Uuid::new_v4().to_string(), title: "Construction boom!".into(), description: "Government infrastructure projects are driving high demand for construction materials.".into(), event_type: EventType::Economic, impact: EventImpact { cash_impact: 0.0, revenue_impact: 0.1, expense_impact: 0.0, morale_impact: 2.0, reputation_impact: 2.0, satisfaction_impact: 1.0 }, quarter: state.current_quarter, year: state.current_year });
        } else {
            events.push(GameEvent { id: uuid::Uuid::new_v4().to_string(), title: "Economic slowdown warning".into(), description: "Economic indicators suggest a slowdown. Consumer spending may decrease.".into(), event_type: EventType::Economic, impact: EventImpact { cash_impact: 0.0, revenue_impact: -0.06, expense_impact: 0.0, morale_impact: -3.0, reputation_impact: -1.0, satisfaction_impact: -2.0 }, quarter: state.current_quarter, year: state.current_year });
        }
    }

    if rng.gen_bool(0.07) && state.employees.total_count > 20 {
        events.push(GameEvent {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Employee achievement recognized".into(),
            description: "A Bahay Depot employee team received an industry excellence award."
                .into(),
            event_type: EventType::Employee,
            impact: EventImpact {
                cash_impact: 0.0,
                revenue_impact: 0.01,
                expense_impact: 100_000.0,
                morale_impact: 5.0,
                reputation_impact: 3.0,
                satisfaction_impact: 1.0,
            },
            quarter: state.current_quarter,
            year: state.current_year,
        });
    }

    if rng.gen_bool(0.05) {
        let strong = rng.gen_bool(0.5);
        if strong {
            events.push(GameEvent { id: uuid::Uuid::new_v4().to_string(), title: "New competitor enters the market".into(), description: "A new international home improvement chain has announced entry into the Philippines.".into(), event_type: EventType::Competition, impact: EventImpact { cash_impact: 0.0, revenue_impact: -0.05, expense_impact: 0.0, morale_impact: -2.0, reputation_impact: -1.0, satisfaction_impact: 0.0 }, quarter: state.current_quarter, year: state.current_year });
        } else {
            events.push(GameEvent { id: uuid::Uuid::new_v4().to_string(), title: "Competitor struggles with operations".into(), description: "A major competitor is experiencing operational difficulties, weakening their market position.".into(), event_type: EventType::Competition, impact: EventImpact { cash_impact: 0.0, revenue_impact: 0.04, expense_impact: 0.0, morale_impact: 2.0, reputation_impact: 2.0, satisfaction_impact: 0.0 }, quarter: state.current_quarter, year: state.current_year });
        }
    }

    if state.current_quarter == 4 && rng.gen_bool(0.15) {
        let bonus = rng.gen_range(2_000_000.0..8_000_000.0);
        events.push(GameEvent {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Holiday season exceeds expectations!".into(),
            description: "Christmas spending and home renovation demand have driven record sales."
                .into(),
            event_type: EventType::Positive,
            impact: EventImpact {
                cash_impact: bonus,
                revenue_impact: 0.08,
                expense_impact: 0.0,
                morale_impact: 3.0,
                reputation_impact: 2.0,
                satisfaction_impact: 2.0,
            },
            quarter: state.current_quarter,
            year: state.current_year,
        });
    }

    if rng.gen_bool(0.04) {
        events.push(GameEvent { id: uuid::Uuid::new_v4().to_string(), title: "New government regulation".into(), description: "The DTI has introduced new product safety standards requiring additional compliance costs.".into(), event_type: EventType::Regulation, impact: EventImpact { cash_impact: -1_000_000.0, revenue_impact: 0.0, expense_impact: 1_500_000.0, morale_impact: -1.0, reputation_impact: 1.0, satisfaction_impact: 0.0 }, quarter: state.current_quarter, year: state.current_year });
    }

    events
}

pub fn generate_pending_events(
    state: &GameState,
    rng: &mut rand::rngs::ThreadRng,
) -> Vec<PendingEvent> {
    let mut events = Vec::new();
    let count = rng.gen_range(0..=2);
    if count == 0 {
        return events;
    }

    let mut pool: Vec<EventGenerator> = vec![
        Box::new(crisis_typhoon),
        Box::new(crisis_flood),
        Box::new(crisis_fire),
        Box::new(crisis_structural),
        Box::new(financial_credit_line),
        Box::new(financial_bulk_discount),
        Box::new(financial_real_estate),
        Box::new(financial_cost_cutting),
        Box::new(marketing_viral_trend),
        Box::new(marketing_trade_show),
        Box::new(marketing_celebrity),
        Box::new(marketing_viral_complaint),
        Box::new(hr_raise_demand),
        Box::new(hr_union),
        Box::new(hr_key_employee_poached),
        Box::new(hr_training_program),
        Box::new(supply_supplier_bankruptcy),
        Box::new(supply_shipping_increase),
        Box::new(supply_new_supplier),
        Box::new(supply_port_congestion),
        Box::new(comp_competitor_nearby),
        Box::new(comp_clearance_sale),
        Box::new(comp_competitor_struggles),
        Box::new(comp_consolidation_rumor),
        Box::new(tech_pos_system),
        Box::new(tech_cybersecurity),
        Box::new(tech_ecommerce),
        Box::new(tech_rfid),
        Box::new(reg_safety_standards),
        Box::new(reg_tax_audit),
        Box::new(reg_minimum_wage),
        Box::new(reg_environmental),
    ];

    pool.shuffle(rng);
    for gen in pool.iter() {
        if events.len() >= count {
            break;
        }
        if let Some(event) = gen(state, rng) {
            events.push(event);
        }
    }
    events
}

fn no_effects() -> EventEffects {
    EventEffects {
        cash: 0.0,
        revenue_modifier: 0.0,
        expense_modifier: 0.0,
        morale: 0.0,
        reputation: 0.0,
        satisfaction: 0.0,
    }
}

fn make_choice(label: &str, desc: &str, risk: &str, eff: EventEffects) -> EventChoice {
    EventChoice {
        id: uuid::Uuid::new_v4().to_string(),
        label: label.into(),
        description: desc.into(),
        effects: eff,
        risk_level: risk.into(),
    }
}

fn make_event(
    title: &str,
    desc: &str,
    cat: EventCategory,
    choices: Vec<EventChoice>,
    q: i32,
    y: i32,
) -> PendingEvent {
    PendingEvent {
        id: uuid::Uuid::new_v4().to_string(),
        title: title.into(),
        description: desc.into(),
        category: cat,
        choices,
        quarter: q,
        year: y,
    }
}

fn crisis_typhoon(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    let t = match s.current_quarter {
        3 => 0.2,
        4 => 0.3,
        _ => 0.03,
    };
    if !r.gen_bool(t) {
        return None;
    }
    Some(make_event("Typhoon Approaching the Philippines", "PAGASA has raised storm signal warnings. Your stores and supply chain are at risk.", EventCategory::Crisis, vec![
        make_choice("Full Preparation", "Board up stores, secure inventory, pre-position emergency supplies. Expensive but minimizes damage.", "Low", EventEffects { cash: -3_000_000.0, revenue_modifier: -0.02, expense_modifier: 1_500_000.0, morale: 5.0, reputation: 4.0, satisfaction: 3.0 }),
        make_choice("Minimal Preparation", "Basic precautions only. Save money but risk significant damage.", "High", EventEffects { cash: -500_000.0, revenue_modifier: -0.06, expense_modifier: 4_000_000.0, morale: -3.0, reputation: -2.0, satisfaction: -4.0 }),
        make_choice("Evacuate & Close", "Close all stores and evacuate staff. Maximum safety but loses revenue.", "Low", EventEffects { cash: -5_000_000.0, revenue_modifier: -0.10, expense_modifier: 2_000_000.0, morale: 8.0, reputation: 6.0, satisfaction: 2.0 }),
    ], s.current_quarter, s.current_year))
}

fn crisis_flood(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) || s.operating_store_count() == 0 {
        return None;
    }
    Some(make_event("Store Flood Damage", "Heavy rains have caused flooding in one of your stores. Inventory and fixtures are damaged.", EventCategory::Crisis, vec![
        make_choice("Fast Professional Repair", "Hire top contractors immediately. Store reopens quickly but costs more.", "Low", EventEffects { cash: -4_000_000.0, revenue_modifier: -0.01, expense_modifier: 1_000_000.0, morale: 2.0, reputation: 3.0, satisfaction: 2.0 }),
        make_choice("Budget Repair", "Use in-house team with cheaper materials. Takes longer but saves money.", "Medium", EventEffects { cash: -1_500_000.0, revenue_modifier: -0.05, expense_modifier: 500_000.0, morale: -2.0, reputation: -1.0, satisfaction: -3.0 }),
        make_choice("Outsource to Contractor", "Hire a general contractor. Moderate cost and timeline.", "Medium", EventEffects { cash: -2_500_000.0, revenue_modifier: -0.03, expense_modifier: 800_000.0, morale: 0.0, reputation: 1.0, satisfaction: 0.0 }),
    ], s.current_quarter, s.current_year))
}

fn crisis_fire(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.03) {
        return None;
    }
    Some(make_event("Fire in Warehouse", "A fire has broken out in your main warehouse. Significant inventory loss reported.", EventCategory::Crisis, vec![
        make_choice("Full Rebuild", "Rebuild warehouse with modern fire suppression. Expensive but prevents future incidents.", "Low", EventEffects { cash: -8_000_000.0, revenue_modifier: -0.02, expense_modifier: 2_000_000.0, morale: 3.0, reputation: 4.0, satisfaction: 1.0 }),
        make_choice("Relocate Warehouse", "Move to a new location. Fresh start but disruption to supply chain.", "Medium", EventEffects { cash: -6_000_000.0, revenue_modifier: -0.04, expense_modifier: 1_500_000.0, morale: -1.0, reputation: 0.0, satisfaction: -2.0 }),
        make_choice("File Insurance Claim", "Rely on insurance payout. Cheaper but slow and may not cover everything.", "High", EventEffects { cash: -2_000_000.0, revenue_modifier: -0.07, expense_modifier: 3_000_000.0, morale: -3.0, reputation: -2.0, satisfaction: -4.0 }),
    ], s.current_quarter, s.current_year))
}

fn crisis_structural(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.03) {
        return None;
    }
    Some(make_event("Structural Concern in Store", "Engineers found structural issues in one of your older stores. It may not meet updated building codes.", EventCategory::Crisis, vec![
        make_choice("Close & Renovate", "Close the store for full renovation. Ensures safety but loses revenue.", "Low", EventEffects { cash: -5_000_000.0, revenue_modifier: -0.04, expense_modifier: 1_000_000.0, morale: 3.0, reputation: 5.0, satisfaction: 1.0 }),
        make_choice("Reinforce & Continue", "Add structural reinforcements while keeping the store open. Riskier but maintains revenue.", "Medium", EventEffects { cash: -2_000_000.0, revenue_modifier: -0.01, expense_modifier: 800_000.0, morale: -1.0, reputation: 1.0, satisfaction: 0.0 }),
        make_choice("Monitor Only", "Monitor the situation and defer action. Saves money but risks future incidents.", "High", EventEffects { cash: -200_000.0, revenue_modifier: 0.0, expense_modifier: 0.0, morale: -4.0, reputation: -5.0, satisfaction: -2.0 }),
    ], s.current_quarter, s.current_year))
}

fn financial_credit_line(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.08) {
        return None;
    }
    Some(make_event("Bank Offers Credit Line", "BDO has offered Bahay Depot a credit line. This provides a financial safety net but adds debt risk.", EventCategory::Financial, vec![
        make_choice("Accept Full Credit Line", "Take the full credit line. Gives flexibility for expansion and emergencies.", "Medium", EventEffects { cash: 15_000_000.0, revenue_modifier: 0.02, expense_modifier: 500_000.0, morale: 1.0, reputation: 0.0, satisfaction: 0.0 }),
        make_choice("Decline", "Maintain financial independence. No debt but less flexibility.", "Low", no_effects()),
        make_choice("Negotiate Better Terms", "Push for lower interest rate. May succeed or the offer may be withdrawn.", "Low", EventEffects { cash: 10_000_000.0, revenue_modifier: 0.01, expense_modifier: 200_000.0, morale: 0.0, reputation: 1.0, satisfaction: 0.0 }),
    ], s.current_quarter, s.current_year))
}

fn financial_bulk_discount(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.07) {
        return None;
    }
    Some(make_event(
        "Supplier Offers Bulk Discount",
        "A major supplier is offering 15% discount on bulk orders. Minimum order of ₱10M required.",
        EventCategory::Financial,
        vec![
            make_choice(
                "Accept Full Deal",
                "Place a large bulk order. Saves on per-unit cost but ties up cash.",
                "Medium",
                EventEffects {
                    cash: -8_000_000.0,
                    revenue_modifier: 0.03,
                    expense_modifier: -2_000_000.0,
                    morale: 0.0,
                    reputation: 0.0,
                    satisfaction: 1.0,
                },
            ),
            make_choice(
                "Decline",
                "Maintain current ordering. No savings but no cash tied up.",
                "Low",
                no_effects(),
            ),
            make_choice(
                "Partial Order",
                "Order a moderate amount. Some savings with less cash commitment.",
                "Low",
                EventEffects {
                    cash: -4_000_000.0,
                    revenue_modifier: 0.01,
                    expense_modifier: -800_000.0,
                    morale: 0.0,
                    reputation: 0.0,
                    satisfaction: 0.0,
                },
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}

fn financial_real_estate(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event("Real Estate Investment Opportunity", "A prime commercial lot in a growing area is available for ₱25M. Could be used for a future store or leased out.", EventCategory::Financial, vec![
        make_choice("Buy the Property", "Purchase the lot. Long-term investment but large cash outlay.", "Medium", EventEffects { cash: -25_000_000.0, revenue_modifier: 0.01, expense_modifier: -500_000.0, morale: 1.0, reputation: 3.0, satisfaction: 0.0 }),
        make_choice("Lease Instead", "Sign a long-term lease. Less upfront cost but no asset ownership.", "Low", EventEffects { cash: -3_000_000.0, revenue_modifier: 0.005, expense_modifier: 500_000.0, morale: 0.0, reputation: 1.0, satisfaction: 0.0 }),
        make_choice("Pass", "Focus on existing operations. Keep cash reserves.", "Low", no_effects()),
    ], s.current_quarter, s.current_year))
}

fn financial_cost_cutting(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event(
        "Auditor Recommends Cost Cutting",
        "Your external auditor has identified areas where costs could be reduced.",
        EventCategory::Financial,
        vec![
            make_choice(
                "Across the Board Cuts",
                "Cut 10% across all departments. Quick savings but hurts morale broadly.",
                "High",
                EventEffects {
                    cash: 2_000_000.0,
                    revenue_modifier: -0.02,
                    expense_modifier: -3_000_000.0,
                    morale: -8.0,
                    reputation: -1.0,
                    satisfaction: -3.0,
                },
            ),
            make_choice(
                "Targeted Cuts",
                "Cut only in non-essential areas. Smarter savings with less disruption.",
                "Low",
                EventEffects {
                    cash: 1_000_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: -1_500_000.0,
                    morale: -2.0,
                    reputation: 1.0,
                    satisfaction: -1.0,
                },
            ),
            make_choice(
                "Decline Cuts",
                "Maintain current spending. Invest in growth rather than cutting.",
                "Low",
                EventEffects {
                    cash: 0.0,
                    revenue_modifier: 0.01,
                    expense_modifier: 500_000.0,
                    morale: 2.0,
                    reputation: 0.0,
                    satisfaction: 1.0,
                },
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}

fn marketing_viral_trend(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.08) {
        return None;
    }
    Some(make_event("Viral Social Media Trend", "A DIY home renovation challenge is trending on TikTok and Facebook. #BahayDepotMakeover is gaining traction.", EventCategory::Marketing, vec![
        make_choice("Capitalize on Trend", "Launch a branded campaign. Invest in social media ads and influencer partnerships.", "Low", EventEffects { cash: -2_000_000.0, revenue_modifier: 0.06, expense_modifier: 1_500_000.0, morale: 4.0, reputation: 7.0, satisfaction: 3.0 }),
        make_choice("Ignore", "Let the organic trend run its course. No cost but missed opportunity.", "Low", EventEffects { cash: 0.0, revenue_modifier: 0.01, expense_modifier: 0.0, morale: 0.0, reputation: 1.0, satisfaction: 0.0 }),
        make_choice("Counter with Own Campaign", "Create a competing hashtag and campaign. Expensive but owns the narrative.", "Medium", EventEffects { cash: -3_500_000.0, revenue_modifier: 0.04, expense_modifier: 2_000_000.0, morale: 2.0, reputation: 5.0, satisfaction: 2.0 }),
    ], s.current_quarter, s.current_year))
}

fn marketing_trade_show(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event(
        "Trade Show Invitation",
        "The Philippine Construction & Home Improvement Expo has invited Bahay Depot.",
        EventCategory::Marketing,
        vec![
            make_choice(
                "Sponsor the Event",
                "Become a gold sponsor. Maximum brand exposure but costs ₱3M.",
                "Medium",
                EventEffects {
                    cash: -3_000_000.0,
                    revenue_modifier: 0.04,
                    expense_modifier: 1_000_000.0,
                    morale: 3.0,
                    reputation: 6.0,
                    satisfaction: 1.0,
                },
            ),
            make_choice(
                "Attend with Booth",
                "Set up a standard booth. Good visibility at moderate cost.",
                "Low",
                EventEffects {
                    cash: -800_000.0,
                    revenue_modifier: 0.02,
                    expense_modifier: 300_000.0,
                    morale: 2.0,
                    reputation: 3.0,
                    satisfaction: 1.0,
                },
            ),
            make_choice(
                "Skip Event",
                "Focus on existing marketing channels. Save money but miss networking.",
                "Low",
                no_effects(),
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}

fn marketing_celebrity(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.04) {
        return None;
    }
    Some(make_event("Celebrity Endorsement Offer", "A popular Filipino celebrity has offered to endorse Bahay Depot. Their team proposes a ₱5M deal for a year.", EventCategory::Marketing, vec![
        make_choice("Accept Endorsement", "Sign the celebrity for full endorsement deal. Big brand boost.", "Medium", EventEffects { cash: -5_000_000.0, revenue_modifier: 0.07, expense_modifier: 2_000_000.0, morale: 5.0, reputation: 8.0, satisfaction: 3.0 }),
        make_choice("Decline", "Not worth the investment. Focus on organic growth.", "Low", no_effects()),
        make_choice("Negotiate Shorter Deal", "Propose a 3-month trial at ₱2M. Test before committing fully.", "Low", EventEffects { cash: -2_000_000.0, revenue_modifier: 0.03, expense_modifier: 800_000.0, morale: 2.0, reputation: 4.0, satisfaction: 1.0 }),
    ], s.current_quarter, s.current_year))
}

fn marketing_viral_complaint(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event("Customer Complaint Goes Viral", "A customer's complaint about poor service has gone viral on social media with 100K+ shares.", EventCategory::Marketing, vec![
        make_choice("Apologize & Compensate", "Issue public apology, compensate the customer, and announce service improvements.", "Low", EventEffects { cash: -1_000_000.0, revenue_modifier: 0.01, expense_modifier: 500_000.0, morale: -1.0, reputation: 5.0, satisfaction: 4.0 }),
        make_choice("Ignore It", "Let it blow over. Risk further damage to reputation.", "High", EventEffects { cash: 0.0, revenue_modifier: -0.03, expense_modifier: 0.0, morale: -3.0, reputation: -8.0, satisfaction: -5.0 }),
        make_choice("Investigate First", "Launch an internal investigation before responding publicly.", "Medium", EventEffects { cash: -300_000.0, revenue_modifier: -0.01, expense_modifier: 200_000.0, morale: -1.0, reputation: 2.0, satisfaction: 2.0 }),
    ], s.current_quarter, s.current_year))
}

fn hr_raise_demand(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.07) || s.employees.total_count < 10 {
        return None;
    }
    Some(make_event("Workers Demanding a Raise", "Store employees have submitted a petition requesting a 15% salary increase, citing rising cost of living.", EventCategory::HR, vec![
        make_choice("Grant Full Raise", "Approve the 15% raise. Happy employees but significant payroll increase.", "Low", EventEffects { cash: -3_000_000.0, revenue_modifier: 0.01, expense_modifier: 3_000_000.0, morale: 10.0, reputation: 4.0, satisfaction: 2.0 }),
        make_choice("Negotiate to 8%", "Counter-offer with 8% raise. Balanced approach that may satisfy most.", "Low", EventEffects { cash: -1_500_000.0, revenue_modifier: 0.0, expense_modifier: 1_500_000.0, morale: 5.0, reputation: 2.0, satisfaction: 1.0 }),
        make_choice("Reject Demand", "Deny the raise request. Save costs but risk strike and turnover.", "High", EventEffects { cash: 0.0, revenue_modifier: -0.02, expense_modifier: 0.0, morale: -12.0, reputation: -4.0, satisfaction: -3.0 }),
    ], s.current_quarter, s.current_year))
}

fn hr_union(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.04) || s.employees.total_count < 20 {
        return None;
    }
    Some(make_event("Union Organizing Effort", "Employees are discussing forming a union. This could lead to better labor relations or increased costs.", EventCategory::HR, vec![
        make_choice("Engage Proactively", "Open dialogue with employees, address concerns voluntarily.", "Low", EventEffects { cash: -1_500_000.0, revenue_modifier: 0.0, expense_modifier: 1_000_000.0, morale: 6.0, reputation: 3.0, satisfaction: 2.0 }),
        make_choice("Neutralize Effort", "Hire consultants to discourage unionization. Risky legally but effective short-term.", "High", EventEffects { cash: -800_000.0, revenue_modifier: 0.0, expense_modifier: 500_000.0, morale: -5.0, reputation: -3.0, satisfaction: -4.0 }),
        make_choice("Allow Union Formation", "Let the process unfold naturally. Legal but may increase costs long-term.", "Medium", EventEffects { cash: 0.0, revenue_modifier: -0.01, expense_modifier: 1_200_000.0, morale: 3.0, reputation: 1.0, satisfaction: 0.0 }),
    ], s.current_quarter, s.current_year))
}

fn hr_key_employee_poached(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event("Key Employee Poached by Competitor", "Your top warehouse manager has received a lucrative offer from a competitor. They're considering leaving.", EventCategory::HR, vec![
        make_choice("Counteroffer", "Match or exceed the competitor's offer. Keep the talent but set a precedent.", "Low", EventEffects { cash: -800_000.0, revenue_modifier: 0.01, expense_modifier: 600_000.0, morale: 3.0, reputation: 0.0, satisfaction: 1.0 }),
        make_choice("Let Them Go", "Accept the departure. Promote from within. Saves money but loses expertise.", "Medium", EventEffects { cash: 0.0, revenue_modifier: -0.02, expense_modifier: 0.0, morale: -4.0, reputation: -1.0, satisfaction: -2.0 }),
        make_choice("Negotiate Retention Package", "Offer a retention bonus tied to 1-year stay. Balanced approach.", "Low", EventEffects { cash: -400_000.0, revenue_modifier: 0.005, expense_modifier: 300_000.0, morale: 2.0, reputation: 0.0, satisfaction: 1.0 }),
    ], s.current_quarter, s.current_year))
}

fn hr_training_program(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event("Training Program Proposal", "HR has proposed a comprehensive employee training program covering customer service, product knowledge, and safety.", EventCategory::HR, vec![
        make_choice("Approve Full Program", "₱2M investment for all staff. Best long-term ROI but expensive upfront.", "Low", EventEffects { cash: -2_000_000.0, revenue_modifier: 0.03, expense_modifier: 500_000.0, morale: 8.0, reputation: 3.0, satisfaction: 5.0 }),
        make_choice("Approve Partial Program", "₱800K for key staff only. Good impact at lower cost.", "Low", EventEffects { cash: -800_000.0, revenue_modifier: 0.01, expense_modifier: 200_000.0, morale: 4.0, reputation: 1.0, satisfaction: 2.0 }),
        make_choice("Decline", "Not in the budget right now. Save money but miss skill improvement.", "Medium", EventEffects { cash: 0.0, revenue_modifier: 0.0, expense_modifier: 0.0, morale: -2.0, reputation: -1.0, satisfaction: -1.0 }),
    ], s.current_quarter, s.current_year))
}

fn supply_supplier_bankruptcy(
    s: &GameState,
    r: &mut rand::rngs::ThreadRng,
) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event("Major Supplier Bankruptcy", "One of your key suppliers has filed for bankruptcy. You need to secure alternative supply quickly.", EventCategory::SupplyChain, vec![
        make_choice("Find New Supplier", "Immediately source from alternative suppliers. Quick but may cost more.", "Medium", EventEffects { cash: -1_500_000.0, revenue_modifier: -0.02, expense_modifier: 2_000_000.0, morale: -1.0, reputation: 0.0, satisfaction: -2.0 }),
        make_choice("Negotiate Extension", "Work with the bankruptcy court to extend existing contracts temporarily.", "High", EventEffects { cash: -500_000.0, revenue_modifier: -0.04, expense_modifier: 1_000_000.0, morale: -2.0, reputation: -1.0, satisfaction: -3.0 }),
        make_choice("Prepay New Supplier", "Pay upfront for priority service from a new supplier. Most reliable but expensive.", "Low", EventEffects { cash: -3_000_000.0, revenue_modifier: -0.01, expense_modifier: 1_500_000.0, morale: 0.0, reputation: 1.0, satisfaction: -1.0 }),
    ], s.current_quarter, s.current_year))
}

fn supply_shipping_increase(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.07) {
        return None;
    }
    Some(make_event("Shipping Costs Increase", "Global shipping rates have increased 20% due to fuel costs. This affects your imported inventory significantly.", EventCategory::SupplyChain, vec![
        make_choice("Absorb Costs", "Maintain current prices and absorb the increase. Protects customers but hurts margins.", "Low", EventEffects { cash: -1_500_000.0, revenue_modifier: 0.0, expense_modifier: 2_500_000.0, morale: 0.0, reputation: 2.0, satisfaction: 2.0 }),
        make_choice("Pass to Customers", "Increase prices to cover shipping. Protects margins but may reduce sales.", "High", EventEffects { cash: 500_000.0, revenue_modifier: -0.04, expense_modifier: 500_000.0, morale: 0.0, reputation: -3.0, satisfaction: -4.0 }),
        make_choice("Mixed Approach", "Split the increase: absorb half, pass half to customers.", "Medium", EventEffects { cash: -500_000.0, revenue_modifier: -0.02, expense_modifier: 1_500_000.0, morale: 0.0, reputation: 0.0, satisfaction: -1.0 }),
    ], s.current_quarter, s.current_year))
}

fn supply_new_supplier(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event("New Supplier with Better Prices", "A new supplier from Vietnam is offering products at 12% lower cost. Quality is unproven but pricing is attractive.", EventCategory::SupplyChain, vec![
        make_choice("Switch Partially", "Move 30% of orders to new supplier. Test quality while saving some costs.", "Low", EventEffects { cash: -300_000.0, revenue_modifier: 0.01, expense_modifier: -800_000.0, morale: 0.0, reputation: 0.0, satisfaction: 0.0 }),
        make_choice("Switch Fully", "Move all orders to new supplier. Maximum savings but quality risk.", "High", EventEffects { cash: -200_000.0, revenue_modifier: 0.02, expense_modifier: -2_000_000.0, morale: 0.0, reputation: -2.0, satisfaction: -2.0 }),
        make_choice("Stay with Current", "Keep existing supplier relationships. Reliable but more expensive.", "Low", no_effects()),
    ], s.current_quarter, s.current_year))
}

fn supply_port_congestion(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event("Port Congestion Delays", "Manila ports are experiencing severe congestion. Your imported inventory is delayed by 3-4 weeks.", EventCategory::SupplyChain, vec![
        make_choice("Air Freight", "Ship critical inventory by air. Fast but 5x more expensive per kg.", "Low", EventEffects { cash: -3_000_000.0, revenue_modifier: -0.01, expense_modifier: 2_500_000.0, morale: 1.0, reputation: 1.0, satisfaction: 1.0 }),
        make_choice("Wait It Out", "Wait for port clearance. No extra cost but stockouts may occur.", "High", EventEffects { cash: 0.0, revenue_modifier: -0.06, expense_modifier: 500_000.0, morale: -2.0, reputation: -2.0, satisfaction: -5.0 }),
        make_choice("Reroute to Cebu Port", "Redirect shipments to Cebu port, then truck to Manila.", "Medium", EventEffects { cash: -1_200_000.0, revenue_modifier: -0.02, expense_modifier: 1_200_000.0, morale: 0.0, reputation: 0.0, satisfaction: -1.0 }),
    ], s.current_quarter, s.current_year))
}

fn comp_competitor_nearby(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event(
        "Competitor Opens Nearby",
        "A well-funded competitor has opened a store within 2km of one of your locations.",
        EventCategory::Competition,
        vec![
            make_choice(
                "Price War",
                "Aggressively cut prices to retain customers. Effective but hurts margins.",
                "High",
                EventEffects {
                    cash: -1_000_000.0,
                    revenue_modifier: 0.01,
                    expense_modifier: 1_500_000.0,
                    morale: -2.0,
                    reputation: -1.0,
                    satisfaction: 3.0,
                },
            ),
            make_choice(
                "Focus on Service",
                "Invest in customer experience and loyalty programs. Differentiate on quality.",
                "Low",
                EventEffects {
                    cash: -1_500_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: 1_000_000.0,
                    morale: 2.0,
                    reputation: 4.0,
                    satisfaction: 4.0,
                },
            ),
            make_choice(
                "Monitor & Adapt",
                "Watch the competitor's performance and adapt gradually.",
                "Medium",
                EventEffects {
                    cash: 0.0,
                    revenue_modifier: -0.03,
                    expense_modifier: 0.0,
                    morale: -1.0,
                    reputation: -1.0,
                    satisfaction: -1.0,
                },
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}

fn comp_clearance_sale(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event("Competitor Has Clearance Sale", "A major competitor is running a massive 50% clearance sale. Your customers are asking why you can't match.", EventCategory::Competition, vec![
        make_choice("Match Prices", "Match competitor prices on key items. Protects market share but hurts margins.", "High", EventEffects { cash: -2_000_000.0, revenue_modifier: 0.0, expense_modifier: 2_500_000.0, morale: -1.0, reputation: 0.0, satisfaction: 2.0 }),
        make_choice("Wait It Out", "Hold firm on prices. The sale will end.", "Medium", EventEffects { cash: 0.0, revenue_modifier: -0.04, expense_modifier: 0.0, morale: 0.0, reputation: 1.0, satisfaction: -2.0 }),
        make_choice("Counter-Promote", "Launch a value-added promotion (buy more, save more) instead of discounting.", "Low", EventEffects { cash: -1_000_000.0, revenue_modifier: 0.01, expense_modifier: 800_000.0, morale: 1.0, reputation: 2.0, satisfaction: 2.0 }),
    ], s.current_quarter, s.current_year))
}

fn comp_competitor_struggles(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event("Competitor Struggles Financially", "A competitor chain is reportedly on the verge of bankruptcy. Their prime locations may become available.", EventCategory::Competition, vec![
        make_choice("Acquire Their Leases", "Negotiate to take over their best store locations. Expensive but strategic.", "Medium", EventEffects { cash: -15_000_000.0, revenue_modifier: 0.05, expense_modifier: 2_000_000.0, morale: 3.0, reputation: 5.0, satisfaction: 2.0 }),
        make_choice("Aggressively Expand", "Open new stores in cities where they're weak. Capitalize on their weakness.", "Medium", EventEffects { cash: -8_000_000.0, revenue_modifier: 0.03, expense_modifier: 3_000_000.0, morale: 2.0, reputation: 3.0, satisfaction: 1.0 }),
        make_choice("Stay Course", "Focus on your own growth plan. Don't overextend.", "Low", EventEffects { cash: 0.0, revenue_modifier: 0.01, expense_modifier: 0.0, morale: 0.0, reputation: 1.0, satisfaction: 0.0 }),
    ], s.current_quarter, s.current_year))
}

fn comp_consolidation_rumor(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.04) {
        return None;
    }
    Some(make_event(
        "Industry Consolidation Rumor",
        "Rumors suggest two major competitors may merge, creating a dominant player.",
        EventCategory::Competition,
        vec![
            make_choice(
                "Prepare Defensively",
                "Strengthen customer loyalty programs and lock in supplier contracts.",
                "Low",
                EventEffects {
                    cash: -2_000_000.0,
                    revenue_modifier: 0.01,
                    expense_modifier: 1_000_000.0,
                    morale: 1.0,
                    reputation: 3.0,
                    satisfaction: 2.0,
                },
            ),
            make_choice(
                "Ignore Rumors",
                "Focus on your own strategy. Rumors may not materialize.",
                "Medium",
                no_effects(),
            ),
            make_choice(
                "Seek Partnership",
                "Explore partnerships with smaller retailers to create your own alliance.",
                "Medium",
                EventEffects {
                    cash: -1_000_000.0,
                    revenue_modifier: 0.02,
                    expense_modifier: 500_000.0,
                    morale: 0.0,
                    reputation: 2.0,
                    satisfaction: 1.0,
                },
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}

fn tech_pos_system(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event(
        "New POS System Available",
        "A modern cloud-based POS system with inventory integration and analytics is available.",
        EventCategory::Technology,
        vec![
            make_choice(
                "Upgrade All Stores",
                "Deploy across all locations. ₱4M investment for maximum efficiency gains.",
                "Low",
                EventEffects {
                    cash: -4_000_000.0,
                    revenue_modifier: 0.02,
                    expense_modifier: -1_000_000.0,
                    morale: 3.0,
                    reputation: 2.0,
                    satisfaction: 2.0,
                },
            ),
            make_choice(
                "Pilot in One Store",
                "Test in one location first. Lower risk but slower rollout.",
                "Low",
                EventEffects {
                    cash: -1_000_000.0,
                    revenue_modifier: 0.005,
                    expense_modifier: -200_000.0,
                    morale: 1.0,
                    reputation: 1.0,
                    satisfaction: 1.0,
                },
            ),
            make_choice(
                "Skip for Now",
                "Current systems work fine. Save money for other priorities.",
                "Low",
                no_effects(),
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}

fn tech_cybersecurity(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.04) {
        return None;
    }
    Some(make_event("Cybersecurity Incident", "Your IT team has detected unauthorized access to customer data. A breach may have occurred.", EventCategory::Technology, vec![
        make_choice("Full Incident Response", "Engage cybersecurity firm, notify affected customers, and upgrade all systems.", "Low", EventEffects { cash: -3_000_000.0, revenue_modifier: -0.01, expense_modifier: 2_000_000.0, morale: -1.0, reputation: 2.0, satisfaction: 1.0 }),
        make_choice("Minimal Response", "Patch the vulnerability quietly. Cheaper but risks regulatory penalties.", "High", EventEffects { cash: -500_000.0, revenue_modifier: 0.0, expense_modifier: 300_000.0, morale: 0.0, reputation: -4.0, satisfaction: -2.0 }),
        make_choice("PR-Focused Response", "Focus on communication and transparency while doing basic fixes.", "Medium", EventEffects { cash: -1_500_000.0, revenue_modifier: -0.01, expense_modifier: 1_000_000.0, morale: -1.0, reputation: 0.0, satisfaction: 0.0 }),
    ], s.current_quarter, s.current_year))
}

fn tech_ecommerce(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event("E-Commerce Platform Opportunity", "Online home improvement shopping is growing 30% year-over-year. Should Bahay Depot build an e-commerce presence?", EventCategory::Technology, vec![
        make_choice("Build In-House", "Develop custom e-commerce platform. ₱6M investment but full control.", "Medium", EventEffects { cash: -6_000_000.0, revenue_modifier: 0.05, expense_modifier: 2_000_000.0, morale: 3.0, reputation: 5.0, satisfaction: 3.0 }),
        make_choice("Use Existing Platform", "Sell through Shopee/Lazada. Quick to start but less brand control.", "Low", EventEffects { cash: -1_500_000.0, revenue_modifier: 0.02, expense_modifier: 1_000_000.0, morale: 1.0, reputation: 2.0, satisfaction: 2.0 }),
        make_choice("Skip E-Commerce", "Focus on physical retail. Online shopping is not core to home improvement.", "Medium", EventEffects { cash: 0.0, revenue_modifier: -0.01, expense_modifier: 0.0, morale: 0.0, reputation: -1.0, satisfaction: -1.0 }),
    ], s.current_quarter, s.current_year))
}

fn tech_rfid(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event("RFID Inventory Tracking", "A vendor is offering RFID inventory tracking systems that reduce stockout rates by up to 40%.", EventCategory::Technology, vec![
        make_choice("Adopt in All Stores", "Roll out RFID everywhere. ₱3M investment for maximum inventory efficiency.", "Low", EventEffects { cash: -3_000_000.0, revenue_modifier: 0.02, expense_modifier: -1_500_000.0, morale: 2.0, reputation: 2.0, satisfaction: 3.0 }),
        make_choice("Pilot in Flagship Store", "Test RFID in your best-performing store first.", "Low", EventEffects { cash: -800_000.0, revenue_modifier: 0.005, expense_modifier: -400_000.0, morale: 1.0, reputation: 1.0, satisfaction: 1.0 }),
        make_choice("Skip RFID", "Current barcode system works adequately. Not worth the investment yet.", "Low", no_effects()),
    ], s.current_quarter, s.current_year))
}

fn reg_safety_standards(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event(
        "New Product Safety Standards",
        "The DTI has issued new mandatory product safety standards for construction materials.",
        EventCategory::Regulation,
        vec![
            make_choice(
                "Comply Fully",
                "Upgrade all products and processes to meet standards.",
                "Low",
                EventEffects {
                    cash: -3_000_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: 1_500_000.0,
                    morale: 1.0,
                    reputation: 4.0,
                    satisfaction: 1.0,
                },
            ),
            make_choice(
                "Minimal Compliance",
                "Do only what's legally required. Cheapest but may face issues.",
                "High",
                EventEffects {
                    cash: -800_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: 400_000.0,
                    morale: 0.0,
                    reputation: -1.0,
                    satisfaction: 0.0,
                },
            ),
            make_choice(
                "Challenge Regulation",
                "File a petition questioning the standards.",
                "High",
                EventEffects {
                    cash: -500_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: 200_000.0,
                    morale: 0.0,
                    reputation: -2.0,
                    satisfaction: 0.0,
                },
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}

fn reg_tax_audit(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.04) {
        return None;
    }
    Some(make_event("Tax Audit Notice", "The BIR has sent a notice of audit for the last 3 years of financial records.", EventCategory::Regulation, vec![
        make_choice("Cooperate Fully", "Provide all records promptly and engage a tax consultant.", "Low", EventEffects { cash: -1_500_000.0, revenue_modifier: 0.0, expense_modifier: 1_000_000.0, morale: -1.0, reputation: 2.0, satisfaction: 0.0 }),
        make_choice("Prepare Legal Defense", "Hire tax lawyers to aggressively defend your position. More expensive but stronger protection.", "Medium", EventEffects { cash: -2_500_000.0, revenue_modifier: 0.0, expense_modifier: 1_500_000.0, morale: 0.0, reputation: 0.0, satisfaction: 0.0 }),
        make_choice("Negotiate Settlement", "Seek to negotiate a settlement with BIR before formal audit begins. May reduce exposure.", "Low", EventEffects { cash: -1_000_000.0, revenue_modifier: 0.0, expense_modifier: 800_000.0, morale: -1.0, reputation: 1.0, satisfaction: 0.0 }),
    ], s.current_quarter, s.current_year))
}

fn reg_minimum_wage(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.06) {
        return None;
    }
    Some(make_event("Minimum Wage Increase Expected", "Congress is discussing a significant minimum wage hike. How should Bahay Depot prepare?", EventCategory::Regulation, vec![
        make_choice("Preemptive Raise", "Raise wages now before the mandate. Good PR and easier transition.", "Low", EventEffects { cash: -2_000_000.0, revenue_modifier: 0.0, expense_modifier: 2_500_000.0, morale: 8.0, reputation: 5.0, satisfaction: 3.0 }),
        make_choice("Wait and See", "Wait for the law to pass before making changes. Keep current costs.", "Medium", EventEffects { cash: 0.0, revenue_modifier: 0.0, expense_modifier: 2_000_000.0, morale: -2.0, reputation: -1.0, satisfaction: -1.0 }),
        make_choice("Lobby Against", "Engage lobbyists to oppose or soften the wage increase. Expensive but may reduce impact.", "High", EventEffects { cash: -1_000_000.0, revenue_modifier: 0.0, expense_modifier: 1_000_000.0, morale: -5.0, reputation: -3.0, satisfaction: -2.0 }),
    ], s.current_quarter, s.current_year))
}

fn reg_environmental(s: &GameState, r: &mut rand::rngs::ThreadRng) -> Option<PendingEvent> {
    if !r.gen_bool(0.05) {
        return None;
    }
    Some(make_event(
        "Environmental Compliance Required",
        "New environmental regulations require waste management upgrades across all stores.",
        EventCategory::Regulation,
        vec![
            make_choice(
                "Full Upgrade",
                "Invest in comprehensive environmental upgrades. Most expensive but future-proof.",
                "Low",
                EventEffects {
                    cash: -4_000_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: 1_000_000.0,
                    morale: 1.0,
                    reputation: 5.0,
                    satisfaction: 1.0,
                },
            ),
            make_choice(
                "Minimum Compliance",
                "Do only the bare minimum required. Cheapest but risks future penalties.",
                "High",
                EventEffects {
                    cash: -1_000_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: 500_000.0,
                    morale: 0.0,
                    reputation: -1.0,
                    satisfaction: 0.0,
                },
            ),
            make_choice(
                "Phase Implementation",
                "Spread upgrades over 4 quarters. Manageable costs but slow compliance.",
                "Medium",
                EventEffects {
                    cash: -2_000_000.0,
                    revenue_modifier: 0.0,
                    expense_modifier: 800_000.0,
                    morale: 0.0,
                    reputation: 2.0,
                    satisfaction: 0.0,
                },
            ),
        ],
        s.current_quarter,
        s.current_year,
    ))
}
