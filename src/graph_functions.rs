use redis::Commands;

pub fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    redis::cmd("SET").arg("my_key").arg(42).execute(&mut con);
    // https://oss.redislabs.com/redisgraph/
    redis::cmd("GRAPH.QUERY").arg("social")
        .arg("CREATE (:person {name:'roi', age:33, gender:'male', status:'married'})").execute(&mut con);

    let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}