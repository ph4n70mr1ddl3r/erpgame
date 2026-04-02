mod game;
mod api;
mod templates;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let game_state = Arc::new(Mutex::new(game::GameState::new()));

    let app = Router::new()
        .route("/", get(api::routes::dashboard))
        .route("/tick", post(api::routes::tick))
        .route("/stores", get(api::routes::stores_page))
        .route("/stores/open", post(api::routes::open_store))
        .route("/stores/{id}/close", post(api::routes::close_store))
        .route("/executives", get(api::routes::executives_page))
        .route("/executives/hire", post(api::routes::hire_executive))
        .route("/executives/{id}/fire", post(api::routes::fire_executive))
        .route("/policies", get(api::routes::policies_page))
        .route("/policies", post(api::routes::update_policies))
        .route("/finances", get(api::routes::finances_page))
        .route("/finances/loan", post(api::routes::take_loan))
        .route("/events", get(api::routes::events_page))
        .route("/new-game", get(api::routes::new_game))
        .route("/decisions", get(api::routes::decisions_page))
        .route("/decisions/resolve", post(api::routes::resolve_decision))
        .route("/delegation", get(api::routes::delegation_page))
        .route("/delegation", post(api::routes::update_delegation))
        .route("/products", get(api::routes::products_page))
        .route("/products/invest", post(api::routes::invest_product))
        .route("/upgrades", get(api::routes::upgrades_page))
        .route("/upgrades/purchase", post(api::routes::purchase_upgrade))
        .route("/board", get(api::routes::board_page))
        .route("/competitors", get(api::routes::competitors_page))
        .route("/achievements", get(api::routes::achievements_page))
        .with_state(game_state)
        .nest_service("/static", ServeDir::new("static"));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to bind to port 3000: {}", e);
            std::process::exit(1);
        }
    };
    tracing::info!("Bahay Depot CEO Simulator running on http://localhost:3000");
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}
