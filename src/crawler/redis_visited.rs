use redis::AsyncCommands;

pub async fn is_visited(client: &redis::Client, url: &str) -> bool {
    let mut con: redis::aio::MultiplexedConnection =
        match client.get_multiplexed_async_connection().await {
            Ok(con) => con,
            Err(error) => {
                eprintln!("failed to connect to Redis visited set: {error}");
                return false;
            }
        };

    match con.sismember("visited_urls", url).await {
        Ok(exists) => exists,
        Err(error) => {
            eprintln!("failed to check Redis visited set: {error}");
            false
        }
    }
}

pub async fn mark_visited(client: &redis::Client, url: &str) {
    let mut con: redis::aio::MultiplexedConnection =
        match client.get_multiplexed_async_connection().await {
            Ok(con) => con,
            Err(error) => {
                eprintln!("failed to connect to Redis visited set: {error}");
                return;
            }
        };

    if let Err(error) = con.sadd::<_, _, ()>("visited_urls", url).await {
        eprintln!("failed to mark URL visited in Redis: {error}");
    }
}
