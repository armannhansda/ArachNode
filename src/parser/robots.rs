pub async fn fetch_robots_txt(domain: &str) -> Vec<String> {
    let robots_url = format!("https://{}/robots.txt", domain);

    match reqwest::get(&robots_url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                let mut allowed = false;
                let mut rules = Vec::new();
                for line in text.lines() {
                    let line = line.trim();

                    if line.starts_with("User-agent: *") {
                        allowed = true;
                    }else if line.starts_with("User-agent:") {
                        allowed = false;
                    }
                    if allowed && line.starts_with("Disallow:"){
                        let path = line.replace("Disallow:", "").trim().to_string();
                        rules.push(path);
                    }
                }
                rules
            }
            Err(_) => vec![],
        },
        Err(_) => vec![],
    }
}