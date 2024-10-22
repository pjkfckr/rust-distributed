
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
    pub group_id: String,
}

impl KafkaConfig {
    pub fn new() -> Self {
        KafkaConfig {
            brokers: "localhost:9092".to_string(),
            topic: "tasks".to_string(),
            group_id: "distributed_processor".to_string(),
        }
    }
}