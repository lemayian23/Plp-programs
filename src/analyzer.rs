use crate::models::{StudyAnalysis, StudyTrend, Recommendation};
use crate::data_processing::StudySession;
use std::collections::HashMap;

pub struct SmartAnalyzer {
    pub student_data: Vec<StudySession>,
}

impl SmartAnalyzer {
    pub fn new(sessions: Vec<StudySession>) -> Self {
        Self {
            student_data: sessions,
        }
    }
    
    pub fn generate_comprehensive_analysis(&self, student_id: &str) -> StudyAnalysis {
        println!("🎯 Generating smart analysis for student: {}", student_id);
        
        StudyAnalysis {
            student_id: student_id.to_string(),
            weekly_trend: self.calculate_weekly_trend(),
            subject_performance: self.analyze_subject_performance(),
            optimal_times: self.find_optimal_study_times(),
            predicted_scores: self.predict_future_scores(),
            recommendations: self.generate_recommendations(),
        }
    }
    
    fn calculate_weekly_trend(&self) -> StudyTrend {
        let total_hours: f64 = self.student_data.iter()
            .map(|s| s.hours_studied)
            .sum();
        
        let weekly_hours = total_hours / (self.student_data.len() as f64 / 7.0).max(1.0);
        
        // Calculate efficiency (retention per hour)
        let total_retention: f64 = self.student_data.iter()
            .map(|s| s.retention_score as f64)
            .sum();
        let efficiency_score = total_retention / total_hours.max(1.0);
        
        // Calculate consistency (standard deviation of daily hours)
        let daily_hours: Vec<f64> = self.group_by_day().into_iter()
            .map(|(_, hours)| hours)
            .collect();
        
        let avg_daily_hours: f64 = daily_hours.iter().sum::<f64>() / daily_hours.len() as f64;
        let variance: f64 = daily_hours.iter()
            .map(|h| (h - avg_daily_hours).powi(2))
            .sum::<f64>() / daily_hours.len() as f64;
        let consistency_score = 100.0 / (1.0 + variance.sqrt());
        
        StudyTrend {
            weekly_hours,
            efficiency_score,
            consistency_score: consistency_score.min(100.0),
            improvement_rate: self.calculate_improvement_rate(),
        }
    }
    
    fn analyze_subject_performance(&self) -> HashMap<String, f64> {
        let mut subject_scores = HashMap::new();
        let mut subject_counts = HashMap::new();
        
        for session in &self.student_data {
            let entry = subject_scores.entry(session.subject.clone())
                .or_insert(0.0);
            *entry += session.retention_score as f64;
            
            let count = subject_counts.entry(session.subject.clone())
                .or_insert(0);
            *count += 1;
        }
        
        // Calculate average per subject
        subject_scores.into_iter()
            .map(|(subject, total_score)| {
                let count = subject_counts[&subject] as f64;
                (subject, total_score / count)
            })
            .collect()
    }
    
    fn find_optimal_study_times(&self) -> Vec<String> {
        let mut time_performance = HashMap::new();
        let mut time_counts = HashMap::new();
        
        for session in &self.student_data {
            let entry = time_performance.entry(session.time_of_day.clone())
                .or_insert(0.0);
            *entry += session.retention_score as f64;
            
            let count = time_counts.entry(session.time_of_day.clone())
                .or_insert(0);
            *count += 1;
        }
        
        // Find top 2 performing times
        let mut times: Vec<(String, f64)> = time_performance.into_iter()
            .map(|(time, total_score)| {
                let count = time_counts[&time] as f64;
                (time, total_score / count)
            })
            .collect();
        
        times.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        times.into_iter()
            .take(2)
            .map(|(time, _)| time)
            .collect()
    }
    
    fn predict_future_scores(&self) -> HashMap<String, f64> {
        let subject_performance = self.analyze_subject_performance();
        
        // Simple prediction: current average + improvement potential
        subject_performance.into_iter()
            .map(|(subject, current_score)| {
                let predicted = current_score + 5.0; // Base improvement
                (subject, predicted.min(95.0)) // Cap at 95%
            })
            .collect()
    }
    
    fn analyze_understanding_vs_retention(&self) -> f64 {
        // Calculate correlation between understanding and retention
        let total_sessions = self.student_data.len() as f64;
        let understanding_sum: f64 = self.student_data.iter()
            .map(|s| s.understanding_score as f64)
            .sum();
        let retention_sum: f64 = self.student_data.iter()
            .map(|s| s.retention_score as f64)
            .sum();
        
        let avg_understanding = understanding_sum / total_sessions;
        let avg_retention = retention_sum / total_sessions;
        
        // Simple correlation: how close are understanding and retention?
        let correlation = 100.0 - (avg_understanding - avg_retention).abs();
        correlation.max(0.0)
    }
    
    fn generate_recommendations(&self) -> Vec<Recommendation> {
        let trend = self.calculate_weekly_trend();
        let understanding_correlation = self.analyze_understanding_vs_retention();
        let mut recommendations = Vec::new();
        
        // Timing recommendations
        if trend.weekly_hours < 10.0 {
            recommendations.push(Recommendation {
                category: "duration".to_string(),
                message: "Consider increasing study time to 10+ hours weekly for better results".to_string(),
                confidence: 0.8,
                impact_score: 7.5,
            });
        }
        
        // Efficiency recommendations
        if trend.efficiency_score < 30.0 {
            recommendations.push(Recommendation {
                category: "efficiency".to_string(),
                message: "Focus on active recall techniques to improve retention per study hour".to_string(),
                confidence: 0.7,
                impact_score: 8.0,
            });
        }
        
        // Consistency recommendations
        if trend.consistency_score < 70.0 {
            recommendations.push(Recommendation {
                category: "consistency".to_string(),
                message: "Try studying at consistent times each day to build better habits".to_string(),
                confidence: 0.9,
                impact_score: 6.5,
            });
        }
        
        // Understanding vs Retention recommendations
        if understanding_correlation < 80.0 {
            recommendations.push(Recommendation {
                category: "learning".to_string(),
                message: "Work on converting understanding to long-term retention through spaced repetition".to_string(),
                confidence: 0.75,
                impact_score: 7.0,
            });
        }
        
        recommendations
    }
    
    // Helper methods
    fn group_by_day(&self) -> HashMap<String, f64> {
        let mut daily_hours = HashMap::new();
        
        for session in &self.student_data {
            // Simplified: group by time_of_day for demo
            let entry = daily_hours.entry(session.time_of_day.clone())
                .or_insert(0.0);
            *entry += session.hours_studied;
        }
        
        daily_hours
    }
    
    fn calculate_improvement_rate(&self) -> f64 {
        // Simplified: calculate average improvement over sessions
        if self.student_data.len() < 2 {
            return 0.0;
        }
        
        let first_half_avg: f64 = self.student_data[..self.student_data.len()/2].iter()
            .map(|s| s.retention_score as f64)
            .sum::<f64>() / (self.student_data.len()/2) as f64;
        
        let second_half_avg: f64 = self.student_data[self.student_data.len()/2..].iter()
            .map(|s| s.retention_score as f64)
            .sum::<f64>() / (self.student_data.len()/2) as f64;
        
        ((second_half_avg - first_half_avg) / first_half_avg.max(1.0)) * 100.0
    }
}