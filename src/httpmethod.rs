pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    Unknown
}
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "DELETE" => Self::DELETE,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            _ => Self::Unknown
        }
    }
}
