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
        .set("security.protocol", "SSL")
        .set("ssl.keystore.location", "/path/to/keystore.jks")
        .set("ssl.keystore.password", "your_keystore_password")
        .set("ssl.truststore.location", "/path/to/truststore.jks")
        .set("ssl.truststore.password", "your_truststore_password")
        .create()
        .expect("Producer creation error");

    let task = Task {
        ip: "192.168.1.1".to_string(),
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    match produce_task(&producer, "rdp_tasks", task).await {
        Ok(_) => println!("Task produced successfully"),
        Err(e) => {
            eprintln!("Failed to produce task: {}", e);
            // Retry mechanism
            let mut retry_count = 0;
            let max_retries = 3;
            while retry_count < max_retries {
                match produce_task(&producer, "rdp_tasks", task.clone()).await {
                    Ok(_) => {
                        println!("Task produced successfully after retry");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Retry {}/{} failed: {}", retry_count + 1, max_retries, e);
                        retry_count += 1;
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        }
    }
}
