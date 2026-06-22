use tokio_postgres::{Client, Error, NoTls};

use crate::infrastructure::createtables::create_tables;
use crate::infrastructure::loadsampledata::load_sample_data;

pub async fn connect_to_db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 port=5432 user=arnoslabbinck password=Ypsp2man! dbname=arnoslabbinck",
        NoTls,
    )
    .await?;

    // Spawn the connection to run in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    println!("Connected to the database successfully.");

    let row = client
        .query_one("SELECT current_database()", &[])
        .await?;

    let db_name: String = row.get(0);

    println!("Connected to database: {}", db_name);

    // Drop existing tables
    client
        .batch_execute("DROP TABLE IF EXISTS lapsplits CASCADE;")
        .await?;

    client
        .batch_execute("DROP TABLE IF EXISTS runsession CASCADE;")
        .await?;

    client
        .batch_execute("DROP TABLE IF EXISTS dailywellness CASCADE;")
        .await?;

    client
        .batch_execute("DROP TABLE IF EXISTS trainingmarkers CASCADE;")
        .await?;

    // Recreate tables
    create_tables(&client).await?;

    // Load sample data
    let sample_data_folder =
        "/Users/arnoslabbinck/Documents/rust/vitalMile/src/infrastructure/sampledata";

    let paths = std::fs::read_dir(sample_data_folder)
        .expect("Failed to read sample data directory");


       
    for path in paths {
    let path = match path {
        Ok(entry) => entry.path(),
        Err(err) => {
            println!("Error reading directory entry: {:?}", err);
            continue;
        }
    };

    if path.is_file() {
        let filepath = match path.to_str() {
            Some(filepath) => filepath,
            None => {
                println!("Could not convert path to string");
                continue;
            }
        };

        println!("Loading sample data from file: {}", filepath);

        match load_sample_data(&client, filepath).await {
            Ok(_) => {
                println!("Successfully loaded {}", filepath);
            }
            Err(err) => {
                eprintln!(
                    "Failed to load sample data from '{}': {:#?}",
                    filepath,
                    err
                );
            }
        }
    }
    }

    println!("Database initialization complete.");

    Ok(client)
}


