use handler::{Handler, ErrorHandler};
use http::{Method, ContentType};

pub trait StaticRouteInfo {
    fn method(&self) -> Method;
    fn path(&self) -> &'static str;
    fn format(&self) -> Option<ContentType> {
        None
    }
    fn handler(&self) -> Handler;
    fn rank(&self) -> Option<isize> {
        None
    }
}

pub trait StaticCatchInfo {
    fn code(&self) -> u16;
    fn handler(&self) -> ErrorHandler;
}
