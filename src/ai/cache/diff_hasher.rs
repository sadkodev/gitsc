use sha2::{Digest, Sha256};

pub fn normalize_diff(diff: &str) -> String {
    diff.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn generate_diff_hash(diff: &str) -> String {
    let normalized_diff = normalize_diff(diff);
    let mut hasher = Sha256::new();
    hasher.update(normalized_diff.as_bytes());
    format!("{:x}", hasher.finalize())
}
