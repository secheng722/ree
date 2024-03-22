use hyper::Method;

use crate::handler::Handler;

pub type Router = (Method, String, Handler);
