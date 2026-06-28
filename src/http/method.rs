
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
}
