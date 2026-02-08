use lsp_types::Uri;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Extension trait for Uri to provide file path conversion methods
/// that were removed when lsp-types migrated from url::Url to fluent_uri::Uri
pub trait UriExt {
    /// Convert URI to file path
    fn to_file_path(&self) -> Option<PathBuf>;
}

impl UriExt for Uri {
    fn to_file_path(&self) -> Option<PathBuf> {
        // Check if this is a file:// URI
        if !self.as_str().starts_with("file://") {
            return None;
        }

        let path_str = self.path().as_str();
        
        #[cfg(windows)]
        {
            // On Windows, file URLs have the form file:///C:/path or file://host/path
            if path_str.starts_with('/') && path_str.len() > 2 {
                // Remove leading slash for absolute Windows paths like /C:/path
                let without_slash = &path_str[1..];
                // Check if this is a drive letter path (e.g., C:/path)
                if without_slash.len() >= 2 
                    && without_slash.chars().nth(0).map(|c| c.is_ascii_alphabetic()).unwrap_or(false)
                    && without_slash.chars().nth(1) == Some(':') 
                {
                    return Some(PathBuf::from(percent_decode_path(without_slash)));
                }
            }
            // Try as-is for UNC paths or other formats
            Some(PathBuf::from(percent_decode_path(path_str)))
        }

        #[cfg(not(windows))]
        {
            // On Unix, the path component is the file path
            Some(PathBuf::from(percent_decode_path(path_str)))
        }
    }
}

/// Convert a file path to a URI
pub fn uri_from_file_path(path: &Path) -> Option<Uri> {
    let path = if path.is_absolute() {
        Cow::Borrowed(path)
    } else {
        Cow::Owned(std::fs::canonicalize(path).ok()?)
    };

    #[cfg(windows)]
    {
        // On Windows, use file:/// format
        let path_str = percent_encode_path(&path.to_string_lossy().replace('\\', "/"));
        let uri_str = format!("file:///{}", path_str);
        Uri::from_str(&uri_str).ok()
    }

    #[cfg(not(windows))]
    {
        let path_str = percent_encode_path(&path.to_string_lossy());
        let uri_str = format!("file://{}", path_str);
        Uri::from_str(&uri_str).ok()
    }
}

/// Simple percent-decode for URI path components
/// Properly handles UTF-8 multi-byte sequences
fn percent_decode_path(s: &str) -> String {
    let mut bytes = Vec::new();
    let mut chars = s.chars();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            // Try to parse the next two hex digits
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    bytes.push(byte);
                    continue;
                }
            }
            // If parsing failed, just add the % and the chars we consumed
            bytes.extend_from_slice(b"%");
            bytes.extend_from_slice(hex.as_bytes());
        } else {
            // Regular ASCII character - can convert directly
            if c.is_ascii() {
                bytes.push(c as u8);
            } else {
                // Non-ASCII character - encode as UTF-8
                let mut buf = [0u8; 4];
                let str_slice = c.encode_utf8(&mut buf);
                bytes.extend_from_slice(str_slice.as_bytes());
            }
        }
    }
    
    // Convert accumulated bytes to String, replacing invalid UTF-8 with �
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Simple percent-encode for URI path components
/// Encodes special characters except for /
fn percent_encode_path(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    
    for byte in s.as_bytes() {
        match byte {
            // Unreserved characters (RFC 3986)
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                result.push(*byte as char);
            }
            // Percent-encode everything else
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    
    result
}
