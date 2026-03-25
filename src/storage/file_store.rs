use tokio::fs;

pub async fn save_page(content: &str, id: usize) {
  let _ = fs::create_dir_all("data").await;
  let filename = format!("data/{}.html", id);
  let _ = fs::write(&filename, content).await;
}
