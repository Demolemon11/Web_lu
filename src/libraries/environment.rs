#[derive(PartialEq, Eq, Debug)]
pub enum Parameter {
    Run(u8),
    Help,
    Unknown,
}
impl From<Vec<String>> for Parameter {
    fn from(parameter: Vec<String>) -> Self {
        let mut parameter = parameter.into_iter();
        match parameter.nth(1) {
            Some(t) => match t.as_str() {
                "run" => match parameter.next() {
                    Some(u) => match u.parse() {
                        Ok(v) => Self::Run(v),
                        Err(_) => panic!("The count of thread that you type is not a number!"),
                    },
                    None => {
                        println!("Server has run 4 as a default count of threads\nType -h | --help to custom count.");
                        Self::Run(4)
                    }
                },
                "-h" | "--help" => Self::Help,
                _ => Self::Unknown,
            },

            None => {
                panic!("Err usage, Please type -h | --help to see correct usage.")
            }
        }
    }
}
