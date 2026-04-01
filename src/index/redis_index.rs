use redis::AsyncCommands;
use std::collections::HashMap;

pub async fn add_document(client: &redis::Client, url: &str, words: Vec<String>) {
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    let mut word_counts: HashMap<String, usize> = HashMap::new();

    //count the frequency of each word in the document
    for word in words {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    //store the word counts in Redis
    for (word, count) in word_counts {
        let _: () = con
            .hincr(format!("index: {}", word), url, count as i32)
            .await
            .unwrap();
    }

    // track total document

    let _: () = con.incr("total_docs", 1).await.unwrap();
}

pub async fn search(client: &redis::Client, query: &str) -> Vec<(String, f64)> {
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    let total_docs: f64 = con.get("total_docs").await.unwrap_or(1.0);

    let mut scores: HashMap<String, f64> = HashMap::new();

    for word in query.split_whitespace() {
        let key = format!("index: {}", word.to_lowercase());
        let docs_map: HashMap<String, usize> = con.hgetall(key).await.unwrap_or_default();

        let df = docs_map.len() as f64;
        let idf = (total_docs / (1.0 + df)).ln().max(0.0);

        for (url, tf) in docs_map {
            let score = (tf as f64) * idf;
            *scores.entry(url).or_insert(0.0) += score;
        }
    }

    let mut results: Vec<(String, f64)> = scores.into_iter().collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    results
}
