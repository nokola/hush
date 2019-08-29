use actix_web::{
    web, 
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    error, Result};
use failure::Fail;
use listenfd::ListenFd;
use serde::{ Serialize, Deserialize };
mod graph_functions;

struct AppState {
    app_name: String,
}

#[derive(Serialize)]
struct MyObj {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Info {
    user_id: u32,
    friend: String,
}

#[derive(Fail, Debug)]
#[fail(display = "my error")]
pub struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

// Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn return_json(info: web::Path<Info>) -> impl Responder {
    let result: isize = graph_functions::fetch_an_integer().unwrap();
    // TODO: error handling
    MyObj { id: result as u32, name: info.friend.clone() } // TODO: avoid .clone() somehow?
}

fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

fn index2(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello again {}!", app_name))
}

#[rustfmt::skip]
fn main() {
    println!("Listening on http://localhost:3000");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("test")
            })
            .route("/", web::to(index))
            .service(
                web::scope("/api")
                    .route("view/{user_id}/{friend}", web::get().to(return_json))
            )
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index2)))
    });
    // .route("/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
}
