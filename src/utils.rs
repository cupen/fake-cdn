pub fn is_safe_url(s: &String) -> bool {
    if s.contains("..") {
        return false;
    }
    let is_bad = s.chars().any(|c| {
        if c.is_control() {
            return true;
        }
        if c.is_ascii_whitespace() {
            return true;
        }
        if c.is_ascii_punctuation() {
            return match c {
                '_' | '-' | '/' | '.' => return false,
                _ => true
            }
        }   
        return false
    });
    return !is_bad;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_paths() {
        assert!(is_safe_url(&"folder/file".to_string()));
        assert!(is_safe_url(&"abc123".to_string()));
        assert!(is_safe_url(&"folder_subfolder_file".to_string()));
    }

    #[test]
    fn test_paths_with_dotdot() {
        assert!(!is_safe_url(&"../etc/passwd".to_string()));
        assert!(!is_safe_url(&"folder/../file".to_string()));
        assert!(!is_safe_url(&"..".to_string()));
    }

    #[test]
    fn test_paths_with_control_chars() {
        assert!(!is_safe_url(&"file\nname".to_string()));
        assert!(!is_safe_url(&"file\tname".to_string()));
        assert!(!is_safe_url(&"file\x00name".to_string()));
    }

    #[test]
    fn test_paths_with_punctuation() {
        assert!(!is_safe_url(&"file:name".to_string()));
        assert!(!is_safe_url(&"file|name".to_string()));
        assert!(!is_safe_url(&"file*name".to_string()));
        assert!(!is_safe_url(&"file?name".to_string()));
        assert!(!is_safe_url(&"file<name>".to_string()));
        assert!(is_safe_url(&"file/name".to_string())); // '/' is punctuation
    }

    #[test]
    fn test_paths_with_whitespace() {
        assert!(!is_safe_url(&"file name".to_string()));
        assert!(!is_safe_url(&" file".to_string()));
        assert!(!is_safe_url(&"file ".to_string()));
    }

    #[test]
    fn test_empty_string() {
        assert!(is_safe_url(&"".to_string()));
    }
}


pub fn file_extension(path: &str) -> Option<&str> {
    if let Some(pos) = path.rfind('.') {
        if pos < path.len() - 1 {
            return Some(&path[pos + 1..]);
        }
    }
    None
}