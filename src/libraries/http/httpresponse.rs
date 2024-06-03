use std::collections::HashMap;
pub struct HttpResponse {
    pub version: String,
    pub status_code: usize,
    pub status_text: String,
    headers: HashMap<String, String>,
    pub msg_body: String,
}
impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status_code: 200,
            status_text: String::from("OK"),
            headers: HashMap::with_capacity(1),
            msg_body: String::from(""),
        }
    }
}
