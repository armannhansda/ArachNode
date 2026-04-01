use redis::AsyncCommands;

pub async fn add_if_not_seen(client: &redis::Client, url: &str) -> bool {
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    let added: i32 = con.sadd("seen_urls", url).await.unwrap();

    added == 1
}
