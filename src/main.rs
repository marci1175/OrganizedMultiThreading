use std::{thread::sleep, time::Duration};

use OrganizedMultiThreading::{OrganizedThreads, ThreadWrapper};

fn expensive_calculation() -> i32 {
    sleep(Duration::from_secs(2));

    // println!("{}", 342423 * 23);

    23
}

fn more_expensive_calculation() -> i32 {
    sleep(Duration::from_secs(3));

    // println!("{}", 342423 * 23);

    234
}

#[tokio::main]
async fn main() {
    let wrapped_threads = vec![
        ThreadWrapper::new(expensive_calculation),
        ThreadWrapper::new(expensive_calculation),
        ThreadWrapper::new(more_expensive_calculation),
    ];
    
    let mut values = OrganizedThreads::new(wrapped_threads);
    
    let values = values.excecute_tasks().await;

    for i in 0..99 {
        println!("ASD{i}")   
    }

    

    println!("{values:?}");
}
