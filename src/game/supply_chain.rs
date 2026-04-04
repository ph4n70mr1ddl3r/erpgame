use rand::Rng;
use serde::{Deserialize, Serialize};

use super::state::{format_currency, ExecutivePosition, GameState, Region};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupplierCategory {
    BuildingMaterials,
    Plumbing,
    Electrical,
    Tools,
    Paint,
    Flooring,
    Fixtures,
    General,
}

impl SupplierCategory {
    pub fn label(&self) -> &str {
        match self {
            SupplierCategory::BuildingMaterials => "Building Materials",
            SupplierCategory::Plumbing => "Plumbing",
            SupplierCategory::Electrical => "Electrical",
            SupplierCategory::Tools => "Tools & Hardware",
            SupplierCategory::Paint => "Paint & Finishes",
            SupplierCategory::Flooring => "Flooring & Tiles",
            SupplierCategory::Fixtures => "Fixtures & Fittings",
            SupplierCategory::General => "General Merchandise",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            SupplierCategory::BuildingMaterials => "building_materials",
            SupplierCategory::Plumbing => "plumbing",
            SupplierCategory::Electrical => "electrical",
            SupplierCategory::Tools => "tools",
            SupplierCategory::Paint => "paint",
            SupplierCategory::Flooring => "flooring",
            SupplierCategory::Fixtures => "fixtures",
            SupplierCategory::General => "general",
        }
    }

