use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::state::{format_currency_full, generate_executive_name};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum EmployeeRole {
    Cashier,
    SalesAssociate,
    StockClerk,
    WarehouseWorker,
    DeliveryDriver,
    DepartmentManager,
    StoreManager,
    Security,
    Janitorial,
    CustomerService,
    Accountant,
    ITSupport,
    HRStaff,
    MarketingStaff,
}

impl EmployeeRole {
    pub fn label(&self) -> &str {
        match self {
            EmployeeRole::Cashier => "Cashier",
            EmployeeRole::SalesAssociate => "Sales Associate",
            EmployeeRole::StockClerk => "Stock Clerk",
            EmployeeRole::WarehouseWorker => "Warehouse Worker",
            EmployeeRole::DeliveryDriver => "Delivery Driver",
            EmployeeRole::DepartmentManager => "Department Manager",
            EmployeeRole::StoreManager => "Store Manager",
            EmployeeRole::Security => "Security",
            EmployeeRole::Janitorial => "Janitorial",
            EmployeeRole::CustomerService => "Customer Service",
            EmployeeRole::Accountant => "Accountant",
            EmployeeRole::ITSupport => "IT Support",
            EmployeeRole::HRStaff => "HR Staff",
            EmployeeRole::MarketingStaff => "Marketing Staff",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            EmployeeRole::Cashier => "cashier",
            EmployeeRole::SalesAssociate => "sales_associate",
            EmployeeRole::StockClerk => "stock_clerk",
            EmployeeRole::WarehouseWorker => "warehouse_worker",
            EmployeeRole::DeliveryDriver => "delivery_driver",
            EmployeeRole::DepartmentManager => "department_manager",
            EmployeeRole::StoreManager => "store_manager",
            EmployeeRole::Security => "security",
            EmployeeRole::Janitorial => "janitorial",
            EmployeeRole::CustomerService => "customer_service",
            EmployeeRole::Accountant => "accountant",
            EmployeeRole::ITSupport => "it_support",
            EmployeeRole::HRStaff => "hr_staff",
            EmployeeRole::MarketingStaff => "marketing_staff",
        }
    }

    pub fn from_key(key: &str) -> Option<EmployeeRole> {
        match key {
            "cashier" => Some(EmployeeRole::Cashier),
            "sales_associate" => Some(EmployeeRole::SalesAssociate),
            "stock_clerk" => Some(EmployeeRole::StockClerk),
            "warehouse_worker" => Some(EmployeeRole::WarehouseWorker),
            "delivery_driver" => Some(EmployeeRole::DeliveryDriver),
            "department_manager" => Some(EmployeeRole::DepartmentManager),
            "store_manager" => Some(EmployeeRole::StoreManager),
            "security" => Some(EmployeeRole::Security),
            "janitorial" => Some(EmployeeRole::Janitorial),
            "customer_service" => Some(EmployeeRole::CustomerService),
            "accountant" => Some(EmployeeRole::Accountant),
            "it_support" => Some(EmployeeRole::ITSupport),
            "hr_staff" => Some(EmployeeRole::HRStaff),
            "marketing_staff" => Some(EmployeeRole::MarketingStaff),
            _ => None,
        }
    }

    pub fn base_salary(&self) -> f64 {
        match self {
            EmployeeRole::Cashier => 15_000.0,
            EmployeeRole::SalesAssociate => 16_000.0,
            EmployeeRole::StockClerk => 14_000.0,
            EmployeeRole::WarehouseWorker => 15_000.0,
            EmployeeRole::DeliveryDriver => 16_000.0,
            EmployeeRole::DepartmentManager => 35_000.0,
            EmployeeRole::StoreManager => 50_000.0,
            EmployeeRole::Security => 14_000.0,
            EmployeeRole::Janitorial => 12_000.0,
            EmployeeRole::CustomerService => 18_000.0,
            EmployeeRole::Accountant => 28_000.0,
            EmployeeRole::ITSupport => 25_000.0,
            EmployeeRole::HRStaff => 22_000.0,
            EmployeeRole::MarketingStaff => 22_000.0,
        }
    }

