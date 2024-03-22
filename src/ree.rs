use crate::router::Router;

#[derive(Debug, Clone)]
pub struct Ree {
    routers: Vec<Router>,
}

impl Ree {
    pub fn new() -> Self {
        Self {
            routers: Vec::new(),
        }
    }

    pub fn routers(&self) -> &Vec<Router> {
        &self.routers
    }

    pub fn add_route(
        &mut self,
        method: hyper::Method,
        path: &str,
        handler: crate::handler::Handler,
    ) {
        self.routers.push((method, path.to_string(), handler));
    }

    pub fn get(&mut self, path: &str, handler: crate::handler::Handler) {
        self.add_route(hyper::Method::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: crate::handler::Handler) {
        self.add_route(hyper::Method::POST, path, handler);
    }
}
