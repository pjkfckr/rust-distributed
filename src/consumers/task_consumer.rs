use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use crate::config::kafka_config::KafkaConfig;
use crate::models::task::Task;
use crate::processors::task_processor::process_task;

pub struct TaskConsumer {
    consumer: StreamConsumer,
}

impl TaskConsumer {
    pub fn new(config: &KafkaConfig) -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &config.group_id)
            .set("bootstrap.servers", &config.brokers)
            .set("enable.auto.commit", "true")
            .create()
            .expect("Consumer Creation Failed");

        consumer.subscribe(&[&config.topic]).expect("Can't subscribe to specified topics");

        TaskConsumer { consumer }
    }

    pub async fn run(&self) {
        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    let payload = msg.payload().expect("Empty payload");
                    let task: Task = serde_json::from_slice(payload).expect("Invalid payload");
                    process_task(task).await;
                }
                Err(e) => println!("Kafka error: {}", e),
            }
        }
    }
}
