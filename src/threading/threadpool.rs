use dataMgmt::TestDataSet;
use dataMgmt::{Message, EvalResult};
use evo_sys;
use params;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
//use std::time::Duration;


pub struct ThreadPool {
    job_sender: mpsc::Sender<Message>,
    result_receiver: mpsc::Receiver<EvalResult>,
    handles: Vec<Option<thread::JoinHandle<()>>>,
    current_job_count: u32,
}


impl ThreadPool{

    pub fn new(size: usize, data_ref: Arc<TestDataSet>) -> ThreadPool {
        let mut handles = Vec::with_capacity(size);

        let (job_sender, job_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        let job_receiver = Arc::new(Mutex::new(job_receiver)); //clone so all threads can receive

//        let data_ref = Arc::new(data_set);

        for _ in 0..size{
            let rx = job_receiver.clone();
            let tx = result_sender.clone();
            let dr = data_ref.clone();

            let handle = thread::spawn(move ||{
                worker(rx, tx, &dr);
            });
            handles.push(Some(handle));
        }

        ThreadPool{
            job_sender, result_receiver, handles, current_job_count:0// evaluator// data_set
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
            _ => panic!("Error getting result!!")
        }

    }

    pub fn terminate(&mut self){
        for _ in 0..self.handles.len()*params::params::WORKER_QUEUE_SIZE {  //make sure to issue enough reques
            self.job_sender.send(Message::Quit);
        }

        for thread in self.handles.iter_mut(){
            thread.take().unwrap().join();
        }
    }

    pub fn current_job_count(&self)->u32{
        self.current_job_count
    }

}

fn worker(job_receiver: Arc<Mutex<mpsc::Receiver<Message>>>, result_sender: mpsc::Sender<EvalResult>, data_ref: &TestDataSet){
    let mut queue = VecDeque::with_capacity(params::params::WORKER_QUEUE_SIZE);
    let data_size = data_ref.records.len() as f32;

    const refill_after: usize = params::params::WORKER_QUEUE_SIZE/3 + 1;

    loop {
        match queue.len() {
            0 => { //block and wait for jobs
                let job_lock = job_receiver.lock().unwrap();
                while let Ok(job) = job_lock.try_recv() {
                    queue.push_back(job);
                    if queue.len() >= params::params::WORKER_QUEUE_SIZE {break;}
                }
            },
            1 ... refill_after => { //get jobs if receiver not locked
                if let Ok(job_lock) = job_receiver.try_lock(){
                    while let Ok(job) = job_lock.try_recv() {
                        queue.push_back(job);
                        if queue.len() >= params::params::WORKER_QUEUE_SIZE {break;}
                    }

                }
            },
            _ => ()
        }

        if let Some(next_job) = queue.pop_front() {
            match next_job {
                Message::Cont(mut prog) => {
                    let fit = evo_sys::prog::eval::eval_program_corrects_testing_with_assert(&prog, data_ref)/data_size;
                    prog.test_fit = Some(fit);
                    result_sender.send(EvalResult{prog} );
                }
                Message::Quit => break,
            }
        }
    }

}
