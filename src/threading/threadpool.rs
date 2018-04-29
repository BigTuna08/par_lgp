use dataMgmt::dataset::TestDataSet;
use dataMgmt::message::{EvalResult, Message};
use evo_sys;
use params;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;


pub struct ThreadPool {
    job_sender: mpsc::Sender<EvalResult>,
    result_receiver: mpsc::Receiver<EvalResult>,
    handles: Vec<Option<thread::JoinHandle<()>>>,

}


impl ThreadPool{

    pub fn new(size: usize, data_set: TestDataSet, evaluator_ind: usize) -> ThreadPool {
        let mut handles = Vec::with_capacity(size);

        let (job_sender, job_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        let job_receiver = Arc::new(Mutex::new(job_receiver)); //clone so all threads can receive

        let data_ref = Arc::new(data_set);

        for _ in 0..size{
            let rx = job_receiver.clone();
            let tx = result_sender.clone();
            let dr = data_ref.clone();
            let ev_code = evaluator_ind.clone();

            let handle = thread::spawn(move ||{
                worker(rx, tx, &dr, ev_code);
            });
            handles.push(Some(handle));
        }

        ThreadPool{
            job_sender, result_receiver, handles, // evaluator// data_set
        }

    }


    pub fn add_task(&mut self, task: EvalResult) {

        self.job_sender.send(task);
    }

    pub fn next_result(&mut self) -> Option<EvalResult> {
        if let Ok(result) = self.result_receiver.try_recv() {
            return Some(result)
        }
        None
    }

    pub fn next_result_wait(&mut self) -> EvalResult {
        match self.result_receiver.recv() {
            Ok(message) => message,
            _ => panic!("Error getting result!!")
        }

    }

    pub fn terminate(&mut self){
        for thread in 0..self.handles.len()*params::params::WORKER_QUEUE_SIZE {  //make sure to issue enough reques
            self.job_sender.send(EvalResult::quit());
        }

        for (i,thread) in self.handles.iter_mut().enumerate(){
            thread.take().unwrap().join();
        }
    }

}

fn worker(job_receiver: Arc<Mutex<mpsc::Receiver<EvalResult>>>, result_sender: mpsc::Sender<EvalResult>, data_ref: &TestDataSet, evaluator_ind: usize){
    let mut queue = VecDeque::with_capacity(params::params::WORKER_QUEUE_SIZE);
    let evaluator = evo_sys::prog::eval::get_fn(evaluator_ind);

    const refill_after: usize = params::params::WORKER_QUEUE_SIZE/3 + 1;

    loop {
//        println!("before matching q len, lenth is {:?}", queue.len());

        match queue.len() {
            0 => { //block and wait for jobs
//                println!(" before getting lock ");
                let job_lock = job_receiver.lock().unwrap();
//                println!("got lock ");
                while let Ok(job) = job_lock.try_recv() {
//                    println!(" got job");
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
            match next_job.signal {
                Message::Cont => result_sender.send( evaluator(next_job, data_ref) ),
                Message::Quit => {
                    break
                }
            };

        }
    }

}
