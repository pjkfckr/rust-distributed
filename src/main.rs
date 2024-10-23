
use distributed_project::api::rest_api::start_api;
use distributed_project::config::kafka_config::KafkaConfig;
use distributed_project::consumers::task_consumer::TaskConsumer;
use distributed_project::producers::task_producer::TaskProducer;


#[tokio::main]
async fn main() {
    let kafka_config = KafkaConfig::new();
    let producer = TaskProducer::new(&kafka_config);
    let consumer = TaskConsumer::new(&kafka_config);

    let api_task = tokio::spawn(start_api(producer));
    let consumer_task = tokio::spawn(async move {
        consumer.run().await;
    });

    // 모든 작업이 완료될 때까지 대기
    tokio::try_join!(api_task, consumer_task).expect("Failed to run tasks");
}