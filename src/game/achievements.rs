use super::state::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
    pub unlocked_quarter: Option<String>,
}

pub fn default_achievements() -> Vec<Achievement> {
    vec![
        Achievement {
            id: "first_steps".into(),
            title: "First Steps".into(),
            description: "Complete your first quarter as CEO".into(),
            icon: "Footprints".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "five_stores".into(),
            title: "Growing Chain".into(),
            description: "Operate 5 stores simultaneously".into(),
            icon: "Store".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "ten_stores".into(),
            title: "National Retailer".into(),
            description: "Operate 10 stores simultaneously".into(),
            icon: "Building2".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "first_100m".into(),
            title: "Nine-Figure Revenue".into(),
            description: "Generate P100M+ revenue in a single quarter".into(),
            icon: "TrendingUp".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "first_500m".into(),
            title: "Half Billion Quarter".into(),
            description: "Generate P500M+ revenue in a single quarter".into(),
            icon: "Rocket".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "billion".into(),
            title: "Billionaire Club".into(),
            description: "Reach P1B company value".into(),
            icon: "Gem".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "five_billion".into(),
            title: "Five Billion".into(),
            description: "Reach P5B company value".into(),
            icon: "Crown".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "dream_team".into(),
            title: "Dream Team".into(),
            description: "Fill all 6 C-suite positions".into(),
            icon: "Users".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "all_regions".into(),
            title: "Nationwide Presence".into(),
            description: "Have operating stores in all 4 regions".into(),
            icon: "Map".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "market_leader".into(),
            title: "Market Leader".into(),
            description: "Reach 15% market share".into(),
            icon: "Award".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "dominant".into(),
            title: "Dominant Force".into(),
            description: "Reach 25% market share".into(),
            icon: "Trophy".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "emp_champ".into(),
            title: "Employee Champion".into(),
            description: "Reach 90%+ employee satisfaction".into(),
            icon: "Heart".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "cust_fav".into(),
            title: "Customer Favorite".into(),
            description: "Reach 90%+ customer satisfaction".into(),
            icon: "Star".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "cash_king".into(),
            title: "Cash King".into(),
            description: "Have P200M+ cash on hand".into(),
            icon: "Banknote".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "debt_free".into(),
            title: "Debt Free".into(),
            description: "Have no outstanding loans".into(),
            icon: "CheckCircle".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "profit_streak".into(),
            title: "Profit Machine".into(),
            description: "5 consecutive profitable quarters".into(),
            icon: "Flame".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "product_pioneer".into(),
            title: "Product Pioneer".into(),
            description: "Max investment (100%) in all product categories".into(),
            icon: "Package".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "brand_legend".into(),
            title: "Brand Legend".into(),
            description: "Reach 95%+ brand reputation".into(),
            icon: "Sparkles".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "typhoon_vet".into(),
            title: "Typhoon Veteran".into(),
            description: "Survive 3 typhoon events".into(),
            icon: "CloudLightning".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
        Achievement {
            id: "winner".into(),
            title: "The P10B Club".into(),
            description: "Win the game: reach P10B+ company value".into(),
            icon: "Medal".into(),
            unlocked: false,
            unlocked_quarter: None,
        },
    ]
}

pub fn check_achievements(state: &mut GameState, last_revenue: f64) {
    let q_label = format!("Q{} {}", state.current_quarter, state.current_year);
    let operating = state.operating_store_count();

    check_one(state, "first_steps", &q_label, true);

    check_one(state, "five_stores", &q_label, operating >= 5);
    check_one(state, "ten_stores", &q_label, operating >= 10);

    check_one(state, "first_100m", &q_label, last_revenue >= 100_000_000.0);
    check_one(state, "first_500m", &q_label, last_revenue >= 500_000_000.0);

    check_one(
        state,
        "billion",
        &q_label,
        state.company.company_value >= 1_000_000_000.0,
    );
    check_one(
        state,
        "five_billion",
        &q_label,
        state.company.company_value >= 5_000_000_000.0,
    );

    check_one(state, "dream_team", &q_label, state.executives.len() >= 6);

    let has_all_regions = {
        let mut regions = std::collections::HashSet::new();
        for store in &state.stores {
            if store.status == StoreStatus::Operating {
                regions.insert(store.region);
            }
        }
        regions.len() >= 4
    };
    check_one(state, "all_regions", &q_label, has_all_regions);

    check_one(
        state,
        "market_leader",
        &q_label,
        state.company.market_share >= 15.0,
    );
    check_one(
        state,
        "dominant",
        &q_label,
        state.company.market_share >= 25.0,
    );
    check_one(
        state,
        "emp_champ",
        &q_label,
        state.company.employee_satisfaction >= 90.0,
    );
    check_one(
        state,
        "cust_fav",
        &q_label,
        state.company.customer_satisfaction >= 90.0,
    );
    check_one(
        state,
        "cash_king",
        &q_label,
        state.company.cash >= 200_000_000.0,
    );
    check_one(state, "debt_free", &q_label, state.company.loans.is_empty());

    let consecutive_profit = state
        .financial_history
        .iter()
        .rev()
        .take(5)
        .all(|r| r.profit > 0.0);
    check_one(
        state,
        "profit_streak",
        &q_label,
        consecutive_profit && state.financial_history.len() >= 5,
    );

    let all_max_invest = if let Some(products) = Some(&state.products) {
        products.iter().all(|p| p.investment_level >= 99.0)
    } else {
        false
    };
    check_one(state, "product_pioneer", &q_label, all_max_invest);
    check_one(
        state,
        "brand_legend",
        &q_label,
        state.company.brand_reputation >= 95.0,
    );

    let typhoon_count = state
        .event_log
        .iter()
        .filter(|e| e.title.contains("Typhoon"))
        .count();
    check_one(state, "typhoon_vet", &q_label, typhoon_count >= 3);
    check_one(
        state,
        "winner",
        &q_label,
        state.company.company_value >= 10_000_000_000.0,
    );
}

fn check_one(state: &mut GameState, id: &str, q_label: &str, condition: bool) {
    if !condition {
        return;
    }
    if let Some(ach) = state.achievements.iter_mut().find(|a| a.id == id) {
        if !ach.unlocked {
            ach.unlocked = true;
            ach.unlocked_quarter = Some(q_label.to_string());
            state.messages.push(format!(
                "[ACHIEVEMENT] Unlocked: {} - {}",
                ach.title, ach.description
            ));
        }
    }
}

pub fn unlocked_count(achievements: &[Achievement]) -> usize {
    achievements.iter().filter(|a| a.unlocked).count()
}
