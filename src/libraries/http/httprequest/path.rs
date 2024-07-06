#[derive(PartialEq, Eq, Debug)]
pub enum Path {
    Js,
    Css,
    Root,
    DataBase,
    Sources,
    Count,
    Unknown,
}
impl From<&str> for Path {
    fn from(path: &str) -> Self {
        match path {
            "/js/main.js" => Self::Js,
            "/public.css" => Self::Css,
            "/" => Self::Root,
            "/db" => Self::DataBase,
            "/src" => Self::Sources,
            "/count" => Self::Count,
            _ => Self::Unknown,
        }
    }
}
