use redis::AsyncCommands;

pub async fn add_if_not_seen(client: &redis::Client, url: &str) -> bool {
    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(con) => con,
        Err(error) => {
            eprintln!("failed to connect to Redis seen set: {error}");
            return false;
        }
    };

    match con.sadd::<_, _, i32>("seen_urls", url).await {
        Ok(added) => added == 1,
        Err(error) => {
            eprintln!("failed to add URL to Redis seen set: {error}");
            false
        }
    }
}
  