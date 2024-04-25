pub fn split_string(s: &String, c: char) -> Vec<&str> {
    let parts = s.split(c);
    let collection = parts.collect();
    collection
}
