pub fn normalize(url: &str) -> String {
    if url.contains(':') {
        url.to_string()
    } else {
        format!("https://github.com/{}.git", url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_github_slug() {
        let normalized_url = normalize("deorbil/zd-cd");
        assert_eq!(normalized_url, "https://github.com/deorbil/zd-cd.git");
    }

    #[test]
    fn normalize_https() {
        let normalized_url = normalize("https://github.com/deorbil/zd-cd.git");
        assert_eq!(normalized_url, "https://github.com/deorbil/zd-cd.git");
    }

    #[test]
    fn normalize_ssh() {
        let normalized_url = normalize("git@github.com:deorbil/zd-cd.git");
        assert_eq!(normalized_url, "git@github.com:deorbil/zd-cd.git");
    }
}
