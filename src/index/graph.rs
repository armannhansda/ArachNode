use std::collections::{HashMap, HashSet};

pub struct LinkGraph {
  pub edges: HashMap<String, HashSet<String>>,
}

impl LinkGraph {
  pub fn new() ->Self {
    Self {
      edges: HashMap::new(),
    }
  }

  pub fn add_links(&mut self, from: String, to_links: Vec<String>) {
    let entry = self.edges.entry(from).or_insert(HashSet::new());

    for link in to_links {
      entry.insert(link);
    }
  }

  pub fn get_links(&self, url: &String) -> Option<&HashSet<String>> {
    self.edges.get(url)
  }

  pub fn compute_pagerank(&self, iterations: usize, d: f64) -> HashMap<String, f64> {
    let mut ranks = HashMap::new();

    let pages: Vec<String> = self.edges.keys().cloned().collect();

    //initialize ranks
    for page in &pages {
      ranks.insert(page.clone(), 1.0);
    }

    for _ in 0..iterations {
      let mut new_ranks = HashMap::new();

      for page in &pages {
        let mut rank_sum = 0.0;

        for (other_page, links) in &self.edges {
          if links.contains(page) {
            let out_degree = links.len() as f64;

            if out_degree > 0.0 {
              rank_sum += ranks.get(other_page).unwrap_or(&0.0) / out_degree;
            }
          }
        }

        let new_rank = (1.0 - d) + d *rank_sum;
        new_ranks.insert(page.clone(), new_rank);
      }

      ranks = new_ranks;

    }
    ranks
  }
}