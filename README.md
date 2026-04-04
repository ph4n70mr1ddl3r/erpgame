# Bahay Depot CEO Simulator

A web-based business simulation game where you take on the role of CEO of Bahay Depot, a Philippine home improvement retail chain. Grow your company from a single store to a nationwide retail empire.

## Features

- **Store Management**: Open new stores in various Philippine cities, choose store types (Express, Standard, Mega, Depot), manage construction, and close underperforming locations.
- **Executive Team**: Hire and fire C‑suite executives (CFO, COO, CMO, CTO, CHRO, CSCO) with unique skills, salaries, morale, and loyalty.
- **Policy Settings**: Set company policies for pricing, HR, expansion, customer service, marketing, and inventory.
- **Financial Management**: Track revenue, expenses, profits, taxes, and take out loans with interest.
- **Dynamic Economy**: Experience changing economic conditions (GDP growth, inflation, interest rates, consumer confidence).
- **Product Categories**: Invest in product categories (building materials, plumbing, electrical, etc.) to improve margins and demand.
- **Store Upgrades**: Purchase upgrades for individual stores to boost revenue, reduce costs, and increase customer satisfaction.
- **Events & Decisions**: Random events (typhoons, economic shifts, competitor actions, supply‑chain issues, etc.) present choices that affect your company.
- **Delegation**: Delegate decision‑making to your executives based on their expertise.
- **Board of Directors**: Keep the board's patience high through strong performance or face warnings and eventual firing.
- **Competitors**: Compete against other home‑improvement chains (Wilcon Depot, CW Home Depot, AllHome) for market share.
 - **Loyalty Program**: Launch and manage a customer loyalty rewards program with multiple tiers (Basic, Silver, Gold, Platinum) to boost revenue, satisfaction, and repeat business.
 - **Achievements**: Unlock achievements for various milestones (store count, revenue, company value, etc.).
- **E-commerce**: Launch and upgrade your online channel from a basic website to full omnichannel integration. Your CTO's skill amplifies online revenue.
- **Sponsorships**: Sponsor sports teams, community events, trade shows, and more to boost brand visibility and reputation, and revenue. Your CMO's skill amplifies sponsorship effects.
- **Charts & History**: View financial history and performance charts over time.
- **Employee Benefits**: Offer comprehensive benefits packages (health insurance, retirement plans, wellness programs, etc.) to improve employee morale, reduce turnover, and boost performance. Each active benefit adds to your monthly payroll costs.

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Templating**: Askama (Jinja‑like templates)
- **Frontend**: TailwindCSS for styling, Chart.js for charts
- **Game Logic**: Pure Rust with random number generation (rand) and serialization (serde)

## Installation

### Prerequisites

- Rust (stable) and Cargo
- Git

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/ph4n70mr1ddl3r/erpgame.git
   cd erpgame
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run --release
   ```

4. Open your browser and navigate to `http://localhost:3000`.

## How to Play

1. **Dashboard**: Get an overview of your company's financial health, store count, market share, and satisfaction metrics.
2. **Tick**: Press the "Next Quarter" button (or POST `/tick`) to advance the game by one quarter. This triggers all economic changes, store operations, and event generation.
3. **Manage Stores**: Open new stores, close existing ones, and view their performance.
4. **Hire Executives**: Fill C‑suite positions to gain strategic advantages.
5. **Set Policies**: Adjust company policies to balance profitability, growth, and employee/customer satisfaction.
6. **Handle Decisions**: When events occur, make choices that affect your company's future.
7. **Delegate**: Automate decision‑making for certain event categories by delegating to your executives.
8. **Monitor Finances**: Keep an eye on cash flow, loans, and profitability. Avoid bankruptcy (cash < ‑₱10M) and aim for a company value of ₱10B to win.

## Project Structure

```
├── src/
│   ├── main.rs          # Axum server setup and routing
│   ├── templates.rs     # Askama template structs
│   ├── api/
│   │   ├── dto.rs       # Data transfer objects
│   │   ├── routes.rs    # HTTP route handlers
│   │   └── mod.rs
│   └── game/
│       ├── state.rs     # Core game state structures
│       ├── simulation.rs # Quarterly simulation logic
│       ├── achievements.rs
│       ├── board.rs
│       ├── competitors.rs
│       ├── economy.rs
│       ├── ecommerce.rs
│       ├── events.rs
│       ├── executive_ai.rs
│       ├── products.rs
│       ├── upgrades.rs
│       └── mod.rs
├── templates/           # Askama HTML templates (Jinja‑like)
│   ├── base.html
│   ├── dashboard.html
│   ├── stores.html
│   └── ... (other pages)
├── static/              # Static assets (CSS, JS) – currently empty
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License – see the LICENSE file for details (if present).