#[derive(PartialEq, Eq, Debug)]
pub enum Path {
    Root,
    DataBase,
    Sources,
    Count,
    Unknown,
}
impl From<&str> for Path {
    fn from(path: &str) -> Self {
        match path {
            "/" => Self::Root,
            "/db" => Self::DataBase,
            "/src" => Self::Sources,
            "/count" => Self::Count,
            _ => Self::Unknown,
        }
    }
}
