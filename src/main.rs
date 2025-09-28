mod data_processing;
mod ml_model;
mod models;
mod analyzer;

use crate::data_processing::StudySession;
use crate::analyzer::SmartAnalyzer;
use crate::models::StudyAnalysis;

fn main() {
    println!("🧠 SMART STUDY RECOMMENDER SYSTEM");
    println!("==================================\n");
    
    // Load study data
    let sessions = match StudySession::load_from_csv("data/study_sessions.csv") {
        Ok(sessions) if !sessions.is_empty() => {
            println!("✅ Loaded {} study sessions", sessions.len());
            sessions
        }
        Ok(_) => {
            println!("❌ No data found - using sample data");
            create_sample_data();
            return;
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            return;
        }
    };
    
    // Generate comprehensive analysis
    let analyzer = SmartAnalyzer::new(sessions);
    let analysis = analyzer.generate_comprehensive_analysis("student_001");
    
    // Display results
    display_analysis_results(&analysis);
}

fn display_analysis_results(analysis: &StudyAnalysis) {
    println!("\n📊 COMPREHENSIVE STUDY ANALYSIS");
    println!("================================");
    
    println!("\n🎯 STUDENT: {}", analysis.student_id);
    
    println!("\n📈 WEEKLY TRENDS:");
    println!("  Hours per week: {:.1}h", analysis.weekly_trend.weekly_hours);
    println!("  Efficiency score: {:.1}/100", analysis.weekly_trend.efficiency_score);
    println!("  Consistency: {:.1}%", analysis.weekly_trend.consistency_score);
    println!("  Improvement rate: {:.1}%", analysis.weekly_trend.improvement_rate);
    
    println!("\n📚 SUBJECT PERFORMANCE:");
    for (subject, score) in &analysis.subject_performance {
        println!("  {}: {:.1}%", subject, score);
    }
    
    println!("\n🕐 OPTIMAL STUDY TIMES:");
    for time in &analysis.optimal_times {
        println!("  {}", time);
    }
    
    println!("\n🔮 PREDICTED SCORES (with improvements):");
    for (subject, score) in &analysis.predicted_scores {
        println!("  {}: {:.1}%", subject, score);
    }
    
    println!("\n💡 AI RECOMMENDATIONS:");
    for (i, rec) in analysis.recommendations.iter().enumerate() {
        println!("  {}. [{}] {} (Confidence: {:.0}%, Impact: {}/10)", 
                 i + 1, rec.category.to_uppercase(), rec.message, 
                 rec.confidence * 100.0, rec.impact_score);
    }
    
    println!("\n🚀 NEXT STEPS:");
    if let Some(best_rec) = analysis.recommendations.iter()
        .max_by(|a, b| a.impact_score.partial_cmp(&b.impact_score).unwrap()) {
        println!("  Priority: {}", best_rec.message);
    }
    
    println!("\n🎊 ANALYSIS COMPLETE!");
    println!("Your ML system analyzed {} data points and generated personalized recommendations.", 
             analysis.subject_performance.len() * 4);
}

fn create_sample_data() {
    use std::fs::File;
    use std::io::Write;
    
    println!("📝 Creating sample data file...");
    
    // Create data directory if it doesn't exist
    std::fs::create_dir_all("data").ok();
    
    let sample_data = "subject,hours_studied,time_of_day,understanding_score,retention_score
mathematics,2.0,morning,85,90
physics,1.5,afternoon,70,65
programming,3.0,evening,90,80
history,1.0,morning,60,75
english,1.5,evening,75,70
chemistry,2.5,morning,80,85
mathematics,1.5,afternoon,75,70
programming,2.0,evening,85,80
physics,2.0,morning,80,85
history,1.0,afternoon,65,60";
    
    if let Ok(mut file) = File::create("data/study_sessions.csv") {
        file.write_all(sample_data.as_bytes()).ok();
        println!("✅ Sample data created at data/study_sessions.csv");
        println!("💡 Run the program again to analyze the data");
    } else {
        println!("❌ Failed to create sample data file");
    }
}