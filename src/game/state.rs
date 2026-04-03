use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

use super::achievements::default_achievements;
use super::board::BoardState;
use super::competitors::default_competitors;
use super::products::default_product_categories;

// Game constants
pub const STARTING_CASH: f64 = 80_000_000.0;
pub const BANKRUPTCY_THRESHOLD: f64 = -10_000_000.0;
pub const WINNING_VALUE: f64 = 10_000_000_000.0;
pub const MINIMUM_LOAN_AMOUNT: f64 = 10_000_000.0;
pub const MAX_EVENT_LOG_SIZE: usize = 50;
pub const MAX_MESSAGES: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub company: Company,
    pub stores: Vec<Store>,
    pub executives: Vec<Executive>,
    pub employees: EmployeePool,
    pub policies: Policies,
    pub economy: EconomyState,
    pub market: MarketState,
    pub financial_history: Vec<QuarterlyReport>,
    pub event_log: Vec<GameEvent>,
    pub current_quarter: i32,
    pub current_year: i32,
    pub game_over: bool,
    pub messages: Vec<String>,
    pub pending_events: Vec<PendingEvent>,
    pub delegation: DelegationSettings,
    pub decisions_made: u32,
    pub decisions_delegated: u32,
    pub products: Vec<super::products::ProductCategory>,
    pub upgrades: Vec<super::upgrades::StoreUpgrade>,
    pub board: BoardState,
    pub competitors: Vec<super::competitors::Competitor>,
    pub achievements: Vec<super::achievements::Achievement>,
    pub loyalty: super::loyalty::LoyaltyProgram,
    #[serde(default)]
    pub campaigns: Vec<super::campaigns::Campaign>,
    #[serde(default)]
    pub ecommerce: super::ecommerce::EcommerceChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub cash: f64,
    pub total_revenue: f64,
    pub total_expenses: f64,
    pub total_profit: f64,
    pub company_value: f64,
    pub brand_reputation: f64,
    pub customer_satisfaction: f64,
    pub employee_satisfaction: f64,
    pub market_share: f64,
    pub founded_quarter: i32,
    pub founded_year: i32,
    pub loans: Vec<Loan>,
    #[serde(default)]
    pub has_ever_had_loan: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    pub id: String,
    pub amount: f64,
    pub interest_rate: f64,
    pub remaining: f64,
    pub quarterly_payment: f64,
    pub quarters_remaining: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub id: String,
    pub name: String,
    pub city: String,
    pub region: Region,
    pub store_type: StoreType,
    pub size_sqm: u32,
    pub status: StoreStatus,
    pub quarterly_revenue: f64,
    pub quarterly_expenses: f64,
    pub customer_count: u32,
    pub employee_count: u32,
    pub satisfaction: f64,
    pub age_quarters: i32,
    pub construction_quarters_left: i32,
    pub opened_quarter: i32,
    pub opened_year: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum Region {
    MetroManila,
    Luzon,
    Visayas,
    Mindanao,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StoreType {
    Express,
    Standard,
    Mega,
    Depot,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StoreStatus {
    Operating,
    UnderConstruction,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Executive {
    pub id: String,
    pub name: String,
    pub position: ExecutivePosition,
    pub skill: f64,
    pub salary_monthly: f64,
    pub morale: f64,
    pub loyalty: f64,
    pub tenure_quarters: i32,
    pub performance_rating: f64,
    pub recommendation: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum ExecutivePosition {
    CFO,
    COO,
    CMO,
    CTO,
    CHRO,
    CSCO,
}

impl ExecutivePosition {
    pub fn title(&self) -> &str {
        match self {
            ExecutivePosition::CFO => "Chief Financial Officer",
            ExecutivePosition::COO => "Chief Operations Officer",
            ExecutivePosition::CMO => "Chief Marketing Officer",
            ExecutivePosition::CTO => "Chief Technology Officer",
            ExecutivePosition::CHRO => "Chief Human Resources Officer",
            ExecutivePosition::CSCO => "Chief Supply Chain Officer",
        }
    }

    pub fn short_title(&self) -> &str {
        match self {
            ExecutivePosition::CFO => "CFO",
            ExecutivePosition::COO => "COO",
            ExecutivePosition::CMO => "CMO",
            ExecutivePosition::CTO => "CTO",
            ExecutivePosition::CHRO => "CHRO",
            ExecutivePosition::CSCO => "CSCO",
        }
    }

    pub fn salary_range(&self) -> (f64, f64) {
        match self {
            ExecutivePosition::CFO => (250_000.0, 500_000.0),
            ExecutivePosition::COO => (250_000.0, 500_000.0),
            ExecutivePosition::CMO => (200_000.0, 400_000.0),
            ExecutivePosition::CTO => (200_000.0, 400_000.0),
            ExecutivePosition::CHRO => (180_000.0, 350_000.0),
            ExecutivePosition::CSCO => (200_000.0, 400_000.0),
        }
    }

    pub fn all_positions() -> Vec<ExecutivePosition> {
        vec![
            ExecutivePosition::CFO,
            ExecutivePosition::COO,
            ExecutivePosition::CMO,
            ExecutivePosition::CTO,
            ExecutivePosition::CHRO,
            ExecutivePosition::CSCO,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeePool {
    pub total_count: u32,
    pub monthly_payroll: f64,
    pub avg_morale: f64,
    pub avg_skill: f64,
    pub turnover_rate: f64,
    pub department_breakdown: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policies {
    pub pricing: PricingPolicy,
    pub hr: HrPolicy,
    pub expansion: ExpansionPolicy,
    pub customer_service: CustomerServicePolicy,
    pub marketing: MarketingPolicy,
    pub inventory: InventoryPolicy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PricingPolicy {
    Budget,
    Competitive,
    Premium,
    Dynamic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HrPolicy {
    Minimal,
    Standard,
    Generous,
    Elite,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ExpansionPolicy {
    Conservative,
    Moderate,
    Aggressive,
    Blitz,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CustomerServicePolicy {
    Basic,
    Good,
    Excellent,
    WhiteGlove,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarketingPolicy {
    LowKey,
    Moderate,
    Heavy,
    Aggressive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InventoryPolicy {
    Lean,
    Standard,
    Buffered,
    Abundant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyState {
    pub gdp_growth_rate: f64,
    pub inflation_rate: f64,
    pub interest_rate: f64,
    pub construction_index: f64,
    pub consumer_confidence: f64,
    pub peso_usd_rate: f64,
    pub minimum_wage_daily: f64,
    pub corporate_tax_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketState {
    pub total_market_size: f64,
    pub competitor_count: u32,
    pub competitor_strength: f64,
    pub seasonal_multiplier: f64,
    pub demand_trend: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarterlyReport {
    pub quarter: i32,
    pub year: i32,
    pub revenue: f64,
    pub expenses: f64,
    pub profit: f64,
    pub tax_paid: f64,
    pub cash_flow: f64,
    pub store_count: u32,
    pub employee_count: u32,
    pub market_share: f64,
    pub customer_satisfaction: f64,
    pub employee_satisfaction: f64,
    pub brand_reputation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub impact: EventImpact,
    pub quarter: i32,
    pub year: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventType {
    NaturalDisaster,
    Economic,
    Competition,
    Employee,
    Marketing,
    Regulation,
    SupplyChain,
    Positive,
    Negative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventImpact {
    pub cash_impact: f64,
    pub revenue_impact: f64,
    pub expense_impact: f64,
    pub morale_impact: f64,
    pub reputation_impact: f64,
    pub satisfaction_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityData {
    pub name: String,
    pub region: Region,
    pub rent_per_sqm: f64,
    pub demand_factor: f64,
    pub population: u32,
    pub has_competitor: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum EventCategory {
    Crisis,
    Financial,
    Marketing,
    HR,
    SupplyChain,
    Competition,
    Technology,
    Regulation,
}

impl EventCategory {
    pub fn label(&self) -> &str {
        match self {
            EventCategory::Crisis => "Crisis",
            EventCategory::Financial => "Financial",
            EventCategory::Marketing => "Marketing",
            EventCategory::HR => "HR",
            EventCategory::SupplyChain => "Supply Chain",
            EventCategory::Competition => "Competition",
            EventCategory::Technology => "Technology",
            EventCategory::Regulation => "Regulation",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            EventCategory::Crisis => "crisis",
            EventCategory::Financial => "financial",
            EventCategory::Marketing => "marketing",
            EventCategory::HR => "hr",
            EventCategory::SupplyChain => "supply_chain",
            EventCategory::Competition => "competition",
            EventCategory::Technology => "technology",
            EventCategory::Regulation => "regulation",
        }
    }

    pub fn delegate_position(&self) -> ExecutivePosition {
        match self {
            EventCategory::Crisis => ExecutivePosition::COO,
            EventCategory::Financial => ExecutivePosition::CFO,
            EventCategory::Marketing => ExecutivePosition::CMO,
            EventCategory::HR => ExecutivePosition::CHRO,
            EventCategory::SupplyChain => ExecutivePosition::CSCO,
            EventCategory::Competition => ExecutivePosition::COO,
            EventCategory::Technology => ExecutivePosition::CTO,
            EventCategory::Regulation => ExecutivePosition::CFO,
        }
    }

    pub fn all_categories() -> Vec<EventCategory> {
        vec![
            EventCategory::Crisis,
            EventCategory::Financial,
            EventCategory::Marketing,
            EventCategory::HR,
            EventCategory::SupplyChain,
            EventCategory::Competition,
            EventCategory::Technology,
            EventCategory::Regulation,
        ]
    }

    pub fn icon(&self) -> &str {
        match self {
            EventCategory::Crisis => "Tornado",
            EventCategory::Financial => "Chart",
            EventCategory::Marketing => "Megaphone",
            EventCategory::HR => "Users",
            EventCategory::SupplyChain => "Ship",
            EventCategory::Competition => "Building",
            EventCategory::Technology => "Cpu",
            EventCategory::Regulation => "Clipboard",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEffects {
    pub cash: f64,
    pub revenue_modifier: f64,
    pub expense_modifier: f64,
    pub morale: f64,
    pub reputation: f64,
    pub satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventChoice {
    pub id: String,
    pub label: String,
    pub description: String,
    pub effects: EventEffects,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingEvent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: EventCategory,
    pub choices: Vec<EventChoice>,
    pub quarter: i32,
    pub year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationSettings {
    pub delegated: HashMap<EventCategory, bool>,
}

impl DelegationSettings {
    pub fn all_false() -> Self {
        DelegationSettings {
            delegated: EventCategory::all_categories()
                .into_iter()
                .map(|c| (c, false))
                .collect(),
        }
    }

    pub fn is_delegated(&self, category: EventCategory) -> bool {
        self.delegated.get(&category).copied().unwrap_or(false)
    }

    pub fn set(&mut self, category: EventCategory, value: bool) {
        self.delegated.insert(category, value);
    }
}

impl Default for DelegationSettings {
    fn default() -> Self {
        Self::all_false()
    }
}

impl StoreType {
    pub fn default_size(&self) -> u32 {
        match self {
            StoreType::Express => 800,
            StoreType::Standard => 3500,
            StoreType::Mega => 12000,
            StoreType::Depot => 18000,
        }
    }

    pub fn opening_cost(&self) -> f64 {
        match self {
            StoreType::Express => 8_000_000.0,
            StoreType::Standard => 25_000_000.0,
            StoreType::Mega => 80_000_000.0,
            StoreType::Depot => 120_000_000.0,
        }
    }

    pub fn construction_quarters(&self) -> i32 {
        match self {
            StoreType::Express => 1,
            StoreType::Standard => 2,
            StoreType::Mega => 3,
            StoreType::Depot => 4,
        }
    }

    pub fn employees_per_sqm(&self) -> f64 {
        match self {
            StoreType::Express => 0.025,
            StoreType::Standard => 0.02,
            StoreType::Mega => 0.015,
            StoreType::Depot => 0.012,
        }
    }
}

impl Region {
    pub fn rent_multiplier(&self) -> f64 {
        match self {
            Region::MetroManila => 1.5,
            Region::Luzon => 1.0,
            Region::Visayas => 0.8,
            Region::Mindanao => 0.7,
        }
    }
}

pub fn get_available_cities() -> &'static [CityData] {
    static CITIES: LazyLock<Vec<CityData>> = LazyLock::new(|| {
        vec![
            CityData {
                name: "Makati".into(),
                region: Region::MetroManila,
                rent_per_sqm: 1200.0,
                demand_factor: 1.4,
                population: 630_000,
                has_competitor: true,
            },
            CityData {
                name: "BGC Taguig".into(),
                region: Region::MetroManila,
                rent_per_sqm: 1100.0,
                demand_factor: 1.35,
                population: 800_000,
                has_competitor: true,
            },
            CityData {
                name: "Quezon City".into(),
                region: Region::MetroManila,
                rent_per_sqm: 800.0,
                demand_factor: 1.3,
                population: 3_000_000,
                has_competitor: true,
            },
            CityData {
                name: "Pasig".into(),
                region: Region::MetroManila,
                rent_per_sqm: 750.0,
                demand_factor: 1.2,
                population: 800_000,
                has_competitor: false,
            },
            CityData {
                name: "Manila".into(),
                region: Region::MetroManila,
                rent_per_sqm: 700.0,
                demand_factor: 1.1,
                population: 1_800_000,
                has_competitor: true,
            },
            CityData {
                name: "Caloocan".into(),
                region: Region::MetroManila,
                rent_per_sqm: 400.0,
                demand_factor: 0.9,
                population: 1_600_000,
                has_competitor: false,
            },
            CityData {
                name: "Parañaque".into(),
                region: Region::MetroManila,
                rent_per_sqm: 650.0,
                demand_factor: 1.15,
                population: 700_000,
                has_competitor: false,
            },
            CityData {
                name: "Las Piñas".into(),
                region: Region::MetroManila,
                rent_per_sqm: 500.0,
                demand_factor: 1.0,
                population: 600_000,
                has_competitor: false,
            },
            CityData {
                name: "Cebu City".into(),
                region: Region::Visayas,
                rent_per_sqm: 600.0,
                demand_factor: 1.2,
                population: 1_000_000,
                has_competitor: true,
            },
            CityData {
                name: "Mandaue".into(),
                region: Region::Visayas,
                rent_per_sqm: 400.0,
                demand_factor: 1.0,
                population: 400_000,
                has_competitor: false,
            },
            CityData {
                name: "Davao City".into(),
                region: Region::Mindanao,
                rent_per_sqm: 450.0,
                demand_factor: 1.1,
                population: 1_800_000,
                has_competitor: true,
            },
            CityData {
                name: "Cagayan de Oro".into(),
                region: Region::Mindanao,
                rent_per_sqm: 350.0,
                demand_factor: 0.9,
                population: 700_000,
                has_competitor: false,
            },
            CityData {
                name: "Iloilo City".into(),
                region: Region::Visayas,
                rent_per_sqm: 380.0,
                demand_factor: 0.95,
                population: 500_000,
                has_competitor: false,
            },
            CityData {
                name: "Baguio City".into(),
                region: Region::Luzon,
                rent_per_sqm: 500.0,
                demand_factor: 0.85,
                population: 370_000,
                has_competitor: false,
            },
            CityData {
                name: "Clark Angeles".into(),
                region: Region::Luzon,
                rent_per_sqm: 400.0,
                demand_factor: 0.9,
                population: 500_000,
                has_competitor: false,
            },
            CityData {
                name: "Angeles".into(),
                region: Region::Luzon,
                rent_per_sqm: 380.0,
                demand_factor: 0.85,
                population: 450_000,
                has_competitor: false,
            },
            CityData {
                name: "San Fernando Pampanga".into(),
                region: Region::Luzon,
                rent_per_sqm: 350.0,
                demand_factor: 0.8,
                population: 350_000,
                has_competitor: false,
            },
            CityData {
                name: "General Santos".into(),
                region: Region::Mindanao,
                rent_per_sqm: 300.0,
                demand_factor: 0.75,
                population: 600_000,
                has_competitor: false,
            },
            CityData {
                name: "Zamboanga City".into(),
                region: Region::Mindanao,
                rent_per_sqm: 280.0,
                demand_factor: 0.7,
                population: 900_000,
                has_competitor: false,
            },
            CityData {
                name: "Batangas City".into(),
                region: Region::Luzon,
                rent_per_sqm: 350.0,
                demand_factor: 0.85,
                population: 330_000,
                has_competitor: false,
            },
            CityData {
                name: "Lipa City".into(),
                region: Region::Luzon,
                rent_per_sqm: 320.0,
                demand_factor: 0.8,
                population: 280_000,
                has_competitor: false,
            },
            CityData {
                name: "Sta. Rosa Laguna".into(),
                region: Region::Luzon,
                rent_per_sqm: 450.0,
                demand_factor: 0.95,
                population: 500_000,
                has_competitor: false,
            },
            CityData {
                name: "Tagaytay".into(),
                region: Region::Luzon,
                rent_per_sqm: 500.0,
                demand_factor: 0.8,
                population: 70_000,
                has_competitor: false,
            },
            CityData {
                name: "Antipolo".into(),
                region: Region::MetroManila,
                rent_per_sqm: 400.0,
                demand_factor: 0.95,
                population: 800_000,
                has_competitor: false,
            },
        ]
    });
    &CITIES
}

pub fn generate_executive_name(rng: &mut rand::rngs::ThreadRng) -> String {
    let first_names = [
        "Maria",
        "Jose",
        "Ana",
        "Juan",
        "Carmen",
        "Ricardo",
        "Elena",
        "Miguel",
        "Rosa",
        "Antonio",
        "Luz",
        "Fernando",
        "Grace",
        "Roberto",
        "Isabel",
        "Daniel",
        "Teresa",
        "Alberto",
        "Patricia",
        "Eduardo",
        "Concepcion",
        "Ramon",
        "Victoria",
        "Alejandro",
        "Cristina",
        "Francisco",
        "Nora",
        "Manuel",
        "Lourdes",
        "Arturo",
        "Socorro",
        "Gregorio",
        "Remedios",
        "Ernesto",
        "Milagros",
        "Rafael",
        "Purificacion",
        "Adolfo",
        "Charito",
    ];
    let last_names = [
        "Santos",
        "Reyes",
        "Cruz",
        "Garcia",
        "Mendoza",
        "Tan",
        "Lim",
        "Lopez",
        "Gonzales",
        "Bautista",
        "Vergara",
        "Aquino",
        "Roxas",
        "Ong",
        "Chua",
        "Co",
        "Sy",
        "Dizon",
        "Navarro",
        "Villanueva",
        "Fernandez",
        "Del Rosario",
        "Santiago",
        "Rivera",
        "Torres",
        "Ramos",
        "Flores",
        "Hernandez",
        "Molina",
        "Sanchez",
        "David",
        "Carillo",
        "Dela Cruz",
        "Paredes",
        "Aguirre",
        "Yanson",
        "Gokongwei",
        "Sy-Cip",
        "Ang",
        "Razon",
    ];
    let first = first_names[rng.gen_range(0..first_names.len())];
    let last = last_names[rng.gen_range(0..last_names.len())];
    format!("{} {}", first, last)
}

pub fn format_currency(amount: f64) -> String {
    let prefix = if amount < 0.0 { "-₱" } else { "₱" };
    let abs = amount.abs();
    if abs >= 1_000_000_000.0 {
        format!("{}{:.1}B", prefix, abs / 1_000_000_000.0)
    } else if abs >= 1_000_000.0 {
        format!("{}{:.1}M", prefix, abs / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{}{:.0}K", prefix, abs / 1_000.0)
    } else {
        format!("{}{:.0}", prefix, abs)
    }
}

pub fn format_currency_full(amount: f64) -> String {
    let abs = (amount.abs()).round() as u64;
    let s = abs.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*c);
    }
    if amount < 0.0 {
        format!("-₱{}", result)
    } else {
        format!("₱{}", result)
    }
}

pub fn pct(value: f64) -> String {
    format!("{:.1}%", value)
}

impl GameState {
    pub fn new() -> Self {
        let starting_cash = STARTING_CASH;

        let store = Store {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Bahay Depot Quezon City".into(),
            city: "Quezon City".into(),
            region: Region::MetroManila,
            store_type: StoreType::Standard,
            size_sqm: 3500,
            status: StoreStatus::Operating,
            quarterly_revenue: 0.0,
            quarterly_expenses: 0.0,
            customer_count: 0,
            employee_count: 70,
            satisfaction: 70.0,
            age_quarters: 0,
            construction_quarters_left: 0,
            opened_quarter: 1,
            opened_year: 2025,
        };

        GameState {
            company: Company {
                name: "Bahay Depot Inc.".into(),
                cash: starting_cash,
                total_revenue: 0.0,
                total_expenses: 0.0,
                total_profit: 0.0,
                company_value: starting_cash,
                brand_reputation: 50.0,
                customer_satisfaction: 65.0,
                employee_satisfaction: 60.0,
                market_share: 2.0,
                founded_quarter: 1,
                founded_year: 2025,
                loans: vec![],
                has_ever_had_loan: false,
            },
            stores: vec![store],
            executives: vec![],
            employees: EmployeePool {
                total_count: 70,
                monthly_payroll: 1_200_000.0,
                avg_morale: 60.0,
                avg_skill: 50.0,
                turnover_rate: 5.0,
                department_breakdown: HashMap::from([
                    ("Store Operations".into(), 55),
                    ("Warehouse".into(), 8),
                    ("Administration".into(), 4),
                    ("IT".into(), 2),
                    ("Marketing".into(), 1),
                ]),
            },
            policies: Policies {
                pricing: PricingPolicy::Competitive,
                hr: HrPolicy::Standard,
                expansion: ExpansionPolicy::Moderate,
                customer_service: CustomerServicePolicy::Good,
                marketing: MarketingPolicy::Moderate,
                inventory: InventoryPolicy::Standard,
            },
            economy: EconomyState {
                gdp_growth_rate: 5.8,
                inflation_rate: 3.5,
                interest_rate: 5.5,
                construction_index: 65.0,
                consumer_confidence: 60.0,
                peso_usd_rate: 55.5,
                minimum_wage_daily: 610.0,
                corporate_tax_rate: 25.0,
            },
            market: MarketState {
                total_market_size: 500_000_000_000.0,
                competitor_count: 5,
                competitor_strength: 80.0,
                seasonal_multiplier: 1.0,
                demand_trend: 1.0,
            },
            financial_history: vec![],
            event_log: vec![],
            current_quarter: 1,
            current_year: 2025,
            game_over: false,
            messages: vec![
                "Welcome to Bahay Depot! You are the new CEO. Your first store is open in Quezon City. Set your policies, hire your executive team, and grow the company!".into(),
            ],
            pending_events: vec![],
            delegation: DelegationSettings::all_false(),
            decisions_made: 0,
            decisions_delegated: 0,
            products: default_product_categories(),
            upgrades: vec![],
            board: BoardState::new(),
            competitors: default_competitors(),
            achievements: default_achievements(),
            loyalty: super::loyalty::LoyaltyProgram::new(),
            campaigns: vec![],
            ecommerce: super::ecommerce::EcommerceChannel::new(),
        }
    }

    pub fn operating_store_count(&self) -> u32 {
        self.stores
            .iter()
            .filter(|s| s.status == StoreStatus::Operating)
            .count() as u32
    }

    pub fn is_executive_hired(&self, position: ExecutivePosition) -> bool {
        self.executives.iter().any(|e| e.position == position)
    }

    pub fn advance_quarter_label(&self) -> String {
        let q = self.current_quarter;
        let y = self.current_year;
        format!("Q{} {}", q, y)
    }

    pub fn log_event(&mut self, event: GameEvent) {
        self.event_log.insert(0, event);
        if self.event_log.len() > MAX_EVENT_LOG_SIZE {
            self.event_log.pop();
        }
    }

    pub fn has_pending_events(&self) -> bool {
        !self.pending_events.is_empty()
    }

    pub fn pending_event_count(&self) -> usize {
        self.pending_events.len()
    }

    pub fn resolve_pending_event(
        &mut self,
        event_id: &str,
        choice_id: &str,
    ) -> Option<PendingEvent> {
        let idx = self.pending_events.iter().position(|e| e.id == event_id)?;
        let event = self.pending_events.remove(idx);
        if let Some(choice) = event.choices.iter().find(|c| c.id == choice_id) {
            apply_event_effects(self, &choice.effects);
            self.decisions_made += 1;
            self.log_event(GameEvent {
                id: event.id.clone(),
                title: event.title.clone(),
                description: format!("You chose: {}", choice.label),
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
            self.messages
                .push(format!("[DECISION] {}: {}", event.title, choice.label));
        }
        Some(event)
    }
}

pub fn apply_event_effects(state: &mut GameState, effects: &EventEffects) {
    let recent_revenue = state
        .financial_history
        .last()
        .map(|r| r.revenue)
        .unwrap_or(0.0);
    state.company.cash += effects.cash;
    state.company.cash += effects.revenue_modifier * recent_revenue;
    state.company.cash -= effects.expense_modifier;
    state.company.brand_reputation =
        (state.company.brand_reputation + effects.reputation).clamp(5.0, 100.0);
    state.company.employee_satisfaction =
        (state.company.employee_satisfaction + effects.morale).clamp(5.0, 100.0);
    state.company.customer_satisfaction =
        (state.company.customer_satisfaction + effects.satisfaction).clamp(5.0, 100.0);
}

pub fn category_to_event_type(cat: EventCategory) -> EventType {
    match cat {
        EventCategory::Crisis => EventType::NaturalDisaster,
        EventCategory::Financial => EventType::Economic,
        EventCategory::Marketing => EventType::Marketing,
        EventCategory::HR => EventType::Employee,
        EventCategory::SupplyChain => EventType::SupplyChain,
        EventCategory::Competition => EventType::Competition,
        EventCategory::Technology => EventType::Positive,
        EventCategory::Regulation => EventType::Regulation,
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
