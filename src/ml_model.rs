use crate::data_processing::StudySession;

pub struct StudyPredictor {
    avg_retention: f64,
    best_time: String,
}

impl StudyPredictor {
    pub fn train(sessions: &[StudySession]) -> Self {
        println!("\n🤖 ANALYZING STUDY PATTERNS...");
        
        // Calculate average retention
        let total_retention: f64 = sessions.iter()
            .map(|s| s.retention_score as f64)
            .sum();
        let avg_retention = total_retention / sessions.len() as f64;
        
        // Find best time of day
        let mut time_scores = std::collections::HashMap::new();
        let mut time_counts = std::collections::HashMap::new();
        
        for session in sessions {
            let entry = time_scores.entry(&session.time_of_day).or_insert(0.0);
            *entry += session.retention_score as f64;
            
            let count = time_counts.entry(&session.time_of_day).or_insert(0);
            *count += 1;
        }
        
        let best_time = time_scores.iter()
            .max_by_key(|(_, &score)| score as i32)
            .map(|(time, _)| time.to_string())
            .unwrap_or_else(|| "morning".to_string());
        
        println!("✅ Analysis complete!");
        println!("   Average retention: {:.1}%", avg_retention);
        println!("   Best study time: {}", best_time);
        
        StudyPredictor {
            avg_retention,
            best_time,
        }
    }
    
    pub fn predict_retention(&self, hours: f64, time_of_day: &str, understanding: u32) -> f64 {
        let mut base_score = self.avg_retention;
        
        // Adjust based on hours studied (simple heuristic)
        if hours > 2.0 {
            base_score += 10.0; // Longer study sessions help
        } else if hours < 1.0 {
            base_score -= 5.0; // Very short sessions hurt
        }
        
        // Adjust based on time of day
        if time_of_day == self.best_time {
            base_score += 5.0;
        }
        
        // Adjust based on understanding
        base_score += (understanding as f64 - 70.0) * 0.2;
        
        // Clamp between 0-100%
        base_score.max(30.0).min(95.0)
    }
}