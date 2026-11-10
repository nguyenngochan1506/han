use std::{fs, io, path::Path};

pub fn load_hosts(path: &Path) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(parse_hosts(&content))
}

pub fn parse_hosts(content: &str) -> Vec<String> {
    let mut hosts = Vec::new();

    for line in content.lines() {
        let mut words = line.split_whitespace();
        let keyword = match words.next() {
            Some(word) => word,
            None => continue,
        };
        if !keyword.eq_ignore_ascii_case("host") {
            continue;
        };
        for name in words {
            if !name.contains(['*', '?', '!']) {
                hosts.push(name.to_string());
            }
        }
    }
    hosts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_host_name() {
        let input = "Host prod\n HostName 1.2.3.4\n\n Host staging\n";
        assert_eq!(parse_hosts(input), vec!["prod", "staging"])
    }

    #[test]
    fn multiple_names_and_skip_wildcards() {
        let input = "Host *\n  User me\nhost web1 web2 *.internal\nHost !bastion db\n";
        assert_eq!(parse_hosts(input), vec!["web1", "web2", "db"]);
    }

    #[test]
    fn returns_error_when_file_not_found() {
        let result = load_hosts(Path::new("/khong/ton/tai"));
        assert!(result.is_err());
    }
}
