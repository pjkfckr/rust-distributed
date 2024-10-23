use warp::Filter;
use crate::models::task::Task;
use crate::producers::task_producer::TaskProducer;
use crate::exception::kafka_exception::kafka_error_to_rejection;

pub async fn start_api(producer: TaskProducer) {
    let submit_task = warp::post()
        .and(warp::path!("submit" / String))
        .and(warp::body::json())
        .and_then(move |path_param: String, task: Task| {
            let producer = producer.clone();
            async move {
                println!("Path parameter: {}", path_param);

                match producer.send(&task).await {
                    Ok(_) => Ok(warp::reply::json(&"Task submitted successfully")),
                    Err(e) => Err(kafka_error_to_rejection(e)),
                }
            }
        });

    warp::serve(submit_task)
        .run(([0, 0, 0, 0], 3012))
        .await;
}