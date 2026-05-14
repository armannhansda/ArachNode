use redis::AsyncCommands;

pub async fn push_url(client: &redis::Client, url: String) {
    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(con) => con,
        Err(error) => {
            eprintln!("failed to connect to Redis queue: {error}");
            return;
        }
    };

    if let Err(error) = con.lpush::<_, _, ()>("url_queue", url).await {
        eprintln!("failed to push URL to Redis queue: {error}");
    }
}

pub async fn pop_url(client: &redis::Client) -> Option<String> {
    let mut conn = match client.get_multiplexed_async_connection().await {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("failed to connect to Redis queue: {error}");
            return None;
        }
    };

    match conn.rpop("url_queue", None).await {
        Ok(url) => url,
        Err(error) => {
            eprintln!("failed to pop URL from Redis queue: {error}");
            None
        }
    }
}
