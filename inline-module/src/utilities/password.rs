pub fn hash(text: &str) -> String {
    let hash = bcrypt::hash(text, bcrypt::DEFAULT_COST).unwrap();
    return hash.to_string();
}

pub fn is_valid(text: &str, hash: &str) -> bool {
    let is_valid = bcrypt::verify(text, hash).unwrap();
    return is_valid;
}