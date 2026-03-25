use reqwest;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use redis::Client;

use crate::index::inverted_index::InvertedIndex;
use crate::index::tokenizer::tokenizer;
use crate::parser::html::{extract_links, extract_text};
use crate::parser::robots::fetch_robots_txt;
use crate::storage::file_store::save_page;
use crate::utils::url_utils::{get_domain, get_path};
use crate::index::graph::LinkGraph;
use crate::crawler::redis_queue::{push_url,pop_url};

pub async fn start_workers(
    client: Client,
    visited: Arc<Mutex<HashSet<String>>>,
    seen: Arc<Mutex<HashSet<String>>>,
    domain_last_access: Arc<Mutex<HashMap<String, Instant>>>,
    robots_cache: Arc<Mutex<HashMap<String, Vec<String>>>>,
    crawler_count: Arc<Mutex<usize>>,
    index: Arc<Mutex<InvertedIndex>>,
    graph: Arc<Mutex<LinkGraph>>,
    max_pages: usize,
    worker_count: usize,
) {
    let mut handles = Vec::new();

    for _ in 0..worker_count {
        let client = client.clone();
        let visited = Arc::clone(&visited);
        let seen = Arc::clone(&seen);
        let domain_last_access = Arc::clone(&domain_last_access);
        let robots_cache = Arc::clone(&robots_cache);
        let crawler_count = Arc::clone(&crawler_count);
        let index = Arc::clone(&index);
        let graph = Arc::clone(&graph);
        let handle = tokio::spawn(async move {
            loop {
                // =========================
                // 1. Get URL from queue
                // =========================
                let url = pop_url(&client).await;

                let url = match url {
                    Some(u) => u,
                    None => {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                };

                // =========================
                // 2. Check visited
                // =========================
                {
                    let mut v = visited.lock().await;
                    if v.contains(&url) {
                        continue;
                    }
                    v.insert(url.clone());
                }

                // =========================
                // 3. Stop condition
                // =========================
                {
                    let mut count = crawler_count.lock().await;
                    if *count >= max_pages {
                        break;
                    }
                    *count += 1;
                }

                println!("Crawling {}", url);

                // =========================
                // 4. Politeness (rate limit)
                // =========================
                let domain = match get_domain(&url) {
                    Some(d) => d,
                    None => continue,
                };

                let delay = Duration::from_millis(1000);

                loop {
                    let mut map = domain_last_access.lock().await;

                    if let Some(last_time) = map.get(&domain) {
                        if last_time.elapsed() < delay {
                            drop(map);
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            continue;
                        }
                    }

                    map.insert(domain.clone(), Instant::now());
                    break;
                }

                // =========================
                // 5. robots.txt check
                // =========================
                let path = match get_path(&url) {
                    Some(p) => p,
                    None => continue,
                };

                let disallowed_paths = {
                    let mut cache = robots_cache.lock().await;

                    if !cache.contains_key(&domain) {
                        let rules = fetch_robots_txt(&domain).await;
                        cache.insert(domain.clone(), rules);
                    }

                    cache.get(&domain).cloned().unwrap_or_default()
                };

                let blocked = disallowed_paths.iter().any(|rule| {
                    !rule.is_empty() && path.starts_with(rule)
                });

                if blocked {
                    println!("Blocked by robots.txt: {}", url);
                    continue;
                }

                // =========================
                // 6. Fetch with retries
                // =========================
                let mut retries = 0;
                let max_retries = 3;

                let body = loop {
                    if retries >= max_retries {
                        println!("Failed: {}", url);
                        break None;
                    }

                    match reqwest::get(&url).await {
                        Ok(resp) => match resp.text().await {
                            Ok(text) => break Some(text),
                            Err(_) => {}
                        },
                        Err(_) => {}
                    }

                    retries += 1;
                };

                let body = match body {
                    Some(b) => b,
                    None => continue,
                };

                // =========================
                // 7. Indexing (inverted index) 
                // =========================
                let text = extract_text(&body);
                let words = tokenizer(&text);

                {
                    let mut idx = index.lock().await;
                    idx.add_document(url.clone(), words);
                }

                // =========================
                //  8.Store page
                // =========================
                let len = visited.lock().await.len();
                save_page(&body, len).await;

                // =========================
                // 9. Extract links
                // =========================
                let new_links = extract_links(&url, &body);
                {
                    let mut g = graph.lock().await;
                    g.add_links(url.clone(), new_links.clone());
                }

                // =========================
                // 10. Add to queue
                // =========================
                let mut s = seen.lock().await;

                for link in new_links {
                    if !s.contains(&link) {
                        s.insert(link.clone());
                        push_url(&client, link).await;
                    }
                }
            }
        });

        handles.push(handle);
    }

    // wait for all workers
    for handle in handles {
        handle.await.unwrap();
    }
}
