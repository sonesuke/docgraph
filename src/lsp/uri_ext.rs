use lsp_types::Uri;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

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
        let path_str = path.to_string_lossy().replace('\\', "/");
        let uri_str = format!("file:///{}", path_str);
        Uri::from_str(&uri_str).ok()
    }

    #[cfg(not(windows))]
    {
        let uri_str = format!("file://{}", path.to_string_lossy());
        Uri::from_str(&uri_str).ok()
    }
}

/// Simple percent-decode for URI path components
fn percent_decode_path(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            // Try to parse the next two hex digits
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                    continue;
                }
            }
            // If parsing failed, just add the % and the chars we consumed
            result.push('%');
            result.push_str(&hex);
        } else {
            result.push(c);
        }
    }
    
    result
}

use std::str::FromStr;
