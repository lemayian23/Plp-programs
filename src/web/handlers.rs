use actix_web::{web, HttpResponse, Result};
use crate::analyzer::SmartAnalyzer;
use crate::data_processing::StudySession;
use crate::models::StudyAnalysis;
use std::sync::Mutex;

// App state to share between requests
pub struct AppState {
    pub analyzer: Mutex<Option<SmartAnalyzer>>,
}

// Home page handler
pub async fn index() -> Result<HttpResponse> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Smart Study Planner</title>
        <style>
            body { 
                font-family: Arial, sans-serif; 
                max-width: 800px; 
                margin: 0 auto; 
                padding: 20px; 
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                color: white;
            }
            .container { 
                background: rgba(255,255,255,0.1); 
                padding: 30px; 
                border-radius: 15px; 
                backdrop-filter: blur(10px);
            }
            h1 { text-align: center; margin-bottom: 30px; }
            .form-group { margin-bottom: 20px; }
            label { display: block; margin-bottom: 5px; font-weight: bold; }
            input, textarea, button { 
                width: 100%; 
                padding: 10px; 
                border: none; 
                border-radius: 5px; 
                margin-bottom: 10px;
            }
            button { 
                background: #4CAF50; 
                color: white; 
                cursor: pointer; 
                font-size: 16px;
            }
            button:hover { background: #45a049; }
            .result { 
                background: rgba(255,255,255,0.2); 
                padding: 20px; 
                border-radius: 10px; 
                margin-top: 20px;
            }
            .recommendation { 
                background: rgba(255,255,255,0.3); 
                padding: 10px; 
                margin: 10px 0; 
                border-radius: 5px;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>🧠 Smart Study Planner</h1>
            <p>Upload your study data CSV file to get AI-powered recommendations!</p>
            
            <form id="uploadForm" enctype="multipart/form-data">
                <div class="form-group">
                    <label for="csvFile">Upload Study Data CSV:</label>
                    <input type="file" id="csvFile" name="csvFile" accept=".csv" required>
                </div>
                <button type="submit">Analyze Study Patterns</button>
            </form>
            
            <div id="results" style="display: none;">
                <h2>📊 Analysis Results</h2>
                <div id="analysisContent"></div>
            </div>
        </div>

        <script>
            document.getElementById('uploadForm').addEventListener('submit', async (e) => {
                e.preventDefault();
                const fileInput = document.getElementById('csvFile');
                const formData = new FormData();
                formData.append('csvFile', fileInput.files[0]);

                try {
                    const response = await fetch('/analyze', {
                        method: 'POST',
                        body: formData
                    });
                    
                    const result = await response.json();
                    displayResults(result);
                } catch (error) {
                    alert('Error analyzing data: ' + error);
                }
            });

            function displayResults(data) {
                const resultsDiv = document.getElementById('results');
                const contentDiv = document.getElementById('analysisContent');
                
                let html = `
                    <div class="result">
                        <h3>🎯 Student: ${data.student_id}</h3>
                        
                        <h4>📈 Weekly Trends</h4>
                        <p><strong>Hours per week:</strong> ${data.weekly_trend.weekly_hours.toFixed(1)}h</p>
                        <p><strong>Efficiency score:</strong> ${data.weekly_trend.efficiency_score.toFixed(1)}/100</p>
                        <p><strong>Consistency:</strong> ${data.weekly_trend.consistency_score.toFixed(1)}%</p>
                        <p><strong>Improvement rate:</strong> ${data.weekly_trend.improvement_rate.toFixed(1)}%</p>
                        
                        <h4>📚 Subject Performance</h4>
                        ${Object.entries(data.subject_performance).map(([subject, score]) => 
                            `<p><strong>${subject}:</strong> ${score.toFixed(1)}%</p>`
                        ).join('')}
                        
                        <h4>💡 AI Recommendations</h4>
                        ${data.recommendations.map(rec => 
                            `<div class="recommendation">
                                <strong>${rec.category.toUpperCase()}:</strong> ${rec.message}<br>
                                <small>Confidence: ${(rec.confidence * 100).toFixed(0)}%, Impact: ${rec.impact_score}/10</small>
                            </div>`
                        ).join('')}
                    </div>
                `;
                
                contentDiv.innerHTML = html;
                resultsDiv.style.display = 'block';
            }
        </script>
    </body>
    </html>
    "#;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

// CSV analysis handler
pub async fn analyze_csv(
    data: web::Data<AppState>,
    payload: web::Payload,
) -> Result<HttpResponse> {
    // For now, we'll use the sample data
    // In a real implementation, you'd parse the uploaded CSV
    
    let sessions = match crate::data_processing::StudySession::load_from_csv("data/study_sessions.csv") {
        Ok(sessions) if !sessions.is_empty() => sessions,
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!({"error": "Failed to load or analyze data"})
            ));
        }
    };

    let analyzer = SmartAnalyzer::new(sessions);
    let analysis = analyzer.generate_comprehensive_analysis("web_user_001");
    
    // Store analyzer in app state for future use
    if let Ok(mut analyzer_state) = data.analyzer.lock() {
        *analyzer_state = Some(analyzer);
    }

    Ok(HttpResponse::Ok().json(analysis))
}

// API endpoint to get analysis (JSON)
pub async fn get_analysis(data: web::Data<AppState>) -> Result<HttpResponse> {
    let sessions = match crate::data_processing::StudySession::load_from_csv("data/study_sessions.csv") {
        Ok(sessions) if !sessions.is_empty() => sessions,
        _ => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!({"error": "No data available"})
            ));
        }
    };

    let analyzer = SmartAnalyzer::new(sessions);
    let analysis = analyzer.generate_comprehensive_analysis("api_user_001");

    Ok(HttpResponse::Ok().json(analysis))
}