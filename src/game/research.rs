use serde::{Deserialize, Serialize};

use super::state::GameState;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResearchTrack {
    OperationalEfficiency,
    InventoryOptimization,
    CustomerExperience,
    DigitalTransformation,
    SupplyChainInnovation,
    GreenBuilding,
    ProductInnovation,
}

impl ResearchTrack {
    pub fn label(&self) -> &str {
        match self {
            ResearchTrack::OperationalEfficiency => "Operational Efficiency",
            ResearchTrack::InventoryOptimization => "Inventory Optimization",
            ResearchTrack::CustomerExperience => "Customer Experience",
            ResearchTrack::DigitalTransformation => "Digital Transformation",
            ResearchTrack::SupplyChainInnovation => "Supply Chain Innovation",
            ResearchTrack::GreenBuilding => "Green Building",
            ResearchTrack::ProductInnovation => "Product Innovation",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            ResearchTrack::OperationalEfficiency => "operational_efficiency",
            ResearchTrack::InventoryOptimization => "inventory_optimization",
            ResearchTrack::CustomerExperience => "customer_experience",
            ResearchTrack::DigitalTransformation => "digital_transformation",
            ResearchTrack::SupplyChainInnovation => "supply_chain_innovation",
            ResearchTrack::GreenBuilding => "green_building",
            ResearchTrack::ProductInnovation => "product_innovation",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            ResearchTrack::OperationalEfficiency => "Zap",
            ResearchTrack::InventoryOptimization => "Archive",
            ResearchTrack::CustomerExperience => "Heart",
            ResearchTrack::DigitalTransformation => "Monitor",
            ResearchTrack::SupplyChainInnovation => "Truck",
            ResearchTrack::GreenBuilding => "Leaf",
            ResearchTrack::ProductInnovation => "Lightbulb",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            ResearchTrack::OperationalEfficiency => "\u{26a1}",
            ResearchTrack::InventoryOptimization => "\u{1f4e6}",
            ResearchTrack::CustomerExperience => "\u{2764}",
            ResearchTrack::DigitalTransformation => "\u{1f4bb}",
            ResearchTrack::SupplyChainInnovation => "\u{1f69a}",
            ResearchTrack::GreenBuilding => "\u{1f33f}",
            ResearchTrack::ProductInnovation => "\u{1f4a1}",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ResearchTrack::OperationalEfficiency => "Streamline store operations with automation, lean processes, and workflow optimization. Reduces overall store expenses.",
            ResearchTrack::InventoryOptimization => "Develop smart inventory management with demand forecasting and just-in-time replenishment. Reduces inventory carrying costs.",
            ResearchTrack::CustomerExperience => "Research innovative retail experiences including AR product visualization, smart store layouts, and personalized service. Boosts customer satisfaction.",
            ResearchTrack::DigitalTransformation => "Advance digital infrastructure including IoT sensors, AI-powered analytics, and automated customer support. Boosts e-commerce revenue.",
            ResearchTrack::SupplyChainInnovation => "Innovate logistics with route optimization, predictive maintenance, and blockchain traceability. Improves supply chain efficiency.",
            ResearchTrack::GreenBuilding => "Develop sustainable building practices, energy-efficient store designs, and eco-friendly materials. Boosts brand reputation and employee satisfaction.",
            ResearchTrack::ProductInnovation => "Research new product formulations, materials science, and exclusive product features. Improves product category margins.",
        }
    }

    pub fn cost_per_level(&self, level: u32) -> f64 {
        let base = match self {
            ResearchTrack::OperationalEfficiency => 5_000_000.0,
            ResearchTrack::InventoryOptimization => 4_000_000.0,
            ResearchTrack::CustomerExperience => 6_000_000.0,
            ResearchTrack::DigitalTransformation => 8_000_000.0,
            ResearchTrack::SupplyChainInnovation => 5_500_000.0,
            ResearchTrack::GreenBuilding => 4_500_000.0,
            ResearchTrack::ProductInnovation => 7_000_000.0,
        };
        base * (1.5_f64).powi(level as i32)
    }

    pub fn quarters_per_level(&self, _level: u32) -> i32 {
        3
    }

    pub fn max_level(&self) -> u32 {
        3
    }

