#[derive(PartialEq, Eq, Debug)]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
    Unknown,
}
impl From<&str> for Method {
    fn from(method: &str) -> Self {
        match method {
            "GET" => Self::Get,
            "PUT" => Self::Put,
            "POST" => Self::Post,
            "DELETE" => Self::Delete,
            _ => Self::Unknown,
        }
    }
}
