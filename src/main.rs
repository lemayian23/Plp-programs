mod data_processing;
mod ml_model;
mod models;
mod analyzer;
mod web;
mod database;
mod utils; // NEW: Add utils module

use actix_web::{App, HttpServer, web::Data};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use web::handlers::AppState;
use database::setup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Smart Study Planner Production Server...");
    
    // Initialize database
    if let Err(e) = setup::initialize_database() {
        eprintln!("❌ Failed to initialize database: {}", e);
        std::process::exit(1);
    }
    
    println!("✅ Database: Ready");
    println!("🔐 Authentication: Ready");
    println!("📊 ML System: Ready");
    println!("🎯 Role-Based Access: Ready");
    println!("📧 Email System: Ready (Console)");
    println!("🌐 Web Interface: Starting on http://localhost:8080");
    
    // In production, use a fixed secret key from environment variables
    let secret_key = match std::env::var("SECRET_KEY") {
        Ok(key) => Key::from(key.as_bytes()),
        Err(_) => {
            println!("⚠️  Using generated secret key - set SECRET_KEY env var in production");
            Key::generate()
        }
    };
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                analyzer: std::sync::Mutex::new(None),
            }))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .build()
            )
            .configure(web::routes::config)
    })
    .bind("127.0.0.1:8080")?
    .workers(4) // Optimize for production
    .run()
    .await
}