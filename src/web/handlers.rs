use actix_web::{HttpResponse, Result, web};
use actix_multipart::Multipart;
use actix_session::Session;
use crate::analyzer::SmartAnalyzer;
use crate::data_processing::StudySession;
use crate::database::repository::Database;
use crate::database::models::{StudySessionDb, RegisterRequest, LoginRequest, User, ProfileUpdate, PasswordResetRequest, PasswordResetConfirm};
use crate::utils::validation::Validation;
use std::sync::Mutex;
use futures_util::StreamExt as _;

// App state to share between requests
pub struct AppState {
    pub analyzer: Mutex<Option<SmartAnalyzer>>,
}

// Home page handler
pub async fn index() -> Result<HttpResponse> {
    let html = include_str!("../../templates/index.html");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

// User registration
pub async fn register(
    db: web::Data<Database>,
    session: Session,
    user_data: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    // Validate email
    if let Err(e) = Validation::validate_email(&user_data.email) {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": e})
        ));
    }

    // Validate username
    if let Err(e) = Validation::validate_username(&user_data.username) {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": e})
        ));
    }

    // Validate password strength
    if let Err(e) = Validation::validate_password_strength(&user_data.password) {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": e})
        ));
    }

    // Check if username already exists
    if db.get_user_by_username(&user_data.username).is_ok() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": "Username already exists"})
        ));
    }
    
    // Check if email already exists
    if db.get_user_by_email(&user_data.email).is_ok() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": "Email already exists"})
        ));
    }
    
    // Hash password
    let password_hash = Validation::hash_password(&user_data.password)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(
                format!("Password hashing error: {}", e)
            )
        })?;
    
    // Create user
    let user = User::new(
        user_data.username.clone(),
        user_data.email.clone(),
        password_hash,
    );
    
    // Save user to database
    if let Err(e) = db.create_user(&user) {
        return Ok(HttpResponse::InternalServerError().json(
            serde_json::json!({"error": format!("Failed to create user: {}", e)})
        ));
    }
    
    // Store user ID in session
    session.insert("user_id", &user.id)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session error: {}", e)))?;
    session.insert("username", &user.username)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session error: {}", e)))?;
    session.insert("role", &user.role)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session error: {}", e)))?;
    
    // Generate email verification token (in production, send email)
    let verification_token = db.create_email_verification_token(&user.id)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Token creation error: {}", e)))?;
    
    println!("🔐 Email verification token for {}: {}", user.email, verification_token);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User registered successfully. Please check your email for verification.",
        "user_id": user.id,
        "username": user.username,
        "role": user.role,
        "email_verified": user.email_verified
    })))
}

// User login
pub async fn login(
    db: web::Data<Database>,
    session: Session,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Verify password
    let is_valid = db.verify_password(&login_data.username, &login_data.password)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(
                format!("Database error: {}", e)
            )
        })?;
    
    if !is_valid {
        return Ok(HttpResponse::Unauthorized().json(
            serde_json::json!({"error": "Invalid username or password"})
        ));
    }
    
    // Get user details
    let user = db.get_user_by_username(&login_data.username)
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError("User not found after verification")
        })?;
    
    // Check if email is verified
    if !user.email_verified {
        return Ok(HttpResponse::Unauthorized().json(
            serde_json::json!({"error": "Please verify your email before logging in"})
        ));
    }
    
    // Store user ID in session
    session.insert("user_id", &user.id)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session error: {}", e)))?;
    session.insert("username", &user.username)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session error: {}", e)))?;
    session.insert("role", &user.role)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session error: {}", e)))?;
    
    // Set longer session for "remember me"
    if login_data.remember_me {
        session.renew();
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Login successful",
        "user_id": user.id,
        "username": user.username,
        "role": user.role,
        "email_verified": user.email_verified
    })))
}

// User logout
pub async fn logout(session: Session) -> Result<HttpResponse> {
    session.clear();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logout successful"
    })))
}

// Get current user info
pub async fn get_current_user(session: Session) -> Result<HttpResponse> {
    let user_id: Option<String> = session.get("user_id")?;
    let username: Option<String> = session.get("username")?;
    
    if let (Some(user_id), Some(username)) = (user_id, username) {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "user_id": user_id,
            "username": username,
            "is_authenticated": true
        })))
    } else {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "is_authenticated": false
        })))
    }
}

