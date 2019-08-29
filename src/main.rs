use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/test")]
fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn main() {
    println!("Listening on http://localhost:3000");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new()
        .service(index)
        .service(index2));
        // .route("/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
}