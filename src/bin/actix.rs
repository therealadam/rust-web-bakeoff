extern crate actix_web;
extern crate env_logger;

use actix_web::{ server, App, HttpRequest, middleware::Logger, fs::StaticFiles };
use actix_web::actix;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    std::env::set_var("RUST_LOG", "actix=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("hello");

    server::new(||
        App::new()
            .middleware(Logger::default())
            .resource("/", |r| r.f(index))
            .handler("/", StaticFiles::new("./static/").unwrap())
    )
        .bind("0.0.0.0:3000")
        .unwrap()
        .run();

    println!("started actix on 0.0.0.0:3000");
    let _ = sys.run();
}