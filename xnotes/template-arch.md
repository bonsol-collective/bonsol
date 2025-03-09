# Architecture Template

<version>1.0.0</version>

## Requirements

- Document architectural decisions clearly
- Maintain a Changelog
- Judicious use of mermaid diagrams

## Structure

### Required Sections

#### 1. Title {Architecture for {project}}

#### 2. Status

- Draft
- Approved

#### 3. Technical Summary

#### 4. Technology Table

Table listing choices for languages, libraries, infra, etc...

- column for technology
- column for descrpition

#### 5. Arhictectural Diagrams

- Mermaid Diagrams as needed

#### 6. Data Models, API Specs, Schemas, etc...

- not exhaustive - but key ideas that need to be retained and followed across stories

#### 7. Project Structure

document the folder and file organization and structure along with descriptions

#### 8. Change Log

markdown table of key changes after document is no longer in draft and is updated, table includes the change title, the story id that the change happened during, and a description if the title is not clear enough

## Examples

<example>
# Architecture for Sensor Data Processing Platform

## Status: Approved

## Technical Summary

This architecture defines a scalable, fault-tolerant platform for processing real-time sensor data from multiple sources. The system employs a microservices architecture to ensure high availability, scalability, and maintainability while supporting real-time data processing and analysis.

## Technology Table

| Technology   | Description                                                   |
| ------------ | ------------------------------------------------------------- |
| Kubernetes   | Container orchestration platform for microservices deployment |
| Apache Kafka | Event streaming platform for real-time data ingestion         |
| TimescaleDB  | Time-series database for sensor data storage                  |
| Go           | Primary language for data processing services                 |
| GoRilla Mux  | REST API Framework                                            |
| Python       | Used for data analysis and ML services                        |
| gRPC         | Inter-service communication protocol                          |
| Prometheus   | Metrics collection and monitoring                             |
| Grafana      | Visualization and dashboarding                                |

## Architectural Diagrams

```mermaid
graph TD
    A[Sensor Gateway] -->|Raw Data| B[Kafka]
    B --> C[Data Processor]
    B --> D[Alert Service]
    C --> E[(TimescaleDB)]
    C --> F[Analytics Engine]
    D --> G[Notification Service]
    F --> H[ML Predictor]

    style A fill:#f9f,stroke:#333
    style B fill:#bbf,stroke:#333
    style C fill:#bfb,stroke:#333
```

```mermaid
sequenceDiagram
    participant S as Sensor
    participant G as Gateway
    participant K as Kafka
    participant P as Processor
    participant DB as TimescaleDB

    S->>G: Send Data
    G->>K: Publish Event
    K->>P: Consume Event
    P->>DB: Store Processed Data
```

## Data Models

### Sensor Reading Schema

```json
{
  "sensor_id": "string",
  "timestamp": "datetime",
  "readings": {
    "temperature": "float",
    "pressure": "float",
    "humidity": "float"
  },
  "metadata": {
    "location": "string",
    "calibration_date": "datetime"
  }
}
```

## Project Structure

```
/
├── /services
│   ├── /gateway        # Sensor data ingestion
│   ├── /processor      # Data processing and validation
│   ├── /analytics      # Data analysis and ML
│   └── /notifier       # Alert and notification system
├── /deploy
│   ├── /kubernetes     # K8s manifests
│   └── /terraform      # Infrastructure as Code
└── /docs
    ├── /api           # API documentation
    └── /schemas       # Data schemas
```

## Change Log

| Change               | Story ID | Description                                                   |
| -------------------- | -------- | ------------------------------------------------------------- |
| Initial Architecture | N/A      | Initial approved system design and documentation              |
| Add ML Pipeline      | story-4  | Integration of machine learning prediction service            |
| Kafka Upgrade        | story-6  | Upgraded from Kafka 2.0 to Kafka 3.0 for improved performance |

</example>

<example type="invalid">
# Simple Architecture

Just use a database and some APIs. Maybe add caching later if needed.

Tech stack:

- Whatever is easiest
- Probably MongoDB
- Some framework

No diagrams or proper documentation included.
</example>
