use rusqlite::Connection;

pub fn initialize_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("study_planner.db")?;

    // Create users table with enhanced fields
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'student',
            email_verified BOOLEAN NOT NULL DEFAULT 0,
            profile_data TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create email verification tokens table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS email_verification_tokens (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            token TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;

    // Create password reset tokens table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_reset_tokens (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            token TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            used BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;

    // Create study_sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS study_sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            subject TEXT NOT NULL,
            hours_studied REAL NOT NULL,
            time_of_day TEXT NOT NULL,
            understanding_score INTEGER NOT NULL,
            retention_score INTEGER NOT NULL,
            session_date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;

    // Create analysis_results table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS analysis_results (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            analysis_data TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;

    println!("✅ Production database initialized successfully");
    Ok(())
}