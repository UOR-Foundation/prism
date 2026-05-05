//! Scan source/markdown/manifest files for UOR-Framework wiki URLs.
//!
//! Pure-std walker: descends from a root directory, skips `target/`,
//! `.git/`, `.cache/`, and `node_modules/`, and extracts every URL that
//! matches the wiki prefix from `*.rs`, `*.md`, and `*.toml` files. The
//! extractor parses each URL into its page-name and anchor components so
//! the validator can check both independently.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Prefix of every UOR-Framework wiki URL.
pub(crate) const WIKI_PREFIX: &str = "https://github.com/UOR-Foundation/UOR-Framework/wiki/";

/// Directory names skipped by default during the walk.
const DEFAULT_SKIPS: &[&str] = &["target", ".git", ".cache", "node_modules"];

/// File extensions scanned for wiki URLs.
const SCANNABLE_EXTS: &[&str] = &["rs", "md", "toml", "yml", "yaml"];

/// One occurrence of a wiki URL in the scanned tree.
#[derive(Debug, Clone)]
pub(crate) struct Found {
    pub(crate) file: PathBuf,
    pub(crate) line: usize,
    pub(crate) url: String,
    pub(crate) page: String,
    pub(crate) anchor: Option<String>,
}

/// Walk `root` and return every wiki URL found in scannable files.
pub(crate) fn collect(root: &Path) -> io::Result<Vec<Found>> {
    let mut paths = Vec::new();
    walk(root, &mut paths)?;
    let mut out = Vec::new();
    for path in paths {
        if !is_scannable(&path) {
            continue;
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) if e.kind() == io::ErrorKind::InvalidData => continue, // non-utf8 file
            Err(e) => return Err(e),
        };
        for (idx, line) in content.lines().enumerate() {
            extract_into(line, &path, idx + 1, &mut out);
        }
    }
    Ok(out)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if file_type.is_dir() {
            if DEFAULT_SKIPS.iter().any(|s| name_str == *s) {
                continue;
            }
            walk(&path, out)?;
        } else if file_type.is_file() {
            out.push(path);
        }
    }
    Ok(())
}

fn is_scannable(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| SCANNABLE_EXTS.iter().any(|x| x.eq_ignore_ascii_case(e)))
}

fn extract_into(line: &str, file: &Path, line_no: usize, out: &mut Vec<Found>) {
    let mut cursor = 0;
    while let Some(rel) = line[cursor..].find(WIKI_PREFIX) {
        let abs_start = cursor + rel;
        let from = &line[abs_start..];
        let end_rel = url_terminator(from).unwrap_or(from.len());
        let url = &from[..end_rel];
        let after_prefix = &url[WIKI_PREFIX.len()..];
        if !after_prefix.is_empty() {
            let (page, anchor) = match after_prefix.split_once('#') {
                Some((p, a)) => (p.to_string(), Some(a.to_string())),
                None => (after_prefix.to_string(), None),
            };
            // strip a trailing `.` or `,` that often follows a URL in prose
            let (page, anchor) = trim_trailing_prose(page, anchor);
            if !page.is_empty() {
                out.push(Found {
                    file: file.to_path_buf(),
                    line: line_no,
                    url: url.to_string(),
                    page,
                    anchor,
                });
            }
        }
        cursor = abs_start + end_rel.max(1);
    }
}

/// Position of the first character that ends a URL in `s`.
///
/// URLs are terminated by whitespace or by characters that commonly close
/// markdown link syntax, comments, or string literals: `)`, `]`, `>`, `<`,
/// `"`, `'`, backtick, comma, and the brace pair. The angle brackets are
/// included in both directions so that placeholder syntax such as
/// `wiki/<page>(#<anchor>)?` in documentation does not yield bogus pages.
fn url_terminator(s: &str) -> Option<usize> {
    s.find(|c: char| {
        c.is_whitespace()
            || matches!(
                c,
                ')' | ']' | '>' | '<' | '"' | '\'' | '`' | ',' | '{' | '}'
            )
    })
}

