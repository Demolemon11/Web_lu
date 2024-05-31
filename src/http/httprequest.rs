pub mod method;
pub mod path;
use self::method::Method;
use self::path::Path;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    iter::zip,
    net::TcpStream,
};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub path: Path,
    _version: String,
    _headers: HashMap<String, String>,
}
impl PartialEq for HttpRequest {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method && self.path == other.path && self._version == other._version
    }
}
impl HttpRequest {
    pub fn new(mut stream: &TcpStream) -> Self {
        let mut iter = BufReader::new(&mut stream).lines().map_while(|i| i.ok());
        let firstline = iter.next().unwrap();
        let mut firstline = firstline.split(' ');
        let method = firstline.next().unwrap().into();
        let path = firstline.next().unwrap().into();
        let _version = firstline.next().unwrap().to_string();
        let mut key = Vec::with_capacity(3);
        let mut val = Vec::with_capacity(3);
        iter.filter(|item| {
            item.contains("User-Agent") || item.contains("Host") || item.contains("Query")
        })
        .for_each(|item| {
            let mut ii = item.split(": ");
            key.push(ii.next().unwrap().to_string());
            val.push(ii.next().unwrap().to_string());
        });
        let _headers: HashMap<_, _> = zip(key, val).collect();
        println!("{:?} {:?} {}{:?}", method, path, _version, _headers);
        Self {
            method,
            path,
            _version,
            _headers,
        }
    }
}
#[cfg(test)]
mod test {}
