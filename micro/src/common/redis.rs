use redis::RedisError;

pub async fn fetch_async_string() -> Result<String, RedisError> {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_async_connection().await?;

    redis::cmd("SET")
        .arg(&["key1", "bar"])
        .query_async(&mut con)
        .await?;

    let result: Result<String, RedisError> =
        redis::cmd("GET").arg("key1").query_async(&mut con).await;

    return result;
}
