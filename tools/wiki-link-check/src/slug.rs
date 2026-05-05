//! GitHub anchor (slugger) algorithm.
//!
//! Mirrors the `github-slugger` Ruby/JS gem exactly:
//!
//! 1. Lowercase the header text.
//! 2. Remove every character that is not a Unicode word character, `-`,
//!    or ` `.
//! 3. Replace each ` ` with `-`.
//! 4. If the resulting slug has already appeared earlier in document order
//!    on the same page, append `-1`, `-2`, … until unique.
//!
//! This module is a value type — one [`Slugger`] per wiki page — so that
//! the dedupe counter scopes correctly.
//!
//! See [`AGENTS.md` § 6.2](../../../../AGENTS.md) for the canonical
//! specification of this algorithm.

use std::collections::HashMap;

/// Stateful slugger: tracks counts per base slug for dedupe.
pub(crate) struct Slugger {
    seen: HashMap<String, u32>,
}

impl Slugger {
    /// Construct a fresh slugger with no prior history.
    pub(crate) fn new() -> Self {
        Self {
            seen: HashMap::new(),
        }
    }

    /// Slugify `header`, returning the dedup'd anchor.
    pub(crate) fn slug(&mut self, header: &str) -> String {
        let base = base_slug(header);
        let n = self.seen.entry(base.clone()).or_insert(0);
        let result = if *n == 0 { base } else { format!("{base}-{n}") };
        *n += 1;
        result
    }
}

/// Apply the non-dedup steps of the slugger: lowercase, strip, hyphenate.
fn base_slug(s: &str) -> String {
    let lowered = s.to_lowercase();
    let kept: String = lowered
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
        .collect();
    kept.replace(' ', "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real headers harvested from
    /// `UOR-Framework.wiki/05-Building-Block-View.md`. Locking these in
    /// guards us against any future drift in the slugger algorithm.
    #[test]
    fn wiki_05_real_headers() {
        let mut s = Slugger::new();
        assert_eq!(s.slug("Building Block View"), "building-block-view");
        assert_eq!(s.slug("Whitebox Overall System"), "whitebox-overall-system");
        assert_eq!(
            s.slug("Black box description: `uor-foundation`"),
            "black-box-description-uor-foundation"
        );
        assert_eq!(
            s.slug("Black box description: `prism`"),
            "black-box-description-prism"
        );
        assert_eq!(
            s.slug("Black box description: `prism-verify`"),
            "black-box-description-prism-verify"
        );
        assert_eq!(
            s.slug("Important interfaces between containers"),
            "important-interfaces-between-containers"
        );
        assert_eq!(s.slug("Level 2"), "level-2");
        assert_eq!(
            s.slug("Whitebox: `uor-foundation`"),
            "whitebox-uor-foundation"
        );
        assert_eq!(s.slug("Whitebox: `prism`"), "whitebox-prism");
        assert_eq!(s.slug("Whitebox: `prism-verify`"), "whitebox-prism-verify");
        assert_eq!(s.slug("Level 3"), "level-3");
    }

    #[test]
    fn em_dash_yields_double_hyphen() {
        // Headers like "### Whitebox: `prism` pipeline — staged transitions"
        // contain an em-dash. The em-dash itself is non-word and stripped,
        // but the spaces flanking it survive and become hyphens — yielding
        // the characteristic `--` in the slug.
        let mut s = Slugger::new();
        assert_eq!(
            s.slug("Whitebox: `prism` pipeline — staged transitions"),
            "whitebox-prism-pipeline--staged-transitions"
        );
    }

    #[test]
    fn dedupe_collisions_are_appended() {
        let mut s = Slugger::new();
        assert_eq!(s.slug("Same"), "same");
        assert_eq!(s.slug("Same"), "same-1");
        assert_eq!(s.slug("Same"), "same-2");
        assert_eq!(s.slug("Different"), "different");
        assert_eq!(s.slug("Same"), "same-3");
    }

    #[test]
    fn punctuation_stripped() {
        let mut s = Slugger::new();
        assert_eq!(s.slug("Hello, World!"), "hello-world");
        assert_eq!(s.slug("a.b.c"), "abc");
        assert_eq!(s.slug("(1) Some Title"), "1-some-title");
    }

    #[test]
    fn underscores_and_hyphens_preserved() {
        let mut s = Slugger::new();
        assert_eq!(s.slug("snake_case"), "snake_case");
        assert_eq!(s.slug("kebab-case"), "kebab-case");
        assert_eq!(s.slug("Mix_of-both 1"), "mix_of-both-1");
    }

    #[test]
    fn unicode_letters_kept() {
        let mut s = Slugger::new();
        assert_eq!(s.slug("Über alles"), "über-alles");
    }

    #[test]
    fn empty_and_whitespace() {
        let mut s = Slugger::new();
        assert_eq!(s.slug(""), "");
        assert_eq!(s.slug("   "), "---");
    }
}
