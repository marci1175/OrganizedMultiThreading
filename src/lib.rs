use std::fmt;

use tokio::sync::mpsc::{self, Receiver};

#[derive(Debug, Clone)]
pub struct WrappedValue<V> {
    inner: V,
    ///The index of the function this value is returned by
    index: usize,
}

impl<V> WrappedValue<V> {
    pub fn new(value: V, index: usize) -> Self {
        Self { inner: value, index }
    }
}

pub struct ThreadWrapper<T> {
    _inner_function_handle: tokio::task::JoinHandle<()>,
    reciver: mpsc::Receiver<T>,

    index: usize,
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
            _inner_function_handle: tokio::spawn(async move {
                sender.send((function)()).await.expect("Failed to send out on channel");
            }),
            reciver: reciver,
            //Set as default
            index: 0,
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
    T: std::marker::Send,
{
    pub fn new(tasks: Vec<ThreadWrapper<T>>) -> Self {
        //Index them
        Self { tasks: Self::count(tasks) }
    }

    fn count(mut tasks: Vec<ThreadWrapper<T>>) -> Vec<ThreadWrapper<T>> {
        for (iter_index, task) in tasks.iter_mut().enumerate() {
            task.index = iter_index; 
        }

        tasks
    }

    pub async fn excecute_tasks(&mut self) -> Vec<WrappedValue<T>> {
        let (sender, mut reciver) = mpsc::channel::<WrappedValue<T>>(self.tasks.len());

        for task in self.tasks.iter_mut() {
            sender.send(WrappedValue::new(task.recive_thread_output().await, task.index)).await.expect("Failed to send value to main thread");
        }

        let mut returned_values: Vec<WrappedValue<T>> = Vec::new();

        for _ in 0..self.tasks.len() {
            returned_values.push(reciver.recv().await.expect("Recived a None value"));
        }

        returned_values
    }
}
