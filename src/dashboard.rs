use warp::Filter;
use serde::{Deserialize, Serialize};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[derive(Deserialize)]
struct InputData {
    ip_list: Vec<String>,
    user_list: Vec<String>,
    password_list: Vec<String>,
}

#[derive(Serialize)]
struct Task {
    ip: String,
    username: String,
    password: String,
}

async fn produce_task(producer: &FutureProducer, topic: &str, task: Task) -> Result<(), rdkafka::error::KafkaError> {
    let payload = serde_json::to_string(&task).unwrap();
    let record = FutureRecord::to(topic)
        .payload(&payload)
        .key(&task.ip);

    producer.send(record, Duration::from_secs(0)).await.map(|_| ()).map_err(|(e, _)| e)
}

async fn handle_input(input: InputData) -> Result<impl warp::Reply, warp::Rejection> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    for ip in input.ip_list {
        for username in &input.user_list {
            for password in &input.password_list {
                let task = Task {
                    ip: ip.clone(),
                    username: username.clone(),
                    password: password.clone(),
                };

                match produce_task(&producer, "rdp_tasks", task).await {
                    Ok(_) => println!("Task produced successfully"),
                    Err(e) => eprintln!("Failed to produce task: {}", e),
                }
            }
        }
    }

    Ok(warp::reply::json(&"Tasks submitted successfully"))
}

#[tokio::main]
async fn main() {
    let input_route = warp::path("input")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_input);

    warp::serve(input_route).run(([127, 0, 0, 1], 3030)).await;
}
