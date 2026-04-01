// use std::collections::HashMap;


// pub struct InvertedIndex {
//   //word -> {url -> count}
//   pub map: HashMap<String, HashMap<String, usize>>,
//   //total number of documents indexed
//   pub total_docs: usize,
// }

// impl InvertedIndex{
//   pub fn new() -> Self {
//     Self {
//       map: HashMap::new(),
//       total_docs: 0,
//     }
//   }

//   pub fn add_document(&mut self, url:String, words:Vec<String>) {
//     self.total_docs += 1;
//     let mut word_count = HashMap::new();

//     //count word frequency in this document
//     for word in words {
//       *word_count.entry(word).or_insert(0) += 1;
//     }

//     for (word, count) in word_count {
//       let entry = self.map.entry(word).or_insert(HashMap::new());
//       entry.insert(url.clone(), count);
//     }
//   }

//   pub fn search(&self, query: &str) -> Vec<(String, f64)> {
//     let mut scores: HashMap<String,f64> = HashMap::new();

//     for word in query.split_whitespace() {
//       let word = word.to_lowercase();

//       if let Some(doc_map) = self.map.get(&word) {
//         let df = doc_map.len() as f64;
//         let idf = (self.total_docs as f64 / (1.0) + df).ln();

//         for (url,tf) in doc_map {
//           let tf_idf = *tf as f64 * idf;
//           *scores.entry(url.clone()).or_insert(0.0) += tf_idf;
//         }
//       }
//     }
//     let mut results: Vec<(String, f64)> = scores.into_iter().collect();

//     results.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap() );
//     results
//   }
// }