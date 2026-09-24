#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok,
    Created,
    NoContent,

    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,

    InternalServerError,
    NotImplemented,
    BadGateway,
}
