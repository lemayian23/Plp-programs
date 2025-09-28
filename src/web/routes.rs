use actix_web::web;
use super::handlers;
use crate::database::repository::Database;

pub fn config(cfg: &mut web::ServiceConfig) {
    let database = Database::new().expect("Failed to create database connection");
    
    cfg.app_data(web::Data::new(database))
        .service(
            web::scope("/api")
                // Authentication routes
                .route("/register", web::post().to(handlers::register))
                .route("/login", web::post().to(handlers::login))
                .route("/logout", web::post().to(handlers::logout))
                .route("/user", web::get().to(handlers::get_current_user))
                
                // Email verification
                .route("/verify-email/{token}", web::get().to(handlers::verify_email))
                
                // Password reset
                .route("/request-password-reset", web::post().to(handlers::request_password_reset))
                .route("/confirm-password-reset", web::post().to(handlers::confirm_password_reset))
                
                // Profile management
                .route("/profile", web::get().to(handlers::get_profile))
                .route("/profile", web::put().to(handlers::update_profile))
                
                // Admin routes
                .route("/admin/stats", web::get().to(handlers::admin_dashboard))
                
                // Study data routes
                .route("/analysis", web::get().to(handlers::get_analysis))
                .route("/session", web::post().to(handlers::add_study_session))
                .route("/analysis/history", web::get().to(handlers::get_analysis_history))
                .route("/upload", web::post().to(handlers::upload_csv))
        )
        .route("/", web::get().to(handlers::index));
}