use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardState {
    pub patience: f64,
    pub warnings: u32,
    pub pressure_level: f64,
    pub last_review_quarter: i32,
    pub last_review_year: i32,
    pub quarters_since_review: i32,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            patience: 75.0,
            warnings: 0,
            pressure_level: 25.0,
            last_review_quarter: 0,
            last_review_year: 0,
            quarters_since_review: 0,
        }
    }

    pub fn description(&self) -> String {
        if self.patience > 70.0 {
            "The board is satisfied with your leadership.".into()
        } else if self.patience > 50.0 {
            "The board has some concerns about company performance.".into()
        } else if self.patience > 30.0 {
            "The board is growing impatient. Improve results or face consequences.".into()
        } else {
            "WARNING: The board is considering replacing you as CEO!".into()
        }
    }

    pub fn patience_class(&self) -> &'static str {
        if self.patience > 70.0 {
            "text-green-400"
        } else if self.patience > 50.0 {
            "text-yellow-400"
        } else if self.patience > 30.0 {
            "text-orange-400"
        } else {
            "text-red-400"
        }
    }

    pub fn pressure_class(&self) -> &'static str {
        if self.pressure_level < 30.0 {
            "text-green-400"
        } else if self.pressure_level < 60.0 {
            "text-yellow-400"
        } else {
            "text-red-400"
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_board(
    board: &mut BoardState,
    quarterly_revenue: f64,
    quarterly_expenses: f64,
    market_share: f64,
    store_count: u32,
    quarter: i32,
    year: i32,
    messages: &mut VecDeque<String>,
) -> bool {
    board.quarters_since_review += 1;

    let is_review = quarter == 4;

    if is_review {
        board.last_review_quarter = quarter;
        board.last_review_year = year;
        board.quarters_since_review = 0;

        let profit_margin = if quarterly_revenue > 0.0 {
            (quarterly_revenue - quarterly_expenses) / quarterly_revenue * 100.0
        } else {
            0.0
        };

        let mut patience_delta = 0.0;

        if quarterly_revenue > 50_000_000.0 {
            patience_delta += 5.0;
        } else if quarterly_revenue > 20_000_000.0 {
            patience_delta += 2.0;
        } else if quarterly_revenue > 0.0 {
            patience_delta += 0.0;
        } else {
            patience_delta -= 8.0;
        }

        if profit_margin > 8.0 {
            patience_delta += 10.0;
        } else if profit_margin > 3.0 {
            patience_delta += 5.0;
        } else if profit_margin > 0.0 {
            patience_delta += 1.0;
        } else if profit_margin > -5.0 {
            patience_delta -= 8.0;
        } else {
            patience_delta -= 15.0;
        }

        if market_share > 8.0 {
            patience_delta += 5.0;
        } else if market_share < 2.0 && store_count > 3 {
            patience_delta -= 3.0;
        }

        if store_count >= 5 {
            patience_delta += 3.0;
        }

        board.patience = (board.patience + patience_delta).clamp(0.0, 100.0);
        board.pressure_level = (100.0 - board.patience).clamp(0.0, 100.0);

        if board.patience < 30.0 {
            board.warnings += 1;
            messages.push_back(format!(
                "[BOARD] Annual Review Q4 {}: Board issued warning #{}. Patience at {:.0}%. {}",
                year,
                board.warnings,
                board.patience,
                board.description()
            ));
        } else if board.patience < 50.0 {
            messages.push_back(format!(
                "[BOARD] Annual Review Q4 {}: Board expressed concerns. Patience at {:.0}%.",
                year, board.patience
            ));
        } else {
            messages.push_back(format!(
                "[BOARD] Annual Review Q4 {}: Board is pleased with progress. Patience at {:.0}%.",
                year, board.patience
            ));
        }

        board.patience <= 0.0
    } else {
        false
    }
}
