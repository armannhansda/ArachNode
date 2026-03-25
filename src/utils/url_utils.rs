use url::Url;

pub fn normalize_url(link: &str) -> Option<String> {
    if let Ok(mut url) = Url::parse(link) {
        // remove fragement (#section)
        url.set_fragment(None);

        //remove query parameter (?ref=..)
        url.set_query(None);

        //normalize trailing slash
        let mut normalized = url.to_string();

        if normalized.ends_with('/') {
            normalized.pop();
        }
        return Some(normalized);
    }
    None
}

pub fn get_domain(url: &str) -> Option<String> {
    if let Ok(parsed) = Url::parse(url) {
        return parsed.host_str().map(|s| s.to_string());
    }
    None
}

pub fn get_path(url: &str) -> Option<String> {
    if let Ok(parsed) = Url::parse(url) {
        return Some(parsed.path().to_string());
    }
    None
}