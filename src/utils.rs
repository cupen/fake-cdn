pub fn is_safe_path(s: &String) -> bool {
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
                _ => true,
            };
        }
        return false;
    });
    return !is_bad;
}

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
                _ => true,
            };
        }
        return false;
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

// 生成测试用例
#[test]
fn test_file_extension() {
    assert_eq!(file_extension("file.txt"), Some("txt"));
    assert_eq!(file_extension("file"), None);
    assert_eq!(file_extension("file."), None);
    assert_eq!(file_extension(".file"), Some("file"));
    assert_eq!(file_extension("abc.tar.gz"), Some("tar.gz"));
    assert_eq!(file_extension("abc.tgz"), Some("tgz"));
    assert_eq!(file_extension("abc.zip"), Some("zip"));
    assert_eq!(file_extension("abc.7z"), Some("7z"));
}

use rand::{thread_rng, Rng};

const ALPHANUM_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

const TOKEN_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@$%&*+";

pub fn generate_token(length: usize) -> String {
    if length == 0 {
        return String::new();
    }

    let mut rng = thread_rng();

    // Handle short tokens where start/end rule is not applicable
    if length < 2 {
        let idx = rng.gen_range(0..ALPHANUM_CHARSET.len());
        return (ALPHANUM_CHARSET[idx] as char).to_string();
    }

    // First character must be alphanumeric
    let first_char_idx = rng.gen_range(0..ALPHANUM_CHARSET.len());
    let mut token = String::with_capacity(length);
    token.push(ALPHANUM_CHARSET[first_char_idx] as char);

    // Middle characters can be anything from the full charset
    for _ in 1..length - 1 {
        let idx = rng.gen_range(0..TOKEN_CHARSET.len());
        token.push(TOKEN_CHARSET[idx] as char);
    }

    // Last character must be alphanumeric
    let last_char_idx = rng.gen_range(0..ALPHANUM_CHARSET.len());
    token.push(ALPHANUM_CHARSET[last_char_idx] as char);

    token
}
