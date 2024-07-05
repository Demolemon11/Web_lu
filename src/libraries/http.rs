use self::{
    httprequest::{method::Method, path::Path, HttpRequest},
    httpresponse::HttpResponse,
};
use std::{fs, net::TcpStream};
pub mod httprequest;
pub mod httpresponse;
pub struct Connection {
    pub request: HttpRequest,
    pub response: HttpResponse,
}
impl Connection {
    pub fn new(stream: &TcpStream) -> Self {
        let request = HttpRequest::new(stream);
        let response = HttpResponse::default();
        Self { request, response }
    }
}

impl Connection {
    pub fn standardize_response(mut self) -> Self {
        let mut change_status = |status_code: usize, status_text: &str| {
            self.response.status_code = status_code;
            self.response.status_text = status_text.to_string();
        };
        let mut change_msg_body =
            |path: &str| self.response.msg_body = fs::read_to_string(path).unwrap();

        match self.request.method {
            Method::Get | Method::Put | Method::Post => match self.request.path {
                Path::Root => change_msg_body("front/index.html"),
                Path::DataBase => change_msg_body("front/index.html"),
                Path::Sources => change_msg_body("front/index.html"),
                Path::Count => change_msg_body("front/index.html"),
                Path::Unknown => {
                    change_status(404, "front/index.html");
                    change_msg_body("front/index.html")
                }
            },
            Method::Delete => {}
            Method::Unknown => change_status(400, "Bad Request"),
        };
        self
    }
    pub fn format_response_to_bytes(self) -> Vec<u8> {
        format!(
            "{} {} {}\r\nContent-Type:{}\r\nContent-Length:{}\r\n\r\n{}",
            self.response.version,
            self.response.status_code,
            self.response.status_text,
            "text/html",
            self.response.msg_body.len(),
            self.response.msg_body
        )
        .as_bytes()
        .to_vec()
    }
}
