mod crawler;
mod parser;
mod storage;
mod utils;
mod index;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use std::{io};
use redis::Client;

use crawler::worker::start_workers;
// use index::inverted_index::InvertedIndex;
use index::graph::LinkGraph;
use crawler::redis_queue::push_url;
use std::env;
use index::redis_index::search;

#[tokio::main]
async fn main() {
    let seen = Arc::new(Mutex::new(HashSet::new()));
    let domain_last_access = Arc::new(Mutex::new(HashMap::<String, Instant>::new()));
    let robots_cache = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
    let crawler_count = Arc::new(Mutex::new(0));

    // let index = Arc::new(Mutex::new(InvertedIndex::new()));

    let graph = Arc::new(Mutex::new(LinkGraph::new()));

    let redis_client = Client::open("redis://127.0.0.1/").unwrap();

    let seed_url = env::var("SEED_URL")
        .unwrap_or_else(|_| "https://doc.rust-lang.org/book/".to_string());

    push_url(&redis_client, seed_url.clone()).await;
    seen.lock().await.insert(seed_url);

    start_workers(
        redis_client.clone(),
        seen,
        domain_last_access,
        robots_cache,
        crawler_count,
        graph.clone(),
        100, // max pages
        50,   // workers
    ).await;

    let g = graph.lock().await;
    let pagerank = g.compute_pagerank(20, 0.85);


    println!("Enter search query:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut  results = search(&redis_client, input.trim()).await;

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
