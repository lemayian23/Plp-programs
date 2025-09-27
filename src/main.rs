mod data_processing;
mod ml_model;
mod study_planner;

use crate::data_processing::StudySession;
use crate::ml_model::StudyPredictor;
use crate::study_planner::StudyPlanner;

fn main() {
    println!("🎯 Smart Study Planner - ML Powered");
    println!("====================================\n");
    
    // Load and analyze study data
    let sessions = match StudySession::load_from_csv("data/study_sessions.csv") {
        Ok(sessions) if !sessions.is_empty() => {
            println!("✅ Loaded {} study sessions", sessions.len());
            sessions
        }
        Ok(_) => {
            println!("❌ No data found in CSV file");
            return;
        }
        Err(e) => {
            println!("❌ Error loading data: {}", e);
            println!("💡 Check if data/study_sessions.csv exists and has correct format");
            return;
        }
    };
    
    // Analyze patterns
    analyze_study_patterns(&sessions);
    
    // Train ML model
    let predictor = StudyPredictor::train(&sessions);
    
    // Generate optimal study plan
    let planner = StudyPlanner::new(predictor);
    let plan = planner.generate_weekly_plan();
    
    println!("\n📅 YOUR OPTIMAL STUDY PLAN:");
    println!("{}", plan);
}

fn analyze_study_patterns(sessions: &[StudySession]) {
    println!("\n📊 STUDY PATTERN ANALYSIS:");
    
    let avg_understanding: f64 = sessions.iter()
        .map(|s| s.understanding_score as f64)
        .sum::<f64>() / sessions.len() as f64;
    
    let avg_retention: f64 = sessions.iter()
        .map(|s| s.retention_score as f64)
        .sum::<f64>() / sessions.len() as f64;
    
    println!("Average Understanding Score: {:.1}%", avg_understanding);
    println!("Average Retention Score: {:.1}%", avg_retention);
    
    // Find best time for studying
    let mut morning_score = 0;
    let mut afternoon_score = 0;
    let mut evening_score = 0;
    let mut morning_count = 0;
    let mut afternoon_count = 0;
    let mut evening_count = 0;
    
    for session in sessions {
        match session.time_of_day.as_str() {
            "morning" => {
                morning_score += session.understanding_score;
                morning_count += 1;
            }
            "afternoon" => {
                afternoon_score += session.understanding_score;
                afternoon_count += 1;
            }
            "evening" => {
                evening_score += session.understanding_score;
                evening_count += 1;
            }
            _ => {}
        }
    }
    
    if morning_count > 0 {
        println!("🌅 Morning sessions: {:.1}% avg score", morning_score as f64 / morning_count as f64);
    }
    if afternoon_count > 0 {
        println!("☀️ Afternoon sessions: {:.1}% avg score", afternoon_score as f64 / afternoon_count as f64);
    }
    if evening_count > 0 {
        println!("🌙 Evening sessions: {:.1}% avg score", evening_score as f64 / evening_count as f64);
    }
}