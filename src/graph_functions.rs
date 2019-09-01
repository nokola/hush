use redis:: { 
    // Commands, 
    // RedisError,
    RedisFuture,
};
use futures::future::{ ok, Future };

pub fn fetch_an_integer_async() -> RedisFuture<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let connect = client.get_async_connection();
    // let result: RedisFuture<isize> = connect
    //     .and_then(|con| {
    //         redis::cmd("SET").arg("my_key").arg(42).query_async(con).map(|t| t.0)
    //     })
    //     .then(|res| {
    //         Ok(4)
    //     });
    Box::new(ok(45))


        // .and_then(|con| {
        //     redis::cmd("GRAPH.QUERY").arg("social")
        //         .arg("CREATE (:person {name:'roi', age:33, gender:'male', status:'married'})").query_async(con)
        // })
        // .and_then(|con| {
        //     redis::cmd("GET").arg("my_key").query_async(con)
        // })
    // https://oss.redislabs.com/redisgraph/
}
