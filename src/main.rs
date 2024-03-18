// use std::path::Path;

// use std::rc::Rc;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::clone;
use std::rc::Rc;

use tokio::net::{tcp, TcpListener, TcpStream};
use httpserver::request::Request;
use httpserver::response::{Response, ResponseCode};
use httpserver::router::Path;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let stream = Rc::new(RefCell::new(stream));
        
        let request_code = Request::new(*stream.clone()).await.request_code;
        Response::write(&Response::new(Path::new(request_code.as_str()).router(), ResponseCode::new(request_code.as_str()).gen_code()).await, *stream.clone()).await;
    }
}
trait Copy {}
impl Copy for tokio::net::TcpStream {
    
}