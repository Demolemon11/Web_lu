use crate::http::httpmethod::Method;
use std::{collections::HashMap, iter::zip};
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    _version: String,
    _headers: HashMap<String, String>,
    _msg_body: Option<String>,
}
impl HttpRequest {
    pub async fn build(s: String) -> Self {
        let mut s_iter = s.lines();
        let f_line = s_iter.next().unwrap();
        let mut f_line_iter = f_line.split(' ');

        let method = f_line_iter.next().unwrap().into();
        let path = f_line_iter.next().unwrap().to_string();
        let _version = f_line_iter.next().unwrap().to_string();

        let vec = s_iter
            .flat_map(|item| item.split(": "))
            .enumerate()
            .collect::<Vec<_>>();

        let _headers = zip(
            vec.iter()
                .filter(|(index, _)| index % 2 == 0)
                .map(|(_, key)| key.to_string()),
            vec.iter()
                .filter(|(index, _)| index % 2 != 0)
                .map(|(_, value)| value.to_string()),
        )
        .collect::<HashMap<_, _>>();

        let _msg_body = None;

        Self {
            method,
            path,
            _version,
            _headers,
            _msg_body,
        }
    }
}
