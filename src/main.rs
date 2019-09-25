use actix_web::{
    error,
    web,
    // Either,
    App,
    Error,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};
use failure::Fail;
use futures::future::{
    err,
    ok,
    // result,
    Future,
    // IntoFuture,
};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};

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
// type FutureHttpResponse = Box<Future<Item=HttpResponse, Error=HttpError>>;

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

// type RegisterResult = Box<dyn Future<Item = HttpResponse, Error = HttpResponse>>;

fn return_json(info: web::Path<Info>) -> Box<dyn Future<Item = MyObj, Error = HttpResponse>> {
    if info.friend != "hoho5" {
        return Box::new(err(HttpResponse::Unauthorized().body(format!(
            "Unauthorized: {}. Only hoho5 is authorized",
            info.friend
        )))); // WORKING with HttpRespose
    }

    // WORKING for impl Responder:
    // HttpResponse::Unauthorized().body(format!("Unauthorized: {}", info.friend))
    // "all good"
    // format!("Unauthorized: !! {}", info.friend).with_status(StatusCode::UNAUTHORIZED)
    // ...do not return anything will return "200 OK"...

    let result2: redis::RedisFuture<isize> = graph_functions::fetch_an_integer_async();
    let result_final = result2
        .and_then(move |data| {
            ok(MyObj {
                id: data as u32,
                name: info.into_inner().friend,
                // on into_inner() from https://users.rust-lang.org/t/explanation-of-into-inner/13872, also see https://www.reddit.com/r/rust/comments/d4v6at/hey_rustaceans_got_an_easy_question_ask_here/f15unks/
                // This is not related to interior mutability. into_inner() is simply (by convention) a method that consumes self and returns an inner, 
                // “wrapped” object. In this case, the BufWriter wraps the stdout.
                // It moves it into itself in new, so here (as is often the case) into_inner is kind of the reverse of new.
                // In general, Rust methods are named into_something when they consume self, avoiding clones as much as possible, and to_something when they take &self, potentially cloning some data.
            })
        })
        .or_else(|redis_error| {
            err(HttpResponse::InternalServerError().body(redis_error.to_string()))
        });

    Box::new(result_final) // WORKING

    // Box::new(err(error::ErrorInternalServerError("test"))) // WORKING with actix_web::Error
}

fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

fn index2(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello again {}!", app_name))
}

fn main() {
    println!("Listening on http://localhost:3000");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("test"),
            })
            .route("/", web::to(index))
            .service(
                web::scope("/api").route("view/{user_id}/{friend}", web::get().to(return_json)),
            )
            .service(web::scope("/app").route("/index.html", web::get().to(index2)))
    });
    // .route("/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
}

// interesting example for Futures:
// https://github.com/DoumanAsh/roseline.rs/blob/master/web/src/server/mod.rs

// graphbase.io - serverless graph applications, blazing fast
