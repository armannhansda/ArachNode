use redis::Client;
use reqwest;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

// use crate::index::inverted_index::InvertedIndex;
use crate::crawler::redis_queue::{pop_url, push_url};
use crate::crawler::redis_seen::add_if_not_seen;
use crate::crawler::redis_visited::{is_visited, mark_visited};
use crate::index::graph::LinkGraph;
use crate::index::redis_index::add_document;
use crate::index::tokenizer::tokenizer;
use crate::parser::html::{extract_links, extract_metadata, extract_text};
use crate::parser::robots::fetch_robots_txt;
use crate::storage::file_store::save_page;
use crate::storage::mongo_db::{MongoDB, Page};
use crate::utils::url_utils::{get_domain, get_path};

pub async fn start_workers(
    client: Client,
    seen: Arc<Mutex<HashSet<String>>>,
    domain_last_access: Arc<Mutex<HashMap<String, Instant>>>,
    robots_cache: Arc<Mutex<HashMap<String, Vec<String>>>>,
    crawler_count: Arc<Mutex<usize>>,
    graph: Arc<Mutex<LinkGraph>>,
    mongo: Arc<MongoDB>,
    max_pages: usize,
    worker_count: usize,
) {
    let mut handles = Vec::new();
    let max_empty_polls = 10;

    for _ in 0..worker_count {
        let client = client.clone();
        let seen = Arc::clone(&seen);
        let domain_last_access = Arc::clone(&domain_last_access);
        let robots_cache = Arc::clone(&robots_cache);
        let crawler_count = Arc::clone(&crawler_count);
        let graph = Arc::clone(&graph);
        let mongo = Arc::clone(&mongo);

        let handle = tokio::spawn(async move {
            let mut empty_polls = 0;

            loop {
                // =========================
                // 1. Get URL from queue
                // =========================
                let url = pop_url(&client).await;

                let url = match url {
                    Some(u) => {
                        empty_polls = 0;
                        u
                    }
                    None => {
                        empty_polls += 1;
                        if empty_polls >= max_empty_polls {
                            break;
                        }
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                };

                // =========================
                // 2. Check visited
                // =========================
                if is_visited(&client, &url).await {
                    continue;
                }

                // Stop quickly once we already have enough completed pages.
                {
                    let count = crawler_count.lock().await;
                    if *count >= max_pages {
                        break;
                    }
                }

                // =========================
                // 3. Domain validation
                // =========================
                let domain = match get_domain(&url) {
                    Some(d) => d,
                    None => continue,
                };

                println!("Crawling {}", url);

                // =========================
                // 4. Politeness (rate limit)
                // =========================
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

                let blocked = disallowed_paths
                    .iter()
                    .any(|rule| !rule.is_empty() && path.starts_with(rule));

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


                // ==================
                //  store in mongoDB
                // ==================
                let text = extract_text(&body);
                let (title, description) = extract_metadata(&body);

                mongo
                    .insert_page(Page {
                        url: url.clone(),
                        title,
                        description,
                        content: text.clone(),
                    })
                    .await;

                // =========================
                // 7. Reserve a completed-page slot
                // =========================
                let page_id = {
                    let mut count = crawler_count.lock().await;
                    if *count >= max_pages {
                        break;
                    }
                    *count += 1;
                    *count
                };

                mark_visited(&client, &url).await;

                // =========================
                // 8. Indexing (inverted index)
                // =========================
                let text = extract_text(&body);
                let words = tokenizer(&text);

                {
                    add_document(&client, &url, words).await;
                }

                // =========================
                // 9. Store page
                // =========================
                save_page(&body, page_id).await;

                // =========================
                // 10. Extract links
                // =========================
                let new_links = extract_links(&url, &body);
                {
                    let mut g = graph.lock().await;
                    g.add_links(url.clone(), new_links.clone());
                }

                // =========================
                // 11. Add to queue
                // =========================
                for link in new_links {
                    if add_if_not_seen(&client, &link).await {
                        push_url(&client, link.clone()).await;
                        seen.lock().await.insert(link);
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
