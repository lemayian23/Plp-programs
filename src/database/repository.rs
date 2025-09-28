use rusqlite::{Connection, Result, params};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Utc, Duration as ChronoDuration};
use rand::{distributions::Alphanumeric, Rng};
use crate::database::models::{User, StudySessionDb, ProfileUpdate};
use crate::models::StudyAnalysis;
use serde_json;
use chrono::DateTime;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("study_planner.db")?;
        Ok(Self { conn })
    }

    // Enhanced user operations
    pub fn create_user(&self, user: &User) -> Result<()> {
        self.conn.execute(
            "INSERT INTO users (id, username, email, password_hash, role, email_verified, profile_data, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                user.id,
                user.username,
                user.email,
                user.password_hash,
                user.role,
                user.email_verified,
                user.profile_data,
                Self::datetime_to_string(&user.created_at),
                Self::datetime_to_string(&user.updated_at)
            ],
        )?;
        Ok(())
    }

    pub fn get_user_by_id(&self, user_id: &str) -> Result<User> {
        self.conn.query_row(
            "SELECT id, username, email, password_hash, role, email_verified, profile_data, created_at, updated_at FROM users WHERE id = ?1",
            params![user_id],
            |row| {
                let created_at_str: String = row.get(7)?;
                let updated_at_str: String = row.get(8)?;
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    role: row.get(4)?,
                    email_verified: row.get(5)?,
                    profile_data: row.get(6)?,
                    created_at: Self::string_to_datetime(&created_at_str)?,
                    updated_at: Self::string_to_datetime(&updated_at_str)?,
                })
            },
        )
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<User> {
        self.conn.query_row(
            "SELECT id, username, email, password_hash, role, email_verified, profile_data, created_at, updated_at FROM users WHERE username = ?1",
            params![username],
            |row| {
                let created_at_str: String = row.get(7)?;
                let updated_at_str: String = row.get(8)?;
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    role: row.get(4)?,
                    email_verified: row.get(5)?,
                    profile_data: row.get(6)?,
                    created_at: Self::string_to_datetime(&created_at_str)?,
                    updated_at: Self::string_to_datetime(&updated_at_str)?,
                })
            },
        )
    }

    pub fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.conn.query_row(
            "SELECT id, username, email, password_hash, role, email_verified, profile_data, created_at, updated_at FROM users WHERE email = ?1",
            params![email],
            |row| {
                let created_at_str: String = row.get(7)?;
                let updated_at_str: String = row.get(8)?;
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    role: row.get(4)?,
                    email_verified: row.get(5)?,
                    profile_data: row.get(6)?,
                    created_at: Self::string_to_datetime(&created_at_str)?,
                    updated_at: Self::string_to_datetime(&updated_at_str)?,
                })
            },
        )
    }

    // Password verification
    pub fn verify_password(&self, username: &str, password: &str) -> Result<bool> {
        match self.get_user_by_username(username) {
            Ok(user) => {
                Ok(bcrypt::verify(password, &user.password_hash).unwrap_or(false))
            }
            Err(_) => Ok(false),
        }
    }

    // Email verification
    pub fn create_email_verification_token(&self, user_id: &str) -> Result<String> {
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        
        let expires_at = Utc::now() + ChronoDuration::hours(24);
        
        self.conn.execute(
            "INSERT INTO email_verification_tokens (id, user_id, token, expires_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                uuid::Uuid::new_v4().to_string(),
                user_id,
                &token,
                Self::datetime_to_string(&expires_at),
                Self::datetime_to_string(&Utc::now())
            ],
        )?;
        
        Ok(token)
    }

    pub fn verify_email_token(&self, token: &str) -> Result<()> {
        let now = Utc::now();
        
        // Find valid token
        let user_id: String = self.conn.query_row(
            "SELECT user_id FROM email_verification_tokens WHERE token = ?1 AND expires_at > ?2",
            params![token, Self::datetime_to_string(&now)],
            |row| row.get(0),
        )?;
        
        // Update user email verification status
        self.conn.execute(
            "UPDATE users SET email_verified = 1, updated_at = ?1 WHERE id = ?2",
            params![Self::datetime_to_string(&now), user_id],
        )?;
        
        // Delete used token
        self.conn.execute(
            "DELETE FROM email_verification_tokens WHERE token = ?1",
            params![token],
        )?;
        
        Ok(())
    }

    // Password reset functionality
    pub fn create_password_reset_token(&self, user_id: &str) -> Result<String> {
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        
        let expires_at = Utc::now() + ChronoDuration::hours(1);
        
        self.conn.execute(
            "INSERT INTO password_reset_tokens (id, user_id, token, expires_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                uuid::Uuid::new_v4().to_string(),
                user_id,
                &token,
                Self::datetime_to_string(&expires_at),
                Self::datetime_to_string(&Utc::now())
            ],
        )?;
        
        Ok(token)
    }

    pub fn reset_password(&self, token: &str, new_password: &str) -> Result<()> {
        let now = Utc::now();
        
        // Find valid token
        let (user_id, token_id): (String, String) = self.conn.query_row(
            "SELECT user_id, id FROM password_reset_tokens WHERE token = ?1 AND expires_at > ?2 AND used = 0",
            params![token, Self::datetime_to_string(&now)],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        
        // Hash new password
        let password_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        // Update user password
        self.conn.execute(
            "UPDATE users SET password_hash = ?1, updated_at = ?2 WHERE id = ?3",
            params![password_hash, Self::datetime_to_string(&now), user_id],
        )?;
        
        // Mark token as used
        self.conn.execute(
            "UPDATE password_reset_tokens SET used = 1 WHERE id = ?1",
            params![token_id],
        )?;
        
        Ok(())
    }

    // Profile management
    pub fn update_user_profile(&self, user_id: &str, profile: &ProfileUpdate) -> Result<()> {
        let profile_json = serde_json::to_string(profile)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        self.conn.execute(
            "UPDATE users SET profile_data = ?1, updated_at = ?2 WHERE id = ?3",
            params![profile_json, Self::datetime_to_string(&Utc::now()), user_id],
        )?;
        
        Ok(())
    }

    // Admin functionality
    pub fn get_admin_stats(&self) -> Result<serde_json::Value> {
        let user_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM users",
            [],
            |row| row.get(0),
        )?;
        
        let session_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM study_sessions",
            [],
            |row| row.get(0),
        )?;
        
        let analysis_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM analysis_results",
            [],
            |row| row.get(0),
        )?;
        
        Ok(serde_json::json!({
            "total_users": user_count,
            "total_sessions": session_count,
            "total_analyses": analysis_count,
            "server_time": Self::datetime_to_string(&Utc::now())
        }))
    }

    // Study session operations
    pub fn create_study_session(&self, session: &StudySessionDb) -> Result<()> {
        self.conn.execute(
            "INSERT INTO study_sessions (id, user_id, subject, hours_studied, time_of_day, understanding_score, retention_score, session_date, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                session.id,
                session.user_id,
                session.subject,
                session.hours_studied,
                session.time_of_day,
                session.understanding_score,
                session.retention_score,
                Self::datetime_to_string(&session.session_date),
                Self::datetime_to_string(&session.created_at)
            ],
        )?;
        Ok(())
    }

    pub fn get_user_sessions(&self, user_id: &str) -> Result<Vec<StudySessionDb>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, user_id, subject, hours_studied, time_of_day, understanding_score, retention_score, session_date, created_at FROM study_sessions WHERE user_id = ?1 ORDER BY session_date DESC"
        )?;
        
        let session_iter = stmt.query_map(params![user_id], |row| {
            let session_date_str: String = row.get(7)?;
            let created_at_str: String = row.get(8)?;
            
            Ok(StudySessionDb {
                id: row.get(0)?,
                user_id: row.get(1)?,
                subject: row.get(2)?,
                hours_studied: row.get(3)?,
                time_of_day: row.get(4)?,
                understanding_score: row.get(5)?,
                retention_score: row.get(6)?,
                session_date: Self::string_to_datetime(&session_date_str)?,
                created_at: Self::string_to_datetime(&created_at_str)?,
            })
        })?;

        let mut sessions = Vec::new();
        for session in session_iter {
            sessions.push(session?);
        }
        Ok(sessions)
    }

    // Analysis results operations
    pub fn save_analysis_result(&self, user_id: &str, analysis: &StudyAnalysis) -> Result<()> {
        let analysis_json = serde_json::to_string(analysis)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        self.conn.execute(
            "INSERT INTO analysis_results (id, user_id, analysis_data, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                uuid::Uuid::new_v4().to_string(), 
                user_id, 
                analysis_json, 
                Self::datetime_to_string(&Utc::now())
            ],
        )?;
        Ok(())
    }

    pub fn get_latest_analysis(&self, user_id: &str) -> Result<Option<StudyAnalysis>> {
        let mut stmt = self.conn.prepare(
            "SELECT analysis_data FROM analysis_results WHERE user_id = ?1 ORDER BY created_at DESC LIMIT 1"
        )?;
        
        let mut rows = stmt.query(params![user_id])?;
        if let Some(row) = rows.next()? {
            let analysis_json: String = row.get(0)?;
            let analysis: StudyAnalysis = serde_json::from_str(&analysis_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(Some(analysis))
        } else {
            Ok(None)
        }
    }

    // Helper functions for datetime conversion
    fn datetime_to_string(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }

    fn string_to_datetime(s: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
    }
}