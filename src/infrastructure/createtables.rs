use tokio_postgres::Client;

pub async fn create_tables(client: &Client) -> Result<(), tokio_postgres::Error> {
    // RUN SESSIONS
    client
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS runsession (
            id TEXT PRIMARY KEY,
            date TIMESTAMPTZ NOT NULL,
            distance FLOAT NOT NULL,
            duration_minutes INTEGER NOT NULL,
            avg_heart_rate INTEGER,
            rpe FLOAT,
            created_at TIMESTAMPTZ,
            updated_at TIMESTAMPTZ
                );
            "#,
        )
        .await?;

    // LAP SPLITS
    client
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS lapsplits (
                id TEXT PRIMARY KEY,
                run_session_id TEXT NOT NULL,
                lap_number INTEGER NOT NULL,
                pace_seconds_per_km INTEGER,
                heart_rate INTEGER,
                created_at TIMESTAMPTZ,
                updated_at TIMESTAMPTZ,

                FOREIGN KEY (run_session_id)
                    REFERENCES runsession(id)
                    ON DELETE CASCADE
            );
            "#,
        )
        .await?;

    // DAILY WELLNESS
    client
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS dailywellness (
                id TEXT PRIMARY KEY,
                date TIMESTAMPTZ NOT NULL,
                sleep_hours FLOAT,
                stress_level INTEGER,
                energy_level INTEGER,
                mood INTEGER,
                created_at TIMESTAMPTZ,
                updated_at TIMESTAMPTZ
            );
            "#,
        )
        .await?;

    // TRAINING MARKERS
    client
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS dailywellness (
            id TEXT PRIMARY KEY,
            sleep_hours FLOAT,
            stress_level INTEGER,
            energy_level INTEGER,
            mood INTEGER,
            created_at TIMESTAMPTZ,
            updated_at TIMESTAMPTZ
);
            "#,
        )
        .await?;

    Ok(())
}