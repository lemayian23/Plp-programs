use linfa::traits::Fit;
use linfa_linear::LinearRegression;
use ndarray::{Array, Array1, Array2};
use crate::data_processing::StudySession;

pub struct StudyPredictor {
    model: LinearRegression<f64, usize>,
}

impl StudyPredictor {
    pub fn train(sessions: &[StudySession]) -> Self {
        println!("\n🤖 TRAINING ML MODEL...");
        
        // Prepare features and targets
        let mut features = Vec::new();
        let mut targets = Vec::new();
        
        for session in sessions {
            features.push(session.to_features());
            targets.push(session.retention_score as f64);
        }
        
        // Convert to arrays for linfa
        let n_samples = sessions.len();
        let n_features = features[0].len();
        
        let feature_array = Array2::from_shape_vec(
            (n_samples, n_features),
            features.into_iter().flatten().collect()
        ).unwrap();
        
        let target_array = Array1::from_vec(targets);
        
        // Train linear regression model
        let dataset = linfa::Dataset::new(feature_array, target_array);
        let model = LinearRegression::new().fit(&dataset).unwrap();
        
        println!("✅ Model trained successfully!");
        
        StudyPredictor { model }
    }
    
    pub fn predict_retention(&self, hours: f64, time_of_day: &str, understanding: u32) -> f64 {
        let time_feature = match time_of_day {
            "morning" => 0.0,
            "afternoon" => 1.0,
            "evening" => 2.0,
            _ => 1.0,
        };
        
        let features = vec![hours, time_feature, understanding as f64];
        let feature_array = Array2::from_shape_vec((1, 3), features).unwrap();
        
        let prediction = self.model.predict(&feature_array);
        prediction[0]
    }
}