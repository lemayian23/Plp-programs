use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
pub struct StudySession {
    pub subject: String,
    pub hours_studied: f64,
    pub time_of_day: String,
    pub understanding_score: u32,
    pub retention_score: u32,  // Make sure this matches CSV exactly
}

impl StudySession {
    pub fn load_from_csv(file_path: &str) -> Result<Vec<StudySession>, Box<dyn Error>> {
        println!("📁 Loading data from: {}", file_path);
        
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        
        let mut sessions = Vec::new();
        for (i, result) in rdr.deserialize().enumerate() {
            match result {
                Ok(session) => {
                    sessions.push(session);
                    println!("✅ Loaded record {}", i + 1);
                }
                Err(e) => {
                    println!("❌ Error parsing record {}: {}", i + 1, e);
                    return Err(Box::new(e));
                }
            }
        }
        
        if sessions.is_empty() {
            println!("⚠️  No data loaded - file might be empty or malformed");
        }
        
        Ok(sessions)
    }
    
    // Remove the unused to_features method for now
}