    pub fn all_categories() -> Vec<SupplierCategory> {
        vec![
            SupplierCategory::BuildingMaterials,
            SupplierCategory::Plumbing,
            SupplierCategory::Electrical,
            SupplierCategory::Tools,
            SupplierCategory::Paint,
            SupplierCategory::Flooring,
            SupplierCategory::Fixtures,
            SupplierCategory::General,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "building_materials" => Some(SupplierCategory::BuildingMaterials),
            "plumbing" => Some(SupplierCategory::Plumbing),
            "electrical" => Some(SupplierCategory::Electrical),
            "tools" => Some(SupplierCategory::Tools),
            "paint" => Some(SupplierCategory::Paint),
            "flooring" => Some(SupplierCategory::Flooring),
            "fixtures" => Some(SupplierCategory::Fixtures),
            "general" => Some(SupplierCategory::General),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum LogisticsLevel {
    Basic,
    Regional,
    National,
    Advanced,
}

impl LogisticsLevel {
    pub fn label(&self) -> &str {
        match self {
            LogisticsLevel::Basic => "Basic Logistics",
            LogisticsLevel::Regional => "Regional Distribution",
            LogisticsLevel::National => "National Network",
            LogisticsLevel::Advanced => "Advanced Supply Chain",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            LogisticsLevel::Basic => "basic",
            LogisticsLevel::Regional => "regional",
            LogisticsLevel::National => "national",
            LogisticsLevel::Advanced => "advanced",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            LogisticsLevel::Basic => "Ad-hoc shipping with minimal coordination. High costs, slow delivery, frequent delays.",
            LogisticsLevel::Regional => "Regional warehouses with scheduled deliveries. Moderate costs and reliable for nearby stores.",
            LogisticsLevel::National => "Centralized distribution hubs with cross-docking. Lower costs and fast nationwide delivery.",
            LogisticsLevel::Advanced => "AI-optimized logistics with real-time tracking, predictive restocking, and automated warehouses.",
        }
    }

    pub fn quarterly_cost(&self) -> f64 {
        match self {
            LogisticsLevel::Basic => 0.0,
            LogisticsLevel::Regional => 2_000_000.0,
            LogisticsLevel::National => 5_000_000.0,
            LogisticsLevel::Advanced => 12_000_000.0,
        }
    }

    pub fn cost_reduction_pct(&self) -> f64 {
        match self {
            LogisticsLevel::Basic => 0.0,
            LogisticsLevel::Regional => 0.05,
            LogisticsLevel::National => 0.12,
            LogisticsLevel::Advanced => 0.20,
        }
    }

    pub fn delivery_reliability(&self) -> f64 {
        match self {
            LogisticsLevel::Basic => 0.60,
            LogisticsLevel::Regional => 0.75,
            LogisticsLevel::National => 0.88,
            LogisticsLevel::Advanced => 0.95,
        }
    }

    pub fn stockout_reduction(&self) -> f64 {
        match self {
            LogisticsLevel::Basic => 0.0,
            LogisticsLevel::Regional => 0.15,
            LogisticsLevel::National => 0.30,
            LogisticsLevel::Advanced => 0.45,
        }
    }

    pub fn min_stores(&self) -> u32 {
        match self {
            LogisticsLevel::Basic => 0,
            LogisticsLevel::Regional => 2,
            LogisticsLevel::National => 5,
            LogisticsLevel::Advanced => 10,
        }
    }

    pub fn all_levels() -> Vec<LogisticsLevel> {
        vec![
            LogisticsLevel::Basic,
            LogisticsLevel::Regional,
            LogisticsLevel::National,
            LogisticsLevel::Advanced,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "basic" => Some(LogisticsLevel::Basic),
            "regional" => Some(LogisticsLevel::Regional),
            "national" => Some(LogisticsLevel::National),
            "advanced" => Some(LogisticsLevel::Advanced),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum WarehouseTier {
    None,
    Small,
    Medium,
    Large,
    Mega,
}

impl WarehouseTier {
    pub fn label(&self) -> &str {
        match self {
            WarehouseTier::None => "No Warehouse",
            WarehouseTier::Small => "Small Warehouse",
            WarehouseTier::Medium => "Medium Warehouse",
            WarehouseTier::Large => "Large Warehouse",
            WarehouseTier::Mega => "Mega Distribution Center",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            WarehouseTier::None => "none",
            WarehouseTier::Small => "small",
            WarehouseTier::Medium => "medium",
            WarehouseTier::Large => "large",
            WarehouseTier::Mega => "mega",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            WarehouseTier::None => "No dedicated warehouse. Rely on just-in-time deliveries and store backrooms.",
            WarehouseTier::Small => "5,000 sqm warehouse in Metro Manila. Reduces stockouts and enables bulk purchasing.",
            WarehouseTier::Medium => "15,000 sqm warehouse with cold storage. Supports Luzon and Visayas stores.",
            WarehouseTier::Large => "35,000 sqm facility with automated sorting. Nationwide coverage with safety stock.",
            WarehouseTier::Mega => "60,000 sqm mega distribution center. AI-managed inventory, cross-docking, and multi-region support.",
        }
    }

    pub fn setup_cost(&self) -> f64 {
        match self {
            WarehouseTier::None => 0.0,
            WarehouseTier::Small => 15_000_000.0,
            WarehouseTier::Medium => 45_000_000.0,
            WarehouseTier::Large => 100_000_000.0,
            WarehouseTier::Mega => 200_000_000.0,
        }
    }

    pub fn quarterly_cost(&self) -> f64 {
        match self {
            WarehouseTier::None => 0.0,
            WarehouseTier::Small => 800_000.0,
            WarehouseTier::Medium => 2_000_000.0,
            WarehouseTier::Large => 4_500_000.0,
            WarehouseTier::Mega => 9_000_000.0,
        }
    }

    pub fn bulk_discount_pct(&self) -> f64 {
        match self {
            WarehouseTier::None => 0.0,
            WarehouseTier::Small => 0.03,
            WarehouseTier::Medium => 0.06,
            WarehouseTier::Large => 0.10,
            WarehouseTier::Mega => 0.15,
        }
    }

    pub fn stockout_reduction(&self) -> f64 {
        match self {
            WarehouseTier::None => 0.0,
            WarehouseTier::Small => 0.10,
            WarehouseTier::Medium => 0.20,
            WarehouseTier::Large => 0.30,
            WarehouseTier::Mega => 0.40,
        }
    }

    pub fn min_stores(&self) -> u32 {
        match self {
            WarehouseTier::None => 0,
            WarehouseTier::Small => 1,
            WarehouseTier::Medium => 3,
            WarehouseTier::Large => 6,
            WarehouseTier::Mega => 10,
        }
    }

    pub fn all_tiers() -> Vec<WarehouseTier> {
        vec![
            WarehouseTier::None,
            WarehouseTier::Small,
            WarehouseTier::Medium,
            WarehouseTier::Large,
            WarehouseTier::Mega,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "none" => Some(WarehouseTier::None),
            "small" => Some(WarehouseTier::Small),
            "medium" => Some(WarehouseTier::Medium),
            "large" => Some(WarehouseTier::Large),
            "mega" => Some(WarehouseTier::Mega),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeliveryServiceLevel {
    None,
    Basic,
    SameDay,
    Express,
}

impl DeliveryServiceLevel {
    pub fn label(&self) -> &str {
        match self {
            DeliveryServiceLevel::None => "No Delivery Service",
            DeliveryServiceLevel::Basic => "Basic Delivery",
            DeliveryServiceLevel::SameDay => "Same-Day Delivery",
            DeliveryServiceLevel::Express => "Express Delivery (2‑Hour)",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            DeliveryServiceLevel::None => "none",
            DeliveryServiceLevel::Basic => "basic",
            DeliveryServiceLevel::SameDay => "same_day",
            DeliveryServiceLevel::Express => "express",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            DeliveryServiceLevel::None => "No dedicated delivery service. Customers must pick up in‑store or use third‑party carriers.",
            DeliveryServiceLevel::Basic => "Standard delivery within 3‑5 business days. Handles small parcels and light building materials.",
            DeliveryServiceLevel::SameDay => "Same‑day delivery for orders placed before noon. Requires local dispatch teams and optimized routing.",
            DeliveryServiceLevel::Express => "2‑hour express delivery for urgent orders. Premium service with real‑time tracking and dedicated fleet.",
        }
    }

    pub fn setup_cost(&self) -> f64 {
        match self {
            DeliveryServiceLevel::None => 0.0,
            DeliveryServiceLevel::Basic => 5_000_000.0,
            DeliveryServiceLevel::SameDay => 20_000_000.0,
            DeliveryServiceLevel::Express => 50_000_000.0,
        }
    }

    pub fn quarterly_cost(&self) -> f64 {
        match self {
            DeliveryServiceLevel::None => 0.0,
            DeliveryServiceLevel::Basic => 1_000_000.0,
            DeliveryServiceLevel::SameDay => 3_000_000.0,
            DeliveryServiceLevel::Express => 8_000_000.0,
        }
    }

    pub fn revenue_bonus(&self) -> f64 {
        match self {
            DeliveryServiceLevel::None => 0.0,
            DeliveryServiceLevel::Basic => 0.03,
            DeliveryServiceLevel::SameDay => 0.08,
            DeliveryServiceLevel::Express => 0.15,
        }
    }

    pub fn satisfaction_bonus(&self) -> f64 {
        match self {
            DeliveryServiceLevel::None => 0.0,
            DeliveryServiceLevel::Basic => 1.0,
            DeliveryServiceLevel::SameDay => 3.0,
            DeliveryServiceLevel::Express => 6.0,
        }
    }

    pub fn min_stores(&self) -> u32 {
        match self {
            DeliveryServiceLevel::None => 0,
            DeliveryServiceLevel::Basic => 1,
            DeliveryServiceLevel::SameDay => 3,
            DeliveryServiceLevel::Express => 6,
        }
    }

    pub fn all_levels() -> Vec<DeliveryServiceLevel> {
        vec![
            DeliveryServiceLevel::None,
            DeliveryServiceLevel::Basic,
            DeliveryServiceLevel::SameDay,
            DeliveryServiceLevel::Express,
        ]
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "none" => Some(DeliveryServiceLevel::None),
            "basic" => Some(DeliveryServiceLevel::Basic),
            "same_day" => Some(DeliveryServiceLevel::SameDay),
            "express" => Some(DeliveryServiceLevel::Express),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub category: SupplierCategory,
    pub region: Region,
    pub reliability: f64,
    pub cost_modifier: f64,
    pub lead_time_quarters: i32,
    pub contract_length_quarters: i32,
    pub quarters_remaining: i32,
    pub relationship_score: f64,
    pub is_active: bool,
}

impl Supplier {
    pub fn effective_cost_modifier(&self) -> f64 {
        let relationship_discount = (self.relationship_score / 100.0) * 0.05;
        self.cost_modifier - relationship_discount
    }

    pub fn delivery_chance(&self) -> f64 {
        self.reliability / 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainState {
    pub suppliers: Vec<Supplier>,
    pub logistics: LogisticsLevel,
    pub warehouse: WarehouseTier,
    pub delivery_service: DeliveryServiceLevel,
    pub stockout_rate: f64,
    pub avg_delivery_time: f64,
    pub quarterly_logistics_cost: f64,
    pub total_supply_savings: f64,
    pub last_stockout_penalty: f64,
    pub quarters_since_disruption: i32,
}

impl SupplyChainState {
    pub fn new() -> Self {
        SupplyChainState {
            suppliers: vec![],
            logistics: LogisticsLevel::Basic,
            warehouse: WarehouseTier::None,
            delivery_service: DeliveryServiceLevel::None,
            stockout_rate: 15.0,
            avg_delivery_time: 2.5,
            quarterly_logistics_cost: 0.0,
            total_supply_savings: 0.0,
            last_stockout_penalty: 0.0,
            quarters_since_disruption: 0,
        }
    }
}

impl Default for SupplyChainState {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_supplier_name(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefixes = [
        "Mega", "Pacific", "Atlas", "Apex", "Prime", "Grand", "Royal", "Summit", "Vanguard",
        "Pinnacle", "Heritage", "Golden", "Star", "Alliance", "Union", "Fortune",
    ];
    let suffixes = [
        "Supply Co.",
        "Trading",
        "Industrial",
        "Distribution",
        "Enterprises",
        "Materials Inc.",
        "Resources Ltd.",
        "Partners",
        "Logistics",
        "Holding Co.",
    ];
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];
    format!("{} {}", prefix, suffix)
}

pub fn available_supplier_categories(state: &GameState) -> Vec<SupplierCategory> {
    let contracted: std::collections::HashSet<String> = state
        .supply_chain
        .suppliers
        .iter()
        .filter(|s| s.is_active)
        .map(|s| s.category.key().to_string())
        .collect();
    SupplierCategory::all_categories()
        .into_iter()
        .filter(|c| !contracted.contains(c.key()))
        .collect()
}

pub fn negotiate_supplier(
    state: &mut GameState,
    category: SupplierCategory,
) -> Result<f64, &'static str> {
    let available = available_supplier_categories(state);
    if !available.iter().any(|c| *c == category) {
        return Err("Category already has an active supplier contract.");
    }

    let active_contracts: usize = state
        .supply_chain
        .suppliers
        .iter()
        .filter(|s| s.is_active)
        .count();
    if active_contracts >= 8 {
        return Err("Maximum of 8 active supplier contracts reached.");
    }

    let negotiation_cost = 500_000.0;
    if state.company.cash < negotiation_cost {
        return Err("Cannot afford negotiation costs (P500K).");
    }

    let mut rng = rand::thread_rng();

    let csco_skill = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CSCO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let base_reliability = rng.gen_range(50.0..85.0);
    let reliability = (base_reliability + csco_skill * 0.15).min(98.0);

    let base_cost = rng.gen_range(0.85..1.15);
    let cost_modifier = (base_cost - csco_skill * 0.003).max(0.70);

    let base_lead = rng.gen_range(1..4);
    let lead_time = (base_lead as f64 - csco_skill * 0.02).round().max(1.0) as i32;

    let contract_length = rng.gen_range(4..12);
    let relationship_score = rng.gen_range(40.0..70.0);

    let regions = [
        Region::Luzon,
        Region::Visayas,
        Region::Mindanao,
        Region::MetroManila,
    ];
    let region = regions[rng.gen_range(0..regions.len())];

    let supplier = Supplier {
        id: uuid::Uuid::new_v4().to_string(),
        name: generate_supplier_name(&mut rng),
        category,
        region,
        reliability,
        cost_modifier,
        lead_time_quarters: lead_time,
        contract_length_quarters: contract_length,
        quarters_remaining: contract_length,
        relationship_score,
        is_active: true,
    };

    state.company.cash -= negotiation_cost;
    state.supply_chain.suppliers.push(supplier);

    state.push_message(format!(
        "Signed supplier contract with {} for {} (Reliability: {:.0}%, Cost: {:.0}%, Lead: {}Q, Term: {}Q). Negotiation cost: {}",
        state.supply_chain.suppliers.last().unwrap().name,
        category.label(),
        reliability,
        cost_modifier * 100.0,
        lead_time,
        contract_length,
        format_currency(negotiation_cost),
    ));

    Ok(negotiation_cost)
}

pub fn terminate_supplier(state: &mut GameState, supplier_id: &str) -> Result<(), &'static str> {
    let supplier_name = {
        let supplier = state
            .supply_chain
            .suppliers
            .iter_mut()
            .find(|s| s.id == supplier_id);
        match supplier {
            Some(s) if s.is_active => {
                s.is_active = false;
                s.name.clone()
            }
            Some(_) => return Err("Supplier contract is already inactive."),
            None => return Err("Supplier not found."),
        }
    };
    let penalty = 1_000_000.0;
    state.company.cash -= penalty;
    state.push_message(format!(
        "Terminated contract with {}. Early termination penalty: {}.",
        supplier_name,
        format_currency(penalty),
    ));
    Ok(())
}

pub fn upgrade_logistics(
    state: &mut GameState,
    new_level: LogisticsLevel,
) -> Result<f64, &'static str> {
    let current_ord = state.supply_chain.logistics as u8;
    let new_ord = new_level as u8;

    if new_level == state.supply_chain.logistics {
        return Err("Already at this logistics level.");
    }

    if new_level == LogisticsLevel::Basic {
        state.supply_chain.logistics = LogisticsLevel::Basic;
        state.push_message("Downgraded to basic logistics.".into());
        return Ok(0.0);
    }

    if new_ord != current_ord + 1 {
        return Err("You can only upgrade one logistics level at a time.");
    }

    let operating = state.operating_store_count();
    if operating < new_level.min_stores() {
        return Err("Not enough operating stores for this logistics level.");
    }

    let setup_cost = match new_level {
        LogisticsLevel::Regional => 8_000_000.0,
        LogisticsLevel::National => 20_000_000.0,
        LogisticsLevel::Advanced => 50_000_000.0,
        LogisticsLevel::Basic => 0.0,
    };

    if state.company.cash < setup_cost {
        return Err("Not enough cash for logistics upgrade.");
    }

    state.company.cash -= setup_cost;
    state.supply_chain.logistics = new_level;
    state.push_message(format!(
        "Upgraded logistics to {}! Setup cost: {}. Quarterly operating cost: {}.",
        new_level.label(),
        format_currency(setup_cost),
        format_currency(new_level.quarterly_cost()),
    ));

    Ok(setup_cost)
}

pub fn upgrade_warehouse(
    state: &mut GameState,
    new_tier: WarehouseTier,
) -> Result<f64, &'static str> {
    if new_tier == state.supply_chain.warehouse {
        return Err("Already at this warehouse tier.");
    }

    if new_tier == WarehouseTier::None {
        state.supply_chain.warehouse = WarehouseTier::None;
        state.push_message("Warehouse decommissioned.".into());
        return Ok(0.0);
    }

    let current_ord = state.supply_chain.warehouse as u8;
    let new_ord = new_tier as u8;
    if new_ord != current_ord + 1 {
        return Err("You can only upgrade one warehouse tier at a time.");
    }

    let operating = state.operating_store_count();
    if operating < new_tier.min_stores() {
        return Err("Not enough operating stores for this warehouse tier.");
    }

    let setup_cost = new_tier.setup_cost();
    if state.company.cash < setup_cost {
        return Err("Not enough cash for warehouse upgrade.");
    }

    state.company.cash -= setup_cost;
    state.supply_chain.warehouse = new_tier;
    state.push_message(format!(
        "Built {}! Setup cost: {}. Quarterly operating cost: {}.",
        new_tier.label(),
        format_currency(setup_cost),
        format_currency(new_tier.quarterly_cost()),
    ));

    Ok(setup_cost)
}

pub fn upgrade_delivery_service(
    state: &mut GameState,
    new_level: DeliveryServiceLevel,
) -> Result<f64, &'static str> {
    let current_ord = state.supply_chain.delivery_service as u8;
    let new_ord = new_level as u8;

    if new_level == state.supply_chain.delivery_service {
        return Err("Already at this delivery service level.");
    }

    if new_level == DeliveryServiceLevel::None {
        state.supply_chain.delivery_service = DeliveryServiceLevel::None;
        state.push_message("Delivery service deactivated.".into());
        return Ok(0.0);
    }

    if new_ord != current_ord + 1 {
        return Err("You can only upgrade one delivery service level at a time.");
    }

    let operating = state.operating_store_count();
    if operating < new_level.min_stores() {
        return Err("Not enough operating stores for this delivery service level.");
    }

    let setup_cost = new_level.setup_cost();
    if state.company.cash < setup_cost {
        return Err("Not enough cash for delivery service upgrade.");
    }

    state.company.cash -= setup_cost;
    state.supply_chain.delivery_service = new_level;
    state.push_message(format!(
        "Upgraded delivery service to {}! Setup cost: {}. Quarterly operating cost: {}.",
        new_level.label(),
        super::state::format_currency(setup_cost),
        super::state::format_currency(new_level.quarterly_cost()),
    ));

    Ok(setup_cost)
}

pub fn process_supply_chain(state: &mut GameState) -> f64 {
    let mut total_cost = 0.0;
    total_cost += state.supply_chain.logistics.quarterly_cost();
    total_cost += state.supply_chain.warehouse.quarterly_cost();
    total_cost += state.supply_chain.delivery_service.quarterly_cost();

    let csco_skill = state
        .executives
        .iter()
        .find(|e| e.position == ExecutivePosition::CSCO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let mut rng = rand::thread_rng();

    let mut messages_to_push: Vec<String> = Vec::new();

    for supplier in &mut state.supply_chain.suppliers {
        if !supplier.is_active {
            continue;
        }

        supplier.quarters_remaining -= 1;
        if supplier.quarters_remaining <= 0 {
            supplier.is_active = false;
            messages_to_push.push(format!(
                "Supplier contract with {} ({}) has expired.",
                supplier.name,
                supplier.category.label(),
            ));
            continue;
        }

        let delivered = rng.gen_bool(supplier.delivery_chance());
        if delivered {
            let reliability_improvement = if csco_skill > 50.0 { 0.5 } else { 0.0 };
            supplier.relationship_score =
                (supplier.relationship_score + reliability_improvement + rng.gen_range(-1.0..2.0))
                    .clamp(20.0, 100.0);
        } else {
            supplier.relationship_score =
                (supplier.relationship_score - rng.gen_range(2.0..5.0)).clamp(10.0, 100.0);
            if rng.gen_bool(0.3) {
                messages_to_push.push(format!(
                    "Supply disruption: {} failed to deliver {} on time. Relationship: {:.0}%.",
                    supplier.name,
                    supplier.category.label(),
                    supplier.relationship_score,
                ));
            }
        }
    }

    let active_supplier_count = state
        .supply_chain
        .suppliers
        .iter()
        .filter(|s| s.is_active)
        .count() as f64;
    let supplier_coverage = active_supplier_count / 8.0;

    let logistics_reliability = state.supply_chain.logistics.delivery_reliability();
    let warehouse_stockout_red = state.supply_chain.warehouse.stockout_reduction();
    let logistics_stockout_red = state.supply_chain.logistics.stockout_reduction();
    let csco_stockout_red = csco_skill * 0.002;

    state.supply_chain.stockout_rate = (15.0
        - supplier_coverage * 8.0
        - logistics_stockout_red * 20.0
        - warehouse_stockout_red * 15.0
        - csco_stockout_red * 10.0)
        .clamp(2.0, 25.0);

    let revenue_loss_pct = state.supply_chain.stockout_rate / 100.0 * 0.5;
    let recent_revenue = state
        .financial_history
        .last()
        .map(|r| r.revenue)
        .unwrap_or(0.0);

    state.supply_chain.last_stockout_penalty = recent_revenue * revenue_loss_pct;
    let mut total_savings = 0.0;

    for supplier in state.supply_chain.suppliers.iter().filter(|s| s.is_active) {
        let base_category_cost = recent_revenue * 0.40 / active_supplier_count.max(1.0);
        let savings = base_category_cost * supplier.effective_cost_modifier();
        let logistics_savings = savings * state.supply_chain.logistics.cost_reduction_pct();
        let warehouse_savings = savings * state.supply_chain.warehouse.bulk_discount_pct();
        total_savings += logistics_savings + warehouse_savings;
    }

    state.supply_chain.total_supply_savings = total_savings;
    state.supply_chain.avg_delivery_time = (2.5
        - state.supply_chain.logistics.stockout_reduction() * 2.0
        - state.supply_chain.warehouse.stockout_reduction() * 1.0
        - csco_skill * 0.008)
        .clamp(0.5, 3.0);

    state.supply_chain.quarterly_logistics_cost = total_cost;

    let disruption_chance =
        1.0 - (logistics_reliability * 0.5 + supplier_coverage * 0.3 + csco_skill * 0.002);
    let disruption_penalty;
    if rng.gen_bool(disruption_chance.max(0.02).min(0.15)) {
        state.supply_chain.quarters_since_disruption = 0;
        let disruption_types = [
            "A typhoon disrupted shipments to Visayas and Mindanao stores.",
            "Port congestion in Manila caused delays across all categories.",
            "A key supplier raised prices unexpectedly due to raw material shortages.",
            "Customs delays held up imported materials at the port.",
            "Flooding damaged inventory in the warehouse.",
        ];
        let disruption = disruption_types[rng.gen_range(0..disruption_types.len())];
        disruption_penalty = recent_revenue * rng.gen_range(0.02..0.06);
        messages_to_push.push(format!(
            "[SUPPLY CHAIN] {} Impact: {} revenue lost.",
            disruption,
            format_currency(disruption_penalty),
        ));
        state.company.cash -= disruption_penalty;
        state.company.brand_reputation = (state.company.brand_reputation - 1.5).clamp(10.0, 100.0);
        state.supply_chain.last_stockout_penalty += disruption_penalty;
    } else {
        state.supply_chain.quarters_since_disruption += 1;
    }

    for msg in messages_to_push {
        state.push_message(msg);
    }

    total_cost
}

pub fn supply_chain_cost_modifier(state: &GameState) -> f64 {
    let sc = &state.supply_chain;
    let supplier_savings = sc
        .suppliers
        .iter()
        .filter(|s| s.is_active)
        .map(|s| s.effective_cost_modifier())
        .sum::<f64>()
        / 8.0;

    let logistics_savings = sc.logistics.cost_reduction_pct();
    let warehouse_savings = sc.warehouse.bulk_discount_pct();

    1.0 - (supplier_savings * 0.05 + logistics_savings + warehouse_savings)
}

pub fn _supply_chain_stockout_revenue_penalty(state: &GameState) -> f64 {
    state.supply_chain.last_stockout_penalty
}
