mod crawler;
mod index;
mod parser;
mod storage;
mod utils;

use redis::Client;
use std::collections::{HashMap, HashSet};
use std::io;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use crawler::worker::start_workers;
// use index::inverted_index::InvertedIndex;
use crawler::redis_queue::push_url;
use index::graph::LinkGraph;
use index::redis_index::search;
use std::env;
use storage::mongo_db::MongoDB;
use utils::url_utils::normalize_url;
use dotenv::dotenv;

fn default_seed_urls() -> Vec<String> {
    vec![
        "https://www.wikipedia.org".to_string(),
        "https://news.ycombinator.com".to_string(),
        "https://github.com".to_string(),
        "https://www.bbc.com".to_string(),
        "https://developer.mozilla.org".to_string(),
    ]
}

fn parse_seed_urls(value: &str) -> Vec<String> {
    value
        .split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .filter_map(|seed| {
            let seed = seed.trim();
            if seed.is_empty() {
                None
            } else {
                normalize_url(seed)
            }
        })
        .collect()
}  

fn load_seed_urls() -> Vec<String> {
    let seeds = env::var("SEED_URLS")
        .ok()
        .or_else(|| env::var("SEED_URL").ok())
        .map(|value| parse_seed_urls(&value))
        .unwrap_or_default();

    if seeds.is_empty() {
        default_seed_urls()
    } else {
        seeds
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let seen = Arc::new(Mutex::new(HashSet::new()));
    let domain_last_access = Arc::new(Mutex::new(HashMap::<String, Instant>::new()));
    let robots_cache = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
    let crawler_count = Arc::new(Mutex::new(0));

    // let index = Arc::new(Mutex::new(InvertedIndex::new()));

    let graph = Arc::new(Mutex::new(LinkGraph::new()));

    let redis_client = Client::open("redis://127.0.0.1/").unwrap();

    let mongo = Arc::new(MongoDB::init().await);

    let seed_urls = load_seed_urls();

    println!("Queueing {} seed URL(s):", seed_urls.len());

    for seed_url in seed_urls {
        println!("  {seed_url}");
        push_url(&redis_client, seed_url.clone()).await;
        seen.lock().await.insert(seed_url);
    }

    start_workers(
        redis_client.clone(),
        seen,
        domain_last_access,
        robots_cache,
        crawler_count,
        graph.clone(),
        mongo.clone(),
        1000, // max pages
        10,   // workers
    )
    .await;

    let g = graph.lock().await;
    let pagerank = g.compute_pagerank(20, 0.85);

    println!("Enter search query:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut results = search(&redis_client, input.trim()).await;

    for (url, score) in results.iter_mut() {
        if let Some(pr) = pagerank.get(url) {
            *score += pr;
        }
    }

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Search results:");

    for (url, score) in results {
        println!("{} (score: {:<10.4})", url, score);
    }
}
