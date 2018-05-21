use dataMgmt::TestDataSet;
use dataMgmt::{Message, EvalResult};
use evo_sys;
use params;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use ThreadDefaults;
use config::process_thread_defaults;
//use std::time::Duration;


pub struct ThreadPool {
    job_sender: mpsc::Sender<Message>,
    result_receiver: mpsc::Receiver<EvalResult>,
    handles: Vec<Option<thread::JoinHandle<()>>>,
    current_job_count: u32,
    config: ThreadDefaults,
}


impl ThreadPool{

    // uses settings in configs/threadding.txt
    pub fn new_default(data_ref: Arc<TestDataSet>) -> ThreadPool {
        let config = process_thread_defaults("configs/threadding.txt");

        let mut handles = Vec::with_capacity(config.n_worker_threads as usize);
        let (job_sender, job_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver)); //clone so all threads can receive

        for _ in 0..config.n_worker_threads{
            let rx = job_receiver.clone();
            let tx = result_sender.clone();
            let dr = data_ref.clone();
            let q_size = config.worker_queue_size as usize;

            let handle = thread::spawn(move ||{
                worker(rx, tx, &dr, q_size );
            });
            handles.push(Some(handle));
        }

        ThreadPool{
            job_sender, result_receiver, handles, current_job_count:0, config// evaluator// data_set
        }

    }


    pub fn add_task(&mut self, task: Message) {
        self.job_sender.send(task);
        self.current_job_count += 1;
    }

    pub fn next_result(&mut self) -> Option<EvalResult> {
        if let Ok(result) = self.result_receiver.try_recv() {
            self.current_job_count -= 1;
            return Some(result)
        }
        None
    }

    pub fn next_result_wait(&mut self) -> EvalResult {
        if self.current_job_count == 0{
            panic!("Called next_result_wait, but no results exist! You'll be waiting forever!");
        }
        self.current_job_count -= 1;
        match self.result_receiver.recv() {
            Ok(message) => message,
            Err(e) => panic!("Error getting result!! error was \n{:?}", e)
        }

    }

    pub fn terminate(&mut self){
        for _ in 0..self.handles.len()* self.config.worker_queue_size as usize{  //make sure to issue enough reques
            self.job_sender.send(Message::Quit);
        }

        for thread in self.handles.iter_mut(){
            thread.take().unwrap().join();
        }
    }

    pub fn current_job_count(&self)->u32{
        self.current_job_count
    }

    pub fn can_recieve(&self) -> bool{
        self.current_job_count < self.config.cap
    }

}

fn worker(job_receiver: Arc<Mutex<mpsc::Receiver<Message>>>, result_sender: mpsc::Sender<EvalResult>, data_ref: &TestDataSet, queue_size: usize){
    let mut queue = VecDeque::with_capacity(queue_size);
    let data_size = data_ref.records.len() as f32;


    loop {
        match queue.len() {
            0 => { //block and wait for jobs
                let job_lock = job_receiver.lock().unwrap();
                while let Ok(job) = job_lock.try_recv() {
                    queue.push_back(job);
                    if queue.len() >= queue_size {break;}
                }
            },
            1 ... 6 => { //get jobs if receiver not locked
                if let Ok(job_lock) = job_receiver.try_lock(){
                    while let Ok(job) = job_lock.try_recv() {
                        queue.push_back(job);
                        if queue.len() >= queue_size {break;}
                    }

                }
            },
            _ => ()
        }

        if let Some(next_job) = queue.pop_front() {
            match next_job {
                Message::Cont(mut prog) => {
                    let fit = evo_sys::prog::eval::eval_program_corrects(&prog, data_ref)/data_size;
                    prog.test_fit = Some(fit);
                    result_sender.send(EvalResult{prog} );
                }
                Message::Quit => break,
            }
        }
    }

}
