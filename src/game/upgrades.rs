use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum UpgradeType {
    Renovation,
    Technology,
    GreenEnergy,
    CustomerExperience,
}

impl UpgradeType {
    pub fn label(&self) -> &str {
        match self {
            UpgradeType::Renovation => "Store Renovation",
            UpgradeType::Technology => "Technology Upgrade",
            UpgradeType::GreenEnergy => "Green Energy",
            UpgradeType::CustomerExperience => "Customer Experience",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            UpgradeType::Renovation => "renovation",
            UpgradeType::Technology => "technology",
            UpgradeType::GreenEnergy => "green_energy",
            UpgradeType::CustomerExperience => "customer_experience",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            UpgradeType::Renovation => "Paintbrush",
            UpgradeType::Technology => "Monitor",
            UpgradeType::GreenEnergy => "Leaf",
            UpgradeType::CustomerExperience => "Heart",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            UpgradeType::Renovation => "Modernize store interior. Increases revenue and customer satisfaction.",
            UpgradeType::Technology => "Upgrade POS, inventory systems, and digital tools. Reduces operating costs.",
            UpgradeType::GreenEnergy => "Solar panels, LED lighting, water recycling. Reduces utilities and boosts reputation.",
            UpgradeType::CustomerExperience => "Signage, wayfinding, demo areas, consultation services. Boosts satisfaction.",
        }
    }

    pub fn cost_per_level(&self) -> f64 {
        match self {
            UpgradeType::Renovation => 3_000_000.0,
            UpgradeType::Technology => 2_000_000.0,
            UpgradeType::GreenEnergy => 2_500_000.0,
            UpgradeType::CustomerExperience => 1_500_000.0,
        }
    }

    pub fn max_level(&self) -> u32 {
        3
    }

    pub fn revenue_bonus_per_level(&self) -> f64 {
        match self {
            UpgradeType::Renovation => 0.05,
            UpgradeType::Technology => 0.0,
            UpgradeType::GreenEnergy => 0.0,
            UpgradeType::CustomerExperience => 0.03,
        }
    }

    pub fn cost_reduction_per_level(&self) -> f64 {
        match self {
            UpgradeType::Renovation => 0.0,
            UpgradeType::Technology => 0.03,
            UpgradeType::GreenEnergy => 0.02,
            UpgradeType::CustomerExperience => 0.0,
        }
    }

    pub fn satisfaction_bonus_per_level(&self) -> f64 {
        match self {
            UpgradeType::Renovation => 3.0,
            UpgradeType::Technology => 0.0,
            UpgradeType::GreenEnergy => 2.0,
            UpgradeType::CustomerExperience => 5.0,
        }
    }

    pub fn all_types() -> Vec<UpgradeType> {
        vec![
            UpgradeType::Renovation,
            UpgradeType::Technology,
            UpgradeType::GreenEnergy,
            UpgradeType::CustomerExperience,
        ]
    }

    pub fn from_key(key: &str) -> Option<UpgradeType> {
        match key {
            "renovation" => Some(UpgradeType::Renovation),
            "technology" => Some(UpgradeType::Technology),
            "green_energy" => Some(UpgradeType::GreenEnergy),
            "customer_experience" => Some(UpgradeType::CustomerExperience),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreUpgrade {
    pub store_id: String,
    pub upgrade_type: UpgradeType,
    pub level: u32,
}

pub fn get_upgrade_level(
    upgrades: &[StoreUpgrade],
    store_id: &str,
    upgrade_type: UpgradeType,
) -> u32 {
    upgrades
        .iter()
        .find(|u| u.store_id == store_id && u.upgrade_type == upgrade_type)
        .map(|u| u.level)
        .unwrap_or(0)
}

pub fn get_store_revenue_modifier(upgrades: &[StoreUpgrade], store_id: &str) -> f64 {
    let mut modifier = 1.0;
    for ut in UpgradeType::all_types() {
        let level = get_upgrade_level(upgrades, store_id, ut);
        modifier += ut.revenue_bonus_per_level() * level as f64;
    }
    modifier
}

pub fn get_store_cost_modifier(upgrades: &[StoreUpgrade], store_id: &str) -> f64 {
    let mut modifier = 1.0;
    for ut in UpgradeType::all_types() {
        let level = get_upgrade_level(upgrades, store_id, ut);
        modifier -= ut.cost_reduction_per_level() * level as f64;
    }
    modifier.max(0.7)
}

pub fn get_store_satisfaction_modifier(upgrades: &[StoreUpgrade], store_id: &str) -> f64 {
    let mut modifier = 0.0;
    for ut in UpgradeType::all_types() {
        let level = get_upgrade_level(upgrades, store_id, ut);
        modifier += ut.satisfaction_bonus_per_level() * level as f64;
    }
    modifier
}

pub fn purchase_upgrade(
    upgrades: &mut Vec<StoreUpgrade>,
    store_id: &str,
    upgrade_type: UpgradeType,
) -> Result<f64, &'static str> {
    let current_level = get_upgrade_level(upgrades, store_id, upgrade_type);
    if current_level >= upgrade_type.max_level() {
        return Err("Already at max level");
    }
    let cost = upgrade_type.cost_per_level();
    if let Some(existing) = upgrades
        .iter_mut()
        .find(|u| u.store_id == store_id && u.upgrade_type == upgrade_type)
    {
        existing.level += 1;
    } else {
        upgrades.push(StoreUpgrade {
            store_id: store_id.to_string(),
            upgrade_type,
            level: 1,
        });
    }
    Ok(cost)
}