// Email verification endpoint
pub async fn verify_email(
    db: web::Data<Database>,
    session: Session,
    token: web::Path<String>,
) -> Result<HttpResponse> {
    if let Err(e) = db.verify_email_token(&token) {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": format!("Verification failed: {}", e)})
        ));
    }
    
    // Update session if user is logged in
    if let Ok(Some(_user_id)) = session.get::<String>("user_id") {
        session.insert("email_verified", &true)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session error: {}", e)))?;
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Email verified successfully"
    })))
}

// Password reset request
pub async fn request_password_reset(
    db: web::Data<Database>,
    request: web::Json<PasswordResetRequest>,
) -> Result<HttpResponse> {
    let user = match db.get_user_by_email(&request.email) {
        Ok(user) => user,
        Err(_) => {
            // Don't reveal if email exists
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "If the email exists, a reset link has been sent"
            })));
        }
    };
    
    let reset_token = db.create_password_reset_token(&user.id)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Token creation error: {}", e)))?;
    
    println!("🔐 Password reset token for {}: {}", user.email, reset_token);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "If the email exists, a reset link has been sent"
    })))
}

// Password reset confirmation
pub async fn confirm_password_reset(
    db: web::Data<Database>,
    confirm_data: web::Json<PasswordResetConfirm>,
) -> Result<HttpResponse> {
    // Validate new password strength
    if let Err(e) = Validation::validate_password_strength(&confirm_data.new_password) {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": e})
        ));
    }
    
    if let Err(e) = db.reset_password(&confirm_data.token, &confirm_data.new_password) {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": format!("Password reset failed: {}", e)})
        ));
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password reset successfully"
    })))
}

// User profile management
pub async fn get_profile(
    db: web::Data<Database>,
    session: Session,
) -> Result<HttpResponse> {
    let user_id = match get_user_id_from_session(&session) {
        Some(id) => id,
        None => {
            return Ok(HttpResponse::Unauthorized().json(
                serde_json::json!({"error": "Authentication required"})
            ));
        }
    };
    
    let user = db.get_user_by_id(&user_id)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?;
    
    let profile = serde_json::json!({
        "username": user.username,
        "email": user.email,
        "role": user.role,
        "email_verified": user.email_verified,
        "profile_data": user.profile_data,
        "created_at": user.created_at,
        "updated_at": user.updated_at
    });
    
    Ok(HttpResponse::Ok().json(profile))
}

pub async fn update_profile(
    db: web::Data<Database>,
    session: Session,
    profile_data: web::Json<ProfileUpdate>,
) -> Result<HttpResponse> {
    let user_id = match get_user_id_from_session(&session) {
        Some(id) => id,
        None => {
            return Ok(HttpResponse::Unauthorized().json(
                serde_json::json!({"error": "Authentication required"})
            ));
        }
    };
    
    if let Err(e) = db.update_user_profile(&user_id, &profile_data) {
        return Ok(HttpResponse::InternalServerError().json(
            serde_json::json!({"error": format!("Profile update failed: {}", e)})
        ));
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Profile updated successfully"
    })))
}

// Role-based access control helper
pub async fn require_role(session: &Session, required_role: &str) -> Result<(), HttpResponse> {
    let user_role: Option<String> = session.get("role").unwrap_or(None);
    
    match user_role {
        Some(role) if role == required_role || role == "admin" => Ok(()),
        Some(_) => Err(HttpResponse::Forbidden().json(
            serde_json::json!({"error": "Insufficient permissions"})
        )),
        None => Err(HttpResponse::Unauthorized().json(
            serde_json::json!({"error": "Authentication required"})
        )),
    }
}

// Admin-only endpoint example
pub async fn admin_dashboard(
    db: web::Data<Database>,
    session: Session,
) -> Result<HttpResponse> {
    if let Err(response) = require_role(&session, "admin").await {
        return Ok(response);
    }
    
    let stats = db.get_admin_stats()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?;
    
    Ok(HttpResponse::Ok().json(stats))
}

