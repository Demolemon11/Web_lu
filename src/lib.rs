use crate::environment::Parameter;
use crate::http::Connection;
use crate::threadpool::Pool;
use std::env;
use std::io::Write;
use std::net::TcpListener;
pub mod environment;
pub mod http;
pub mod threadpool;
const ADDR: &str = "127.0.0.1:7878";

pub struct Application {
    parameter: Parameter,
    pool: Pool,
}
impl Default for Application {
    fn default() -> Self {
        let parameter: Parameter = env::args().collect::<Vec<String>>().into();
        let pool = Pool::default();
        Self { parameter, pool }
    }
}
impl Application {
    pub fn run(&self) {
        match self.parameter {
            Parameter::Run(thread_count) => {
                let mut listener = TcpListener::bind(ADDR);
                while listener.is_err() {
                    listener = TcpListener::bind(ADDR);
                }
                listener
                    .unwrap()
                    .incoming()
                    .map_while(|i| i.ok())
                    .for_each(|mut stream| {
                        self.pool.gen_threads_receive_do(thread_count);
                        self.pool.let_send(move || {
                            let connection = Connection::new(&stream).standardize_response();
                            let bytes = connection.format_response_to_bytes();
                            stream.write_all(&bytes).unwrap();
                        });
                    })
            }
            Parameter::Help => {
                println!("A mini web server without any third-party crates.\nhow to use: web_lu run Option<number>\n4 threads is about to run if you dont type number.")
            }
            Parameter::Unknown => {
                println!("Unknown Command, Please Type -h or --help to get the correct usage.")
            }
        }
    }
}
