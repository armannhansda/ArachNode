mod crawler;
mod parser;
mod storage;
mod utils;
mod index;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use std::{io};

use crawler::worker::start_workers;
use index::inverted_index::InvertedIndex;
use index::graph::LinkGraph;

#[tokio::main]
async fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let seen = Arc::new(Mutex::new(HashSet::new()));
    let domain_last_access = Arc::new(Mutex::new(HashMap::<String, Instant>::new()));
    let robots_cache = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
    let crawler_count = Arc::new(Mutex::new(0));

    let index = Arc::new(Mutex::new(InvertedIndex::new()));

    let graph = Arc::new(Mutex::new(LinkGraph::new()));

    let seed_url = "https://www.google.com".to_string();

    queue.lock().await.push_back(seed_url.clone());
    seen.lock().await.insert(seed_url);

    start_workers(
        queue,
        visited,
        seen,
        domain_last_access,
        robots_cache,
        crawler_count,
        index.clone(),
        graph.clone(),
        100, // max pages
        50,   // workers
    ).await;

    let g = graph.lock().await;
    let pagerank = g.compute_pagerank(20, 0.85);

    let idx = index.lock().await;

    println!("Enter search query:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut  results = idx.search(input.trim());

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
