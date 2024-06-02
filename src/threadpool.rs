use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
type Job = Box<dyn FnOnce() + 'static + Send>;
pub struct Pool {
    tx: mpsc::Sender<Job>,
    rx: Arc<Mutex<mpsc::Receiver<Job>>>,
}
impl Default for Pool {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        Self { tx, rx }
    }
}
impl Pool {
    pub fn let_send<J>(&self, job: J)
    where
        J: FnOnce() + 'static + Send,
    {
        let job = Box::new(job);
        self.tx.send(job).unwrap();
    }

    pub fn gen_threads_receive_do(&self, threadcount: u8) {
        (0..threadcount).for_each(|id| {
            let rx = Arc::clone(&self.rx);
            thread::spawn(move || loop {
                match rx.lock().unwrap().recv() {
                    Ok(job) => job(),
                    Err(_) => {
                        println!("Thread {id} disconnected!");
                        break;
                    }
                }
            });
        })
    }
}
