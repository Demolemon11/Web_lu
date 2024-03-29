use std::collections::HashMap;

pub struct HttpResponse<'a> {
    pub version:&'a str,
    pub status_code:usize,
    pub status_text:&'a str,
    pub headers:HashMap<&'a str,String>,
    pub msg_body:String
}
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {

        Self {
            
            version:"HTTP/1.1",
            status_code:200,
            status_text:"OK",
            headers:HashMap::new(),
            msg_body:"None".to_string()
        }

    }
}