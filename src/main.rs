mod producer;
mod task;
mod worker;


#[tokio::main]
#[allow(unused)]
async fn main() {
    let (sender , receiver) = task::create_new_task();

    for i in 1..6{
        let rx = receiver.clone();

        let worker = tokio::spawn(async move{
        let w = worker::Worker::new(i).await;
        println!("Worker of id {:?} created" , w.id);
        w.consume_task(rx).await;
    });
    }

    producer::producer(sender);
}