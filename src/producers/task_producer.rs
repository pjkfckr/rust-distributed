use rdkafka::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use crate::models::task::Task;
use crate::config::kafka_config::KafkaConfig;

pub struct TaskProducer {
    producer: FutureProducer,
    topic: String
}

impl TaskProducer {
    pub fn new(config: &KafkaConfig) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .create()
            .expect("Producer Can't Connection");
        
        TaskProducer {
            producer,
            topic: config.topic.clone()
        }
    }
    
    pub async fn send(&self, task: &Task) -> Result<(), (KafkaError, OwnedMessage)> {
        let payload = serde_json::to_string(&task).expect("Serialization Failed");
        self.producer.send(
            FutureRecord::to(&self.topic)
                .payload(&payload)
                .key(&task.id),
            std::time::Duration::from_secs(0),
        ).await.map(|_| ())
    }
}