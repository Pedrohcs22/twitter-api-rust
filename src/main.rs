extern crate actix_web;
use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse};

// Here is the handler,
// we are returning a json response with an ok status
// that contains the text Hello World
fn index(_req: HttpRequest) -> HttpResponse  {
    HttpResponse::Ok().json("Hello world!")
}

fn main() {
    // We are creating an Application instance and
    // register the request handler with a route and a resource
    // that creates a specific path, then the application instance
    // can be used with HttpServer to listen for incoming connections.
    HttpServer::new(|| App::new().service(
        web::resource("/").route(web::get().to_async(index))))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}