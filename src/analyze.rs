use std::collections::HashMap;

/// Very naive keyword extraction: count word frequencies.
pub fn summarize(text: &str) -> (Vec<String>, Vec<String>) {
    let cleaned = text.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), " ");
    let words: Vec<&str> = cleaned.split_whitespace().collect();

    let mut freq: HashMap<&str, usize> = HashMap::new();
    for w in words {
        if w.len() > 3 { // skip short words
            *freq.entry(w).or_insert(0) += 1;
        }
    }

    // Top 5 keywords
    let mut keywords: Vec<(String, usize)> = freq.into_iter()
        .map(|(w, c)| (w.to_string(), c))
        .collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1));
    let keywords: Vec<String> = keywords.into_iter().take(5).map(|(w, _)| w).collect();

    // Naive summarization: pick 3 longest sentences
    let mut sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?').collect();
    sentences.sort_by_key(|s| -(s.len() as isize));
    let summary: Vec<String> = sentences.into_iter().take(3).map(|s| s.trim().to_string()).collect();

    (keywords, summary)
}