    pub fn color_class(&self) -> &str {
        match self {
            ResearchTrack::OperationalEfficiency => "text-yellow-400",
            ResearchTrack::InventoryOptimization => "text-blue-400",
            ResearchTrack::CustomerExperience => "text-pink-400",
            ResearchTrack::DigitalTransformation => "text-purple-400",
            ResearchTrack::SupplyChainInnovation => "text-orange-400",
            ResearchTrack::GreenBuilding => "text-green-400",
            ResearchTrack::ProductInnovation => "text-cyan-400",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "operational_efficiency" => Some(ResearchTrack::OperationalEfficiency),
            "inventory_optimization" => Some(ResearchTrack::InventoryOptimization),
            "customer_experience" => Some(ResearchTrack::CustomerExperience),
            "digital_transformation" => Some(ResearchTrack::DigitalTransformation),
            "supply_chain_innovation" => Some(ResearchTrack::SupplyChainInnovation),
            "green_building" => Some(ResearchTrack::GreenBuilding),
            "product_innovation" => Some(ResearchTrack::ProductInnovation),
            _ => None,
        }
    }

    pub fn all_tracks() -> Vec<ResearchTrack> {
        vec![
            ResearchTrack::OperationalEfficiency,
            ResearchTrack::InventoryOptimization,
            ResearchTrack::CustomerExperience,
            ResearchTrack::DigitalTransformation,
            ResearchTrack::SupplyChainInnovation,
            ResearchTrack::GreenBuilding,
            ResearchTrack::ProductInnovation,
        ]
    }

    pub fn effect_description(&self, level: u32) -> String {
        match self {
            ResearchTrack::OperationalEfficiency => {
                format!("-{}% store expenses", level * 4)
            }
            ResearchTrack::InventoryOptimization => {
                format!("-{}% inventory costs", level * 5)
            }
            ResearchTrack::CustomerExperience => {
                format!("+{:.0} customer satisfaction", level as f64 * 2.5)
            }
            ResearchTrack::DigitalTransformation => {
                format!("+{}% e-commerce revenue", level * 8)
            }
            ResearchTrack::SupplyChainInnovation => {
                format!("-{}% supply chain costs", level * 4)
            }
            ResearchTrack::GreenBuilding => {
                format!(
                    "+{:.0} brand reputation, +{:.0} employee satisfaction",
                    level as f64 * 3.0,
                    level as f64 * 2.0
                )
            }
            ResearchTrack::ProductInnovation => {
                format!("+{}% product margins", level * 3)
            }
        }
    }

