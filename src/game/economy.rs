use super::state::*;

impl EconomyState {
    pub fn description(&self) -> String {
        if self.gdp_growth_rate > 7.0 {
            "Booming economy with strong growth".into()
        } else if self.gdp_growth_rate > 5.0 {
            "Healthy economic growth".into()
        } else if self.gdp_growth_rate > 3.0 {
            "Moderate economic growth".into()
        } else if self.gdp_growth_rate > 0.0 {
            "Slowing economy".into()
        } else {
            "Economy in recession".into()
        }
    }
}

impl MarketState {
    pub fn competition_description(&self) -> String {
        if self.competitor_strength > 85.0 {
            format!(
                "Fierce competition ({} rivals, high strength)",
                self.competitor_count
            )
        } else if self.competitor_strength > 70.0 {
            format!("Strong competition ({} rivals)", self.competitor_count)
        } else if self.competitor_strength > 50.0 {
            format!("Moderate competition ({} rivals)", self.competitor_count)
        } else {
            format!("Weak competition ({} rivals)", self.competitor_count)
        }
    }
}

impl PricingPolicy {
    pub fn label(&self) -> &str {
        match self {
            PricingPolicy::Budget => "Budget (Low prices, high volume)",
            PricingPolicy::Competitive => "Competitive (Market-rate pricing)",
            PricingPolicy::Premium => "Premium (Higher margins, select clientele)",
            PricingPolicy::Dynamic => "Dynamic (AI-adjusted pricing)",
        }
    }
    pub fn key(&self) -> &str {
        match self {
            PricingPolicy::Budget => "budget",
            PricingPolicy::Competitive => "competitive",
            PricingPolicy::Premium => "premium",
            PricingPolicy::Dynamic => "dynamic",
        }
    }
}

impl HrPolicy {
    pub fn label(&self) -> &str {
        match self {
            HrPolicy::Minimal => "Minimal (Basic compensation)",
            HrPolicy::Standard => "Standard (Market-rate benefits)",
            HrPolicy::Generous => "Generous (Above-market compensation)",
            HrPolicy::Elite => "Elite (Top-tier benefits & perks)",
        }
    }
    pub fn key(&self) -> &str {
        match self {
            HrPolicy::Minimal => "minimal",
            HrPolicy::Standard => "standard",
            HrPolicy::Generous => "generous",
            HrPolicy::Elite => "elite",
        }
    }
}

impl ExpansionPolicy {
    pub fn label(&self) -> &str {
        match self {
            ExpansionPolicy::Conservative => "Conservative (Careful, profitable growth)",
            ExpansionPolicy::Moderate => "Moderate (Balanced expansion)",
            ExpansionPolicy::Aggressive => "Aggressive (Rapid expansion)",
            ExpansionPolicy::Blitz => "Blitz (Maximum speed expansion)",
        }
    }
    pub fn key(&self) -> &str {
        match self {
            ExpansionPolicy::Conservative => "conservative",
            ExpansionPolicy::Moderate => "moderate",
            ExpansionPolicy::Aggressive => "aggressive",
            ExpansionPolicy::Blitz => "blitz",
        }
    }
}

impl CustomerServicePolicy {
    pub fn label(&self) -> &str {
        match self {
            CustomerServicePolicy::Basic => "Basic (Self-service focus)",
            CustomerServicePolicy::Good => "Good (Trained staff available)",
            CustomerServicePolicy::Excellent => "Excellent (Dedicated support teams)",
            CustomerServicePolicy::WhiteGlove => "White Glove (Premium concierge service)",
        }
    }
    pub fn key(&self) -> &str {
        match self {
            CustomerServicePolicy::Basic => "basic",
            CustomerServicePolicy::Good => "good",
            CustomerServicePolicy::Excellent => "excellent",
            CustomerServicePolicy::WhiteGlove => "whiteglove",
        }
    }
}

impl MarketingPolicy {
    pub fn label(&self) -> &str {
        match self {
            MarketingPolicy::LowKey => "Low Key (Minimal marketing spend)",
            MarketingPolicy::Moderate => "Moderate (Regular campaigns)",
            MarketingPolicy::Heavy => "Heavy (Large-scale campaigns)",
            MarketingPolicy::Aggressive => "Aggressive (Maximum market presence)",
        }
    }
    pub fn key(&self) -> &str {
        match self {
            MarketingPolicy::LowKey => "lowkey",
            MarketingPolicy::Moderate => "moderate",
            MarketingPolicy::Heavy => "heavy",
            MarketingPolicy::Aggressive => "aggressive",
        }
    }
}

impl InventoryPolicy {
    pub fn label(&self) -> &str {
        match self {
            InventoryPolicy::Lean => "Lean (Minimal stock, risk of stockouts)",
            InventoryPolicy::Standard => "Standard (Balanced inventory levels)",
            InventoryPolicy::Buffered => "Buffered (Extra safety stock)",
            InventoryPolicy::Abundant => "Abundant (Maximum availability)",
        }
    }
    pub fn key(&self) -> &str {
        match self {
            InventoryPolicy::Lean => "lean",
            InventoryPolicy::Standard => "standard",
            InventoryPolicy::Buffered => "buffered",
            InventoryPolicy::Abundant => "abundant",
        }
    }
}

impl StoreStatus {
    pub fn label(&self) -> &str {
        match self {
            StoreStatus::Operating => "Operating",
            StoreStatus::UnderConstruction => "Under Construction",
            StoreStatus::Closed => "Closed",
        }
    }
    pub fn css_class(&self) -> &str {
        match self {
            StoreStatus::Operating => "text-green-400",
            StoreStatus::UnderConstruction => "text-yellow-400",
            StoreStatus::Closed => "text-red-400",
        }
    }
}

impl StoreType {
    pub fn label(&self) -> &str {
        match self {
            StoreType::Express => "Express",
            StoreType::Standard => "Standard",
            StoreType::Mega => "Mega",
            StoreType::Depot => "Depot",
        }
    }
}

impl EventType {
    pub fn icon(&self) -> &str {
        match self {
            EventType::NaturalDisaster => "Tornado",
            EventType::Economic => "Chart",
            EventType::Competition => "Building",
            EventType::Employee => "Users",
            EventType::Marketing => "Megaphone",
            EventType::Regulation => "Clipboard",
            EventType::SupplyChain => "Ship",
            EventType::Positive => "Sparkles",
            EventType::Negative => "Alert",
        }
    }
}
