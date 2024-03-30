use std::{collections::HashMap,iter::zip};
use crate::httpmethod::Method;
pub struct HttpRequest {
    pub method:Method,
    pub path:String,
    version:String,
    headers:HashMap<String,String>,
    msg_body:Option<String>
}
impl HttpRequest {
    pub async fn build(s:String) -> Self {
        
        let mut s_iter = s.lines();
        let f_line = s_iter.next().unwrap();
        let mut f_line_iter = f_line.split(" ");

        let method = f_line_iter.next().unwrap().into();
        let path = f_line_iter.next().unwrap().to_string();
        let version = f_line_iter.next().unwrap().to_string();

        let vec = s_iter
        .map(|item|{item.split(": ")})
        .flatten()
        .enumerate()
        .collect::<Vec<_>>();
        

        let headers = zip
        (
        vec
        .iter()
        .filter(|(index,_)|{index%2==0})
        .map(|(_,key)|{key.to_string()})

        ,
        
        vec
        .iter()
        .filter(|(index,_)|{index%2!=0})
        .map(|(_,value)|{value.to_string()})
        )
        
        .collect::<HashMap<_,_>>();

        let msg_body = None; 
        Self {
            method,
            path,
            version,
            headers,
            msg_body
        }

    }
}