use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyAnalysis {
    pub student_id: String,
    pub weekly_trend: StudyTrend,
    pub subject_performance: HashMap<String, f64>,
    pub optimal_times: Vec<String>,
    pub predicted_scores: HashMap<String, f64>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyTrend {
    pub weekly_hours: f64,
    pub efficiency_score: f64,
    pub consistency_score: f64,
    pub improvement_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: String,  // "timing", "duration", "subject", "break"
    pub message: String,
    pub confidence: f64,
    pub impact_score: f64,
}