// API endpoint to get analysis (JSON) with database
pub async fn get_analysis(_data: web::Data<AppState>) -> Result<HttpResponse> {
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"error": format!("Database error: {}", e)})
            ));
        }
    };

    let _user_id = "demo_user_001";
    
    let sessions = match db.get_user_sessions(_user_id) {
        Ok(sessions) if !sessions.is_empty() => {
            println!("📁 Loaded {} sessions from database", sessions.len());
            sessions.into_iter().map(|db_session| {
                StudySession {
                    subject: db_session.subject,
                    hours_studied: db_session.hours_studied,
                    time_of_day: db_session.time_of_day,
                    understanding_score: db_session.understanding_score,
                    retention_score: db_session.retention_score,
                }
            }).collect()
        }
        _ => {
            println!("📁 No database sessions found, using CSV fallback");
            match StudySession::load_from_csv("data/study_sessions.csv") {
                Ok(sessions) if !sessions.is_empty() => {
                    for session in &sessions {
                        let mut db_session = StudySessionDb::from(session);
                        db_session.user_id = _user_id.to_string();
                        let _ = db.create_study_session(&db_session);
                    }
                    sessions
                }
                _ => {
                    return Ok(HttpResponse::BadRequest().json(
                        serde_json::json!({"error": "No study data available"})
                    ));
                }
            }
        }
    };

    let analyzer = SmartAnalyzer::new(sessions);
    let analysis = analyzer.generate_comprehensive_analysis(_user_id);

    let _ = db.save_analysis_result(_user_id, &analysis);

    Ok(HttpResponse::Ok().json(analysis))
}

// New endpoint to add study session
pub async fn add_study_session(
    _data: web::Data<AppState>,
    session: web::Json<StudySession>,
) -> Result<HttpResponse> {
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"error": format!("Database error: {}", e)})
            ));
        }
    };

    let user_id = "demo_user_001";
    let mut db_session = StudySessionDb::from(&session.into_inner());
    db_session.user_id = user_id.to_string();
    
    match db.create_study_session(&db_session) {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(
                serde_json::json!({"message": "Study session saved successfully"})
            ))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"error": format!("Failed to save session: {}", e)})
            ))
        }
    }
}

// New endpoint to get analysis history
pub async fn get_analysis_history(_data: web::Data<AppState>) -> Result<HttpResponse> {
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"error": format!("Database error: {}", e)})
            ));
        }
    };

    let user_id = "demo_user_001";
    
    match db.get_latest_analysis(user_id) {
        Ok(Some(analysis)) => {
            Ok(HttpResponse::Ok().json(analysis))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(
                serde_json::json!({"error": "No analysis history found"})
            ))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"error": format!("Database error: {}", e)})
            ))
        }
    }
}

// File upload endpoint
pub async fn upload_csv(
    _data: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse> {
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({"error": format!("Database error: {}", e)})
            ));
        }
    };

    let user_id = "demo_user_001";
    let mut uploaded_sessions = Vec::new();
    let mut file_content = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                return Ok(HttpResponse::BadRequest().json(
                    serde_json::json!({"error": format!("Upload error: {}", e)})
                ));
            }
        };

        if field.content_disposition().get_name() == Some("csvfile") {
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| {
                    actix_web::error::ErrorInternalServerError(
                        format!("File reading error: {}", e)
                    )
                })?;
                file_content.extend_from_slice(&data);
            }
        }
    }

    if file_content.is_empty() {
        return Ok(HttpResponse::BadRequest().json(
            serde_json::json!({"error": "No file uploaded"})
        ));
    }

    let csv_content = String::from_utf8(file_content)
        .map_err(|e| {
            actix_web::error::ErrorBadRequest(
                format!("Invalid file format: {}", e)
            )
        })?;

    match StudySession::load_from_csv_content(&csv_content) {
        Ok(sessions) if !sessions.is_empty() => {
            println!("📁 Processing {} sessions from uploaded CSV", sessions.len());
            
            for session in &sessions {
                let mut db_session = StudySessionDb::from(session);
                db_session.user_id = user_id.to_string();
                if let Err(e) = db.create_study_session(&db_session) {
                    eprintln!("Failed to save session: {}", e);
                } else {
                    uploaded_sessions.push(session.clone());
                }
            }

            let analyzer = SmartAnalyzer::new(uploaded_sessions);
            let analysis = analyzer.generate_comprehensive_analysis(user_id);

            let _ = db.save_analysis_result(user_id, &analysis);

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": format!("Successfully uploaded and processed {} study sessions", sessions.len()),
                "analysis": analysis
            })))
        }
        Ok(_) => {
            Ok(HttpResponse::BadRequest().json(
                serde_json::json!({"error": "Uploaded CSV file contains no valid data"})
            ))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(
                serde_json::json!({"error": format!("CSV processing error: {}", e)})
            ))
        }
    }
}

// Helper functions
fn get_user_id_from_session(session: &Session) -> Option<String> {
    session.get("user_id").unwrap_or(None)
}