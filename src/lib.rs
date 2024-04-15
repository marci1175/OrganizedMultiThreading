use std::fmt;

use tokio::sync::mpsc::{self};

pub struct ThreadWrapper<T> {
    inner_function_handle: tokio::task::JoinHandle<()>,
    reciver: mpsc::Receiver<T>,
}

impl<T> ThreadWrapper<T>
where
    T: std::marker::Send + 'static,
{
    pub fn new<F>(function: F) -> Self
    where
        F: Fn() -> T + std::marker::Send + 'static,
    {
        let (sender, reciver) = mpsc::channel::<T>(1);
        Self {
            inner_function_handle: tokio::spawn(async move {
                //This will block until it gets flagged to go
                sender.send((function)()).await.expect("Failed to send out on channel");
            }),
            reciver: reciver,
        }
    }

    pub async fn recive_thread_output(&mut self) -> T {
        self.reciver.recv().await.expect("Failed to signal start to thread")
    }
}

pub struct OrganizedThreads<T>
where
    T: std::marker::Send + 'static,
{
    tasks: Vec<ThreadWrapper<T>>,
}

impl<T> OrganizedThreads<T>
where
    T: std::marker::Send + 'static + fmt::Debug,
{
    pub fn new(tasks: Vec<ThreadWrapper<T>>) -> Self {
        Self { tasks }
    }

    pub async fn excecute_tasks(self) -> mpsc::Receiver<T> {
        let (sender, reciver) = mpsc::channel::<T>(self.tasks.len());

        for mut task in self.tasks {
            sender.send(task.recive_thread_output().await).await.expect("Failed to send value to main thread");
        }

        reciver
    }
}
