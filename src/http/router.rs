use std::collections::HashMap;

use crate::{HttpMethod, HttpResponse};

type RequestHandler = Box<dyn Fn() -> HttpResponse>;
type Route = (HttpMethod, String);
type RoutesMap = HashMap<Route, RequestHandler>;

pub struct HttpRouter {
    routes: RoutesMap,
}

impl HttpRouter {
    pub fn new() -> Self {
        let routes = HashMap::new();
        HttpRouter { routes }
    }

    fn route<F>(&mut self, method: HttpMethod, route: &str, view: F)
    where
        F: Fn() -> HttpResponse + 'static,
    {
        self.routes
            .insert((method, route.to_string()), Box::new(view));
    }

    pub fn get<F>(&mut self, route: &str, view: F)
    where
        F: Fn() -> HttpResponse + 'static,
    {
        self.route(HttpMethod::Get, route, view);
    }

    pub fn post<F>(&mut self, route: &str, view: F)
    where
        F: Fn() -> HttpResponse + 'static,
    {
        self.route(HttpMethod::Post, route, view);
    }

    pub fn put<F>(&mut self, route: &str, view: F)
    where
        F: Fn() -> HttpResponse + 'static,
    {
        self.route(HttpMethod::Put, route, view);
    }

    pub fn patch<F>(&mut self, route: &str, view: F)
    where
        F: Fn() -> HttpResponse + 'static,
    {
        self.route(HttpMethod::Patch, route, view);
    }

    pub fn delete<F>(&mut self, route: &str, view: F)
    where
        F: Fn() -> HttpResponse + 'static,
    {
        self.route(HttpMethod::Delete, route, view);
    }

    pub fn options<F>(&mut self, route: &str, view: F)
    where
        F: Fn() -> HttpResponse + 'static,
    {
        self.route(HttpMethod::Options, route, view);
    }

    pub fn get_route(&self, route: Route) -> Option<&RequestHandler> {
        self.routes.get(&route)
    }
}
