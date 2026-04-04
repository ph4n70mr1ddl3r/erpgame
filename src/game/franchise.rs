use rand::Rng;
use serde::{Deserialize, Serialize};

use super::state::{GameState, StoreStatus, StoreType};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FranchiseStatus {
    Active,
    Pending,
    Terminated,
}

impl FranchiseStatus {
    pub fn label(&self) -> &str {
        match self {
            FranchiseStatus::Active => "Active",
            FranchiseStatus::Pending => "Pending",
            FranchiseStatus::Terminated => "Terminated",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FranchiseStore {
    pub id: String,
    pub store_id: String,
    pub store_name: String,
    pub city: String,
    pub store_type: StoreType,
    pub franchisee_name: String,
    pub franchise_fee: f64,
    pub royalty_rate: f64,
    pub quarters_active: i32,
    pub total_royalties_paid: f64,
    pub status: FranchiseStatus,
}

impl FranchiseStore {
    pub fn quarterly_royalty(&self, store_revenue: f64) -> f64 {
        store_revenue * self.royalty_rate
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FranchiseState {
    pub franchises: Vec<FranchiseStore>,
    pub total_franchise_fees_received: f64,
    pub total_royalties_received: f64,
    pub franchise_count: u32,
}

impl FranchiseState {
    pub fn new() -> Self {
        FranchiseState {
            franchises: vec![],
            total_franchise_fees_received: 0.0,
            total_royalties_received: 0.0,
            franchise_count: 0,
        }
    }

    pub fn active_count(&self) -> usize {
        self.franchises
            .iter()
            .filter(|f| f.status == FranchiseStatus::Active)
            .count()
    }
}

impl Default for FranchiseState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn franchise_fee(store_type: StoreType) -> f64 {
    match store_type {
        StoreType::Express => 3_000_000.0,
        StoreType::Standard => 8_000_000.0,
        StoreType::Mega => 20_000_000.0,
        StoreType::Depot => 35_000_000.0,
    }
}

pub fn royalty_rate(store_type: StoreType) -> f64 {
    match store_type {
        StoreType::Express => 0.04,
        StoreType::Standard => 0.05,
        StoreType::Mega => 0.055,
        StoreType::Depot => 0.06,
    }
}

pub fn buyback_cost(store_type: StoreType) -> f64 {
    franchise_fee(store_type) * 0.7
}

fn generate_franchisee_name(rng: &mut rand::rngs::ThreadRng) -> String {
    let first_names = [
        "Roberto",
        "Maria",
        "Juan",
        "Ana",
        "Miguel",
        "Carmen",
        "Jose",
        "Rosa",
        "Antonio",
        "Elena",
        "Francisco",
        "Teresa",
        "Manuel",
        "Patricia",
        "Fernando",
        "Lourdes",
        "Ricardo",
        "Isabel",
        "Eduardo",
        "Socorro",
        "Luis",
        "Victoria",
        "Carlos",
        "Cristina",
        "Diego",
        "Nora",
        "Andres",
        "Milagros",
        "Rafael",
    ];
    let last_names = [
        "Reyes",
        "Cruz",
        "Santos",
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
        "Santiago",
        "Rivera",
        "Torres",
        "Ramos",
        "Flores",
        "Hernandez",
        "Sanchez",
        "David",
    ];
    let first = first_names[rng.gen_range(0..first_names.len())];
    let last = last_names[rng.gen_range(0..last_names.len())];
    format!("{} {}", first, last)
}

pub fn franchise_store(state: &mut GameState, store_id: &str) -> Result<f64, &'static str> {
    let store_idx = state
        .stores
        .iter()
        .position(|s| s.id == store_id)
        .ok_or("Store not found.")?;

    let store = &state.stores[store_idx];

    if store.status != StoreStatus::Operating {
        return Err("Only operating stores can be franchised.");
    }

    let operating_count = state
        .stores
        .iter()
        .filter(|s| s.status == StoreStatus::Operating)
        .count();
    let franchise_count = state
        .franchise
        .franchises
        .iter()
        .filter(|f| f.status == FranchiseStatus::Active)
        .count();
    let company_owned = operating_count - franchise_count;

    if company_owned <= 1 {
        return Err("You must keep at least one company-owned store.");
    }

    let fee = franchise_fee(store.store_type);
    let royalty = royalty_rate(store.store_type);

    let mut rng = rand::thread_rng();
    let franchisee_name = generate_franchisee_name(&mut rng);

    let franchise = FranchiseStore {
        id: uuid::Uuid::new_v4().to_string(),
        store_id: store.id.clone(),
        store_name: store.name.clone(),
        city: store.city.clone(),
        store_type: store.store_type,
        franchisee_name: franchisee_name.clone(),
        franchise_fee: fee,
        royalty_rate: royalty,
        quarters_active: 0,
        total_royalties_paid: 0.0,
        status: FranchiseStatus::Active,
    };

    state.franchise.franchises.push(franchise);
    state.franchise.total_franchise_fees_received += fee;
    state.franchise.franchise_count += 1;
    state.company.cash += fee;

    state.push_message(format!(
        "Franchised '{}' in {} to {} for {}. Royalty rate: {:.1}%",
        store.name,
        store.city,
        franchisee_name,
        super::state::format_currency(fee),
        royalty * 100.0
    ));

    Ok(fee)
}

pub fn buyback_franchise(state: &mut GameState, franchise_id: &str) -> Result<f64, &'static str> {
    let franchise_idx = state
        .franchise
        .franchises
        .iter()
        .position(|f| f.id == franchise_id)
        .ok_or("Franchise not found.")?;

    let franchise = &state.franchise.franchises[franchise_idx];

    if franchise.status != FranchiseStatus::Active {
        return Err("This franchise is not active.");
    }

    let cost = buyback_cost(franchise.store_type);

    if state.company.cash < cost {
        return Err("Not enough cash to buy back franchise.");
    }

    let store_name = franchise.store_name.clone();
    let franchisee_name = franchise.franchisee_name.clone();

    state.franchise.franchises[franchise_idx].status = FranchiseStatus::Terminated;
    state.company.cash -= cost;

    state.push_message(format!(
        "Bought back '{}' from {} for {}. Store is now company-owned.",
        store_name,
        franchisee_name,
        super::state::format_currency(cost)
    ));

    Ok(cost)
}

pub fn process_franchise_revenue(state: &mut GameState) -> f64 {
    let mut total_royalties = 0.0;

    for franchise in state.franchise.franchises.iter_mut() {
        if franchise.status != FranchiseStatus::Active {
            continue;
        }

        if let Some(store) = state.stores.iter().find(|s| s.id == franchise.store_id) {
            if store.status == StoreStatus::Operating {
                let royalty = franchise.quarterly_royalty(store.quarterly_revenue);
                total_royalties += royalty;
                franchise.total_royalties_paid += royalty;
                franchise.quarters_active += 1;
            }
        }
    }

    state.franchise.total_royalties_received += total_royalties;
    state.company.cash += total_royalties;

    total_royalties
}
