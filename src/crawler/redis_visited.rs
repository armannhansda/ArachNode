use redis::AsyncCommands;

pub async fn is_visited(client: &redis::Client, url: &str) -> bool {
  let mut con: redis::aio::MultiplexedConnection = client.get_multiplexed_async_connection().await.unwrap();

  let exists: bool = con.sismember("visited_urls", url).await.unwrap();
  exists
}

pub async fn mark_visited(client: &redis::Client, url: &str) {
  let mut con: redis::aio::MultiplexedConnection = client.get_multiplexed_async_connection().await.unwrap();

  let _: () = con.sadd("visited_urls", url).await.unwrap();
}