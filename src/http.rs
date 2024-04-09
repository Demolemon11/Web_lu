pub mod httpmethod;
pub mod httprequest;
pub mod httpresponse;
use crate::http::httprequest::HttpRequest;
use crate::http::httpresponse::HttpResponse;
pub struct Http {
    pub request: HttpRequest,
    pub response: HttpResponse,
}
impl Http {
    pub fn new(request: HttpRequest, response: HttpResponse) -> Self {
        Self { request, response }
    }
}
