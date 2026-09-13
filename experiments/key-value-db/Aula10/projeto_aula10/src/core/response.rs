#[derive(Debug)]
pub enum Response {
    Ok,
    Value(String),
    NotFound,
    Error(String),
}