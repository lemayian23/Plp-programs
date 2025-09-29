use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String, // 'student', 'teacher', 'admin'
    pub email_verified: bool,
    pub profile_data: Option<String>, // JSON string for profile
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub remember_me: bool, // NEW: Remember me functionality
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub message: String,
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileUpdate {
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub preferences: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetConfirm {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudySessionDb {
    pub id: String,
    pub user_id: String,
    pub subject: String,
    pub hours_studied: f64,
    pub time_of_day: String,
    pub understanding_score: u32,
    pub retention_score: u32,
    pub session_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub id: String,
    pub user_id: String,
    pub analysis_data: String,
    pub created_at: DateTime<Utc>,
}

// Conversion from StudySession to StudySessionDb
impl From<&crate::data_processing::StudySession> for StudySessionDb {
    fn from(session: &crate::data_processing::StudySession) -> Self {
        StudySessionDb {
            id: Uuid::new_v4().to_string(),
            user_id: "default_user".to_string(),
            subject: session.subject.clone(),
            hours_studied: session.hours_studied,
            time_of_day: session.time_of_day.clone(),
            understanding_score: session.understanding_score,
            retention_score: session.retention_score,
            session_date: Utc::now(),
            created_at: Utc::now(),
        }
    }
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            role: "student".to_string(),
            email_verified: false,
            profile_data: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    pub fn is_teacher(&self) -> bool {
        self.role == "teacher"
    }

    pub fn is_student(&self) -> bool {
        self.role == "student"
    }
}