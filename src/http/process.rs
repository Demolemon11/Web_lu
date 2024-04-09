use super::Http;
use crate::http::httpmethod::Method;
use std::collections::HashMap;
use std::fs;

impl Http {
    pub async fn processor(&mut self) -> String {
        let mut get_post_closure = |filepath: &str| {
            self.response.msg_body = fs::read_to_string(filepath).unwrap();
            let mut map = HashMap::new();
            map.insert("ct".to_string(), "text/html".to_string());
            map.insert("cl".to_string(), self.response.msg_body.len().to_string());
            self.response.headers = map
        };
        let delete_closure = || {};
        let put_closure = || {};

        match self.request.method {
            Method::GET => match &self.request.path[..] {
                "/" => get_post_closure("root.html"),

                "/about" => get_post_closure("about.html"),

                _ => get_post_closure("404.html"),
            },
            Method::DELETE => match &self.request.path[..] {
                "/data" => delete_closure(),
                _ => {}
            },

            Method::POST => match &self.request.path[..] {
                "/" => get_post_closure("root.html"),
                "/about" => get_post_closure("about.html"),
                _ => get_post_closure("404.html"),
            },
            Method::PUT => match &self.request.path[..] {
                "/data" => put_closure(),
                _ => {}
            },
            Method::Unknown => {
                self.response.status_text = "Bad Request".to_string();
                self.response.status_code = 400;
            }
        }

        format!(
            "{} {} {}\r\nContent-Type:{}\r\nContent-Length:{}\r\n\r\n{}",
            self.response.version,
            self.response.status_code,
            self.response.status_text,
            self.response
                .headers
                .get("ct")
                .unwrap_or(&"UNKNOWN".to_string()),
            self.response
                .headers
                .get("cl")
                .unwrap_or(&"UNKNOWN".to_string()),
            self.response.msg_body
        )
    }
}
