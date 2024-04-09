use std::collections::HashMap;

pub struct HttpResponse {
    pub version: String,
    pub status_code: usize,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}
impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::new(),
            msg_body: "None".to_string(),
        }
    }
}
