use crossbeam::channel;
use futures::task::{self, ArcWake};
use std::task::RawWakerVTable;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::Context,
};

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn run(&self) {
        // 不断接收任务，调用poll
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }

    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();

        MiniTokio { scheduled, sender }
    }

    // 放进消息通道中
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }
}

struct Task {
    // 用于调用future Poll
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    // 
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }

    // 旧所有权不需要了
    fn poll(self: Arc<Self>) {
        // Task绑定Wake
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();

        let _ = future.as_mut().poll(&mut cx);
    }


    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

fn main() {}
