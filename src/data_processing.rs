use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
pub struct StudySession {
    pub subject: String,
    pub hours_studied: f64,
    pub time_of_day: String,
    pub understanding_score: u32,
    pub retention_score: u32,
}

impl StudySession {
    pub fn load_from_csv(file_path: &str) -> Result<Vec<StudySession>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        
        let mut sessions = Vec::new();
        for result in rdr.deserialize() {
            let session: StudySession = result?;
            sessions.push(session);
        }
        
        Ok(sessions)
    }
    
    pub fn to_features(&self) -> Vec<f64> {
        // Convert session to feature vector for ML
        let time_feature = match self.time_of_day.as_str() {
            "morning" => 0.0,
            "afternoon" => 1.0,
            "evening" => 2.0,
            _ => 1.0,
        };
        
        vec![
            self.hours_studied,
            time_feature,
            self.understanding_score as f64,
        ]
    }
}