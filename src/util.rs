/// Suggest resources (MVP: just Google search links)
pub fn suggest_links(keywords: &[String]) -> Vec<String> {
    keywords.iter()
        .map(|k| format!("https://www.google.com/search?q={}", k.replace(" ", "+")))
        .collect()
}
