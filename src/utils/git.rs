pub fn get_name_from_url(url: &str) -> String {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .map(|name| name.strip_suffix(".git").unwrap_or(name))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_name_from_https() {
        let name = get_name_from_url("https://github.com/deorbil/zd-cd.git");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_https_with_trailing_slash() {
        let name = get_name_from_url("https://github.com/deorbil/zd-cd/");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_https_without_suffix() {
        let name = get_name_from_url("https://github.com/deorbil/zd-cd");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_ssh() {
        let name = get_name_from_url("git@github.com:deorbil/zd-cd.git");
        assert_eq!(name, "zd-cd");
    }

    #[test]
    fn get_name_from_ssh_without_suffix() {
        let name = get_name_from_url("git@github.com:deorbil/zd-cd");
        assert_eq!(name, "zd-cd");
    }
}
