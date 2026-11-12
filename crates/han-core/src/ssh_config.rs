use std::{
    fs, io,
    path::{Path, PathBuf},
};

const MAX_INCLUDE_DEPTH: u8 = 16;

pub fn load_hosts(path: &Path) -> io::Result<Vec<String>> {
    let home = std::env::home_dir().unwrap_or_default();
    load_recursive(path, &home, 0)
}

fn load_recursive(path: &Path, home: &Path, depth: u8) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let mut hosts = parse_hosts(&content);
    if depth < MAX_INCLUDE_DEPTH {
        for word in parse_includes(&content) {
            let include = resolve_include(&word, home);
            // OpenSSH bỏ qua file Include không đọc được
            if let Ok(more) = load_recursive(&include, home, depth + 1) {
                hosts.extend(more);
            }
        }
    }
    Ok(hosts)
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
        }
        for name in words {
            if !name.contains(['*', '?', '!']) {
                hosts.push(name.to_string());
            }
        }
    }
    hosts
}

fn resolve_include(word: &str, home: &Path) -> PathBuf {
    match word.strip_prefix("~/") {
        Some(rest) => home.join(rest),
        None => home.join(".ssh").join(word),
    }
}

fn parse_includes(content: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in content.lines() {
        let mut words = line.split_whitespace();
        let keyword = match words.next() {
            Some(word) => word,
            None => continue,
        };

        if !keyword.eq_ignore_ascii_case("include") {
            continue;
        };

        for name in words {
            result.push(name.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_host_name() {
        let input = "Host prod\n HostName 1.2.3.4\n\n Host staging\n";
        assert_eq!(parse_hosts(input), vec!["prod", "staging"]);
    }

    #[test]
    fn multiple_names_and_skip_wildcards() {
        let input = "Host *\n  User me\nhost web1 web2 *.internal\nHost !bastion db\n";
        assert_eq!(parse_hosts(input), vec!["web1", "web2", "db"]);
    }

    #[test]
    fn test_parse_includes() {
        let input = "Include a b\nHost prod\ninclude c\n";
        assert_eq!(parse_includes(input), vec!["a", "b", "c"]);
    }

    #[test]
    fn resolve_include_paths() {
        let home = Path::new("/home/u");
        assert_eq!(
            resolve_include("config.d/work", home),
            Path::new("/home/u/.ssh/config.d/work")
        );
        assert_eq!(
            resolve_include("~/other/config", home),
            Path::new("/home/u/other/config")
        );
        assert_eq!(
            resolve_include("/etc/ssh/extra", home),
            Path::new("/etc/ssh/extra")
        );
    }

    #[test]
    fn returns_error_when_file_not_found() {
        let result = load_hosts(Path::new("/khong/ton/tai"));
        assert!(result.is_err());
    }

    #[test]
    fn doc_host_tu_file_include() {
        let home = std::env::temp_dir().join("han-test-include");
        let ssh_dir = home.join(".ssh");
        fs::create_dir_all(&ssh_dir).unwrap();
        fs::write(
            ssh_dir.join("config"),
            "Host main\nInclude extra khong-ton-tai\n",
        )
        .unwrap();
        fs::write(ssh_dir.join("extra"), "Host phu\n").unwrap();

        let hosts = load_recursive(&ssh_dir.join("config"), &home, 0).unwrap();
        assert_eq!(hosts, vec!["main", "phu"]);
    }
}
