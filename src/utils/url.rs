pub fn get_name<S>(url: S) -> String
where
    S: AsRef<str>,
{
    url.as_ref()
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .map(|name| name.strip_suffix(".git").unwrap_or(name))
        .unwrap()
        .to_string()
}

pub fn normalize<S>(url: S) -> String
where
    S: AsRef<str>,
{
    let url = url.as_ref();
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
    fn get_name_from_github_slug() {
        let name = get_name("deorbil/zd-cd");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_https() {
        let name = get_name("https://github.com/deorbil/zd-cd.git");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_https_with_trailing_slash() {
        let name = get_name("https://github.com/deorbil/zd-cd/");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_https_without_suffix() {
        let name = get_name("https://github.com/deorbil/zd-cd");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_ssh() {
        let name = get_name("git@github.com:deorbil/zd-cd.git");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_ssh_without_suffix() {
        let name = get_name("git@github.com:deorbil/zd-cd");
        assert_eq!(name, "zd-cd");
    }

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
