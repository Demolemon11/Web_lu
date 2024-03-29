use std::collections::HashMap;
use tokio::fs;
use crate::httpmethod::Method;
use crate::httprequest::HttpRequest;
use crate::httpresponse::HttpResponse;
use crate::constents;
pub async fn router(request:HttpRequest,response:&mut HttpResponse<'_>) -> String {
    match request.method {
        Method::GET => {
            match &request.path[..] {
                "/" => {

                    response.msg_body = fs::read_to_string("root.html").await.unwrap();
                    let mut map = HashMap::new();
                    map.insert("ct", "text/html".to_string());
                    map.insert("cl", response.msg_body.len().to_string());
                    response.headers = map

                }

                "/health" => {
                    
                },

                _ => {
                    response.status_code = 404;
                    response.msg_body = fs::read_to_string("404.html").await.unwrap();
                    let mut map = HashMap::new();
                    map.insert("ct", "text/html".to_string());
                    map.insert("cl", response.msg_body.len().to_string());
                    response.headers = map

                }
            }
        }
        Method::DELETE => {
            match &request.path[..] {
                "/data" => (),
                _ => ()
            }
        }
        Method::POST => {
            match &request.path[..] {
                "/" => {
                    response.msg_body = fs::read_to_string("root.html").await.unwrap();
                    let mut map = HashMap::new();
                    map.insert("ct", "text/html".to_string());
                    map.insert("cl", response.msg_body.len().to_string());
                    response.headers = map

                }
                "/health" => (),
                _ => {
                    response.status_code = 404;
                    response.msg_body = fs::read_to_string("404.html").await.unwrap();
                    let mut map = HashMap::new();
                    map.insert("ct", "text/html".to_string());
                    map.insert("cl", response.msg_body.len().to_string());
                    response.headers = map
                }
            }

        }
        Method::PUT => {
            
        }
        Method::Unknown => {
            response.status_text = "Bad Request";
            response.status_code = 400;
        }
    }
    
    format!
    (
        "{} {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}"
        ,
        response.version,
        response.status_code,
        response.status_text,
        response.headers.get("ct").unwrap_or(&constents::UNKNOWN.to_string()),
        response.headers.get("cl").unwrap_or(&constents::UNKNOWN.len().to_string()),
        response.msg_body
    )

}