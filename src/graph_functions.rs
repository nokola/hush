use futures::future::{err, Future};
use redis::RedisFuture;

pub fn fetch_an_integer_async() -> RedisFuture<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/");
    match client {
        Err(error) => return Box::new(err(error)),
        Ok(_) => (),
    };

    let connect = client.unwrap().get_async_connection();
    let result = connect
        .and_then(|con| redis::cmd("SET").arg("my_key").arg(42).query_async(con))
        .and_then(|(con, ())| {
            redis::cmd("GRAPH.QUERY")
                .arg("social")
                .arg("CREATE (:person {name:'roi', age:33, gender:'male', status:'married'})")
                .query_async(con)
        })
        .and_then(|(con, ())| {
            redis::cmd("GET")
                .arg("my_key")
                .query_async(con)
                .map(|t| t.1)
        });

    // https://oss.redislabs.com/redisgraph/
    Box::new(result)
    // Box::new(ok(45))
}