    pub fn category(&self) -> EmployeeCategory {
        match self {
            EmployeeRole::Cashier
            | EmployeeRole::SalesAssociate
            | EmployeeRole::StockClerk
            | EmployeeRole::DepartmentManager
            | EmployeeRole::StoreManager
            | EmployeeRole::CustomerService => EmployeeCategory::StoreOperations,
            EmployeeRole::WarehouseWorker | EmployeeRole::DeliveryDriver => {
                EmployeeCategory::Logistics
            }
            EmployeeRole::Security | EmployeeRole::Janitorial => EmployeeCategory::Facilities,
            EmployeeRole::Accountant
            | EmployeeRole::ITSupport
            | EmployeeRole::HRStaff
            | EmployeeRole::MarketingStaff => EmployeeCategory::Corporate,
        }
    }

    pub fn all_roles() -> Vec<EmployeeRole> {
        vec![
            EmployeeRole::Cashier,
            EmployeeRole::SalesAssociate,
            EmployeeRole::StockClerk,
            EmployeeRole::WarehouseWorker,
            EmployeeRole::DeliveryDriver,
            EmployeeRole::DepartmentManager,
            EmployeeRole::StoreManager,
            EmployeeRole::Security,
            EmployeeRole::Janitorial,
            EmployeeRole::CustomerService,
            EmployeeRole::Accountant,
            EmployeeRole::ITSupport,
            EmployeeRole::HRStaff,
            EmployeeRole::MarketingStaff,
        ]
    }

