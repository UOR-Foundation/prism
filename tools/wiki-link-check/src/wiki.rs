//! Wiki source repo acquisition and validation.
//!
//! [`Wiki`] is a snapshot of the cloned wiki tree: for each `*.md` file
//! at its root, every ATX header is parsed and slugified into the page's
//! anchor list. [`Wiki::validate`] then answers whether a given
//! `(page, anchor)` pair exists in the snapshot.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::slug::Slugger;

/// Snapshot of a cloned wiki tree.
pub(crate) struct Wiki {
    path: PathBuf,
    /// Page name (file stem of `<page>.md`) → anchors in document order.
    pages: BTreeMap<String, Vec<String>>,
}

impl Wiki {
    /// Open a previously cloned wiki at `path` and parse its pages.
    pub(crate) fn open(path: &Path) -> io::Result<Self> {
        let mut pages = BTreeMap::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let stem = match p.file_stem().and_then(|s| s.to_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };
            let content = fs::read_to_string(&p)?;
            pages.insert(stem, parse_anchors(&content));
        }
        Ok(Self {
            path: path.to_path_buf(),
            pages,
        })
    }

    /// Path to the wiki tree on disk.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// Verify that `page` exists in the snapshot, and (if `anchor` is
    /// `Some`) that the slug is present on that page.
    pub(crate) fn validate(&self, page: &str, anchor: Option<&str>) -> Result<(), Mismatch> {
        let anchors = self.pages.get(page).ok_or_else(|| Mismatch::PageMissing {
            page: page.to_string(),
            available: self.pages.keys().cloned().collect(),
        })?;
        if let Some(a) = anchor {
            if !anchors.iter().any(|x| x == a) {
                return Err(Mismatch::AnchorMissing {
                    page: page.to_string(),
                    anchor: a.to_string(),
                    available: anchors.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Outcome of a failed [`Wiki::validate`] call.
#[derive(Debug)]
pub(crate) enum Mismatch {
    PageMissing {
        page: String,
        available: Vec<String>,
    },
    AnchorMissing {
        page: String,
        anchor: String,
        available: Vec<String>,
    },
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PageMissing { page, available } => {
                let near = nearest(page, available, 3);
                write!(
                    f,
                    "page `{page}.md` not found in wiki source. Closest pages: {}",
                    fmt_list(&near)
                )
            }
            Self::AnchorMissing {
                page,
                anchor,
                available,
            } => {
                let near = nearest(anchor, available, 5);
                write!(
                    f,
                    "anchor `#{anchor}` not found on page `{page}.md`. Closest anchors: {}",
                    fmt_list(&near)
                )
            }
        }
    }
}

impl std::error::Error for Mismatch {}

fn fmt_list(items: &[String]) -> String {
    if items.is_empty() {
        "<none>".to_string()
    } else {
        items
            .iter()
            .map(|s| format!("`{s}`"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Nearest `n` items to `target` from `pool` by Levenshtein distance.
fn nearest(target: &str, pool: &[String], n: usize) -> Vec<String> {
    let mut scored: Vec<(usize, &str)> = pool
        .iter()
        .map(|s| (levenshtein(target, s), s.as_str()))
        .collect();
    scored.sort_by_key(|(d, _)| *d);
    scored
        .into_iter()
        .take(n)
        .map(|(_, s)| s.to_string())
        .collect()
}

/// Standard iterative Levenshtein distance (single-row buffer).
fn levenshtein(a: &str, b: &str) -> usize {
    if a == b {
        return 0;
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr: Vec<usize> = vec![0; b.len() + 1];
    for i in 1..=a.len() {
        curr[0] = i;
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            curr[j] = (curr[j - 1] + 1).min(prev[j] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b.len()]
}

/// Parse a markdown document and return every ATX header anchor in order,
/// skipping headers inside fenced code blocks.
fn parse_anchors(md: &str) -> Vec<String> {
    let mut slugger = Slugger::new();
    let mut anchors = Vec::new();
    let mut in_fence = false;
    let mut fence_marker: char = '`';
    for line in md.lines() {
        let trimmed = line.trim_start();
        // Toggle fenced code block on ``` or ~~~
        if let Some(c) = fence_char(trimmed) {
            if !in_fence {
                in_fence = true;
                fence_marker = c;
            } else if c == fence_marker {
                in_fence = false;
            }
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some(text) = atx_header_text(trimmed) {
            anchors.push(slugger.slug(text));
        }
    }
    anchors
}

fn fence_char(line: &str) -> Option<char> {
    let bytes = line.as_bytes();
    if bytes.starts_with(b"```") {
        Some('`')
    } else if bytes.starts_with(b"~~~") {
        Some('~')
    } else {
        None
    }
}

fn atx_header_text(line: &str) -> Option<&str> {
    let bytes = line.as_bytes();
    let mut depth = 0;
    while depth < 6 && bytes.get(depth) == Some(&b'#') {
        depth += 1;
    }
    if depth == 0 || bytes.get(depth) != Some(&b' ') {
        return None;
    }
    // Strip the leading hashes + space and any trailing closing hashes
    // and whitespace, per CommonMark ATX header semantics.
    let after = &line[depth + 1..];
    let trimmed = after.trim_end();
    let trimmed = trimmed.trim_end_matches('#').trim_end();
    Some(trimmed)
}

/// Clone the wiki to `dest`, or update it if already cloned.
pub(crate) fn clone_or_update(dest: &Path, repo: &str, rev: Option<&str>) -> io::Result<()> {
    if dest.join(".git").is_dir() {
        let st = Command::new("git")
            .arg("-C")
            .arg(dest)
            .args(["fetch", "--quiet", "origin"])
            .status()?;
        if !st.success() {
            return Err(io::Error::other(format!(
                "git fetch failed in {}",
                dest.display()
            )));
        }
    } else {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let st = Command::new("git")
            .args(["clone", "--quiet", repo])
            .arg(dest)
            .status()?;
        if !st.success() {
            return Err(io::Error::other(format!(
                "git clone {repo} into {} failed",
                dest.display()
            )));
        }
    }
    let rev = rev.unwrap_or("origin/master");
    let st = Command::new("git")
        .arg("-C")
        .arg(dest)
        .args(["checkout", "--quiet", "--detach", rev])
        .status()?;
    if !st.success() {
        return Err(io::Error::other(format!("git checkout {rev} failed")));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn temp_dir(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "wiki-link-check-wiki-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    fn write(p: &Path, s: &str) {
        let mut f = File::create(p).unwrap();
        f.write_all(s.as_bytes()).unwrap();
    }

    #[test]
    fn anchors_from_atx_headers() {
        let md = "# Top\n\
                  \n\
                  ## Sub one\n\
                  \n\
                  ### Whitebox: `prism`\n\
                  \n\
                  text\n\
                  \n\
                  ## Sub one\n";
        let a = parse_anchors(md);
        assert_eq!(a, vec!["top", "sub-one", "whitebox-prism", "sub-one-1"]);
    }

    #[test]
    fn ignores_headers_inside_code_fences() {
        let md = "# Real\n\
                  \n\
                  ```\n\
                  ## Not a header\n\
                  ```\n\
                  \n\
                  ## Real again\n";
        let a = parse_anchors(md);
        assert_eq!(a, vec!["real", "real-again"]);
    }

    #[test]
    fn open_and_validate_round_trip() {
        let dir = temp_dir("open");
        write(
            &dir.join("05-Building-Block-View.md"),
            "# Building Block View\n\n## Whitebox Overall System\n\n### Whitebox: `prism`\n",
        );
        write(
            &dir.join("12-Glossary.md"),
            "# Glossary\n\n## Term Definitions\n",
        );
        let w = Wiki::open(&dir).unwrap();

        // Page-only: ok
        w.validate("05-Building-Block-View", None).unwrap();
        // Anchor present: ok
        w.validate("05-Building-Block-View", Some("whitebox-prism"))
            .unwrap();
        // Anchor absent on existing page: AnchorMissing
        let e = w
            .validate("05-Building-Block-View", Some("nope"))
            .unwrap_err();
        assert!(matches!(e, Mismatch::AnchorMissing { .. }));
        // Page absent: PageMissing
        let e = w.validate("99-Nonexistent", None).unwrap_err();
        assert!(matches!(e, Mismatch::PageMissing { .. }));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn levenshtein_basic() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("flaw", "lawn"), 2);
        assert_eq!(levenshtein("same", "same"), 0);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
    }

    #[test]
    fn nearest_picks_closest() {
        let pool: Vec<String> = ["whitebox-prism", "whitebox-prism-verify", "level-2"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        let n = nearest("whitebox-prsm", &pool, 1);
        assert_eq!(n, vec!["whitebox-prism"]);
    }

    #[test]
    fn atx_header_text_basics() {
        assert_eq!(atx_header_text("# Hello"), Some("Hello"));
        assert_eq!(atx_header_text("### Foo bar"), Some("Foo bar"));
        assert_eq!(atx_header_text("### Foo bar ###"), Some("Foo bar"));
        assert_eq!(atx_header_text("####### too deep"), None);
        assert_eq!(atx_header_text("#no space"), None);
        assert_eq!(atx_header_text("plain text"), None);
    }
}
