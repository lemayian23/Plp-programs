use lazy_static::lazy_static;
use regex::Regex;
use bcrypt::{hash, DEFAULT_COST};

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    static ref USERNAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_-]{3,20}$"
    ).unwrap();
}

pub struct Validation;

impl Validation {
    pub fn validate_email(email: &str) -> Result<(), String> {
        if EMAIL_REGEX.is_match(email) {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn validate_username(username: &str) -> Result<(), String> {
        if USERNAME_REGEX.is_match(username) {
            Ok(())
        } else {
            Err("Username must be 3-20 characters and contain only letters, numbers, underscores, or hyphens".to_string())
        }
    }

    pub fn validate_password_strength(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
        
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_digit(10));
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        
        let score = [has_upper, has_lower, has_digit, has_special]
            .iter()
            .filter(|&&x| x)
            .count();
        
        if score >= 3 {
            Ok(())
        } else {
            Err("Password must contain at least 3 of: uppercase, lowercase, digits, special characters".to_string())
        }
    }

    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }
}