use std::collections::HashMap;

use crate::{HttpMethod, HttpResponse};

type ViewFunction = Box<dyn Fn() -> HttpResponse>;
type Route = (HttpMethod, String);

pub struct HttpRouter {
    routes: HashMap<Route, ViewFunction>,
}

impl HttpRouter {
    pub fn new() -> Self {
        let routes = HashMap::new();
        HttpRouter { routes }
    }

    fn route(&mut self, method: HttpMethod, route: &str, view: ViewFunction) {
        self.routes.insert((method, route.to_string()), view);
    }

    pub fn get(&mut self, route: &str, view: ViewFunction) {
        self.route(HttpMethod::Get, route, view);
    }

    pub fn post(&mut self, route: &str, view: ViewFunction) {
        self.route(HttpMethod::Post, route, view);
    }

    pub fn put(&mut self, route: &str, view: ViewFunction) {
        self.route(HttpMethod::Put, route, view);
    }

    pub fn patch(&mut self, route: &str, view: ViewFunction) {
        self.route(HttpMethod::Patch, route, view);
    }

    pub fn delete(&mut self, route: &str, view: ViewFunction) {
        self.route(HttpMethod::Delete, route, view);
    }

    pub fn options(&mut self, route: &str, view: ViewFunction) {
        self.route(HttpMethod::Options, route, view);
    }

    pub fn get_route(&self, route: Route) -> Option<&ViewFunction> {
        self.routes.get(&route)
    }
}