/// URL parsers commonly include a trailing `.` from a sentence-final
/// period, or `,` from a list separator. Strip those off the slug we
/// validate.
fn trim_trailing_prose(mut page: String, mut anchor: Option<String>) -> (String, Option<String>) {
    fn strip(s: &mut String) {
        while matches!(s.chars().last(), Some('.' | ',' | ';' | ':')) {
            s.pop();
        }
    }
    if let Some(a) = anchor.as_mut() {
        strip(a);
    } else {
        strip(&mut page);
    }
    (page, anchor)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn temp_dir(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "wiki-link-check-test-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn extracts_simple_url() {
        let mut out = vec![];
        extract_into(
            "see https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View",
            Path::new("x.rs"),
            1,
            &mut out,
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].page, "05-Building-Block-View");
        assert!(out[0].anchor.is_none());
    }

    #[test]
    fn extracts_url_with_anchor() {
        let mut out = vec![];
        extract_into(
            "[link](https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View#whitebox-prism)",
            Path::new("x.md"),
            1,
            &mut out,
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].page, "05-Building-Block-View");
        assert_eq!(out[0].anchor.as_deref(), Some("whitebox-prism"));
    }

    #[test]
    fn extracts_multiple_per_line() {
        // Test fixtures use real wiki page names + anchors so this very
        // file remains valid under wiki-link-check.
        let mut out = vec![];
        extract_into(
            "https://github.com/UOR-Foundation/UOR-Framework/wiki/01-Introduction-and-Goals and https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary#term-definitions",
            Path::new("x.md"),
            7,
            &mut out,
        );
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].page, "01-Introduction-and-Goals");
        assert_eq!(out[1].page, "12-Glossary");
        assert_eq!(out[1].anchor.as_deref(), Some("term-definitions"));
    }

    #[test]
    fn ignores_trailing_punctuation() {
        let mut out = vec![];
        extract_into(
            "see https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary.",
            Path::new("x.md"),
            1,
            &mut out,
        );
        assert_eq!(out[0].page, "12-Glossary");
    }

    #[test]
    fn ignores_other_urls() {
        let mut out = vec![];
        // Neither URL matches the UOR-Framework wiki prefix, so neither
        // should be emitted by the scanner.
        extract_into(
            "https://example.com/somewhere https://github.com/other-org/other-repo/wiki/page",
            Path::new("x.rs"),
            1,
            &mut out,
        );
        assert!(out.is_empty());
    }

    #[test]
    fn empty_page_after_prefix_is_skipped() {
        let mut out = vec![];
        extract_into(
            "https://github.com/UOR-Foundation/UOR-Framework/wiki/ ",
            Path::new("x.rs"),
            1,
            &mut out,
        );
        assert!(out.is_empty());
    }

    #[test]
    fn walk_skips_target_and_dotgit() {
        // Real wiki page names so this source file is valid under
        // wiki-link-check; the test only cares that two files are skipped
        // and one survives.
        let dir = temp_dir("walk-skip");
        fs::create_dir_all(dir.join("target/sub")).unwrap();
        fs::create_dir_all(dir.join(".git/sub")).unwrap();
        fs::create_dir_all(dir.join("src")).unwrap();
        let mut f = fs::File::create(dir.join("target/sub/should_skip.rs")).unwrap();
        writeln!(
            f,
            "https://github.com/UOR-Foundation/UOR-Framework/wiki/01-Introduction-and-Goals"
        )
        .unwrap();
        let mut f = fs::File::create(dir.join(".git/sub/should_skip.md")).unwrap();
        writeln!(
            f,
            "https://github.com/UOR-Foundation/UOR-Framework/wiki/05-Building-Block-View"
        )
        .unwrap();
        let mut f = fs::File::create(dir.join("src/keep.rs")).unwrap();
        writeln!(
            f,
            "https://github.com/UOR-Foundation/UOR-Framework/wiki/12-Glossary"
        )
        .unwrap();
        let founds = collect(&dir).unwrap();
        let pages: Vec<_> = founds.iter().map(|f| f.page.as_str()).collect();
        assert_eq!(pages, vec!["12-Glossary"]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn scannable_extensions() {
        assert!(is_scannable(Path::new("a.rs")));
        assert!(is_scannable(Path::new("a.md")));
        assert!(is_scannable(Path::new("Cargo.toml")));
        assert!(is_scannable(Path::new("ci.yml")));
        assert!(!is_scannable(Path::new("a.png")));
        assert!(!is_scannable(Path::new("Cargo.lock")));
    }
}
