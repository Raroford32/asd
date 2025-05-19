use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::KafkaError;
use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct Task {
    ip: String,
    username: String,
    password: String,
}

async fn produce_task(producer: &FutureProducer, topic: &str, task: Task) -> Result<(), KafkaError> {
    let payload = serde_json::to_string(&task).unwrap();
    let record = FutureRecord::to(topic)
        .payload(&payload)
        .key(&task.ip);

    producer.send(record, Duration::from_secs(0)).await.map(|_| ()).map_err(|(e, _)| e)
}

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let task = Task {
        ip: "192.168.1.1".to_string(),
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    match produce_task(&producer, "rdp_tasks", task).await {
        Ok(_) => println!("Task produced successfully"),
        Err(e) => eprintln!("Failed to produce task: {}", e),
    }
}
