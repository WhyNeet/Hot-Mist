#![allow(unsafe_op_in_unsafe_fn)]

use std::time::Duration;

wit_bindgen::generate!({
  world: "handler-world"
});

struct HelloWorldComponent;

impl Guest for HelloWorldComponent {
    fn handler() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let future = handler();
        rt.block_on(future)
    }
}

export!(HelloWorldComponent);

async fn handler() {
    let mut tasks = vec![];

    for i in 0..10 {
        let task = tokio::spawn(async move {
            println!("Task {i} started.");

            tokio::time::sleep(Duration::from_millis(1000)).await;

            println!("Task {i} finished.");
        });

        tasks.push(task);
    }

    println!("Tasks spawned.");

    futures::future::join_all(tasks).await;
}
