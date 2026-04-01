use rand::Rng;

use super::state::*;

pub fn generate_events(state: &GameState, rng: &mut rand::rngs::ThreadRng) -> Vec<GameEvent> {
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
            impact: EventImpact {
                cash_impact: -damage,
                revenue_impact: -0.1 * severity,
                expense_impact: damage * 0.5,
                morale_impact: -5.0 * severity,
                reputation_impact: -2.0,
                satisfaction_impact: -5.0 * severity,
            },
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
            events.push(GameEvent {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Negative press coverage".into(),
                description: "A news outlet published a negative story about Bahay Depot's pricing practices.".into(),
                event_type: EventType::Marketing,
                impact: EventImpact {
                    cash_impact: -500_000.0,
                    revenue_impact: -0.03,
                    expense_impact: 200_000.0,
                    morale_impact: -3.0,
                    reputation_impact: -5.0,
                    satisfaction_impact: -3.0,
                },
                quarter: state.current_quarter,
                year: state.current_year,
            });
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
            events.push(GameEvent {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Construction boom!".into(),
                description: "Government infrastructure projects are driving high demand for construction materials.".into(),
                event_type: EventType::Economic,
                impact: EventImpact {
                    cash_impact: 0.0,
                    revenue_impact: 0.1,
                    expense_impact: 0.0,
                    morale_impact: 2.0,
                    reputation_impact: 2.0,
                    satisfaction_impact: 1.0,
                },
                quarter: state.current_quarter,
                year: state.current_year,
            });
        } else {
            events.push(GameEvent {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Economic slowdown warning".into(),
                description: "Economic indicators suggest a slowdown. Consumer spending may decrease.".into(),
                event_type: EventType::Economic,
                impact: EventImpact {
                    cash_impact: 0.0,
                    revenue_impact: -0.06,
                    expense_impact: 0.0,
                    morale_impact: -3.0,
                    reputation_impact: -1.0,
                    satisfaction_impact: -2.0,
                },
                quarter: state.current_quarter,
                year: state.current_year,
            });
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
            events.push(GameEvent {
                id: uuid::Uuid::new_v4().to_string(),
                title: "New competitor enters the market".into(),
                description: "A new international home improvement chain has announced entry into the Philippines.".into(),
                event_type: EventType::Competition,
                impact: EventImpact {
                    cash_impact: 0.0,
                    revenue_impact: -0.05,
                    expense_impact: 0.0,
                    morale_impact: -2.0,
                    reputation_impact: -1.0,
                    satisfaction_impact: 0.0,
                },
                quarter: state.current_quarter,
                year: state.current_year,
            });
        } else {
            events.push(GameEvent {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Competitor struggles with operations".into(),
                description: "A major competitor is experiencing operational difficulties, weakening their market position.".into(),
                event_type: EventType::Competition,
                impact: EventImpact {
                    cash_impact: 0.0,
                    revenue_impact: 0.04,
                    expense_impact: 0.0,
                    morale_impact: 2.0,
                    reputation_impact: 2.0,
                    satisfaction_impact: 0.0,
                },
                quarter: state.current_quarter,
                year: state.current_year,
            });
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
        events.push(GameEvent {
            id: uuid::Uuid::new_v4().to_string(),
            title: "New government regulation".into(),
            description: "The DTI has introduced new product safety standards requiring additional compliance costs.".into(),
            event_type: EventType::Regulation,
            impact: EventImpact {
                cash_impact: -1_000_000.0,
                revenue_impact: 0.0,
                expense_impact: 1_500_000.0,
                morale_impact: -1.0,
                reputation_impact: 1.0,
                satisfaction_impact: 0.0,
            },
            quarter: state.current_quarter,
            year: state.current_year,
        });
    }

    events
}