    pub fn store_roles() -> Vec<EmployeeRole> {
        vec![
            EmployeeRole::Cashier,
            EmployeeRole::SalesAssociate,
            EmployeeRole::StockClerk,
            EmployeeRole::DepartmentManager,
            EmployeeRole::StoreManager,
            EmployeeRole::Security,
            EmployeeRole::Janitorial,
            EmployeeRole::CustomerService,
        ]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EmployeeCategory {
    StoreOperations,
    Logistics,
    Facilities,
    Corporate,
}

impl EmployeeCategory {
    pub fn label(&self) -> &str {
        match self {
            EmployeeCategory::StoreOperations => "Store Operations",
            EmployeeCategory::Logistics => "Logistics",
            EmployeeCategory::Facilities => "Facilities",
            EmployeeCategory::Corporate => "Corporate",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TrainingType {
    Basic,
    Advanced,
    Leadership,
    Safety,
    CustomerService,
}

impl TrainingType {
    pub fn label(&self) -> &str {
        match self {
            TrainingType::Basic => "Basic Skills",
            TrainingType::Advanced => "Advanced Skills",
            TrainingType::Leadership => "Leadership",
            TrainingType::Safety => "Safety Training",
            TrainingType::CustomerService => "Customer Service Excellence",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            TrainingType::Basic => "basic",
            TrainingType::Advanced => "advanced",
            TrainingType::Leadership => "leadership",
            TrainingType::Safety => "safety",
            TrainingType::CustomerService => "customer_service",
        }
    }

    pub fn from_key(key: &str) -> Option<TrainingType> {
        match key {
            "basic" => Some(TrainingType::Basic),
            "advanced" => Some(TrainingType::Advanced),
            "leadership" => Some(TrainingType::Leadership),
            "safety" => Some(TrainingType::Safety),
            "customer_service" => Some(TrainingType::CustomerService),
            _ => None,
        }
    }

    pub fn cost_per_employee(&self) -> f64 {
        match self {
            TrainingType::Basic => 2_000.0,
            TrainingType::Advanced => 5_000.0,
            TrainingType::Leadership => 10_000.0,
            TrainingType::Safety => 1_500.0,
            TrainingType::CustomerService => 3_000.0,
        }
    }

    pub fn skill_bonus(&self) -> f64 {
        match self {
            TrainingType::Basic => 3.0,
            TrainingType::Advanced => 6.0,
            TrainingType::Leadership => 4.0,
            TrainingType::Safety => 2.0,
            TrainingType::CustomerService => 4.0,
        }
    }

    pub fn morale_bonus(&self) -> f64 {
        match self {
            TrainingType::Basic => 1.0,
            TrainingType::Advanced => 2.0,
            TrainingType::Leadership => 3.0,
            TrainingType::Safety => 0.5,
            TrainingType::CustomerService => 1.5,
        }
    }

    pub fn all_types() -> Vec<TrainingType> {
        vec![
            TrainingType::Basic,
            TrainingType::Advanced,
            TrainingType::Leadership,
            TrainingType::Safety,
            TrainingType::CustomerService,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub role: EmployeeRole,
    pub store_id: Option<String>,
    pub skill: f64,
    pub morale: f64,
    pub salary_monthly: f64,
    pub tenure_quarters: i32,
    pub performance_rating: f64,
    pub training_count: u32,
}

impl Employee {
    pub fn new(
        rng: &mut rand::rngs::ThreadRng,
        role: EmployeeRole,
        store_id: Option<String>,
        hr_policy_multiplier: f64,
    ) -> Self {
        let name = generate_executive_name(rng);
        let base_salary = role.base_salary();
        let salary = base_salary * hr_policy_multiplier * rng.gen_range(0.95..1.10);

        Employee {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            role,
            store_id,
            skill: rng.gen_range(35.0..65.0),
            morale: rng.gen_range(50.0..80.0),
            salary_monthly: salary,
            tenure_quarters: 0,
            performance_rating: rng.gen_range(50.0..75.0),
            training_count: 0,
        }
    }

    pub fn apply_training(&mut self, training: TrainingType) {
        self.skill = (self.skill + training.skill_bonus()).min(100.0);
        self.morale = (self.morale + training.morale_bonus()).min(100.0);
        self.training_count += 1;
    }

    pub fn give_raise(&mut self, percentage: f64) {
        self.salary_monthly *= 1.0 + percentage / 100.0;
        self.morale = (self.morale + percentage * 0.5).min(100.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeSystem {
    pub employees: Vec<Employee>,
    pub total_count: u32,
    pub monthly_payroll: f64,
    pub avg_morale: f64,
    pub avg_skill: f64,
    pub turnover_rate: f64,
    pub training_budget: f64,
    pub training_spent_quarter: f64,
    pub role_breakdown: HashMap<String, u32>,
}

impl Default for EmployeeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EmployeeSystem {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut employees = Vec::new();
        let mut role_breakdown: HashMap<String, u32> = HashMap::new();

        let initial_staffing = vec![
            (EmployeeRole::Cashier, 20),
            (EmployeeRole::SalesAssociate, 15),
            (EmployeeRole::StockClerk, 10),
            (EmployeeRole::DepartmentManager, 4),
            (EmployeeRole::StoreManager, 1),
            (EmployeeRole::CustomerService, 5),
            (EmployeeRole::Security, 3),
            (EmployeeRole::Janitorial, 4),
            (EmployeeRole::WarehouseWorker, 5),
            (EmployeeRole::Accountant, 1),
            (EmployeeRole::ITSupport, 1),
            (EmployeeRole::HRStaff, 1),
        ];

        for (role, count) in initial_staffing {
            for _ in 0..count {
                let store_id = if role.category() == EmployeeCategory::Corporate {
                    None
                } else {
                    Some("store-1".to_string())
                };
                let emp = Employee::new(&mut rng, role, store_id, 1.0);
                employees.push(emp);
            }
            *role_breakdown.entry(role.key().to_string()).or_insert(0) += count;
        }

        let total_count = employees.len() as u32;
        let monthly_payroll: f64 = employees.iter().map(|e| e.salary_monthly).sum();
        let avg_morale = employees.iter().map(|e| e.morale).sum::<f64>() / employees.len() as f64;
        let avg_skill = employees.iter().map(|e| e.skill).sum::<f64>() / employees.len() as f64;

        EmployeeSystem {
            employees,
            total_count,
            monthly_payroll,
            avg_morale,
            avg_skill,
            turnover_rate: 5.0,
            training_budget: 100_000.0,
            training_spent_quarter: 0.0,
            role_breakdown,
        }
    }

    pub fn hire(
        &mut self,
        rng: &mut rand::rngs::ThreadRng,
        role: EmployeeRole,
        store_id: Option<String>,
        hr_policy_multiplier: f64,
    ) -> f64 {
        let emp = Employee::new(rng, role, store_id, hr_policy_multiplier);
        let hiring_cost = emp.salary_monthly * 0.5;
        self.employees.push(emp);
        *self
            .role_breakdown
            .entry(role.key().to_string())
            .or_insert(0) += 1;
        self.recalculate_totals();
        hiring_cost
    }

    pub fn fire(&mut self, employee_id: &str) -> Option<(String, f64)> {
        if let Some(idx) = self.employees.iter().position(|e| e.id == employee_id) {
            let emp = self.employees.remove(idx);
            let role_key = emp.role.key().to_string();
            if let Some(count) = self.role_breakdown.get_mut(&role_key) {
                *count = count.saturating_sub(1);
            }
            let severance = emp.salary_monthly * 1.0;
            self.recalculate_totals();
            return Some((emp.name, severance));
        }
        None
    }

    pub fn train_employee(&mut self, employee_id: &str, training: TrainingType) -> Option<f64> {
        if let Some(emp) = self.employees.iter_mut().find(|e| e.id == employee_id) {
            let cost = training.cost_per_employee();
            emp.apply_training(training);
            self.training_spent_quarter += cost;
            self.recalculate_totals();
            return Some(cost);
        }
        None
    }

    pub fn train_all_role(&mut self, role: EmployeeRole, training: TrainingType) -> (u32, f64) {
        let mut count = 0u32;
        let mut total_cost = 0.0;

        for emp in self.employees.iter_mut().filter(|e| e.role == role) {
            emp.apply_training(training);
            count += 1;
            total_cost += training.cost_per_employee();
        }

        self.training_spent_quarter += total_cost;
        self.recalculate_totals();
        (count, total_cost)
    }

    pub fn give_raise(&mut self, employee_id: &str, percentage: f64) -> bool {
        if let Some(emp) = self.employees.iter_mut().find(|e| e.id == employee_id) {
            emp.give_raise(percentage);
            self.recalculate_totals();
            return true;
        }
        false
    }

    pub fn get_by_store(&self, store_id: &str) -> Vec<&Employee> {
        self.employees
            .iter()
            .filter(|e| e.store_id.as_deref() == Some(store_id))
            .collect()
    }

    pub fn get_by_role(&self, role: EmployeeRole) -> Vec<&Employee> {
        self.employees.iter().filter(|e| e.role == role).collect()
    }

    pub fn get_corporate_staff(&self) -> Vec<&Employee> {
        self.employees
            .iter()
            .filter(|e| e.store_id.is_none())
            .collect()
    }

    fn recalculate_totals(&mut self) {
        self.total_count = self.employees.len() as u32;
        self.monthly_payroll = self.employees.iter().map(|e| e.salary_monthly).sum();
        if !self.employees.is_empty() {
            self.avg_morale =
                self.employees.iter().map(|e| e.morale).sum::<f64>() / self.employees.len() as f64;
            self.avg_skill =
                self.employees.iter().map(|e| e.skill).sum::<f64>() / self.employees.len() as f64;
        } else {
            self.avg_morale = 0.0;
            self.avg_skill = 0.0;
        }
    }

    pub fn process_quarter(
        &mut self,
        rng: &mut rand::rngs::ThreadRng,
        hr_morale_base: f64,
        is_profitable: bool,
        chro_skill: Option<f64>,
    ) -> f64 {
        self.training_spent_quarter = 0.0;

        let performance_factor = if is_profitable { 2.0 } else { -3.0 };
        let morale_factor = if is_profitable { 2.0 } else { -3.0 };
        let chro_bonus = chro_skill.unwrap_or(0.0) * 0.1;
        let target_morale = (hr_morale_base + chro_bonus).clamp(20.0, 95.0);

        for emp in &mut self.employees {
            emp.tenure_quarters += 1;

            let morale_drift = rng.gen_range(-3.0..3.0);
            emp.morale += (target_morale - emp.morale) * 0.1 + morale_factor * 0.2 + morale_drift;
            emp.morale = emp.morale.clamp(10.0, 100.0);

            let skill_drift = rng.gen_range(-0.5..1.5);
            emp.skill = (emp.skill + skill_drift).clamp(20.0, 100.0);

            let perf_drift = rng.gen_range(-3.0..3.0);
            emp.performance_rating =
                (emp.performance_rating + performance_factor * 0.3 + perf_drift).clamp(20.0, 100.0);

            if emp.tenure_quarters % 4 == 0 && emp.tenure_quarters > 0 {
                let raise_pct = rng.gen_range(3.0..8.0);
                emp.salary_monthly *= 1.0 + raise_pct / 100.0;
            }
        }

        let base_turnover = match hr_morale_base {
            x if x >= 80.0 => 2.0,
            x if x >= 65.0 => 4.0,
            x if x >= 50.0 => 7.0,
            _ => 12.0,
        };

        let morale_turnover_adj = if self.avg_morale < 40.0 {
            6.0
        } else if self.avg_morale < 55.0 {
            2.0
        } else {
            0.0
        };
        self.turnover_rate =
            (base_turnover + morale_turnover_adj + rng.gen_range(-1.0_f64..1.0_f64))
                .clamp(1.0, 25.0);

        let employees_to_remove: Vec<String> = self
            .employees
            .iter()
            .filter(|e| {
                (e.morale < 20.0 && rng.gen_bool(0.4))
                    || (e.morale < 30.0 && e.performance_rating < 40.0 && rng.gen_bool(0.3))
            })
            .map(|e| e.id.clone())
            .collect();

        let mut hiring_costs = 0.0;
        for emp_id in employees_to_remove {
            if let Some(idx) = self.employees.iter().position(|e| e.id == emp_id) {
                let emp = &self.employees[idx];
                let role = emp.role;
                let store_id = emp.store_id.clone();
                self.employees.remove(idx);
                if let Some(count) = self.role_breakdown.get_mut(role.key()) {
                    *count = count.saturating_sub(1);
                }
                let new_emp = Employee::new(rng, role, store_id, 1.0);
                hiring_costs += new_emp.salary_monthly * 0.5;
                self.employees.push(new_emp);
            }
        }

        self.recalculate_totals();
        hiring_costs + self.training_spent_quarter
    }

    pub fn update_for_new_store(
        &mut self,
        store_id: &str,
        store_size_sqm: u32,
        rng: &mut rand::rngs::ThreadRng,
        hr_policy_multiplier: f64,
    ) {
        let staff_count = (store_size_sqm as f64 * 0.015) as u32;
        let distribution = vec![
            (EmployeeRole::StoreManager, 1),
            (
                EmployeeRole::DepartmentManager,
                (staff_count as f64 * 0.05) as u32,
            ),
            (EmployeeRole::Cashier, (staff_count as f64 * 0.30) as u32),
            (
                EmployeeRole::SalesAssociate,
                (staff_count as f64 * 0.25) as u32,
            ),
            (EmployeeRole::StockClerk, (staff_count as f64 * 0.20) as u32),
            (
                EmployeeRole::CustomerService,
                (staff_count as f64 * 0.10) as u32,
            ),
            (EmployeeRole::Security, (staff_count as f64 * 0.05) as u32),
            (EmployeeRole::Janitorial, (staff_count as f64 * 0.04) as u32),
        ];

        for (role, count) in distribution {
            for _ in 0..count.max(1) {
                let emp =
                    Employee::new(rng, role, Some(store_id.to_string()), hr_policy_multiplier);
                self.employees.push(emp);
                *self
                    .role_breakdown
                    .entry(role.key().to_string())
                    .or_insert(0) += 1;
            }
        }

        self.recalculate_totals();
    }

    pub fn remove_store_employees(&mut self, store_id: &str) {
        self.employees
            .retain(|e| e.store_id.as_deref() != Some(store_id));
        self.role_breakdown.clear();
        for emp in &self.employees {
            *self
                .role_breakdown
                .entry(emp.role.key().to_string())
                .or_insert(0) += 1;
        }
        self.recalculate_totals();
    }
}

pub fn employee_salary_by_role(employees: &[Employee], role: EmployeeRole) -> f64 {
    employees
        .iter()
        .filter(|e| e.role == role)
        .map(|e| e.salary_monthly)
        .sum()
}

pub fn employee_count_by_role(employees: &[Employee], role: EmployeeRole) -> u32 {
    employees.iter().filter(|e| e.role == role).count() as u32
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BenefitType {
    HealthInsurance,
    DentalVision,
    LifeInsurance,
    RetirementPlan,
    PaidTimeOff,
    EducationAssistance,
    WellnessProgram,
    TransportationAllowance,
    MealAllowance,
    PerformanceBonus,
}

impl BenefitType {
    pub fn label(&self) -> &str {
        match self {
            BenefitType::HealthInsurance => "Health Insurance",
            BenefitType::DentalVision => "Dental & Vision",
            BenefitType::LifeInsurance => "Life Insurance",
            BenefitType::RetirementPlan => "Retirement Plan (401k)",
            BenefitType::PaidTimeOff => "Extended Paid Time Off",
            BenefitType::EducationAssistance => "Education Assistance",
            BenefitType::WellnessProgram => "Wellness Program",
            BenefitType::TransportationAllowance => "Transportation Allowance",
            BenefitType::MealAllowance => "Meal Allowance",
            BenefitType::PerformanceBonus => "Performance Bonus Scheme",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            BenefitType::HealthInsurance => "health_insurance",
            BenefitType::DentalVision => "dental_vision",
            BenefitType::LifeInsurance => "life_insurance",
            BenefitType::RetirementPlan => "retirement_plan",
            BenefitType::PaidTimeOff => "paid_time_off",
            BenefitType::EducationAssistance => "education_assistance",
            BenefitType::WellnessProgram => "wellness_program",
            BenefitType::TransportationAllowance => "transportation_allowance",
            BenefitType::MealAllowance => "meal_allowance",
            BenefitType::PerformanceBonus => "performance_bonus",
        }
    }

    pub fn from_key(key: &str) -> Option<BenefitType> {
        match key {
            "health_insurance" => Some(BenefitType::HealthInsurance),
            "dental_vision" => Some(BenefitType::DentalVision),
            "life_insurance" => Some(BenefitType::LifeInsurance),
            "retirement_plan" => Some(BenefitType::RetirementPlan),
            "paid_time_off" => Some(BenefitType::PaidTimeOff),
            "education_assistance" => Some(BenefitType::EducationAssistance),
            "wellness_program" => Some(BenefitType::WellnessProgram),
            "transportation_allowance" => Some(BenefitType::TransportationAllowance),
            "meal_allowance" => Some(BenefitType::MealAllowance),
            "performance_bonus" => Some(BenefitType::PerformanceBonus),
            _ => None,
        }
    }

    pub fn monthly_cost_per_employee(&self) -> f64 {
        match self {
            BenefitType::HealthInsurance => 3_500.0,
            BenefitType::DentalVision => 800.0,
            BenefitType::LifeInsurance => 500.0,
            BenefitType::RetirementPlan => 2_000.0,
            BenefitType::PaidTimeOff => 1_500.0,
            BenefitType::EducationAssistance => 2_500.0,
            BenefitType::WellnessProgram => 600.0,
            BenefitType::TransportationAllowance => 1_200.0,
            BenefitType::MealAllowance => 1_000.0,
            BenefitType::PerformanceBonus => 0.0,
        }
    }

    pub fn morale_bonus(&self) -> f64 {
        match self {
            BenefitType::HealthInsurance => 8.0,
            BenefitType::DentalVision => 3.0,
            BenefitType::LifeInsurance => 2.0,
            BenefitType::RetirementPlan => 5.0,
            BenefitType::PaidTimeOff => 6.0,
            BenefitType::EducationAssistance => 4.0,
            BenefitType::WellnessProgram => 3.0,
            BenefitType::TransportationAllowance => 2.5,
            BenefitType::MealAllowance => 2.0,
            BenefitType::PerformanceBonus => 4.0,
        }
    }

    pub fn turnover_reduction(&self) -> f64 {
        match self {
            BenefitType::HealthInsurance => 3.0,
            BenefitType::DentalVision => 1.0,
            BenefitType::LifeInsurance => 0.5,
            BenefitType::RetirementPlan => 2.0,
            BenefitType::PaidTimeOff => 2.5,
            BenefitType::EducationAssistance => 1.5,
            BenefitType::WellnessProgram => 1.0,
            BenefitType::TransportationAllowance => 0.8,
            BenefitType::MealAllowance => 0.5,
            BenefitType::PerformanceBonus => 1.5,
        }
    }

    pub fn performance_bonus(&self) -> f64 {
        match self {
            BenefitType::HealthInsurance => 1.0,
            BenefitType::DentalVision => 0.5,
            BenefitType::LifeInsurance => 0.3,
            BenefitType::RetirementPlan => 0.8,
            BenefitType::PaidTimeOff => 0.7,
            BenefitType::EducationAssistance => 1.2,
            BenefitType::WellnessProgram => 0.6,
            BenefitType::TransportationAllowance => 0.4,
            BenefitType::MealAllowance => 0.3,
            BenefitType::PerformanceBonus => 2.0,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            BenefitType::HealthInsurance => {
                "Comprehensive medical coverage for employees and dependents"
            }
            BenefitType::DentalVision => "Dental and vision care coverage",
            BenefitType::LifeInsurance => "Life insurance and accidental death coverage",
            BenefitType::RetirementPlan => "Company-matched retirement savings plan",
            BenefitType::PaidTimeOff => "Additional vacation and sick leave days",
            BenefitType::EducationAssistance => {
                "Tuition reimbursement and skill development programs"
            }
            BenefitType::WellnessProgram => {
                "Gym membership, mental health support, health screenings"
            }
            BenefitType::TransportationAllowance => "Monthly transportation subsidy",
            BenefitType::MealAllowance => "Daily meal allowance or subsidized cafeteria",
            BenefitType::PerformanceBonus => "Quarterly performance-based bonus pool",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            BenefitType::HealthInsurance => "Heart",
            BenefitType::DentalVision => "Eye",
            BenefitType::LifeInsurance => "Shield",
            BenefitType::RetirementPlan => "PiggyBank",
            BenefitType::PaidTimeOff => "Calendar",
            BenefitType::EducationAssistance => "GraduationCap",
            BenefitType::WellnessProgram => "Activity",
            BenefitType::TransportationAllowance => "Car",
            BenefitType::MealAllowance => "Utensils",
            BenefitType::PerformanceBonus => "Trophy",
        }
    }

    pub fn all_types() -> Vec<BenefitType> {
        vec![
            BenefitType::HealthInsurance,
            BenefitType::DentalVision,
            BenefitType::LifeInsurance,
            BenefitType::RetirementPlan,
            BenefitType::PaidTimeOff,
            BenefitType::EducationAssistance,
            BenefitType::WellnessProgram,
            BenefitType::TransportationAllowance,
            BenefitType::MealAllowance,
            BenefitType::PerformanceBonus,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeBenefits {
    pub active_benefits: Vec<BenefitType>,
    pub quarters_active: HashMap<String, i32>,
}

impl Default for EmployeeBenefits {
    fn default() -> Self {
        Self::new()
    }
}

impl EmployeeBenefits {
    pub fn new() -> Self {
        EmployeeBenefits {
            active_benefits: vec![BenefitType::HealthInsurance],
            quarters_active: vec![("health_insurance".to_string(), 0)]
                .into_iter()
                .collect(),
        }
    }

    pub fn activate(&mut self, benefit: BenefitType) -> f64 {
        if self.active_benefits.contains(&benefit) {
            return 0.0;
        }
        self.active_benefits.push(benefit);
        self.quarters_active.insert(benefit.key().to_string(), 0);
        benefit.monthly_cost_per_employee() * 3.0
    }

    pub fn deactivate(&mut self, benefit: BenefitType) -> bool {
        if let Some(idx) = self.active_benefits.iter().position(|b| *b == benefit) {
            self.active_benefits.remove(idx);
            self.quarters_active.remove(benefit.key());
            return true;
        }
        false
    }

    pub fn is_active(&self, benefit: BenefitType) -> bool {
        self.active_benefits.contains(&benefit)
    }

    pub fn total_morale_bonus(&self) -> f64 {
        self.active_benefits.iter().map(|b| b.morale_bonus()).sum()
    }

    pub fn total_turnover_reduction(&self) -> f64 {
        self.active_benefits
            .iter()
            .map(|b| b.turnover_reduction())
            .sum()
    }

    pub fn total_performance_bonus(&self) -> f64 {
        self.active_benefits
            .iter()
            .map(|b| b.performance_bonus())
            .sum()
    }

    pub fn total_monthly_cost_per_employee(&self) -> f64 {
        self.active_benefits
            .iter()
            .map(|b| b.monthly_cost_per_employee())
            .sum()
    }

    pub fn process_quarter(&mut self) {
        for benefit in &self.active_benefits {
            if let Some(count) = self.quarters_active.get_mut(benefit.key()) {
                *count += 1;
            }
        }
    }
}
