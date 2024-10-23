use std::fmt;
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use warp::reject::Reject;

#[derive(Debug)]
struct KafkaSubmissionError {
    details: String
}

impl fmt::Display for KafkaSubmissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kafka Submission error: {}", self.details)
    }
}

impl Reject for KafkaSubmissionError {}

impl KafkaSubmissionError {
    fn new(kafka_error: &KafkaError, message: &OwnedMessage) -> Self {
        let details = format!("Kafka error: {:?}, Message: {:?}", kafka_error, message);
        KafkaSubmissionError { details }
    }
}

pub fn kafka_error_to_rejection(error: (KafkaError, OwnedMessage)) -> warp::Rejection {
    warp::reject::custom(KafkaSubmissionError::new(&error.0, &error.1))
}