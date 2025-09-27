use crate::ml_model::StudyPredictor;

pub struct StudyPlanner {
    predictor: StudyPredictor,
}

impl StudyPlanner {
    pub fn new(predictor: StudyPredictor) -> Self {
        StudyPlanner { predictor }
    }
    
    pub fn generate_weekly_plan(&self) -> String {
        let subjects = vec!["Math", "Physics", "Programming", "History", "English"];
        let times = vec!["morning", "afternoon", "evening"];
        
        let mut plan = String::new();
        plan.push_str("Optimized Weekly Study Plan:\n");
        plan.push_str("============================\n");
        
        for day in 1..=7 {
            plan.push_str(&format!("\n📅 Day {}:\n", day));
            
            // Simple scheduling logic - real ML would optimize this
            for (i, subject) in subjects.iter().enumerate() {
                let time_index = (day + i) % times.len();
                let time = times[time_index];
                let hours = 1.5 + (i as f64 * 0.5); // Vary hours per subject
                
                let predicted_score = self.predictor.predict_retention(
                    hours, time, 75 // Assume 75% understanding
                );
                
                plan.push_str(&format!(
                    "   {}: {} hours in {} (predicted retention: {:.1}%)\n",
                    subject, hours, time, predicted_score
                ));
            }
        }
        
        plan
    }
}