use tokio::net::TcpStream;

pub mod request {
    use tokio::{io::{AsyncBufReadExt, BufReader},
                net::TcpStream
                };
pub struct Request {
    pub request_code:String
}

impl Request {
    pub async fn new(mut stream:TcpStream) -> Self {
        Self {
            request_code:BufReader::new(&mut stream)
                .lines()
                .next_line()
                .await.unwrap().unwrap().to_ascii_uppercase()
            }
        }
    }
}






pub mod response {

    use tokio::{fs, io::AsyncWriteExt, net::TcpStream};

    pub enum ResponseCode {
        Ok,
        NotFound,
        BadRequest
    }
impl ResponseCode {
        pub fn new(request_code:&str) -> Self {
            if let false = request_code.contains("HTTP/") {
                return Self::BadRequest
            }
            match request_code {
                "GET / HTTP/1.1 " | "GET /ABOUT HTTP/1.1" => Self::Ok,
                _ => Self::NotFound
            }
        }
        pub fn gen_code(&self) -> String {
            match self {
            Self::Ok => "200 OK".to_owned(),
            Self::NotFound =>"404 NOTFOUND".to_owned(),
            Self::BadRequest => "400 BADREQUEST".to_owned()
            }
    }
}
pub struct Response {
    response_code:String,
    length:usize,
    contents:String
}

impl Response {
        
    pub async fn new(filepath:&str,response_code:String) -> Self {
        let contents = fs::read_to_string(filepath).await.unwrap();
        Self {
            response_code,
            length:contents.len(),
            contents,
        }
    }
    pub async fn write(&self,mut stream:TcpStream) -> () {
        // format!("HTTP/1.1 {}\r\n{}\r\n\r\n{}",self.response_code,self.length,self.contents)
        stream.write(format!("HTTP/1.1 {}\r\n{}\r\n\r\n{}",self.response_code,self.length,self.contents).as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}

}







pub mod router {
    
    pub enum Path {
        Bin,
        About,
        NotFound
    }
    impl Path {
        pub fn new(request_code:&str) -> Self {
            match request_code {
                "GET / HTTP/1.1" => Self::Bin,
                "GET /ABOUT HTTP/1.1" =>Self::About,
                _ => Self::NotFound
            }
        }
        pub fn router(&self) -> &str {
            match self {
                Self::Bin => "bin.html",
                Self::About => "about.html",
                Self::NotFound => "404.html"
            }
        }
        
    }
    
}
