use std::collections::HashMap;
use std::fs;
use crate::httpmethod::Method;
use crate::httprequest::HttpRequest;
use crate::httpresponse::HttpResponse;

use crate::constents::{CONTENT_LENGTH, CONTENT_TYPE, UNKNOWN};

pub async fn processer(request:HttpRequest,response:&mut HttpResponse<'_>) -> String {
    let mut get_post_closure = |filepath:&str|{
        response.msg_body = fs::read_to_string(filepath).unwrap();
        let mut map = HashMap::new();
        map.insert(CONTENT_TYPE, "text/html".to_string());
        map.insert(CONTENT_LENGTH, response.msg_body.len().to_string());
        response.headers = map

    };
    let delete_closure = ||{
        ()
    };
    let put_closure = ||{
        ()
    };


    match request.method {
        
        Method::GET => {
            match &request.path[..] {
                "/" => {
                    get_post_closure("root.html")
                }

                "/about" => {
                    get_post_closure("about.html")
                },

                _ => {

                    get_post_closure("404.html")

                }
            }
        }
        Method::DELETE => {
            match &request.path[..] {
                "/data" => delete_closure(),
                _ => {}
            }
        }

        Method::POST => {
            match &request.path[..] {
                "/" => {
                    get_post_closure("root.html")
                }
                "/about" => {
                    get_post_closure("about.html")
                }
                _ => {
                    get_post_closure("404.html")
                }
            }

        }
        Method::PUT => {
            match &request.path[..] {
                "/data" => {
                    put_closure()
                }
                _ => {}
            }
        }
        Method::Unknown => {
            response.status_text = "Bad Request";
            response.status_code = 400;
        }
    }
    
    format!
    (
        "{} {} {}\r\nContent-Type:{}\r\nContent-Length:{}\r\n\r\n{}"
        ,
        response.version,
        response.status_code,
        response.status_text,
        response.headers.get("ct").unwrap_or(&UNKNOWN.to_string()),
        response.headers.get("cl").unwrap_or(&UNKNOWN.len().to_string()),
        response.msg_body
    )

}