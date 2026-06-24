use std::error::Error;
use chrono::{DateTime, Utc};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

pub async fn load_sample_data(
    client: &tokio_postgres::Client,
    filepath: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Loading sample data...");

    let domain_model_names = get_domain_model_names()?;

    for domain_model_name in domain_model_names {
        let domain_model_name = domain_model_name.to_lowercase();

        if filepath.to_lowercase().contains(&domain_model_name) {
            println!(
                "Loading sample data for domain model: {}",
                domain_model_name
            );

            insert_data_into_table(
                client,
                &domain_model_name,
                filepath,
            )
            .await?;

            return Ok(());
        }
    }

    println!("No matching domain model found for {}", filepath);

    Ok(())
}

fn get_domain_model_names() -> Result<Vec<String>, Box<dyn Error>> {
    let mut domain_model_names = Vec::new();

    let domain_folder =
        "/Users/arnoslabbinck/Documents/rust/vitalMile/src/domain";

    let paths = std::fs::read_dir(domain_folder)?;

    for path in paths {
        let path = path?.path();

        if path.is_file() {
            if let Some(filename) =
                path.file_name().and_then(|f| f.to_str())
            {
                if filename.ends_with(".rs") {
                    let model_name =
                        filename.trim_end_matches(".rs").to_string();

                    domain_model_names.push(model_name);
                }
            }
        }
    }

    Ok(domain_model_names)
}


async fn insert_data_into_table(
    client: &tokio_postgres::Client,
    table_name: &str,
    filepath: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(filepath)?;

    let headers = rdr.headers()?.clone();

    let columns: Vec<String> =
        headers.iter().map(String::from).collect();

    let schema_rows = client
        .query(
            "
            SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_name = $1
            ORDER BY ordinal_position
            ",
            &[&table_name],
        )
        .await?;

    let mut column_types = std::collections::HashMap::new();

    for row in schema_rows {
        let name: String = row.get(0);
        let data_type: String = row.get(1);

        column_types.insert(name, data_type);
    }

    let column_list = columns.join(", ");

    let placeholders = (1..=columns.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        column_list,
        placeholders
    );

    for record in rdr.records() {
        let record = record?;

        let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();

        for (value, column_name) in
            record.iter().zip(columns.iter())
        {
            let data_type = column_types
                .get(column_name)
                .map(String::as_str)
                .unwrap_or("text");

            match data_type {
                "integer" => {
                    params.push(Box::new(value.parse::<i32>()?));
                }

                "bigint" => {
                    params.push(Box::new(value.parse::<i64>()?));
                }

                "double precision" => {
                    params.push(Box::new(value.parse::<f64>()?));
                }

                "real" => {
                    params.push(Box::new(value.parse::<f32>()?));
                }

                "boolean" => {
                    params.push(Box::new(value.parse::<bool>()?));
                }

                "uuid" => {
                    params.push(Box::new(
                        Uuid::parse_str(value)?
                    ));
                }

                "timestamp with time zone" => {
                    let dt = DateTime::parse_from_rfc3339(value)?
                        .with_timezone(&Utc);

                    params.push(Box::new(dt));
                }

                "date" => {
                    let date =
                        chrono::NaiveDate::parse_from_str(
                            value,
                            "%Y-%m-%d",
                        )?;

                    params.push(Box::new(date));
                }

                _ => {
                    params.push(Box::new(
                        value.to_string()
                    ));
                }
            }
        }

        let refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|p| p.as_ref()).collect();

        client.execute(&query, &refs).await?;
    }

    Ok(())
}