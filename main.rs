use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread 
    };

fn main() {
    
    thread::spawn(move||{
        for stream in TcpListener::bind("127.0.0.1:7878").unwrap().incoming() {
            if let true = stream.is_ok() {
                println!("Success Connecting");
                deal_connection(stream.unwrap())
            }
        }
    
    }).join().unwrap()


}
fn deal_connection(mut stream:TcpStream) {

    let closure_response = |filename:&str|{
        format!("{}\r\n{}\r\n\r\n{}","HTTP/1.1 200 OK",
        fs::read_to_string(filename).unwrap().len(),
        fs::read_to_string(filename).unwrap())
        .as_bytes().to_owned()
    };


    thread::spawn(move||{
        match BufReader::new(&stream).lines().next().unwrap().unwrap()=="GET / HTTP/1.1" {
            true => stream.write_all(&closure_response("hello.html")).unwrap(),
            false => stream.write_all(&closure_response("404.html")).unwrap()
    
        }
    
        
    }).join().unwrap()

}