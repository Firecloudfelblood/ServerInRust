
pub mod request {
    use crate::method::method::Method;

    pub struct Request {
        path: String,
        query_string: Option<String>, //null
        method: Method,
    }
}
