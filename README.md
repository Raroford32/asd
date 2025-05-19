# RDP Goldbrute

## Purpose and Goals

RDP Goldbrute is a high-throughput, distributed network stress-testing utility designed to simulate credential validation requests across a wide range of remote desktop endpoints. The goal is to achieve maximum efficiency and parallelism, comparable in architectural concept to legacy-scale network scanning tools. This system is intended for stress-testing and high-speed credential validation research.

## System Architecture

### Producer
A central node that produces tasks to Kafka topics.

### Consumer Nodes
Multiple Ubuntu-based nodes running Rust applications that consume tasks from Kafka topics and perform RDP login attempts.

### Message Broker
Use Kafka as the message broker for task distribution.

### Result Storage
Use a database like PostgreSQL or a distributed storage system like Apache Cassandra to store the results.

## Toolchain and Technologies

### Programming Language
- Rust: Offers high performance and memory safety, making it ideal for low-level network programming.

### Distributed Computing Framework
- Apache Kafka: A distributed streaming platform that can handle high-throughput data streams and coordinate tasks across multiple nodes.

### RDP Libraries
- `rdesktop-rs`: Used for RDP login attempts.

### Orchestration
- Docker and Docker Compose: Used to manage and deploy the distributed system.

### Monitoring
- Prometheus and Grafana: Used for monitoring the performance and health of the system.

## Setup and Running the Project

### Dependencies
- Rust
- Docker
- Docker Compose
- Apache Kafka
- PostgreSQL or Apache Cassandra

### Configuration
1. Clone the repository.
2. Install the required dependencies.
3. Configure Kafka, PostgreSQL/Apache Cassandra, and Prometheus.
4. Build and run the Docker containers using Docker Compose.

### Running the Project
1. Start the Kafka, PostgreSQL/Apache Cassandra, and Prometheus services.
2. Run the producer application to produce tasks to Kafka topics.
3. Run the consumer nodes to consume tasks from Kafka topics and perform RDP login attempts.
4. Monitor the performance and health of the system using Prometheus and Grafana.

## Security Configuration

### Kafka SSL Configuration
To enable SSL for Kafka, you need to configure the following environment variables in your `docker-compose.yml` file:

- `KAFKA_SSL_KEYSTORE_LOCATION`: Path to the Kafka keystore file.
- `KAFKA_SSL_KEYSTORE_PASSWORD`: Password for the Kafka keystore.
- `KAFKA_SSL_TRUSTSTORE_LOCATION`: Path to the Kafka truststore file.
- `KAFKA_SSL_TRUSTSTORE_PASSWORD`: Password for the Kafka truststore.

Example configuration:
```yaml
kafka:
  environment:
    KAFKA_SSL_KEYSTORE_LOCATION: /path/to/keystore.jks
    KAFKA_SSL_KEYSTORE_PASSWORD: your_keystore_password
    KAFKA_SSL_TRUSTSTORE_LOCATION: /path/to/truststore.jks
    KAFKA_SSL_TRUSTSTORE_PASSWORD: your_truststore_password
```

### PostgreSQL SSL Configuration
To enable SSL for PostgreSQL, you need to configure the following environment variables in your `docker-compose.yml` file:

- `POSTGRES_SSLMODE`: SSL mode for PostgreSQL (e.g., `require`).
- `POSTGRES_SSLROOTCERT`: Path to the PostgreSQL root certificate.

Example configuration:
```yaml
db:
  environment:
    POSTGRES_SSLMODE: require
    POSTGRES_SSLROOTCERT: /path/to/root.crt
```

## Error Handling and Retry Mechanisms

### Consumer Node
The consumer node includes error handling and retry mechanisms for failed tasks. If a task fails, the system will retry the task up to a maximum number of retries with a delay between each retry.

### Producer Node
The producer node includes error handling and retry mechanisms for failed task production. If a task production fails, the system will retry the task production up to a maximum number of retries with a delay between each retry.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.
