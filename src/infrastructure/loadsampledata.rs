use csv::Reader;
use std::error::Error;
use chrono::{DateTime, FixedOffset};
use tokio_postgres::Client;

#[derive(Debug, Clone)]
enum ColumnType {
    Text,
    Integer,
    TimestampTz,
}

#[derive(Debug)]
enum Value {
    Text(String),
    Int(i32),
    Timestamp(DateTime<FixedOffset>),
}

pub async fn load_sample_data(
    client: &Client,
    filepath: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Loading sample data...");

    let table_name = filepath
        .split('/')
        .last()
        .unwrap()
        .replace(".csv", "");

    let schema = load_schema(client, &table_name).await?;

    insert_data_into_table(client, &table_name, filepath, schema).await?;

    Ok(())
}

async fn load_schema(
    client: &Client,
    table_name: &str,
) -> Result<Vec<ColumnType>, Box<dyn Error>> {
    let rows = client
        .query(
            "
            SELECT data_type
            FROM information_schema.columns
            WHERE table_name = $1
            ORDER BY ordinal_position
            ",
            &[&table_name],
        )
        .await?;

    let mut schema = Vec::new();

    for row in rows {
        let data_type: String = row.get("data_type");

        let column_type = match data_type.as_str() {
            "text" | "character varying" => ColumnType::Text,
            "integer" => ColumnType::Integer,
            "timestamp with time zone" => ColumnType::TimestampTz,
            other => {
                return Err(format!("Unsupported type: {}", other).into());
            }
        };

        schema.push(column_type);
    }

    Ok(schema)
}

fn convert_value(
    value: &str,
    column_type: &ColumnType,
) -> Result<Value, Box<dyn Error>> {
    let v = value.trim();

    Ok(match column_type {
        ColumnType::Text => Value::Text(v.to_string()),

        ColumnType::Integer => {
            Value::Int(v.parse()?)
        }

        ColumnType::TimestampTz => {
            Value::Timestamp(DateTime::parse_from_rfc3339(v)?)
        }
    })
}

async fn insert_data_into_table(
    client: &Client,
    table_name: &str,
    filepath: &str,
    schema: Vec<ColumnType>,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(filepath)?;
    let headers = rdr.headers()?.clone();

    let columns: Vec<&str> = headers.iter().collect();

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

    println!("Generated query: {}", query);

    for result in rdr.records() {
        let record = result?;

        let values: Vec<Value> = record
            .iter()
            .zip(schema.iter())
            .map(|(val, ty)| convert_value(val, ty))
            .collect::<Result<_, _>>()?;

        // Convert Value → ToSql params (IMPORTANT FIX)
        let params: Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> =
            values
                .into_iter()
                .map(|v| match v {
                    Value::Text(s) => Box::new(s) as Box<dyn tokio_postgres::types::ToSql + Sync>,
                    Value::Int(i) => Box::new(i) as Box<dyn tokio_postgres::types::ToSql + Sync>,
                    Value::Timestamp(t) => Box::new(t) as Box<dyn tokio_postgres::types::ToSql + Sync>,
                })
                .collect();

        let refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
            params.iter().map(|p| &**p).collect();

        client.execute(&query, &refs).await?;
    }

    Ok(())
}