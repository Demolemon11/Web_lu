pub enum Method {
    Get,
    Delete,
    Post,
    Put,
    Unknown,
}
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::Get,
            "DELETE" => Self::Delete,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            _ => Self::Unknown,
        }
    }
}
