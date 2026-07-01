#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Returns a friendly greeting.
///
/// # Examples
///
/// ```
/// let message = rdgnru_lib::greet("Rust");
/// assert_eq!(message, "Hello, Rust!");
/// ```
#[must_use]
pub fn greet(name: impl AsRef<str>) -> String {
    let trimmed = name.as_ref().trim();

    if trimmed.is_empty() {
        "Hello!".to_owned()
    } else {
        format!("Hello, {trimmed}!")
    }
}

/// Converts text into a simple ASCII URL slug.
///
/// Non-ASCII characters and punctuation are treated as separators or removed.
/// Consecutive separators collapse into one dash.
///
/// # Examples
///
/// ```
/// let slug = rdgnru_lib::slugify("Hello, Rust World!");
/// assert_eq!(slug, "hello-rust-world");
/// ```
#[must_use]
pub fn slugify(input: impl AsRef<str>) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;

    for ch in input.as_ref().chars() {
        let ch = ch.to_ascii_lowercase();

        if ch.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(ch);
            pending_dash = false;
        } else if !slug.is_empty() {
            pending_dash = true;
        }
    }

    slug
}

/// Checks whether a string is a valid slug produced by [`slugify`].
///
/// Valid slugs contain only lowercase ASCII letters, digits, and single dashes,
/// and they do not begin or end with a dash.
///
/// # Examples
///
/// ```
/// assert!(rdgnru_lib::is_valid_slug("hello-rust-world"));
/// assert!(!rdgnru_lib::is_valid_slug("Hello Rust"));
/// ```
#[must_use]
pub fn is_valid_slug(input: impl AsRef<str>) -> bool {
    let bytes = input.as_ref().as_bytes();

    !bytes.is_empty()
        && bytes[0] != b'-'
        && bytes[bytes.len() - 1] != b'-'
        && bytes
            .iter()
            .all(|byte| matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'-'))
        && !bytes.windows(2).any(|window| window == b"--")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_uses_trimmed_name() {
        assert_eq!(greet(" Rust "), "Hello, Rust!");
    }

    #[test]
    fn greet_handles_blank_name() {
        assert_eq!(greet("   "), "Hello!");
    }

    #[test]
    fn slugify_collapses_separators() {
        assert_eq!(slugify("Hello, Rust---World!"), "hello-rust-world");
    }

    #[test]
    fn slugify_trims_edge_separators() {
        assert_eq!(slugify("---Hello Rust---"), "hello-rust");
    }

    #[test]
    fn valid_slug_accepts_expected_format() {
        assert!(is_valid_slug("hello-rust-2026"));
    }

    #[test]
    fn valid_slug_rejects_invalid_format() {
        assert!(!is_valid_slug("hello--rust"));
        assert!(!is_valid_slug("hello-rust-"));
        assert!(!is_valid_slug("Hello Rust"));
        assert!(!is_valid_slug(""));
    }
}
