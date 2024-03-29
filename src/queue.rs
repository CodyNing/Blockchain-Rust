use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    recv_tasks: spmc::Receiver<TaskType>,
    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // TODO: create the channels; start the worker threads; record their JoinHandles
        let (spmc_sender, spmc_receiver) = spmc::channel();
        let (mpsc_sender, mpsc_receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(n_workers);

        for _i in 0..n_workers{
            let thread_sender = mpsc_sender.clone();
            let thread_receiver = spmc_receiver.clone();

            let worker = thread::spawn(move ||{
                WorkQueue::run(thread_receiver, thread_sender);
            });

            workers.push(worker);
        }

        WorkQueue{
            send_tasks: Some(spmc_sender),
            recv_tasks: spmc_receiver,
            recv_output: mpsc_receiver,
            workers: workers
        }
    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // TODO: the main logic for a worker thread
        loop {
            let task_result = recv_tasks.recv();
            // NOTE: task_result will be Err() if the spmc::Sender has been destroyed and no more messages can be received here
            match task_result{
                Ok(task) =>{
                    let res = task.run();
                    match res{
                        None => {},
                        Some(output) =>{
                            send_output.send(output).expect("Unable to send output");
                        }
                    }
                }
                Err(_) =>{
                    return;
                }
            }
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), mpsc::SendError<TaskType>> {
        // TODO: send this task to a worker
        match self.send_tasks.as_mut(){
            None => {
                Ok(())
            },
            Some(sender) =>{
                sender.send(t)
            }
        }
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(&self, timeout: std::time::Duration) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // TODO: destroy the spmc::Sender so everybody knows no more tasks are incoming;
        // drain any pending tasks in the queue; wait for each worker thread to finish.
        // HINT: Vec.drain(..)
        match self.send_tasks.as_mut(){
            None => {},
            Some(sender) => drop(sender)
        }
        self.send_tasks = None;
        loop{
            let task_result = self.recv_tasks.recv();
            match task_result{
                Ok(_) =>{}
                Err(_) =>{
                    break;
                }
            }
        }
        for worker in self.workers.drain(..){
            worker.join().expect("Failed to exit.");
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
