use rdkafka::config::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;
use rdesktop_rs::RdpClient;
use serde::{Serialize, Deserialize};
use tokio_postgres::{NoTls, Client};

#[derive(Serialize, Deserialize)]
struct Task {
    ip: String,
    username: String,
    password: String,
}

async fn consume_task(consumer: &StreamConsumer, db_client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let message = consumer.recv().await?;
    let payload = match message.payload_view::<str>() {
        Some(Ok(s)) => s,
        _ => "",
    };

    let task: Task = serde_json::from_str(payload)?;

    let rdp_client = RdpClient::new(&task.ip, &task.username, &task.password);
    let result = rdp_client.login().await;

    db_client.execute(
        "INSERT INTO rdp_results (ip, username, password, result) VALUES ($1, $2, $3, $4)",
        &[&task.ip, &task.username, &task.password, &result],
    ).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rdp_group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Consumer creation error");

    consumer.subscribe(&["rdp_tasks"]).expect("Subscription error");

    let (db_client, connection) = tokio_postgres::connect("host=localhost user=postgres password=secret dbname=rdp_results", NoTls).await.expect("Database connection error");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    loop {
        match consume_task(&consumer, &db_client).await {
            Ok(_) => println!("Task consumed successfully"),
            Err(e) => eprintln!("Failed to consume task: {}", e),
        }
    }
}