    pub fn min_stores(&self) -> u32 {
        match self {
            ResearchTrack::OperationalEfficiency => 2,
            ResearchTrack::InventoryOptimization => 2,
            ResearchTrack::CustomerExperience => 3,
            ResearchTrack::DigitalTransformation => 4,
            ResearchTrack::SupplyChainInnovation => 3,
            ResearchTrack::GreenBuilding => 2,
            ResearchTrack::ProductInnovation => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProject {
    pub track: ResearchTrack,
    pub current_level: u32,
    pub progress: f64,
    pub quarters_remaining: i32,
    pub is_researching: bool,
}

impl ResearchProject {
    pub fn new(track: ResearchTrack) -> Self {
        ResearchProject {
            track,
            current_level: 0,
            progress: 0.0,
            quarters_remaining: 0,
            is_researching: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchLab {
    pub projects: Vec<ResearchProject>,
    pub total_invested: f64,
    pub completed_count: u32,
}

impl ResearchLab {
    pub fn new() -> Self {
        ResearchLab {
            projects: ResearchTrack::all_tracks()
                .into_iter()
                .map(ResearchProject::new)
                .collect(),
            total_invested: 0.0,
            completed_count: 0,
        }
    }

    pub fn get_project(&self, track: ResearchTrack) -> Option<&ResearchProject> {
        self.projects.iter().find(|p| p.track == track)
    }

    pub fn get_project_mut(&mut self, track: ResearchTrack) -> Option<&mut ResearchProject> {
        self.projects.iter_mut().find(|p| p.track == track)
    }

    pub fn active_research(&self) -> Option<&ResearchProject> {
        self.projects.iter().find(|p| p.is_researching)
    }

    pub fn research_level(&self, track: ResearchTrack) -> u32 {
        self.get_project(track)
            .map(|p| p.current_level)
            .unwrap_or(0)
    }
}

impl Default for ResearchLab {
    fn default() -> Self {
        Self::new()
    }
}

pub fn start_research(state: &mut GameState, track: ResearchTrack) -> Result<f64, &'static str> {
    let min_stores = track.min_stores();
    if state.operating_store_count() < min_stores {
        return Err("Not enough operating stores for this research.");
    }

    // Immutable borrow to read project current_level
    let lab = &state.research_lab;
    let project = lab.get_project(track).ok_or("Invalid research track.")?;
    if project.is_researching {
        return Err("This track is already being researched.");
    }
    if project.current_level >= track.max_level() {
        return Err("This track is already fully researched.");
    }
    let cost = track.cost_per_level(project.current_level);
    let quarters = track.quarters_per_level(project.current_level);
    let current_level = project.current_level;
    drop(project); // end immutable borrow
    drop(lab);

    // Now mutable borrows
    let lab = &mut state.research_lab;
    if lab.active_research().is_some() {
        return Err("A research project is already in progress. Complete it first.");
    }
    let project_mut = lab.get_project_mut(track).unwrap(); // safe because we validated
    project_mut.is_researching = true;
    project_mut.progress = 0.0;
    project_mut.quarters_remaining = quarters;

    if state.company.cash < cost {
        return Err("Not enough cash.");
    }
    state.company.cash -= cost;
    lab.total_invested += cost;

    state.push_message(format!(
        "Started R&D on '{}' Level {} for {} (estimated {} quarters).",
        track.label(),
        current_level + 1,
        super::state::format_currency(cost),
        quarters
    ));

    Ok(cost)
}

pub fn cancel_research(state: &mut GameState) -> Result<(), &'static str> {
    let lab = &mut state.research_lab;

    let track = lab
        .active_research()
        .ok_or("No research in progress.")?
        .track;

    let project = lab.get_project_mut(track).unwrap();
    project.is_researching = false;
    project.progress = 0.0;
    project.quarters_remaining = 0;

    state.push_message(format!(
        "Cancelled R&D on '{}'. Progress lost.",
        track.label()
    ));

    Ok(())
}

pub fn process_research(state: &mut GameState) {
    let cto_skill = state
        .executives
        .iter()
        .find(|e| e.position == super::state::ExecutivePosition::CTO)
        .map(|e| e.skill)
        .unwrap_or(0.0);

    let cto_speedup = 1.0 + cto_skill * 0.01;

    let active_track = {
        let lab = &state.research_lab;
        lab.active_research().map(|p| p.track)
    };

    if let Some(track) = active_track {
        let lab = &mut state.research_lab;
        let project = lab.get_project_mut(track).unwrap();

        let total_quarters = track.quarters_per_level(project.current_level) as f64;
        let progress_per_quarter = cto_speedup / total_quarters;

        project.progress = (project.progress + progress_per_quarter).min(1.0);
        project.quarters_remaining = (project.quarters_remaining - 1).max(0);

        let completed = project.progress >= 1.0 || project.quarters_remaining <= 0;
        if completed {
            project.progress = 1.0;
            project.current_level += 1;
            project.is_researching = false;
            project.quarters_remaining = 0;
        }
        drop(project); // release borrow

        if completed {
            lab.completed_count += 1;
            let new_level = lab.get_project(track).unwrap().current_level;
            state.push_message(format!(
                "R&D COMPLETE: '{}' upgraded to Level {}! {}",
                track.label(),
                new_level,
                track.effect_description(new_level)
            ));
        }
    }
}

pub fn research_expense_modifier(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::OperationalEfficiency);
    1.0 - (level as f64 * 0.04)
}

pub fn research_inventory_modifier(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::InventoryOptimization);
    1.0 - (level as f64 * 0.05)
}

pub fn research_satisfaction_bonus(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::CustomerExperience);
    level as f64 * 2.5
}

pub fn research_ecommerce_bonus(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::DigitalTransformation);
    1.0 + (level as f64 * 0.08)
}

pub fn research_supply_chain_modifier(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::SupplyChainInnovation);
    1.0 - (level as f64 * 0.04)
}

pub fn research_reputation_bonus(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::GreenBuilding);
    level as f64 * 3.0
}

pub fn research_employee_satisfaction_bonus(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::GreenBuilding);
    level as f64 * 2.0
}

pub fn research_product_margin_bonus(state: &GameState) -> f64 {
    let lab = &state.research_lab;
    let level = lab.research_level(ResearchTrack::ProductInnovation);
    level as f64 * 0.03
}
