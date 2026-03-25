use redis::AsyncCommands;

pub async fn push_url(client: &redis::Client, url: String) {
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    let _: () = con.lpush("url_queue", url).await.unwrap();
} 
pub async fn pop_url(client: &redis::Client) -> Option<String> {
  let mut conn = client.get_multiplexed_async_connection().await.unwrap();
  let url: Option<String> = conn.rpop("url_queue",None).await.unwrap();

  url
}